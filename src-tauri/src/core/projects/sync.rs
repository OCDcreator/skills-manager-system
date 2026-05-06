use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::core::agents::catalog::{find_agent, project_skills_dir_rule};
use crate::core::agents::discovery::{load_agent_inventory, AgentInventoryItem, AgentSystemDirs};
use crate::core::agents::selection::{
    load_skill_selection_context, resolve_agent_skill_selection,
    resolve_project_agent_skill_selection, SkillResolutionDiagnostics, SkillSelectionContext,
};
use crate::core::agents::sync::AgentApplyResult;
use crate::core::agents::target_sync::{
    apply_desired_entries, build_desired_skill_entries, cleanup_managed_entries, DesiredSkillEntry,
    SyncMode,
};
use crate::core::platform_paths::portable_path_string;
use crate::core::settings::{AgentSyncMode, SettingsStore};

use super::store::{ProjectAgentAssignment, ProjectAssignment, ProjectConfigStore};

const PROJECT_LEDGER_FILE_NAME: &str = "project-sync-ledger.json";

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ProjectSyncLedger {
    assignments: BTreeMap<String, ProjectSyncLedgerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ProjectSyncLedgerEntry {
    agent_key: String,
    target_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectAssignmentApplyResult {
    pub project_path: String,
    pub display_name: String,
    pub enabled_skill_count: usize,
    pub results: Vec<AgentApplyResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApplyProjectAssignmentsResponse {
    pub project_count: usize,
    pub results: Vec<ProjectAssignmentApplyResult>,
}

pub fn apply_project_assignments(
    config_dir: &Path,
    repo_path: &Path,
    system_dirs: &AgentSystemDirs,
) -> Result<ApplyProjectAssignmentsResponse> {
    let snapshot = ProjectConfigStore::new(config_dir.to_path_buf()).load()?;
    let inventory = load_agent_inventory(config_dir, system_dirs)?;
    let agents = inventory
        .agents
        .into_iter()
        .map(|agent| (agent.key.clone(), agent))
        .collect::<BTreeMap<_, _>>();
    let skill_context = load_skill_selection_context(config_dir, repo_path)?;
    let sync_mode = load_project_sync_mode(config_dir)?;
    let mut ledger = load_ledger(config_dir)?;
    let mut managed_keys = BTreeSet::new();
    let mut results = Vec::new();

    for project in snapshot.projects.values() {
        let result = apply_for_project(
            project,
            &agents,
            &skill_context,
            sync_mode,
            &mut ledger,
            &mut managed_keys,
        )?;
        results.push(result);
    }

    cleanup_stale_assignments(&mut ledger, &managed_keys)?;
    save_ledger(config_dir, &ledger)?;

    Ok(ApplyProjectAssignmentsResponse {
        project_count: snapshot.projects.len(),
        results,
    })
}

fn apply_for_project(
    project: &ProjectAssignment,
    agents: &BTreeMap<String, AgentInventoryItem>,
    skill_context: &SkillSelectionContext,
    sync_mode: SyncMode,
    ledger: &mut ProjectSyncLedger,
    managed_keys: &mut BTreeSet<String>,
) -> Result<ProjectAssignmentApplyResult> {
    let mut results = Vec::new();
    let mut enabled_skill_ids = BTreeSet::new();

    cleanup_retargeted_assignments(project, ledger)?;

    for agent_key in &project.agent_keys {
        let key = ledger_key(&project.project_path, agent_key);
        managed_keys.insert(key.clone());

        let result = match find_agent(agent_key) {
            Some(agent) => {
                let Some(global_agent) = agents.get(agent_key) else {
                    ledger.assignments.remove(&key);
                    results.push(unsupported_agent_result(agent_key));
                    continue;
                };
                let default_project_agent;
                let project_agent = match project.agents.get(agent_key) {
                    Some(project_agent) => project_agent,
                    None => {
                        default_project_agent = ProjectAgentAssignment::default();
                        &default_project_agent
                    }
                };
                let global_result = resolve_agent_skill_selection(global_agent, skill_context);
                let project_result = resolve_project_agent_skill_selection(
                    &global_result,
                    project_agent,
                    skill_context,
                );
                let enabled_skills = project_result
                    .entries
                    .into_iter()
                    .filter(|entry| !entry.excluded && !entry.globally_disabled)
                    .map(|entry| entry.skill)
                    .collect::<Vec<_>>();
                enabled_skill_ids.extend(enabled_skills.iter().map(|skill| skill.id.clone()));
                let desired_entries = build_desired_skill_entries(&enabled_skills);
                let target_dir =
                    Path::new(&project.project_path).join(project_skills_dir_rule(agent));
                let stats =
                    apply_desired_entries(&target_dir, agent_key, &desired_entries, sync_mode)?;
                let warnings = format_diagnostics(&project_result.diagnostics);
                ledger.assignments.insert(
                    key,
                    ProjectSyncLedgerEntry {
                        agent_key: agent_key.clone(),
                        target_dir: portable_path_string(&target_dir),
                    },
                );

                AgentApplyResult {
                    key: agent_key.clone(),
                    display_name: agent.display_name.to_string(),
                    target_dir: Some(portable_path_string(&target_dir)),
                    status: if stats.conflict_count > 0 || !warnings.is_empty() {
                        crate::core::agents::sync::AgentApplyStatus::Partial
                    } else {
                        crate::core::agents::sync::AgentApplyStatus::Success
                    },
                    written_count: stats.written_count,
                    removed_count: stats.removed_count,
                    conflict_count: stats.conflict_count,
                    message: build_message(
                        &desired_entries,
                        stats.conflict_count,
                        stats.written_count,
                        &warnings,
                    ),
                }
            }
            None => {
                ledger.assignments.remove(&key);
                unsupported_agent_result(agent_key)
            }
        };

        results.push(result);
    }

    Ok(ProjectAssignmentApplyResult {
        project_path: project.project_path.clone(),
        display_name: project.display_name.clone(),
        enabled_skill_count: enabled_skill_ids.len(),
        results,
    })
}

fn cleanup_retargeted_assignments(
    project: &ProjectAssignment,
    ledger: &mut ProjectSyncLedger,
) -> Result<()> {
    for agent_key in &project.agent_keys {
        let key = ledger_key(&project.project_path, agent_key);
        let Some(agent) = find_agent(agent_key) else {
            continue;
        };
        let new_target_dir = Path::new(&project.project_path).join(project_skills_dir_rule(agent));
        let new_target = portable_path_string(&new_target_dir);
        let Some(previous) = ledger.assignments.get(&key) else {
            continue;
        };
        if normalize_legacy_target_for_comparison(&previous.target_dir) == new_target {
            continue;
        }

        cleanup_managed_entries(Path::new(&previous.target_dir), &previous.agent_key)?;
        ledger.assignments.remove(&key);
    }

    Ok(())
}

fn cleanup_stale_assignments(
    ledger: &mut ProjectSyncLedger,
    managed_keys: &BTreeSet<String>,
) -> Result<()> {
    let stale_keys: Vec<String> = ledger
        .assignments
        .keys()
        .filter(|key| !managed_keys.contains(*key))
        .cloned()
        .collect();

    for key in stale_keys {
        if let Some(entry) = ledger.assignments.remove(&key) {
            cleanup_managed_entries(Path::new(&entry.target_dir), &entry.agent_key)?;
        }
    }

    Ok(())
}

fn unsupported_agent_result(agent_key: &str) -> AgentApplyResult {
    AgentApplyResult {
        key: agent_key.to_string(),
        display_name: agent_key.to_string(),
        target_dir: None,
        status: crate::core::agents::sync::AgentApplyStatus::Failed,
        written_count: 0,
        removed_count: 0,
        conflict_count: 0,
        message: format!("Unsupported agent key '{}'.", agent_key),
    }
}

fn load_project_sync_mode(config_dir: &Path) -> Result<SyncMode> {
    match SettingsStore::new(config_dir.to_path_buf())
        .load()?
        .agent_sync_mode
    {
        AgentSyncMode::Copy => Ok(SyncMode::Copy),
        AgentSyncMode::Symlink => Ok(SyncMode::Symlink),
    }
}

fn build_message(
    desired_entries: &BTreeMap<String, DesiredSkillEntry>,
    conflict_count: usize,
    written_count: usize,
    warnings: &[String],
) -> String {
    let message = if conflict_count > 0 {
        format!(
            "Applied {} skill(s) with {} unmanaged conflict(s).",
            written_count, conflict_count
        )
    } else if desired_entries.is_empty() {
        "No selected skills remain; cleaned managed entries.".to_string()
    } else {
        format!("Applied {} skill(s).", written_count)
    };
    if warnings.is_empty() { message } else { format!("{message} Warnings: {}", warnings.join("; ")) }
}

fn format_diagnostics(diagnostics: &SkillResolutionDiagnostics) -> Vec<String> {
    let mut warnings = Vec::new();
    push_warning(&mut warnings, "missing scenes", &diagnostics.missing_scene_ids);
    push_warning(&mut warnings, "missing skills", &diagnostics.missing_skill_ids);
    push_warning(&mut warnings, "globally disabled", &diagnostics.globally_disabled_references);
    warnings
}

fn push_warning(warnings: &mut Vec<String>, label: &str, ids: &[String]) {
    if !ids.is_empty() { warnings.push(format!("{label}: {}", ids.join(", "))); }
}

fn load_ledger(config_dir: &Path) -> Result<ProjectSyncLedger> {
    let path = config_dir.join(PROJECT_LEDGER_FILE_NAME);
    if !path.exists() {
        return Ok(ProjectSyncLedger::default());
    }

    let raw = fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&raw)?)
}

fn save_ledger(config_dir: &Path, ledger: &ProjectSyncLedger) -> Result<()> {
    fs::create_dir_all(config_dir)?;
    fs::write(config_dir.join(PROJECT_LEDGER_FILE_NAME), serde_json::to_string_pretty(ledger)?)?;
    Ok(())
}

fn ledger_key(project_path: &str, agent_key: &str) -> String {
    format!("{}\n{}", project_path, agent_key)
}

fn normalize_legacy_target_for_comparison(target_dir: &str) -> String {
    if cfg!(windows) {
        target_dir.replace('\\', "/")
    } else {
        target_dir.to_string()
    }
}

#[cfg(test)]
#[path = "sync_tests.rs"]
mod tests;
