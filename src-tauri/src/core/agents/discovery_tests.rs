use super::*;
use crate::core::agents::config::{AgentConfigEntry, AgentConfigSnapshot};
use std::fs;

fn empty_snapshot() -> AgentConfigSnapshot {
    AgentConfigSnapshot::default()
}

#[test]
fn detected_default_path_becomes_effective_path() {
    let temp_root = tempfile::tempdir().unwrap();
    let home_dir = temp_root.path().join("home");
    let config_dir = temp_root.path().join("config");
    fs::create_dir_all(home_dir.join(".codex/skills")).unwrap();

    let inventory = build_agent_inventory(
        &empty_snapshot(),
        &AgentSystemDirs {
            home_dir,
            config_dir: Some(config_dir),
        },
    );

    let codex = inventory
        .agents
        .iter()
        .find(|agent| agent.key == "codex")
        .unwrap();
    assert_eq!(codex.path_mode, AgentPathMode::Detected);
    assert!(codex.detected_skills_dir.is_some());
    assert_eq!(codex.effective_skills_dir, codex.detected_skills_dir);
}

#[test]
fn missing_default_path_reports_missing_mode() {
    let temp_root = tempfile::tempdir().unwrap();
    let inventory = build_agent_inventory(
        &empty_snapshot(),
        &AgentSystemDirs {
            home_dir: temp_root.path().join("home"),
            config_dir: Some(temp_root.path().join("config")),
        },
    );

    let claude = inventory
        .agents
        .iter()
        .find(|agent| agent.key == "claude_code")
        .unwrap();
    assert_eq!(claude.path_mode, AgentPathMode::Missing);
    assert_eq!(claude.detected_skills_dir, None);
    assert_eq!(claude.effective_skills_dir, None);
}

#[test]
fn path_override_takes_precedence_over_detected_path() {
    let temp_root = tempfile::tempdir().unwrap();
    let home_dir = temp_root.path().join("home");
    let override_dir = temp_root.path().join("override/opencode-skills");
    fs::create_dir_all(home_dir.join(".config/opencode/skills")).unwrap();

    let mut snapshot = AgentConfigSnapshot::default();
    snapshot.agents.insert(
        "opencode".to_string(),
        AgentConfigEntry {
            enabled: true,
            path_override: Some(override_dir.to_string_lossy().to_string()),
            ..AgentConfigEntry::default()
        },
    );

    let inventory = build_agent_inventory(
        &snapshot,
        &AgentSystemDirs {
            home_dir,
            config_dir: Some(temp_root.path().join("config")),
        },
    );

    let opencode = inventory
        .agents
        .iter()
        .find(|agent| agent.key == "opencode")
        .unwrap();
    assert_eq!(opencode.path_mode, AgentPathMode::Override);
    assert_eq!(
        opencode.effective_skills_dir.as_deref(),
        Some(override_dir.to_string_lossy().replace('\\', "/").as_str())
    );
}

#[test]
fn kimi_uses_hidden_home_skills_directory_when_detected() {
    let temp_root = tempfile::tempdir().unwrap();
    let home_dir = temp_root.path().join("home");
    fs::create_dir_all(home_dir.join(".kimi/skills")).unwrap();

    let inventory = build_agent_inventory(
        &empty_snapshot(),
        &AgentSystemDirs {
            home_dir,
            config_dir: Some(temp_root.path().join("config")),
        },
    );

    let kimi = inventory
        .agents
        .iter()
        .find(|agent| agent.key == "kimi")
        .unwrap();
    assert_eq!(kimi.path_mode, AgentPathMode::Detected);
    assert!(kimi
        .detected_skills_dir
        .as_deref()
        .is_some_and(|path| path.ends_with("/home/.kimi/skills")));
    assert_eq!(kimi.effective_skills_dir, kimi.detected_skills_dir);
}

#[test]
fn inventory_exposes_catalog_rules_for_project_preview() {
    let temp_root = tempfile::tempdir().unwrap();
    let inventory = build_agent_inventory(
        &empty_snapshot(),
        &AgentSystemDirs {
            home_dir: temp_root.path().join("home"),
            config_dir: Some(temp_root.path().join("config")),
        },
    );

    let codex = inventory
        .agents
        .iter()
        .find(|agent| agent.key == "codex")
        .unwrap();

    assert_eq!(codex.skills_dir_rule, ".codex/skills");
    assert_eq!(codex.project_skills_dir_rule, ".codex/skills");
    assert_eq!(codex.detect_dir_rule, ".codex");
}

#[test]
fn inventory_exposes_project_local_rules_separately() {
    let temp_root = tempfile::tempdir().unwrap();
    let inventory = build_agent_inventory(
        &empty_snapshot(),
        &AgentSystemDirs {
            home_dir: temp_root.path().join("home"),
            config_dir: Some(temp_root.path().join("config")),
        },
    );

    let opencode = inventory
        .agents
        .iter()
        .find(|agent| agent.key == "opencode")
        .unwrap();
    let cursor = inventory
        .agents
        .iter()
        .find(|agent| agent.key == "cursor")
        .unwrap();

    assert_eq!(opencode.skills_dir_rule, ".config/opencode/skills");
    assert_eq!(opencode.project_skills_dir_rule, ".opencode/skills");
    assert_eq!(cursor.project_skills_dir_rule, ".cursor/skills");
}
