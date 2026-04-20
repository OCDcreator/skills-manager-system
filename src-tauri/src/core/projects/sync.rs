use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::core::agents::catalog::find_agent;
use crate::core::agents::discovery::AgentSystemDirs;
use crate::core::agents::sync::AgentApplyResult;
use crate::core::skills::scan::{scan_repo_skills, SkillSummary};

use super::store::{ProjectAssignment, ProjectConfigStore};

mod manifest_impl {
    include!("../agents/manifest.rs");
}

use manifest_impl::{
    apply_desired_entries, build_desired_skill_entries, cleanup_managed_entries, DesiredSkillEntry,
};

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
    _system_dirs: &AgentSystemDirs,
) -> Result<ApplyProjectAssignmentsResponse> {
    let snapshot = ProjectConfigStore::new(config_dir.to_path_buf()).load()?;
    let skills = scan_repo_skills(repo_path)?.skills;
    let mut ledger = load_ledger(config_dir)?;
    let mut managed_keys = BTreeSet::new();
    let mut results = Vec::new();

    for project in snapshot.projects.values() {
        let result = apply_for_project(project, &skills, &mut ledger, &mut managed_keys)?;
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
    all_skills: &[SkillSummary],
    ledger: &mut ProjectSyncLedger,
    managed_keys: &mut BTreeSet<String>,
) -> Result<ProjectAssignmentApplyResult> {
    let enabled_skills = select_project_skills(project, all_skills);
    let desired_entries = build_desired_skill_entries(&enabled_skills);
    let mut results = Vec::new();

    for agent_key in &project.agent_keys {
        let key = ledger_key(&project.project_path, agent_key);
        managed_keys.insert(key.clone());

        let result = match find_agent(agent_key) {
            Some(agent) => {
                let target_dir = Path::new(&project.project_path).join(agent.skills_dir_rule);
                let stats = apply_desired_entries(&target_dir, agent_key, &desired_entries)?;
                ledger.assignments.insert(
                    key,
                    ProjectSyncLedgerEntry {
                        agent_key: agent_key.clone(),
                        target_dir: target_dir.to_string_lossy().to_string(),
                    },
                );

                AgentApplyResult {
                    key: agent_key.clone(),
                    display_name: agent.display_name.to_string(),
                    target_dir: Some(target_dir.to_string_lossy().to_string()),
                    status: if stats.conflict_count > 0 {
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
                    ),
                }
            }
            None => {
                ledger.assignments.remove(&key);
                AgentApplyResult {
                    key: agent_key.clone(),
                    display_name: agent_key.clone(),
                    target_dir: None,
                    status: crate::core::agents::sync::AgentApplyStatus::Failed,
                    written_count: 0,
                    removed_count: 0,
                    conflict_count: 0,
                    message: format!("Unsupported agent key '{}'.", agent_key),
                }
            }
        };

        results.push(result);
    }

    Ok(ProjectAssignmentApplyResult {
        project_path: project.project_path.clone(),
        display_name: project.display_name.clone(),
        enabled_skill_count: enabled_skills.len(),
        results,
    })
}

fn select_project_skills(
    project: &ProjectAssignment,
    all_skills: &[SkillSummary],
) -> Vec<SkillSummary> {
    let skill_map: BTreeMap<&str, &SkillSummary> = all_skills
        .iter()
        .map(|skill| (skill.id.as_str(), skill))
        .collect();
    project
        .skill_ids
        .iter()
        .filter_map(|skill_id| skill_map.get(skill_id.as_str()).cloned().cloned())
        .collect()
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

fn build_message(
    desired_entries: &BTreeMap<String, DesiredSkillEntry>,
    conflict_count: usize,
    written_count: usize,
) -> String {
    if conflict_count > 0 {
        format!(
            "Applied {} skill(s) with {} unmanaged conflict(s).",
            written_count, conflict_count
        )
    } else if desired_entries.is_empty() {
        "No selected skills remain; cleaned managed entries.".to_string()
    } else {
        format!("Applied {} skill(s).", written_count)
    }
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
    fs::write(
        config_dir.join(PROJECT_LEDGER_FILE_NAME),
        serde_json::to_string_pretty(ledger)?,
    )?;
    Ok(())
}

fn ledger_key(project_path: &str, agent_key: &str) -> String {
    format!("{}\n{}", project_path, agent_key)
}
