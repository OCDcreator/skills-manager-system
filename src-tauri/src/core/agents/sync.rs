use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use super::discovery::{load_agent_inventory, AgentInventoryItem, AgentPathMode, AgentSystemDirs};
use super::manifest::{load_ledger, save_ledger, AgentSyncLedgerEntry};
use super::selection::{load_skill_selection_context, resolve_agent_skills};
use super::target_sync::{
    apply_desired_entries, build_desired_skill_entries, cleanup_managed_entries, DesiredSkillEntry,
    SyncMode,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AgentApplyStatus {
    Success,
    Partial,
    Skipped,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentApplyResult {
    pub key: String,
    pub display_name: String,
    pub target_dir: Option<String>,
    pub status: AgentApplyStatus,
    pub written_count: usize,
    pub removed_count: usize,
    pub conflict_count: usize,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApplyAgentSyncResponse {
    pub enabled_skill_count: usize,
    pub results: Vec<AgentApplyResult>,
}

pub fn apply_agent_sync(
    config_dir: &Path,
    repo_path: &Path,
    system_dirs: &AgentSystemDirs,
    mode: SyncMode,
) -> Result<ApplyAgentSyncResponse> {
    apply_agent_sync_scoped(config_dir, repo_path, system_dirs, mode, None)
}

pub fn apply_agent_sync_for_agent(
    config_dir: &Path,
    repo_path: &Path,
    system_dirs: &AgentSystemDirs,
    mode: SyncMode,
    agent_key: &str,
) -> Result<ApplyAgentSyncResponse> {
    let agent_keys = BTreeSet::from([agent_key.to_string()]);
    apply_agent_sync_scoped(config_dir, repo_path, system_dirs, mode, Some(&agent_keys))
}

fn apply_agent_sync_scoped(
    config_dir: &Path,
    repo_path: &Path,
    system_dirs: &AgentSystemDirs,
    mode: SyncMode,
    agent_keys: Option<&BTreeSet<String>>,
) -> Result<ApplyAgentSyncResponse> {
    let inventory = load_agent_inventory(config_dir, system_dirs)?;
    let skill_context = load_skill_selection_context(config_dir, repo_path)?;
    let mut ledger = load_ledger(config_dir)?;
    let mut results = Vec::new();

    for agent in inventory.agents {
        if let Some(agent_keys) = agent_keys {
            if !agent_keys.contains(&agent.key) {
                continue;
            }
        }

        let previous_target_dir = ledger
            .agents
            .get(&agent.key)
            .map(|entry| PathBuf::from(&entry.last_applied_target_dir));
        let current_target_dir = agent.effective_skills_dir.as_ref().map(PathBuf::from);
        let selected_skills = resolve_agent_skills(&agent, &skill_context);
        let desired_entries = build_desired_skill_entries(&selected_skills);

        match apply_for_agent(
            &agent,
            current_target_dir.as_deref(),
            previous_target_dir.as_deref(),
            &desired_entries,
            mode,
        ) {
            Ok(result) => {
                if agent.enabled && current_target_dir.is_some() {
                    ledger.agents.insert(
                        agent.key.clone(),
                        AgentSyncLedgerEntry {
                            last_applied_target_dir: current_target_dir
                                .as_ref()
                                .unwrap()
                                .to_string_lossy()
                                .to_string(),
                        },
                    );
                } else {
                    ledger.agents.remove(&agent.key);
                }
                results.push(result);
            }
            Err(error) => {
                results.push(AgentApplyResult {
                    key: agent.key.clone(),
                    display_name: agent.display_name.clone(),
                    target_dir: current_target_dir
                        .as_ref()
                        .or(previous_target_dir.as_ref())
                        .map(|path| path.to_string_lossy().to_string()),
                    status: AgentApplyStatus::Failed,
                    written_count: 0,
                    removed_count: 0,
                    conflict_count: 0,
                    message: error.to_string(),
                });
            }
        }
    }

    save_ledger(config_dir, &ledger)?;

    Ok(ApplyAgentSyncResponse {
        enabled_skill_count: skill_context.available_skill_count(),
        results,
    })
}

fn apply_for_agent(
    agent: &AgentInventoryItem,
    current_target_dir: Option<&Path>,
    previous_target_dir: Option<&Path>,
    desired_entries: &std::collections::BTreeMap<String, DesiredSkillEntry>,
    mode: SyncMode,
) -> Result<AgentApplyResult> {
    let removed_count = cleanup_previous_target(agent, current_target_dir, previous_target_dir)?;

    if !agent.enabled {
        return disabled_agent_result(
            agent,
            current_target_dir,
            previous_target_dir,
            removed_count,
            mode,
        );
    }

    let Some(current_target_dir) = current_target_dir else {
        return Ok(skipped_path_result(
            agent,
            previous_target_dir,
            removed_count,
        ));
    };

    let mut stats = apply_desired_entries(current_target_dir, &agent.key, desired_entries, mode)?;
    stats.removed_count += removed_count;
    let message = if stats.conflict_count > 0 {
        format!(
            "Applied {} skill(s) with {} unmanaged conflict(s).",
            stats.written_count, stats.conflict_count
        )
    } else if desired_entries.is_empty() {
        "No enabled skills remain; cleaned managed entries.".to_string()
    } else {
        format!("Applied {} skill(s).", stats.written_count)
    };

    Ok(AgentApplyResult {
        key: agent.key.clone(),
        display_name: agent.display_name.clone(),
        target_dir: Some(current_target_dir.to_string_lossy().to_string()),
        status: if stats.conflict_count > 0 {
            AgentApplyStatus::Partial
        } else {
            AgentApplyStatus::Success
        },
        written_count: stats.written_count,
        removed_count: stats.removed_count,
        conflict_count: stats.conflict_count,
        message,
    })
}

fn cleanup_previous_target(
    agent: &AgentInventoryItem,
    current_target_dir: Option<&Path>,
    previous_target_dir: Option<&Path>,
) -> Result<usize> {
    match previous_target_dir {
        Some(previous_target_dir) if current_target_dir != Some(previous_target_dir) => {
            cleanup_managed_entries(previous_target_dir, &agent.key)
        }
        _ => Ok(0),
    }
}

fn disabled_agent_result(
    agent: &AgentInventoryItem,
    current_target_dir: Option<&Path>,
    previous_target_dir: Option<&Path>,
    mut removed_count: usize,
    mode: SyncMode,
) -> Result<AgentApplyResult> {
    if let Some(current_target_dir) = current_target_dir {
        removed_count += apply_desired_entries(
            current_target_dir,
            &agent.key,
            &std::collections::BTreeMap::new(),
            mode,
        )?
        .removed_count;
    }

    Ok(AgentApplyResult {
        key: agent.key.clone(),
        display_name: agent.display_name.clone(),
        target_dir: current_target_dir
            .or(previous_target_dir)
            .map(|path| path.to_string_lossy().to_string()),
        status: if removed_count > 0 {
            AgentApplyStatus::Success
        } else {
            AgentApplyStatus::Skipped
        },
        written_count: 0,
        removed_count,
        conflict_count: 0,
        message: if removed_count > 0 {
            "Agent disabled; removed previously managed entries.".to_string()
        } else {
            "Agent disabled; nothing to apply.".to_string()
        },
    })
}

fn skipped_path_result(
    agent: &AgentInventoryItem,
    previous_target_dir: Option<&Path>,
    removed_count: usize,
) -> AgentApplyResult {
    AgentApplyResult {
        key: agent.key.clone(),
        display_name: agent.display_name.clone(),
        target_dir: previous_target_dir.map(|path| path.to_string_lossy().to_string()),
        status: AgentApplyStatus::Skipped,
        written_count: 0,
        removed_count,
        conflict_count: 0,
        message: match agent.path_mode {
            AgentPathMode::Missing => {
                "No detected default path or override is configured for this agent.".to_string()
            }
            _ => "Agent target path is unavailable.".to_string(),
        },
    }
}
