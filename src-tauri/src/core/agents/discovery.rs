use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use super::catalog::{agent_catalog, project_skills_dir_rule, AgentCatalogEntry};
use super::config::{AgentConfigEntry, AgentConfigSnapshot, AgentConfigStore};
use super::target_inventory::{scan_target_skill_entries, AgentTargetSkillEntry};
use crate::core::platform_paths::portable_path_string;

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
    pub selected_skill_ids: Vec<String>,
    pub selected_scene_ids: Vec<String>,
    pub excluded_skill_ids: Vec<String>,
    pub skills_dir_rule: String,
    pub project_skills_dir_rule: String,
    pub detect_dir_rule: String,
    pub default_skills_dir: String,
    pub detected_skills_dir: Option<String>,
    pub effective_skills_dir: Option<String>,
    pub target_skill_entries: Vec<AgentTargetSkillEntry>,
    pub target_skill_scan_error: Option<String>,
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
    let path_override = config
        .path_override
        .map(|path| portable_path_string(Path::new(&path)));

    let (path_mode, effective_skills_dir) = match path_override.clone() {
        Some(path) => (AgentPathMode::Override, Some(path)),
        None => match detected_skills_dir.clone() {
            Some(path) => (AgentPathMode::Detected, Some(path)),
            None => (AgentPathMode::Missing, None),
        },
    };

    let (target_skill_entries, target_skill_scan_error) = effective_skills_dir
        .as_deref()
        .map(|path| match scan_target_skill_entries(Path::new(path), definition.key) {
            Ok(entries) => (entries, None),
            Err(error) => (Vec::new(), Some(error.to_string())),
        })
        .unwrap_or_else(|| (Vec::new(), None));

    AgentInventoryItem {
        key: definition.key.to_string(),
        display_name: definition.display_name.to_string(),
        enabled: config.enabled,
        selected_skill_ids: config.selected_skill_ids,
        selected_scene_ids: config.selected_scene_ids,
        excluded_skill_ids: config.excluded_skill_ids,
        skills_dir_rule: definition.skills_dir_rule.to_string(),
        project_skills_dir_rule: project_skills_dir_rule(definition).to_string(),
        detect_dir_rule: definition.detect_dir_rule.to_string(),
        default_skills_dir: path_to_string(default_skills_dir),
        detected_skills_dir,
        effective_skills_dir,
        target_skill_entries,
        target_skill_scan_error,
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
    portable_path_string(&path)
}

#[cfg(test)]
#[path = "discovery_tests.rs"]
mod tests;
