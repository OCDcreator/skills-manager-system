use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const LEDGER_FILE_NAME: &str = "agent-sync-ledger.json";

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentSyncLedger {
    pub agents: BTreeMap<String, AgentSyncLedgerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentSyncLedgerEntry {
    pub last_applied_target_dir: String,
}

pub(crate) fn load_ledger(config_dir: &Path) -> Result<AgentSyncLedger> {
    let path = ledger_path(config_dir);
    if !path.exists() {
        return Ok(AgentSyncLedger::default());
    }

    let raw = fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
    serde_json::from_str::<AgentSyncLedger>(&raw)
        .with_context(|| format!("Failed to parse {:?}", path))
}

pub(crate) fn save_ledger(config_dir: &Path, ledger: &AgentSyncLedger) -> Result<()> {
    fs::create_dir_all(config_dir).with_context(|| format!("Failed to create {:?}", config_dir))?;
    let json = serde_json::to_string_pretty(ledger)?;
    fs::write(ledger_path(config_dir), json).context("Failed to write agent-sync-ledger.json")?;
    Ok(())
}

fn ledger_path(config_dir: &Path) -> PathBuf {
    config_dir.join(LEDGER_FILE_NAME)
}
