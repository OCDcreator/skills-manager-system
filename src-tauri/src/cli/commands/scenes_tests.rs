use std::path::PathBuf;

use serde_json::Value;
use tempfile::tempdir;

use super::scenes::{apply_with_system_dirs, run};
use crate::app_runtime::{AppRuntimeContext, AppRuntimeOptions, CliStatus};
use crate::cli::args::ScenesCommand;
use crate::core::agents::config::AgentConfigStore;
use crate::core::agents::discovery::AgentSystemDirs;

fn test_context(config_dir: PathBuf, repo_dir: Option<PathBuf>) -> AppRuntimeContext {
    AppRuntimeContext::from_options_with_parts(
        AppRuntimeOptions {
            config_dir_override: Some(config_dir),
            repo_override: repo_dir,
            ..AppRuntimeOptions::default()
        },
        PathBuf::from("/unused"),
        crate::app_runtime::DEVELOPMENT_APP_IDENTIFIER,
    )
}

#[test]
fn scenes_mutations_update_config_snapshot() {
    let temp = tempdir().unwrap();
    let context = test_context(temp.path().join("config"), None);

    let created = run(
        &context,
        &ScenesCommand::Create {
            id: "focus".to_string(),
            name: "Focus".to_string(),
            description: "Deep work".to_string(),
        },
    );
    assert_eq!(created.response.status, CliStatus::Success);

    let updated = run(
        &context,
        &ScenesCommand::Update {
            id: "focus".to_string(),
            name: Some("Focus+".to_string()),
            description: None,
        },
    );
    let config = updated
        .response
        .data
        .unwrap()
        .get("sceneConfig")
        .cloned()
        .unwrap();
    assert_eq!(config["scenes"]["focus"]["name"].as_str(), Some("Focus+"));

    run(
        &context,
        &ScenesCommand::SetSkills {
            id: "focus".to_string(),
            disabled_skill_ids: vec!["custom:beta".to_string()],
        },
    );
    run(
        &context,
        &ScenesCommand::SetAgents {
            id: "focus".to_string(),
            enabled_agent_keys: vec!["codex".to_string()],
        },
    );
    let ordered = run(
        &context,
        &ScenesCommand::SetSkillOrder {
            id: "focus".to_string(),
            skill_order: vec!["custom:alpha".to_string(), "custom:beta".to_string()],
        },
    );
    assert_eq!(
        ordered.response.data.unwrap()["sceneConfig"]["scenes"]["focus"]["skillOrder"]
            .as_array()
            .unwrap()
            .len(),
        2
    );

    let active = run(
        &context,
        &ScenesCommand::SetActive {
            id: "focus".to_string(),
        },
    );
    assert_eq!(
        active.response.data.unwrap()["sceneConfig"]["activeSceneId"].as_str(),
        Some("focus")
    );

    let cleared = run(&context, &ScenesCommand::ClearActive);
    assert_eq!(
        cleared.response.data.unwrap()["sceneConfig"]["activeSceneId"],
        Value::Null
    );

    let deleted = run(
        &context,
        &ScenesCommand::Delete {
            id: "focus".to_string(),
        },
    );
    assert!(deleted.response.data.unwrap()["sceneConfig"]["scenes"]
        .as_object()
        .unwrap()
        .is_empty());
}

#[test]
fn scenes_apply_syncs_agent_targets_and_sets_active_scene() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");
    let target_dir = temp.path().join("targets/codex-skills");
    std::fs::create_dir_all(repo_dir.join("custom/alpha")).unwrap();
    std::fs::create_dir_all(repo_dir.join("custom/beta")).unwrap();
    std::fs::write(repo_dir.join("custom/alpha/SKILL.md"), "# Alpha").unwrap();
    std::fs::write(repo_dir.join("custom/beta/SKILL.md"), "# Beta").unwrap();

    let context = test_context(config_dir.clone(), Some(repo_dir.clone()));
    run(
        &context,
        &ScenesCommand::Create {
            id: "focus".to_string(),
            name: "Focus".to_string(),
            description: String::new(),
        },
    );
    run(
        &context,
        &ScenesCommand::SetSkills {
            id: "focus".to_string(),
            disabled_skill_ids: vec!["custom:beta".to_string()],
        },
    );
    run(
        &context,
        &ScenesCommand::SetAgents {
            id: "focus".to_string(),
            enabled_agent_keys: vec!["codex".to_string()],
        },
    );

    let agent_store = AgentConfigStore::new(config_dir.clone());
    agent_store.set_agent_enabled("codex", true).unwrap();
    agent_store
        .set_agent_path_override("codex", target_dir.to_string_lossy().as_ref())
        .unwrap();

    let result = apply_with_system_dirs(
        &context,
        &repo_dir,
        &AgentSystemDirs {
            home_dir: temp.path().join("home"),
            config_dir: Some(temp.path().join("xdg")),
        },
        "focus",
    );

    assert_eq!(result.response.status, CliStatus::Success);
    assert!(target_dir.join("custom--alpha/SKILL.md").exists());
    assert!(!target_dir.join("custom--beta").exists());
}
