use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::config::SceneConfigStore;
use crate::core::agents::discovery::AgentSystemDirs;
use crate::core::skills::scan::scan_repo_skills;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SceneToolkitSummary {
    pub scene_id: String,
    pub scene_name: String,
    pub enabled_skill_count: usize,
    pub disabled_skill_count: usize,
    pub enabled_agent_count: usize,
}

pub type ApplySceneResult = SceneToolkitSummary;

pub const SCENE_APPLY_BLOCKED_MESSAGE: &str =
    "Scenes are reusable toolkits. Apply them from Agents or Projects.";

pub fn describe_scene_toolkit(
    config_dir: &Path,
    repo_path: &Path,
    scene_id: &str,
) -> Result<SceneToolkitSummary> {
    let scene_config = SceneConfigStore::new(config_dir.to_path_buf()).load()?;
    let scene = scene_config
        .scenes
        .get(scene_id)
        .ok_or_else(|| anyhow::anyhow!("Scene '{}' not found", scene_id))?
        .clone();
    let skills = scan_repo_skills(repo_path)?.skills;
    let disabled_skill_count =
        scene.disabled_skill_count(skills.iter().map(|skill| skill.id.as_str()));

    Ok(SceneToolkitSummary {
        scene_id: scene.id.clone(),
        scene_name: scene.name.clone(),
        enabled_skill_count: skills.len().saturating_sub(disabled_skill_count),
        disabled_skill_count,
        enabled_agent_count: scene.enabled_agent_keys.len(),
    })
}

pub fn apply_scene(
    _config_dir: &Path,
    _repo_path: &Path,
    _system_dirs: &AgentSystemDirs,
    _scene_id: &str,
) -> Result<ApplySceneResult> {
    Err(anyhow::anyhow!(SCENE_APPLY_BLOCKED_MESSAGE))
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
    fn describe_scene_toolkit_reports_counts_without_mutation() {
        let temp = tempfile::tempdir().unwrap();
        let config_dir = temp.path().join("config");
        let repo_dir = temp.path().join("repo");

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

        let before = AgentConfigStore::new(config_dir.clone()).load().unwrap();
        let result = describe_scene_toolkit(&config_dir, &repo_dir, "focus").unwrap();
        let after = AgentConfigStore::new(config_dir.clone()).load().unwrap();

        assert_eq!(result.scene_name, "Focus");
        assert_eq!(result.enabled_skill_count, 1);
        assert_eq!(result.disabled_skill_count, 1);
        assert_eq!(result.enabled_agent_count, 1);
        assert_eq!(after, before);
    }

    #[test]
    fn scene_toolkit_apply_does_not_rewrite_agent_selection() {
        let temp = tempfile::tempdir().unwrap();
        let config_dir = temp.path().join("config");
        let repo_dir = temp.path().join("repo");
        let target_dir = temp.path().join("targets/codex-skills");

        create_skill(&repo_dir, "custom/alpha");

        let scene_store = SceneConfigStore::new(config_dir.clone());
        scene_store.create_scene("focus", "Focus", "").unwrap();
        scene_store
            .set_scene_skills("focus", vec!["custom:alpha".to_string()])
            .unwrap();
        scene_store
            .set_scene_agents("focus", vec!["opencode".to_string()])
            .unwrap();

        let agent_store = AgentConfigStore::new(config_dir.clone());
        agent_store.set_agent_enabled("codex", true).unwrap();
        agent_store
            .set_agent_path_override("codex", target_dir.to_string_lossy().as_ref())
            .unwrap();
        agent_store
            .set_agent_selection(
                "codex",
                vec!["custom:alpha".to_string()],
                vec!["existing".to_string()],
                vec!["custom:beta".to_string()],
            )
            .unwrap();
        let before = agent_store.load().unwrap();

        let summary = describe_scene_toolkit(&config_dir, &repo_dir, "focus").unwrap();
        let error = apply_scene(
            &config_dir,
            &repo_dir,
            &AgentSystemDirs {
                home_dir: temp.path().join("home"),
                config_dir: Some(temp.path().join("config-home")),
            },
            "focus",
        )
        .unwrap_err();
        let after = agent_store.load().unwrap();

        assert_eq!(summary.scene_id, "focus");
        assert_eq!(error.to_string(), SCENE_APPLY_BLOCKED_MESSAGE);
        assert_eq!(after, before);
        assert!(!target_dir.exists());
    }
}
