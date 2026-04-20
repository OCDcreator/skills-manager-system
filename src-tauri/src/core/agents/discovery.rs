use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use super::catalog::{agent_catalog, AgentCatalogEntry};
use super::config::{AgentConfigEntry, AgentConfigSnapshot, AgentConfigStore};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSystemDirs {
    pub home_dir: PathBuf,
    pub config_dir: Option<PathBuf>,
}

impl AgentSystemDirs {
    pub fn current() -> Result<Self> {
        let home_dir = dirs::home_dir().context("Failed to determine home directory")?;
        Ok(Self {
            home_dir,
            config_dir: dirs::config_dir(),
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AgentPathMode {
    Override,
    Detected,
    Missing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentInventoryItem {
    pub key: String,
    pub display_name: String,
    pub enabled: bool,
    pub default_skills_dir: String,
    pub detected_skills_dir: Option<String>,
    pub effective_skills_dir: Option<String>,
    pub path_override: Option<String>,
    pub path_mode: AgentPathMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentInventorySnapshot {
    pub agents: Vec<AgentInventoryItem>,
}

pub fn load_agent_inventory(
    config_dir: &Path,
    system_dirs: &AgentSystemDirs,
) -> Result<AgentInventorySnapshot> {
    let snapshot = AgentConfigStore::new(config_dir.to_path_buf()).load()?;
    Ok(build_agent_inventory(&snapshot, system_dirs))
}

pub fn build_agent_inventory(
    snapshot: &AgentConfigSnapshot,
    system_dirs: &AgentSystemDirs,
) -> AgentInventorySnapshot {
    let agents = agent_catalog()
        .iter()
        .map(|definition| {
            build_agent_inventory_item(
                definition,
                snapshot
                    .agents
                    .get(definition.key)
                    .cloned()
                    .unwrap_or_default(),
                system_dirs,
            )
        })
        .collect();

    AgentInventorySnapshot { agents }
}

fn build_agent_inventory_item(
    definition: &AgentCatalogEntry,
    config: AgentConfigEntry,
    system_dirs: &AgentSystemDirs,
) -> AgentInventoryItem {
    let default_skills_dir =
        select_existing_or_default(candidate_paths(definition.skills_dir_rule, system_dirs));
    let detected_skills_dir = detect_skills_dir(definition, system_dirs).map(path_to_string);
    let path_override = config.path_override;

    let (path_mode, effective_skills_dir) = match path_override.clone() {
        Some(path) => (AgentPathMode::Override, Some(path)),
        None => match detected_skills_dir.clone() {
            Some(path) => (AgentPathMode::Detected, Some(path)),
            None => (AgentPathMode::Missing, None),
        },
    };

    AgentInventoryItem {
        key: definition.key.to_string(),
        display_name: definition.display_name.to_string(),
        enabled: config.enabled,
        default_skills_dir: path_to_string(default_skills_dir),
        detected_skills_dir,
        effective_skills_dir,
        path_override,
        path_mode,
    }
}

fn detect_skills_dir(
    definition: &AgentCatalogEntry,
    system_dirs: &AgentSystemDirs,
) -> Option<PathBuf> {
    candidate_paths(definition.skills_dir_rule, system_dirs)
        .into_iter()
        .zip(candidate_paths(definition.detect_dir_rule, system_dirs))
        .find_map(|(skills_dir, detect_dir)| detect_dir.exists().then_some(skills_dir))
}

fn candidate_paths(rule: &str, system_dirs: &AgentSystemDirs) -> Vec<PathBuf> {
    let mut candidates = vec![system_dirs.home_dir.join(rule)];

    if let Some(suffix) = rule.strip_prefix(".config/") {
        if let Some(config_dir) = &system_dirs.config_dir {
            let config_candidate = config_dir.join(suffix);
            if !candidates.contains(&config_candidate) {
                candidates.push(config_candidate);
            }
        }
    }

    candidates
}

fn select_existing_or_default(candidates: Vec<PathBuf>) -> PathBuf {
    candidates
        .iter()
        .find(|path| path.exists())
        .cloned()
        .unwrap_or_else(|| candidates[0].clone())
}

fn path_to_string(path: PathBuf) -> String {
    path.to_string_lossy().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::agents::config::{AgentConfigEntry, AgentConfigSnapshot};
    use std::fs;

    fn empty_snapshot() -> AgentConfigSnapshot {
        AgentConfigSnapshot::default()
    }

    #[test]
    fn detected_default_path_becomes_effective_path() {
        let temp_root = tempfile::tempdir().unwrap();
        let home_dir = temp_root.path().join("home");
        let config_dir = temp_root.path().join("config");
        fs::create_dir_all(home_dir.join(".codex/skills")).unwrap();

        let inventory = build_agent_inventory(
            &empty_snapshot(),
            &AgentSystemDirs {
                home_dir,
                config_dir: Some(config_dir),
            },
        );

        let codex = inventory
            .agents
            .iter()
            .find(|agent| agent.key == "codex")
            .unwrap();
        assert_eq!(codex.path_mode, AgentPathMode::Detected);
        assert!(codex.detected_skills_dir.is_some());
        assert_eq!(codex.effective_skills_dir, codex.detected_skills_dir);
    }

    #[test]
    fn missing_default_path_reports_missing_mode() {
        let temp_root = tempfile::tempdir().unwrap();
        let inventory = build_agent_inventory(
            &empty_snapshot(),
            &AgentSystemDirs {
                home_dir: temp_root.path().join("home"),
                config_dir: Some(temp_root.path().join("config")),
            },
        );

        let claude = inventory
            .agents
            .iter()
            .find(|agent| agent.key == "claude_code")
            .unwrap();
        assert_eq!(claude.path_mode, AgentPathMode::Missing);
        assert_eq!(claude.detected_skills_dir, None);
        assert_eq!(claude.effective_skills_dir, None);
    }

    #[test]
    fn path_override_takes_precedence_over_detected_path() {
        let temp_root = tempfile::tempdir().unwrap();
        let home_dir = temp_root.path().join("home");
        let override_dir = temp_root.path().join("override/opencode-skills");
        fs::create_dir_all(home_dir.join(".config/opencode/skills")).unwrap();

        let mut snapshot = AgentConfigSnapshot::default();
        snapshot.agents.insert(
            "opencode".to_string(),
            AgentConfigEntry {
                enabled: true,
                path_override: Some(override_dir.to_string_lossy().to_string()),
            },
        );

        let inventory = build_agent_inventory(
            &snapshot,
            &AgentSystemDirs {
                home_dir,
                config_dir: Some(temp_root.path().join("config")),
            },
        );

        let opencode = inventory
            .agents
            .iter()
            .find(|agent| agent.key == "opencode")
            .unwrap();
        assert_eq!(opencode.path_mode, AgentPathMode::Override);
        assert_eq!(
            opencode.effective_skills_dir.as_deref(),
            Some(override_dir.to_string_lossy().as_ref())
        );
    }
}
