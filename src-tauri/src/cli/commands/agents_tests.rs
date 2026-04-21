use std::path::PathBuf;

use serde_json::Value;
use tempfile::tempdir;

use super::agent_sync::{sync_failure_error, sync_with_system_dirs};
use super::agents::run;
use crate::app_runtime::{AppRuntimeContext, AppRuntimeOptions, CliStatus};
use crate::cli::args::AgentsCommand;
use crate::core::agents::config::AgentConfigStore;
use crate::core::agents::discovery::AgentSystemDirs;
use crate::core::agents::target_sync::SyncMode;

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
fn agents_enable_and_path_mutations_return_updated_config() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let context = test_context(config_dir.clone(), None);
    let override_dir = temp.path().join("override/codex-skills");

    let enabled = run(
        &context,
        &AgentsCommand::Enable {
            key: "codex".to_string(),
        },
    );
    assert_eq!(enabled.response.status, CliStatus::Success);

    let set_path = run(
        &context,
        &AgentsCommand::SetPath {
            key: "codex".to_string(),
            path: override_dir.to_string_lossy().to_string(),
        },
    );
    let config = set_path
        .response
        .data
        .unwrap()
        .get("agentConfig")
        .cloned()
        .unwrap();

    assert_eq!(
        config["agents"]["codex"]["pathOverride"].as_str(),
        Some(override_dir.to_string_lossy().as_ref())
    );
    assert_eq!(
        AgentConfigStore::new(config_dir).load().unwrap().agents["codex"]
            .path_override
            .as_deref(),
        Some(override_dir.to_string_lossy().as_ref())
    );
}

#[test]
fn agents_sync_conflicts_return_partial_status() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");
    let target_dir = temp.path().join("targets/codex-skills");
    std::fs::create_dir_all(repo_dir.join("custom/alpha")).unwrap();
    std::fs::write(repo_dir.join("custom/alpha/SKILL.md"), "# Alpha").unwrap();
    std::fs::create_dir_all(&target_dir).unwrap();
    std::fs::create_dir_all(target_dir.join("custom--alpha")).unwrap();

    let store = AgentConfigStore::new(config_dir.clone());
    store.set_agent_enabled("codex", true).unwrap();
    store
        .set_agent_path_override("codex", target_dir.to_string_lossy().as_ref())
        .unwrap();

    let context = test_context(config_dir, Some(repo_dir.clone()));
    let result = sync_with_system_dirs(
        &context,
        &repo_dir,
        &AgentSystemDirs {
            home_dir: temp.path().join("home"),
            config_dir: Some(temp.path().join("xdg")),
        },
        SyncMode::Copy,
    );

    assert_eq!(result.response.status, CliStatus::Partial);
    assert_eq!(result.code(), 8);
    assert_eq!(
        result.response.warnings[0].code,
        "agent_sync_conflicts".to_string()
    );
}

#[test]
fn windows_symlink_failure_maps_to_stable_code() {
    let error = sync_failure_error(
        "Failed to symlink dir C:/src -> C:/dst (Developer Mode may be required)",
        Some(Value::Null),
    );

    assert_eq!(error.code, "windows_symlink_privilege_required");
}
