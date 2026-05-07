use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::core::agents::catalog::{find_agent, project_skills_dir_rule};
use crate::core::agents::discovery::{load_agent_inventory, AgentInventoryItem, AgentSystemDirs};
use crate::core::agents::selection::{
    load_skill_selection_context, resolve_agent_skill_selection,
    resolve_project_agent_skill_selection, SkillResolutionDiagnostics, SkillSelectionContext,
};
use crate::core::agents::sync::AgentApplyResult;
use crate::core::agents::target_sync::{
    apply_desired_entries, build_desired_skill_entries, DesiredSkillEntry, SyncMode,
};
use crate::core::platform_paths::portable_path_string;
use crate::core::settings::{AgentSyncMode, SettingsStore};

use super::apply_status::{project_apply_statuses, project_resolution_hash};
use super::store::{
    ProjectAgentApplyStatus, ProjectAgentAssignment, ProjectApplyFreshness, ProjectAssignment,
    ProjectConfigSnapshot, ProjectConfigStore,
};
#[cfg(test)]
use super::sync_ledger::normalize_legacy_target_for_comparison;
use super::sync_ledger::{
    cleanup_retargeted_assignments, cleanup_stale_assignments, ledger_key, load_ledger,
    save_ledger, ProjectSyncLedger, ProjectSyncLedgerEntry,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectAssignmentApplyResult {
    pub project_path: String,
    pub display_name: String,
    pub enabled_skill_count: usize,
    pub apply_statuses: BTreeMap<String, ProjectAgentApplyStatus>,
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

pub fn attach_project_apply_statuses(
    config_dir: &Path,
    repo_path: &Path,
    system_dirs: &AgentSystemDirs,
    mut snapshot: ProjectConfigSnapshot,
) -> Result<ProjectConfigSnapshot> {
    let inventory = load_agent_inventory(config_dir, system_dirs)?;
    let agents = inventory
        .agents
        .into_iter()
        .map(|agent| (agent.key.clone(), agent))
        .collect::<BTreeMap<_, _>>();
    let skill_context = load_skill_selection_context(config_dir, repo_path)?;
    let sync_mode = load_project_sync_mode(config_dir)?;
    let ledger = load_ledger(config_dir)?;

    for project in snapshot.projects.values_mut() {
        project.apply_statuses =
            project_apply_statuses(project, &agents, &skill_context, sync_mode, &ledger);
    }

    Ok(snapshot)
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
    let mut apply_statuses = BTreeMap::new();
    let mut enabled_skill_ids = BTreeSet::new();

    cleanup_retargeted_assignments(project, ledger)?;

    for agent_key in &project.agent_keys {
        let key = ledger_key(&project.project_path, agent_key);
        managed_keys.insert(key.clone());

        let result = match find_agent(agent_key) {
            Some(agent) => {
                let Some(global_agent) = agents.get(agent_key) else {
                    ledger.assignments.remove(&key);
                    apply_statuses.insert(
                        agent_key.clone(),
                        ProjectAgentApplyStatus {
                            apply_status: ProjectApplyFreshness::Unsupported,
                            last_applied_at: None,
                        },
                    );
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
                let resolution_hash = project_resolution_hash(
                    project,
                    agent_key,
                    project_agent,
                    &project_result,
                    project_skills_dir_rule(agent),
                    sync_mode,
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
                let applied_at = current_unix_timestamp();
                ledger.assignments.insert(
                    key.clone(),
                    ProjectSyncLedgerEntry {
                        agent_key: agent_key.clone(),
                        target_dir: portable_path_string(&target_dir),
                        resolution_hash,
                        applied_at,
                    },
                );
                apply_statuses.insert(
                    agent_key.clone(),
                    ProjectAgentApplyStatus {
                        apply_status: ProjectApplyFreshness::Current,
                        last_applied_at: Some(applied_at),
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
                apply_statuses.insert(
                    agent_key.clone(),
                    ProjectAgentApplyStatus {
                        apply_status: ProjectApplyFreshness::Unsupported,
                        last_applied_at: None,
                    },
                );
                unsupported_agent_result(agent_key)
            }
        };

        results.push(result);
    }

    Ok(ProjectAssignmentApplyResult {
        project_path: project.project_path.clone(),
        display_name: project.display_name.clone(),
        enabled_skill_count: enabled_skill_ids.len(),
        apply_statuses,
        results,
    })
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

fn current_unix_timestamp() -> i64 {
    time::OffsetDateTime::now_utc().unix_timestamp()
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
    if warnings.is_empty() {
        message
    } else {
        format!("{message} Warnings: {}", warnings.join("; "))
    }
}

fn format_diagnostics(diagnostics: &SkillResolutionDiagnostics) -> Vec<String> {
    let mut warnings = Vec::new();
    push_warning(
        &mut warnings,
        "missing scenes",
        &diagnostics.missing_scene_ids,
    );
    push_warning(
        &mut warnings,
        "missing skills",
        &diagnostics.missing_skill_ids,
    );
    push_warning(
        &mut warnings,
        "globally disabled",
        &diagnostics.globally_disabled_references,
    );
    warnings
}

fn push_warning(warnings: &mut Vec<String>, label: &str, ids: &[String]) {
    if !ids.is_empty() {
        warnings.push(format!("{label}: {}", ids.join(", ")));
    }
}

#[cfg(test)]
#[path = "sync_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "sync_status_tests.rs"]
mod status_tests;
