use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::config::{SceneConfigStore, SceneEntry};
use crate::core::agents::config::AgentConfigStore;
use crate::core::agents::discovery::{load_agent_inventory, AgentSystemDirs};
use crate::core::agents::sync::apply_agent_sync;
use crate::core::agents::target_sync::SyncMode;
use crate::core::settings::{AgentSyncMode, SettingsStore};
use crate::core::skills::scan::scan_repo_skills;

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
    let disabled_skill_count = scene.disabled_skill_count(
        scan_repo_skills(repo_path)?
            .skills
            .iter()
            .map(|skill| skill.id.as_str()),
    );

    apply_agent_state_for_scene(config_dir, system_dirs, &scene)?;
    apply_agent_sync(
        config_dir,
        repo_path,
        system_dirs,
        load_sync_mode(config_dir)?,
    )?;
    SceneConfigStore::new(config_dir.to_path_buf()).set_active_scene(Some(scene_id))?;

    Ok(ApplySceneResult {
        scene_id: scene.id.clone(),
        scene_name: scene.name.clone(),
        disabled_skill_count,
        enabled_agent_count: scene.enabled_agent_keys.len(),
    })
}

fn load_sync_mode(config_dir: &Path) -> Result<SyncMode> {
    let settings = SettingsStore::new(config_dir.to_path_buf()).load()?;
    Ok(match settings.agent_sync_mode {
        AgentSyncMode::Copy => SyncMode::Copy,
        AgentSyncMode::Symlink => SyncMode::Symlink,
    })
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
        if should_enable {
            let _ = agent_store.set_agent_selection(
                &agent.key,
                agent.selected_skill_ids.clone(),
                vec![scene.id.clone()],
                agent.excluded_skill_ids.clone(),
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::agents::config::AgentConfigStore;
    use std::fs;

    fn create_skill(repo_root: &Path, relative_path: &str) {
        let skill_dir = repo_root.join(relative_path);
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(skill_dir.join("SKILL.md"), format!("# {}", relative_path)).unwrap();
    }

    #[test]
    fn apply_scene_syncs_enabled_skills_into_agent_targets() {
        let temp = tempfile::tempdir().unwrap();
        let config_dir = temp.path().join("config");
        let repo_dir = temp.path().join("repo");
        let target_dir = temp.path().join("targets/codex-skills");

        create_skill(&repo_dir, "custom/alpha");
        create_skill(&repo_dir, "custom/beta");

        let scene_store = SceneConfigStore::new(config_dir.clone());
        scene_store.create_scene("focus", "Focus", "").unwrap();
        scene_store
            .set_scene_skills("focus", vec!["custom:alpha".to_string()])
            .unwrap();
        scene_store
            .set_scene_agents("focus", vec!["codex".to_string()])
            .unwrap();

        let agent_store = AgentConfigStore::new(config_dir.clone());
        agent_store.set_agent_enabled("codex", true).unwrap();
        agent_store
            .set_agent_path_override("codex", target_dir.to_string_lossy().as_ref())
            .unwrap();

        let result = apply_scene(
            &config_dir,
            &repo_dir,
            &AgentSystemDirs {
                home_dir: temp.path().join("home"),
                config_dir: Some(temp.path().join("config-home")),
            },
            "focus",
        )
        .unwrap();

        assert_eq!(result.scene_name, "Focus");
        assert!(target_dir.join("alpha/SKILL.md").exists());
        assert!(!target_dir.join("beta").exists());
    }

    #[test]
    fn new_scene_defaults_to_syncing_no_skills() {
        let temp = tempfile::tempdir().unwrap();
        let config_dir = temp.path().join("config");
        let repo_dir = temp.path().join("repo");
        let target_dir = temp.path().join("targets/codex-skills");

        create_skill(&repo_dir, "custom/alpha");
        create_skill(&repo_dir, "custom/beta");

        let scene_store = SceneConfigStore::new(config_dir.clone());
        scene_store.create_scene("blank", "Blank", "").unwrap();
        scene_store
            .set_scene_agents("blank", vec!["codex".to_string()])
            .unwrap();

        let agent_store = AgentConfigStore::new(config_dir.clone());
        agent_store.set_agent_enabled("codex", true).unwrap();
        agent_store
            .set_agent_path_override("codex", target_dir.to_string_lossy().as_ref())
            .unwrap();

        let result = apply_scene(
            &config_dir,
            &repo_dir,
            &AgentSystemDirs {
                home_dir: temp.path().join("home"),
                config_dir: Some(temp.path().join("config-home")),
            },
            "blank",
        )
        .unwrap();

        assert_eq!(result.scene_name, "Blank");
        assert_eq!(result.disabled_skill_count, 2);
        assert!(!target_dir.join("alpha").exists());
        assert!(!target_dir.join("beta").exists());
    }

    #[cfg(unix)]
    #[test]
    fn apply_scene_uses_saved_symlink_mode() {
        let temp = tempfile::tempdir().unwrap();
        let config_dir = temp.path().join("config");
        let repo_dir = temp.path().join("repo");
        let target_dir = temp.path().join("targets/codex-skills");

        create_skill(&repo_dir, "custom/alpha");
        SettingsStore::new(config_dir.clone())
            .save_agent_sync_mode(AgentSyncMode::Symlink)
            .unwrap();

        let scene_store = SceneConfigStore::new(config_dir.clone());
        scene_store.create_scene("focus", "Focus", "").unwrap();
        scene_store
            .set_scene_agents("focus", vec!["codex".to_string()])
            .unwrap();

        let agent_store = AgentConfigStore::new(config_dir.clone());
        agent_store.set_agent_enabled("codex", true).unwrap();
        agent_store
            .set_agent_path_override("codex", target_dir.to_string_lossy().as_ref())
            .unwrap();

        apply_scene(
            &config_dir,
            &repo_dir,
            &AgentSystemDirs {
                home_dir: temp.path().join("home"),
                config_dir: Some(temp.path().join("config-home")),
            },
            "focus",
        )
        .unwrap();

        let metadata = fs::symlink_metadata(target_dir.join("alpha")).unwrap();
        assert!(metadata.file_type().is_symlink());
    }
}
