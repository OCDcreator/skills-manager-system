use std::path::PathBuf;

use serde_json::Value;
use tempfile::tempdir;

use super::skills::run;
use crate::app_runtime::{AppRuntimeContext, AppRuntimeOptions, CliStatus};
use crate::cli::args::SkillsCommand;
use crate::core::skills::state::SkillStateStore;

#[test]
fn skills_list_joins_scan_and_state() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();

    std::fs::create_dir_all(repo_dir.path().join("custom/searxng")).unwrap();
    std::fs::create_dir_all(repo_dir.path().join("external/demo")).unwrap();
    std::fs::write(
        repo_dir.path().join("custom/searxng/SKILL.md"),
        "---\nname: SearXNG\ndescription: Search helper\n---",
    )
    .unwrap();
    std::fs::write(
        repo_dir.path().join("external/demo/SKILL.md"),
        "---\nname: Demo\ndescription: Demo skill\n---",
    )
    .unwrap();

    SkillStateStore::new(config_dir.path().to_path_buf())
        .set_skill_enabled(repo_dir.path(), "external:demo", false)
        .unwrap();

    let context = AppRuntimeContext::from_options_with_parts(
        AppRuntimeOptions {
            config_dir_override: Some(config_dir.path().to_path_buf()),
            repo_override: Some(repo_dir.path().to_path_buf()),
            ..AppRuntimeOptions::default()
        },
        PathBuf::from("/unused"),
        crate::app_runtime::DEVELOPMENT_APP_IDENTIFIER,
    );

    let result = run(&context, &SkillsCommand::List);
    let data = result.response.data.unwrap();
    let skills = data.get("skills").and_then(Value::as_array).unwrap();

    assert_eq!(result.response.status, CliStatus::Success);
    assert_eq!(skills.len(), 2);
    assert_eq!(
        skills[0].get("enabled").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        skills[1].get("enabled").and_then(Value::as_bool),
        Some(false)
    );
}

#[test]
fn skills_doc_accepts_skill_ids() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();

    std::fs::create_dir_all(repo_dir.path().join("custom/searxng")).unwrap();
    std::fs::create_dir_all(repo_dir.path().join("external")).unwrap();
    std::fs::write(
        repo_dir.path().join("custom/searxng/SKILL.md"),
        "---\nname: SearXNG\n---\n# Skill document",
    )
    .unwrap();

    let context = AppRuntimeContext::from_options_with_parts(
        AppRuntimeOptions {
            config_dir_override: Some(config_dir.path().to_path_buf()),
            repo_override: Some(repo_dir.path().to_path_buf()),
            ..AppRuntimeOptions::default()
        },
        PathBuf::from("/unused"),
        crate::app_runtime::DEVELOPMENT_APP_IDENTIFIER,
    );

    let result = run(
        &context,
        &SkillsCommand::Doc {
            target: "custom:searxng".to_string(),
        },
    );
    let document = result
        .response
        .data
        .unwrap()
        .get("document")
        .cloned()
        .unwrap();

    assert_eq!(
        document.get("id").and_then(Value::as_str),
        Some("custom:searxng")
    );
    assert_eq!(
        document.get("content").and_then(Value::as_str),
        Some("---\nname: SearXNG\n---\n# Skill document")
    );
}

#[test]
fn skills_disable_and_enable_mutate_state_with_lock() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    std::fs::create_dir_all(repo_dir.path().join("custom/searxng")).unwrap();
    std::fs::create_dir_all(repo_dir.path().join("external")).unwrap();
    std::fs::write(
        repo_dir.path().join("custom/searxng/SKILL.md"),
        "---\nname: SearXNG\n---",
    )
    .unwrap();

    let context = AppRuntimeContext::from_options_with_parts(
        AppRuntimeOptions {
            config_dir_override: Some(config_dir.path().to_path_buf()),
            repo_override: Some(repo_dir.path().to_path_buf()),
            ..AppRuntimeOptions::default()
        },
        PathBuf::from("/unused"),
        crate::app_runtime::DEVELOPMENT_APP_IDENTIFIER,
    );

    let disabled = run(
        &context,
        &SkillsCommand::Disable {
            skill_id: "custom:searxng".to_string(),
        },
    );
    assert_eq!(disabled.response.status, CliStatus::Success);
    assert_eq!(
        SkillStateStore::new(config_dir.path().to_path_buf())
            .load_for_repo(repo_dir.path())
            .unwrap()
            .disabled_skill_ids,
        vec!["custom:searxng".to_string()]
    );

    let enabled = run(
        &context,
        &SkillsCommand::Enable {
            skill_id: "custom:searxng".to_string(),
        },
    );
    assert_eq!(enabled.response.status, CliStatus::Success);
    assert!(SkillStateStore::new(config_dir.path().to_path_buf())
        .load_for_repo(repo_dir.path())
        .unwrap()
        .disabled_skill_ids
        .is_empty());
}

#[test]
fn skills_disable_rejects_unknown_skill_id() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    std::fs::create_dir_all(repo_dir.path().join("custom")).unwrap();
    std::fs::create_dir_all(repo_dir.path().join("external")).unwrap();
    let context = AppRuntimeContext::from_options_with_parts(
        AppRuntimeOptions {
            config_dir_override: Some(config_dir.path().to_path_buf()),
            repo_override: Some(repo_dir.path().to_path_buf()),
            ..AppRuntimeOptions::default()
        },
        PathBuf::from("/unused"),
        crate::app_runtime::DEVELOPMENT_APP_IDENTIFIER,
    );

    let result = run(
        &context,
        &SkillsCommand::Disable {
            skill_id: "custom:missing".to_string(),
        },
    );

    assert_eq!(result.response.error.unwrap().code, "skill_not_found");
}
