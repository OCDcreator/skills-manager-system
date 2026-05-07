use std::path::PathBuf;

use serde_json::Value;
use tempfile::tempdir;

use super::skills::run;
use crate::app_runtime::config_lock::acquire_config_lock;
use crate::app_runtime::{AppRuntimeContext, AppRuntimeOptions, CliStatus};
use crate::cli::args::SkillsCommand;
use crate::core::external_sources::models::{
    ExternalSourceRecord, ExternalSourcesSnapshot, ImportedExternalSkillRecord,
    ManagedSkillMirrorManifest,
};
use crate::core::external_sources::store::ExternalSourcesStore;
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
fn skills_list_enriches_managed_external_mirrors() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    let skill_relative_path = "external/managed/github/demo__repo/codex/impeccable";
    let skill_dir = repo_dir.path().join(skill_relative_path);

    std::fs::create_dir_all(repo_dir.path().join("custom")).unwrap();
    std::fs::create_dir_all(&skill_dir).unwrap();
    std::fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: Impeccable\ndescription: Managed mirror\n---",
    )
    .unwrap();
    std::fs::write(
        skill_dir.join(".skills-manager-source.json"),
        serde_json::to_string_pretty(&ManagedSkillMirrorManifest {
            schema_version: 1,
            managed: true,
            import_id: "imp_01".to_string(),
            source_id: "src_01".to_string(),
            repo_url: "https://github.com/demo/repo".to_string(),
            agent_key: "codex".to_string(),
            variant_path: "dist/agents/.agents/skills/impeccable".to_string(),
            mirror_relative_path: skill_relative_path.to_string(),
            skill_id: "external:managed/github/demo__repo/codex/impeccable".to_string(),
            pinned_commit: "abc123".to_string(),
        })
        .unwrap(),
    )
    .unwrap();

    let store = ExternalSourcesStore::new(config_dir.path().to_path_buf());
    let guard = acquire_config_lock(config_dir.path()).unwrap();
    store
        .save(
            &guard,
            &ExternalSourcesSnapshot {
                schema_version: 1,
                sources: vec![ExternalSourceRecord {
                    id: "src_01".to_string(),
                    repo_url: "https://github.com/demo/repo".to_string(),
                    ..ExternalSourceRecord::default()
                }],
                imports: vec![ImportedExternalSkillRecord {
                    import_id: "imp_01".to_string(),
                    external_source_id: "src_01".to_string(),
                    agent_key: "codex".to_string(),
                    upstream_variant_path: "dist/agents/.agents/skills/impeccable".to_string(),
                    pinned_commit: "abc123".to_string(),
                    pinned_variant_fingerprint: Some("sha256:demo".to_string()),
                    skill_id: "external:managed/github/demo__repo/codex/impeccable".to_string(),
                    mirror_relative_path: skill_relative_path.to_string(),
                    last_checked_commit: Some("abc123".to_string()),
                    imported_at: Some("2026-04-29T00:00:00Z".to_string()),
                    warnings: Vec::new(),
                    update_available: true,
                }],
            },
        )
        .unwrap();
    drop(guard);

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
    let managed = skills
        .iter()
        .find(|item| {
            item.get("id").and_then(Value::as_str)
                == Some("external:managed/github/demo__repo/codex/impeccable")
        })
        .unwrap();

    assert_eq!(result.response.status, CliStatus::Success);
    assert_eq!(
        managed
            .get("managedSource")
            .and_then(|value| value.get("importId"))
            .and_then(Value::as_str),
        Some("imp_01")
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
