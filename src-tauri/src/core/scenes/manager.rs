use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::config::{SceneConfigSnapshot, SceneConfigStore, SceneEntry};
use crate::core::agents::config::AgentConfigStore;
use crate::core::agents::discovery::{load_agent_inventory, AgentSystemDirs};
use crate::core::skills::state::SkillStateStore;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApplySceneResult {
    pub scene_id: String,
    pub scene_name: String,
    pub disabled_skill_count: usize,
    pub enabled_agent_count: usize,
}

pub fn apply_scene(
    config_dir: &Path,
    repo_path: &Path,
    system_dirs: &AgentSystemDirs,
    scene_id: &str,
) -> Result<ApplySceneResult> {
    let scene_config = SceneConfigStore::new(config_dir.to_path_buf()).load()?;
    let scene = scene_config
        .scenes
        .get(scene_id)
        .ok_or_else(|| anyhow::anyhow!("Scene '{}' not found", scene_id))?
        .clone();

    apply_skill_state_for_scene(config_dir, repo_path, &scene)?;
    apply_agent_state_for_scene(config_dir, system_dirs, &scene)?;

    SceneConfigStore::new(config_dir.to_path_buf()).set_active_scene(Some(scene_id))?;

    Ok(ApplySceneResult {
        scene_id: scene.id.clone(),
        scene_name: scene.name.clone(),
        disabled_skill_count: scene.disabled_skill_ids.len(),
        enabled_agent_count: scene.enabled_agent_keys.len(),
    })
}

fn apply_skill_state_for_scene(
    config_dir: &Path,
    repo_path: &Path,
    scene: &SceneEntry,
) -> Result<()> {
    let store = SkillStateStore::new(config_dir.to_path_buf());
    let current = store.load_for_repo(repo_path)?;

    if current.disabled_skill_ids == scene.disabled_skill_ids {
        return Ok(());
    }

    for skill_id in &scene.disabled_skill_ids {
        let _ = store.set_skill_enabled(repo_path, skill_id, false);
    }

    let all_disabled: std::collections::BTreeSet<String> =
        current.disabled_skill_ids.iter().cloned().collect();
    let scene_disabled: std::collections::BTreeSet<String> =
        scene.disabled_skill_ids.iter().cloned().collect();

    for skill_id in all_disabled.difference(&scene_disabled) {
        let _ = store.set_skill_enabled(repo_path, skill_id, true);
    }

    Ok(())
}

fn apply_agent_state_for_scene(
    config_dir: &Path,
    system_dirs: &AgentSystemDirs,
    scene: &SceneEntry,
) -> Result<()> {
    let agent_store = AgentConfigStore::new(config_dir.to_path_buf());
    let inventory = load_agent_inventory(config_dir, system_dirs)?;

    let scene_enabled: std::collections::BTreeSet<&str> = scene
        .enabled_agent_keys
        .iter()
        .map(|s| s.as_str())
        .collect();

    for agent in &inventory.agents {
        let should_enable = scene_enabled.contains(agent.key.as_str());
        if agent.enabled != should_enable {
            let _ = agent_store.set_agent_enabled(&agent.key, should_enable);
        }
    }

    Ok(())
}
