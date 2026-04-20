use std::fs;
use std::path::{Path, PathBuf};

use tempfile::tempdir;

use crate::core::agents::config::AgentConfigStore;
use crate::core::agents::discovery::AgentSystemDirs;
use crate::core::agents::manifest::SyncMode;
use crate::core::agents::sync::apply_agent_sync;
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

#[test]
fn apply_copies_only_enabled_skills() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();
    let skills_dir = target_dir.path().join("codex-skills");

    create_skill(repo_dir.path(), "custom/alpha");
    create_skill(repo_dir.path(), "external/vendor/beta");

    AgentConfigStore::new(config_dir.path().to_path_buf())
        .set_agent_enabled("codex", true)
        .unwrap();
    AgentConfigStore::new(config_dir.path().to_path_buf())
        .set_agent_path_override("codex", skills_dir.to_string_lossy().as_ref())
        .unwrap();

    let response = apply_agent_sync(
        config_dir.path(),
        repo_dir.path(),
        &AgentSystemDirs {
            home_dir: target_dir.path().join("home"),
            config_dir: Some(target_dir.path().join("config")),
        },
        SyncMode::Copy,
    )
    .unwrap();

    assert_eq!(response.enabled_skill_count, 2);
    assert!(skills_dir.join("custom--alpha/SKILL.md").exists());
    assert!(skills_dir.join("external--vendor--beta/SKILL.md").exists());
}

#[test]
fn disabled_skills_are_not_written() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();
    let skills_dir = target_dir.path().join("codex-skills");

    create_skill(repo_dir.path(), "custom/alpha");
    create_skill(repo_dir.path(), "external/vendor/beta");

    AgentConfigStore::new(config_dir.path().to_path_buf())
        .set_agent_enabled("codex", true)
        .unwrap();
    AgentConfigStore::new(config_dir.path().to_path_buf())
        .set_agent_path_override("codex", skills_dir.to_string_lossy().as_ref())
        .unwrap();
    SkillStateStore::new(config_dir.path().to_path_buf())
        .set_skill_enabled(repo_dir.path(), "external:vendor/beta", false)
        .unwrap();

    apply_agent_sync(
        config_dir.path(),
        repo_dir.path(),
        &AgentSystemDirs {
            home_dir: target_dir.path().join("home"),
            config_dir: Some(target_dir.path().join("config")),
        },
        SyncMode::Copy,
    )
    .unwrap();

    assert!(skills_dir.join("custom--alpha/SKILL.md").exists());
    assert!(!skills_dir.join("external--vendor--beta").exists());
}

#[test]
fn reapply_removes_stale_managed_entries() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();
    let skills_dir = target_dir.path().join("claude-skills");

    create_skill(repo_dir.path(), "custom/alpha");
    create_skill(repo_dir.path(), "external/vendor/beta");

    AgentConfigStore::new(config_dir.path().to_path_buf())
        .set_agent_enabled("claude_code", true)
        .unwrap();
    AgentConfigStore::new(config_dir.path().to_path_buf())
        .set_agent_path_override("claude_code", skills_dir.to_string_lossy().as_ref())
        .unwrap();

    apply_agent_sync(
        config_dir.path(),
        repo_dir.path(),
        &AgentSystemDirs {
            home_dir: target_dir.path().join("home"),
            config_dir: Some(target_dir.path().join("config")),
        },
        SyncMode::Copy,
    )
    .unwrap();

    SkillStateStore::new(config_dir.path().to_path_buf())
        .set_skill_enabled(repo_dir.path(), "external:vendor/beta", false)
        .unwrap();

    apply_agent_sync(
        config_dir.path(),
        repo_dir.path(),
        &AgentSystemDirs {
            home_dir: target_dir.path().join("home"),
            config_dir: Some(target_dir.path().join("config")),
        },
        SyncMode::Copy,
    )
    .unwrap();

    assert!(skills_dir.join("custom--alpha/SKILL.md").exists());
    assert!(!skills_dir.join("external--vendor--beta").exists());
}

#[test]
fn unmanaged_entries_are_not_deleted() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();
    let skills_dir = target_dir.path().join("opencode-skills");

    create_skill(repo_dir.path(), "custom/alpha");

    AgentConfigStore::new(config_dir.path().to_path_buf())
        .set_agent_enabled("opencode", true)
        .unwrap();
    AgentConfigStore::new(config_dir.path().to_path_buf())
        .set_agent_path_override("opencode", skills_dir.to_string_lossy().as_ref())
        .unwrap();

    apply_agent_sync(
        config_dir.path(),
        repo_dir.path(),
        &AgentSystemDirs {
            home_dir: target_dir.path().join("home"),
            config_dir: Some(target_dir.path().join("config")),
        },
        SyncMode::Copy,
    )
    .unwrap();

    fs::create_dir_all(skills_dir.join("manual-skill")).unwrap();
    fs::write(skills_dir.join("manual-skill/SKILL.md"), "# manual").unwrap();
    SkillStateStore::new(config_dir.path().to_path_buf())
        .set_skill_enabled(repo_dir.path(), "custom:alpha", false)
        .unwrap();

    apply_agent_sync(
        config_dir.path(),
        repo_dir.path(),
        &AgentSystemDirs {
            home_dir: target_dir.path().join("home"),
            config_dir: Some(target_dir.path().join("config")),
        },
        SyncMode::Copy,
    )
    .unwrap();

    assert!(skills_dir.join("manual-skill/SKILL.md").exists());
    assert!(!skills_dir.join("custom--alpha").exists());
}

#[test]
fn path_override_target_is_used() {
    let config_dir = tempdir().unwrap();
    let repo_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();
    let override_dir = target_dir.path().join("override/codex-skills");
    let detected_dir = target_dir.path().join("home/.codex/skills");

    create_skill(repo_dir.path(), "custom/alpha");
    fs::create_dir_all(&detected_dir).unwrap();

    AgentConfigStore::new(config_dir.path().to_path_buf())
        .set_agent_enabled("codex", true)
        .unwrap();
    AgentConfigStore::new(config_dir.path().to_path_buf())
        .set_agent_path_override("codex", override_dir.to_string_lossy().as_ref())
        .unwrap();

    apply_agent_sync(
        config_dir.path(),
        repo_dir.path(),
        &AgentSystemDirs {
            home_dir: target_dir.path().join("home"),
            config_dir: Some(target_dir.path().join("config")),
        },
        SyncMode::Copy,
    )
    .unwrap();

    assert!(override_dir.join("custom--alpha/SKILL.md").exists());
    assert!(!detected_dir.join("custom--alpha").exists());
}
