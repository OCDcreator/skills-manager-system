use std::fs;
use std::path::{Path, PathBuf};

use tempfile::tempdir;

use crate::core::agents::config::AgentConfigStore;
use crate::core::agents::discovery::AgentSystemDirs;
use crate::core::agents::sync::{apply_agent_sync, apply_agent_sync_for_agent};
use crate::core::agents::target_sync::SyncMode;
use crate::core::scenes::config::SceneConfigStore;
use crate::core::skills::state::SkillStateStore;
fn create_skill(repo_root: &Path, relative_path: &str) -> PathBuf {
    let skill_dir = repo_root.join(relative_path);
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        format!("---\nname: {}\ndescription: test\n---\n", relative_path),
    )
    .unwrap();
    skill_dir
}
fn test_system_dirs(root: &Path) -> AgentSystemDirs {
    AgentSystemDirs {
        home_dir: root.join("home"),
        config_dir: Some(root.join("config-home")),
    }
}
fn configure_agent(
    config_dir: &Path,
    key: &str,
    target_dir: &Path,
    skill_ids: Vec<&str>,
    scene_ids: Vec<&str>,
    excluded_ids: Vec<&str>,
) {
    let store = AgentConfigStore::new(config_dir.to_path_buf());
    store.set_agent_enabled(key, true).unwrap();
    store
        .set_agent_path_override(key, target_dir.to_string_lossy().as_ref())
        .unwrap();
    store
        .set_agent_selection(
            key,
            skill_ids.into_iter().map(str::to_string).collect(),
            scene_ids.into_iter().map(str::to_string).collect(),
            excluded_ids.into_iter().map(str::to_string).collect(),
        )
        .unwrap();
}

#[test]
fn selected_agent_skills_do_not_fall_back_to_all_global_skills() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    let target_root = tempdir().unwrap();
    let skills_dir = target_root.path().join("codex-skills");

    create_skill(repo_dir.path(), "custom/alpha");
    create_skill(repo_dir.path(), "external/vendor/beta");
    configure_agent(
        config_dir.path(),
        "codex",
        &skills_dir,
        vec!["custom:alpha"],
        vec![],
        vec![],
    );

    let response = apply_agent_sync(
        config_dir.path(),
        repo_dir.path(),
        &test_system_dirs(target_root.path()),
        SyncMode::Copy,
    )
    .unwrap();

    assert_eq!(response.enabled_skill_count, 2);
    assert!(skills_dir.join("custom--alpha/SKILL.md").exists());
    assert!(!skills_dir.join("external--vendor--beta").exists());
}

#[test]
fn globally_disabled_skills_are_excluded_even_when_agent_selected() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    let target_root = tempdir().unwrap();
    let skills_dir = target_root.path().join("codex-skills");

    create_skill(repo_dir.path(), "custom/alpha");
    create_skill(repo_dir.path(), "external/vendor/beta");
    configure_agent(
        config_dir.path(),
        "codex",
        &skills_dir,
        vec!["custom:alpha", "external:vendor/beta"],
        vec![],
        vec![],
    );
    SkillStateStore::new(config_dir.path().to_path_buf())
        .set_skill_enabled(repo_dir.path(), "external:vendor/beta", false)
        .unwrap();

    let response = apply_agent_sync(
        config_dir.path(),
        repo_dir.path(),
        &test_system_dirs(target_root.path()),
        SyncMode::Copy,
    )
    .unwrap();

    assert_eq!(response.enabled_skill_count, 1);
    assert!(skills_dir.join("custom--alpha/SKILL.md").exists());
    assert!(!skills_dir.join("external--vendor--beta").exists());
}

#[test]
fn scene_and_direct_skill_duplicates_are_written_once() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    let target_root = tempdir().unwrap();
    let skills_dir = target_root.path().join("codex-skills");

    create_skill(repo_dir.path(), "custom/alpha");
    create_skill(repo_dir.path(), "external/vendor/beta");
    let scene_store = SceneConfigStore::new(config_dir.path().to_path_buf());
    scene_store
        .create_scene("work", "Work", "")
        .unwrap();
    scene_store
        .set_scene_skills(
            "work",
            vec!["custom:alpha".to_string(), "external:vendor/beta".to_string()],
        )
        .unwrap();
    configure_agent(
        config_dir.path(),
        "codex",
        &skills_dir,
        vec!["custom:alpha"],
        vec!["work"],
        vec![],
    );

    let response = apply_agent_sync_for_agent(
        config_dir.path(),
        repo_dir.path(),
        &test_system_dirs(target_root.path()),
        SyncMode::Copy,
        "codex",
    )
    .unwrap();

    let codex_result = response
        .results
        .iter()
        .find(|result| result.key == "codex")
        .unwrap();
    assert_eq!(codex_result.written_count, 2);
    assert!(skills_dir.join("custom--alpha/SKILL.md").exists());
    assert!(skills_dir.join("external--vendor--beta/SKILL.md").exists());
}

#[test]
fn excluded_scene_skill_is_removed_on_reapply() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    let target_root = tempdir().unwrap();
    let skills_dir = target_root.path().join("codex-skills");

    create_skill(repo_dir.path(), "custom/alpha");
    create_skill(repo_dir.path(), "external/vendor/beta");
    let scene_store = SceneConfigStore::new(config_dir.path().to_path_buf());
    scene_store
        .create_scene("work", "Work", "")
        .unwrap();
    scene_store
        .set_scene_skills(
            "work",
            vec!["custom:alpha".to_string(), "external:vendor/beta".to_string()],
        )
        .unwrap();
    configure_agent(
        config_dir.path(),
        "codex",
        &skills_dir,
        vec![],
        vec!["work"],
        vec![],
    );
    apply_agent_sync_for_agent(
        config_dir.path(),
        repo_dir.path(),
        &test_system_dirs(target_root.path()),
        SyncMode::Copy,
        "codex",
    )
    .unwrap();

    AgentConfigStore::new(config_dir.path().to_path_buf())
        .set_agent_selection(
            "codex",
            vec![],
            vec!["work".to_string()],
            vec!["external:vendor/beta".to_string()],
        )
        .unwrap();
    apply_agent_sync_for_agent(
        config_dir.path(),
        repo_dir.path(),
        &test_system_dirs(target_root.path()),
        SyncMode::Copy,
        "codex",
    )
    .unwrap();

    assert!(skills_dir.join("custom--alpha/SKILL.md").exists());
    assert!(!skills_dir.join("external--vendor--beta").exists());
}

#[test]
fn scoped_apply_only_updates_requested_agent() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    let target_root = tempdir().unwrap();
    let codex_dir = target_root.path().join("codex-skills");
    let opencode_dir = target_root.path().join("opencode-skills");

    create_skill(repo_dir.path(), "custom/alpha");
    create_skill(repo_dir.path(), "external/vendor/beta");
    configure_agent(
        config_dir.path(),
        "codex",
        &codex_dir,
        vec!["custom:alpha"],
        vec![],
        vec![],
    );
    configure_agent(
        config_dir.path(),
        "opencode",
        &opencode_dir,
        vec!["external:vendor/beta"],
        vec![],
        vec![],
    );

    let response = apply_agent_sync_for_agent(
        config_dir.path(),
        repo_dir.path(),
        &test_system_dirs(target_root.path()),
        SyncMode::Copy,
        "codex",
    )
    .unwrap();

    assert_eq!(response.results.len(), 1);
    assert!(codex_dir.join("custom--alpha/SKILL.md").exists());
    assert!(!opencode_dir.join("external--vendor--beta").exists());
}

#[test]
fn disabled_agent_still_cleans_managed_entries() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    let target_root = tempdir().unwrap();
    let skills_dir = target_root.path().join("claude-skills");

    create_skill(repo_dir.path(), "custom/alpha");
    configure_agent(
        config_dir.path(),
        "claude_code",
        &skills_dir,
        vec!["custom:alpha"],
        vec![],
        vec![],
    );
    apply_agent_sync_for_agent(
        config_dir.path(),
        repo_dir.path(),
        &test_system_dirs(target_root.path()),
        SyncMode::Copy,
        "claude_code",
    )
    .unwrap();

    AgentConfigStore::new(config_dir.path().to_path_buf())
        .set_agent_enabled("claude_code", false)
        .unwrap();
    apply_agent_sync_for_agent(
        config_dir.path(),
        repo_dir.path(),
        &test_system_dirs(target_root.path()),
        SyncMode::Copy,
        "claude_code",
    )
    .unwrap();

    assert!(!skills_dir.join("custom--alpha").exists());
}

#[test]
fn path_override_target_is_used() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    let target_root = tempdir().unwrap();
    let override_dir = target_root.path().join("override/codex-skills");
    let detected_dir = target_root.path().join("home/.codex/skills");

    create_skill(repo_dir.path(), "custom/alpha");
    fs::create_dir_all(&detected_dir).unwrap();
    configure_agent(
        config_dir.path(),
        "codex",
        &override_dir,
        vec!["custom:alpha"],
        vec![],
        vec![],
    );

    apply_agent_sync_for_agent(
        config_dir.path(),
        repo_dir.path(),
        &test_system_dirs(target_root.path()),
        SyncMode::Copy,
        "codex",
    )
    .unwrap();

    assert!(override_dir.join("custom--alpha/SKILL.md").exists());
    assert!(!detected_dir.join("custom--alpha").exists());
}
