use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::core::agents::catalog::{find_agent, project_skills_dir_rule};
use crate::core::agents::target_sync::cleanup_managed_entries;
use crate::core::platform_paths::portable_path_string;

use super::store::ProjectAssignment;

pub(super) const PROJECT_LEDGER_FILE_NAME: &str = "project-sync-ledger.json";

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(super) struct ProjectSyncLedger {
    pub(super) assignments: BTreeMap<String, ProjectSyncLedgerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(super) struct ProjectSyncLedgerEntry {
    pub(super) agent_key: String,
    pub(super) target_dir: String,
    #[serde(default)]
    pub(super) resolution_hash: String,
    #[serde(default)]
    pub(super) applied_at: i64,
}

pub(super) fn cleanup_retargeted_assignments(
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

pub(super) fn cleanup_stale_assignments(
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

pub(super) fn load_ledger(config_dir: &Path) -> Result<ProjectSyncLedger> {
    let path = config_dir.join(PROJECT_LEDGER_FILE_NAME);
    if !path.exists() {
        return Ok(ProjectSyncLedger::default());
    }

    let raw = fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&raw)?)
}

pub(super) fn save_ledger(config_dir: &Path, ledger: &ProjectSyncLedger) -> Result<()> {
    fs::create_dir_all(config_dir)?;
    fs::write(config_dir.join(PROJECT_LEDGER_FILE_NAME), serde_json::to_string_pretty(ledger)?)?;
    Ok(())
}

pub(super) fn ledger_key(project_path: &str, agent_key: &str) -> String {
    format!("{}\n{}", project_path, agent_key)
}

pub(super) fn normalize_legacy_target_for_comparison(target_dir: &str) -> String {
    if cfg!(windows) {
        target_dir.replace('\\', "/")
    } else {
        target_dir.to_string()
    }
}
