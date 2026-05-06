use std::path::PathBuf;

use tempfile::tempdir;

use super::projects::{apply_with_system_dirs, run};
use crate::app_runtime::{AppRuntimeContext, AppRuntimeOptions, CliStatus};
use crate::cli::args::ProjectsCommand;
use crate::core::agents::discovery::AgentSystemDirs;
use crate::core::projects::project_paths::normalize_project_path;

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

fn project_key(path: &std::path::Path) -> String {
    normalize_project_path(path.to_string_lossy().as_ref()).unwrap()
}

#[test]
fn projects_add_update_list_and_remove_round_trip() {
    let temp = tempdir().unwrap();
    let project_dir = temp.path().join("workspace/app");
    let context = test_context(temp.path().join("config"), None);

    let added = run(
        &context,
        &ProjectsCommand::Add {
            project_path: project_dir.clone(),
            display_name: "App".to_string(),
            skill_ids: vec!["custom:alpha".to_string()],
            agent_keys: vec!["codex".to_string()],
            agent_skill_ids: Vec::new(),
            agent_scene_ids: Vec::new(),
            agent_excluded_skill_ids: Vec::new(),
        },
    );
    assert_eq!(added.response.status, CliStatus::Success);

    let listed = run(&context, &ProjectsCommand::List);
    assert!(listed.response.data.unwrap()["projectConfig"]["projects"]
        .as_object()
        .unwrap()
        .contains_key(project_key(&project_dir).as_str()));

    let updated = run(
        &context,
        &ProjectsCommand::Update {
            project_path: project_dir.clone(),
            display_name: Some("App+".to_string()),
            skill_ids: vec!["custom:beta".to_string()],
            agent_keys: vec!["codex".to_string()],
            agent_skill_ids: Vec::new(),
            agent_scene_ids: Vec::new(),
            agent_excluded_skill_ids: Vec::new(),
        },
    );
    assert_eq!(
        updated.response.data.unwrap()["projectConfig"]["projects"][project_key(&project_dir)]
            ["displayName"]
            .as_str(),
        Some("App+")
    );

    let removed = run(
        &context,
        &ProjectsCommand::Remove {
            project_path: project_dir.clone(),
        },
    );
    assert!(removed.response.data.unwrap()["projectConfig"]["projects"]
        .as_object()
        .unwrap()
        .is_empty());
}

#[test]
fn projects_apply_uses_full_snapshot_without_path_argument() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");
    let project_a = temp.path().join("workspace/a");
    let project_b = temp.path().join("workspace/b");
    std::fs::create_dir_all(repo_dir.join("custom/alpha")).unwrap();
    std::fs::create_dir_all(repo_dir.join("custom/beta")).unwrap();
    std::fs::write(repo_dir.join("custom/alpha/SKILL.md"), "# Alpha").unwrap();
    std::fs::write(repo_dir.join("custom/beta/SKILL.md"), "# Beta").unwrap();

    let context = test_context(config_dir, Some(repo_dir.clone()));
    run(
        &context,
        &ProjectsCommand::Add {
            project_path: project_a.clone(),
            display_name: "A".to_string(),
            skill_ids: vec!["custom:alpha".to_string()],
            agent_keys: vec!["codex".to_string()],
            agent_skill_ids: Vec::new(),
            agent_scene_ids: Vec::new(),
            agent_excluded_skill_ids: Vec::new(),
        },
    );
    run(
        &context,
        &ProjectsCommand::Add {
            project_path: project_b.clone(),
            display_name: "B".to_string(),
            skill_ids: vec!["custom:beta".to_string()],
            agent_keys: vec!["codex".to_string()],
            agent_skill_ids: Vec::new(),
            agent_scene_ids: Vec::new(),
            agent_excluded_skill_ids: Vec::new(),
        },
    );

    let result = apply_with_system_dirs(
        &context,
        &repo_dir,
        &AgentSystemDirs {
            home_dir: temp.path().join("home"),
            config_dir: Some(temp.path().join("xdg")),
        },
    );

    assert_eq!(result.response.status, CliStatus::Success);
    assert!(project_a.join(".codex/skills/alpha/SKILL.md").exists());
    assert!(project_b.join(".codex/skills/beta/SKILL.md").exists());
}

#[test]
fn projects_add_accepts_per_agent_project_layers() {
    let temp = tempdir().unwrap();
    let project_dir = temp.path().join("workspace/app");
    let context = test_context(temp.path().join("config"), None);

    let added = run(
        &context,
        &ProjectsCommand::Add {
            project_path: project_dir.clone(),
            display_name: "App".to_string(),
            skill_ids: vec!["custom:legacy".to_string()],
            agent_keys: vec!["codex".to_string()],
            agent_skill_ids: vec!["opencode=custom:direct".to_string()],
            agent_scene_ids: vec!["codex=focus".to_string()],
            agent_excluded_skill_ids: vec!["codex=custom:legacy".to_string()],
        },
    );

    assert_eq!(added.response.status, CliStatus::Success);
    let project =
        &added.response.data.unwrap()["projectConfig"]["projects"][project_key(&project_dir)];
    assert_eq!(
        project["agents"]["codex"]["selectedSkillIds"],
        serde_json::json!(["custom:legacy"])
    );
    assert_eq!(
        project["agents"]["codex"]["selectedSceneIds"],
        serde_json::json!(["focus"])
    );
    assert_eq!(
        project["agents"]["codex"]["excludedSkillIds"],
        serde_json::json!(["custom:legacy"])
    );
    assert_eq!(
        project["agents"]["opencode"]["selectedSkillIds"],
        serde_json::json!(["custom:direct"])
    );
}

#[test]
fn projects_update_rejects_invalid_per_agent_pair() {
    let temp = tempdir().unwrap();
    let project_dir = temp.path().join("workspace/app");
    let context = test_context(temp.path().join("config"), None);

    let result = run(
        &context,
        &ProjectsCommand::Update {
            project_path: project_dir,
            display_name: None,
            skill_ids: Vec::new(),
            agent_keys: Vec::new(),
            agent_skill_ids: Vec::new(),
            agent_scene_ids: vec!["focus".to_string()],
            agent_excluded_skill_ids: Vec::new(),
        },
    );

    assert_eq!(result.response.status, CliStatus::Error);
    assert_eq!(result.response.error.unwrap().code, "invalid_project_path");
}
