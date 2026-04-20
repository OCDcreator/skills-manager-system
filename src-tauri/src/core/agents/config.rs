use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::catalog::find_agent;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentConfigEntry {
    pub enabled: bool,
    pub path_override: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentConfigSnapshot {
    pub agents: BTreeMap<String, AgentConfigEntry>,
}

pub struct AgentConfigStore {
    base_dir: PathBuf,
}

impl AgentConfigStore {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn load(&self) -> Result<AgentConfigSnapshot> {
        let path = self.config_path();
        if !path.exists() {
            return Ok(AgentConfigSnapshot::default());
        }

        let raw = fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
        serde_json::from_str::<AgentConfigSnapshot>(&raw)
            .with_context(|| format!("Failed to parse {:?}", path))
    }

    pub fn set_agent_enabled(&self, key: &str, enabled: bool) -> Result<AgentConfigSnapshot> {
        self.update_entry(key, |entry| entry.enabled = enabled)
    }

    pub fn set_agent_path_override(&self, key: &str, path: &str) -> Result<AgentConfigSnapshot> {
        let normalized = normalize_path_override(path)?;
        self.update_entry(key, |entry| entry.path_override = Some(normalized.clone()))
    }

    pub fn clear_agent_path_override(&self, key: &str) -> Result<AgentConfigSnapshot> {
        self.update_entry(key, |entry| entry.path_override = None)
    }

    fn update_entry<F>(&self, key: &str, mutator: F) -> Result<AgentConfigSnapshot>
    where
        F: FnOnce(&mut AgentConfigEntry),
    {
        validate_agent_key(key)?;

        let mut snapshot = self.load()?;
        let entry = snapshot.agents.entry(key.to_string()).or_default();
        mutator(entry);
        self.save(&snapshot)?;
        Ok(snapshot)
    }

    fn save(&self, snapshot: &AgentConfigSnapshot) -> Result<()> {
        fs::create_dir_all(&self.base_dir)
            .with_context(|| format!("Failed to create {:?}", self.base_dir))?;

        let json = serde_json::to_string_pretty(snapshot)?;
        fs::write(self.config_path(), json).context("Failed to write agent-config.json")?;
        Ok(())
    }

    fn config_path(&self) -> PathBuf {
        self.base_dir.join("agent-config.json")
    }
}

fn validate_agent_key(key: &str) -> Result<()> {
    let trimmed = key.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("Agent key is required"));
    }
    if find_agent(trimmed).is_none() {
        return Err(anyhow!("Unsupported agent key: {trimmed}"));
    }
    Ok(())
}

fn normalize_path_override(path: &str) -> Result<String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("Path override is required"));
    }

    let candidate = Path::new(trimmed);
    if !candidate.is_absolute() {
        return Err(anyhow!("Path override must be absolute"));
    }

    Ok(candidate.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use tempfile::tempdir;

    #[test]
    fn load_returns_defaults_when_config_missing() {
        let dir = tempdir().unwrap();
        let store = AgentConfigStore::new(dir.path().to_path_buf());

        let snapshot = store.load().unwrap();

        assert!(snapshot.agents.is_empty());
    }

    #[test]
    fn set_agent_enabled_round_trips() {
        let dir = tempdir().unwrap();
        let store = AgentConfigStore::new(dir.path().to_path_buf());

        store.set_agent_enabled("codex", true).unwrap();

        let snapshot = store.load().unwrap();
        assert!(snapshot.agents.get("codex").unwrap().enabled);
    }

    #[test]
    fn path_override_round_trips() {
        let dir = tempdir().unwrap();
        let override_path = PathBuf::from("C:/Users/test/.codex/skills");
        let store = AgentConfigStore::new(dir.path().to_path_buf());

        store
            .set_agent_path_override("codex", override_path.to_string_lossy().as_ref())
            .unwrap();

        let snapshot = store.load().unwrap();
        assert_eq!(
            snapshot
                .agents
                .get("codex")
                .unwrap()
                .path_override
                .as_deref(),
            Some("C:/Users/test/.codex/skills")
        );

        store.clear_agent_path_override("codex").unwrap();
        let cleared = store.load().unwrap();
        assert_eq!(
            cleared
                .agents
                .get("codex")
                .unwrap()
                .path_override
                .as_deref(),
            None
        );
    }
}
