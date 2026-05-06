use super::*;
use crate::core::agents::config::AgentConfigStore;
use crate::core::agents::discovery::load_agent_inventory;
use crate::core::agents::selection::{
    load_skill_selection_context, resolve_agent_skill_selection,
    resolve_project_agent_skill_selection,
};
use crate::core::agents::sync::{apply_agent_sync, AgentApplyStatus};
use crate::core::projects::store::ProjectAgentAssignment;
use crate::core::projects::store::ProjectConfigStore;
use crate::core::scenes::config::SceneConfigStore;
use crate::core::settings::{AgentSyncMode, SettingsStore};
use crate::core::skills::scan::SkillSummary;
use crate::core::skills::state::SkillStateStore;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn create_skill(repo_dir: &Path, relative_path: &str, title: &str) -> String {
    let skill_dir = repo_dir.join(relative_path);
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), format!("# {title}\n")).unwrap();
    relative_path.replace('/', ":")
}

fn create_scene(config_dir: &Path, scene_id: &str, skill_ids: Vec<&str>) {
    let store = SceneConfigStore::new(config_dir.to_path_buf());
    store.create_scene(scene_id, scene_id, "").unwrap();
    store
        .set_scene_skills(
            scene_id,
            skill_ids.into_iter().map(str::to_string).collect(),
        )
        .unwrap();
}

fn test_system_dirs(root: &Path) -> AgentSystemDirs {
    AgentSystemDirs {
        home_dir: root.join("home"),
        config_dir: Some(root.join("xdg")),
    }
}

#[test]
fn project_sync_uses_project_local_opencode_directory() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");
    let project_dir = temp.path().join("project");
    fs::create_dir_all(&project_dir).unwrap();
    let skill_id = create_skill(&repo_dir, "custom/alpha", "Alpha");

    ProjectConfigStore::new(config_dir.clone())
        .add_project(
            project_dir.to_string_lossy().as_ref(),
            "Project",
            vec![skill_id],
            vec!["opencode".to_string()],
        )
        .unwrap();

    let response = apply_project_assignments(
        &config_dir,
        &repo_dir,
        &AgentSystemDirs {
            home_dir: temp.path().join("home"),
            config_dir: Some(temp.path().join("xdg")),
        },
    )
    .unwrap();

    let target_dir = response.results[0].results[0].target_dir.as_ref().unwrap();
    assert!(target_dir.ends_with("/project/.opencode/skills"));
    assert!(project_dir.join(".opencode/skills/alpha/SKILL.md").exists());
    assert!(!project_dir
        .join(".config/opencode/skills/alpha/SKILL.md")
        .exists());
}

#[test]
fn project_sync_inherits_global_agent_scene_and_adds_project_scene() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");
    let project_dir = temp.path().join("project");
    fs::create_dir_all(&project_dir).unwrap();
    let global_skill_id = create_skill(&repo_dir, "custom/global", "Global");
    let project_skill_id = create_skill(&repo_dir, "custom/project", "Project");
    create_scene(&config_dir, "global-scene", vec![&global_skill_id]);
    create_scene(&config_dir, "project-scene", vec![&project_skill_id]);

    AgentConfigStore::new(config_dir.clone())
        .set_agent_selection("codex", vec![], vec!["global-scene".to_string()], vec![])
        .unwrap();
    ProjectConfigStore::new(config_dir.clone())
        .add_project_with_agents(
            project_dir.to_string_lossy().as_ref(),
            "Project",
            BTreeMap::from([(
                "codex".to_string(),
                ProjectAgentAssignment {
                    selected_skill_ids: Vec::new(),
                    selected_scene_ids: vec!["project-scene".to_string()],
                    excluded_skill_ids: Vec::new(),
                },
            )]),
        )
        .unwrap();

    apply_project_assignments(&config_dir, &repo_dir, &test_system_dirs(temp.path())).unwrap();

    assert!(project_dir.join(".codex/skills/global/SKILL.md").exists());
    assert!(project_dir.join(".codex/skills/project/SKILL.md").exists());
}

#[test]
fn project_exclusion_removes_inherited_skill_only_from_project_target() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");
    let global_target = temp.path().join("global-codex-skills");
    let project_dir = temp.path().join("project");
    fs::create_dir_all(&project_dir).unwrap();
    let inherited_skill_id = create_skill(&repo_dir, "custom/inherited", "Inherited");
    let project_skill_id = create_skill(&repo_dir, "custom/project", "Project");
    create_scene(&config_dir, "global-scene", vec![&inherited_skill_id]);

    let agent_store = AgentConfigStore::new(config_dir.clone());
    agent_store.set_agent_enabled("codex", true).unwrap();
    agent_store
        .set_agent_path_override("codex", global_target.to_string_lossy().as_ref())
        .unwrap();
    agent_store
        .set_agent_selection("codex", vec![], vec!["global-scene".to_string()], vec![])
        .unwrap();
    ProjectConfigStore::new(config_dir.clone())
        .add_project_with_agents(
            project_dir.to_string_lossy().as_ref(),
            "Project",
            BTreeMap::from([(
                "codex".to_string(),
                ProjectAgentAssignment {
                    selected_skill_ids: vec![project_skill_id],
                    selected_scene_ids: Vec::new(),
                    excluded_skill_ids: vec![inherited_skill_id],
                },
            )]),
        )
        .unwrap();

    apply_agent_sync(
        &config_dir,
        &repo_dir,
        &test_system_dirs(temp.path()),
        SyncMode::Copy,
    )
    .unwrap();
    apply_project_assignments(&config_dir, &repo_dir, &test_system_dirs(temp.path())).unwrap();

    assert!(global_target.join("inherited/SKILL.md").exists());
    assert!(!project_dir.join(".codex/skills/inherited").exists());
    assert!(project_dir.join(".codex/skills/project/SKILL.md").exists());
}

#[test]
fn globally_disabled_project_layer_skills_are_not_written() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");
    let project_dir = temp.path().join("project");
    fs::create_dir_all(&project_dir).unwrap();
    let global_disabled_id = create_skill(&repo_dir, "custom/global-disabled", "Global Disabled");
    let project_disabled_id =
        create_skill(&repo_dir, "custom/project-disabled", "Project Disabled");
    let active_id = create_skill(&repo_dir, "custom/active", "Active");
    create_scene(
        &config_dir,
        "global-scene",
        vec![&global_disabled_id, &active_id],
    );
    SkillStateStore::new(config_dir.clone())
        .set_skill_enabled(&repo_dir, &global_disabled_id, false)
        .unwrap();
    SkillStateStore::new(config_dir.clone())
        .set_skill_enabled(&repo_dir, &project_disabled_id, false)
        .unwrap();

    AgentConfigStore::new(config_dir.clone())
        .set_agent_selection("codex", vec![], vec!["global-scene".to_string()], vec![])
        .unwrap();
    ProjectConfigStore::new(config_dir.clone())
        .add_project_with_agents(
            project_dir.to_string_lossy().as_ref(),
            "Project",
            BTreeMap::from([(
                "codex".to_string(),
                ProjectAgentAssignment {
                    selected_skill_ids: vec![project_disabled_id],
                    selected_scene_ids: Vec::new(),
                    excluded_skill_ids: Vec::new(),
                },
            )]),
        )
        .unwrap();

    apply_project_assignments(&config_dir, &repo_dir, &test_system_dirs(temp.path())).unwrap();

    assert!(project_dir.join(".codex/skills/active/SKILL.md").exists());
    assert!(!project_dir.join(".codex/skills/global-disabled").exists());
    assert!(!project_dir.join(".codex/skills/project-disabled").exists());
}

#[test]
fn missing_project_refs_report_diagnostics_and_apply_without_panic() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");
    let project_dir = temp.path().join("project");
    fs::create_dir_all(&project_dir).unwrap();
    let active_id = create_skill(&repo_dir, "custom/active", "Active");
    let project_agent = ProjectAgentAssignment {
        selected_skill_ids: vec![active_id, "custom:missing".to_string()],
        selected_scene_ids: vec!["missing-scene".to_string()],
        excluded_skill_ids: Vec::new(),
    };
    ProjectConfigStore::new(config_dir.clone())
        .add_project_with_agents(
            project_dir.to_string_lossy().as_ref(),
            "Project",
            BTreeMap::from([("codex".to_string(), project_agent.clone())]),
        )
        .unwrap();

    let response =
        apply_project_assignments(&config_dir, &repo_dir, &test_system_dirs(temp.path())).unwrap();

    let context = load_skill_selection_context(&config_dir, &repo_dir).unwrap();
    let agent = load_agent_inventory(&config_dir, &test_system_dirs(temp.path()))
        .unwrap()
        .agents
        .into_iter()
        .find(|agent| agent.key == "codex")
        .unwrap();
    let global_result = resolve_agent_skill_selection(&agent, &context);
    let project_result =
        resolve_project_agent_skill_selection(&global_result, &project_agent, &context);

    assert!(project_dir.join(".codex/skills/active/SKILL.md").exists());
    assert_eq!(
        project_result.diagnostics.missing_scene_ids,
        vec!["missing-scene".to_string()]
    );
    assert!(!project_dir.join(".codex/skills/missing").exists());
    assert_eq!(
        project_result.diagnostics.missing_skill_ids,
        vec!["custom:missing".to_string()]
    );
    let agent_result = &response.results[0].results[0];
    assert_eq!(agent_result.status, AgentApplyStatus::Partial);
    assert!(agent_result
        .message
        .contains("missing scenes: missing-scene"));
    assert!(agent_result
        .message
        .contains("missing skills: custom:missing"));
}

#[test]
fn project_sync_reports_globally_disabled_reference_warnings() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");
    let project_dir = temp.path().join("project");
    fs::create_dir_all(&project_dir).unwrap();
    let disabled_id = create_skill(&repo_dir, "custom/disabled", "Disabled");
    SkillStateStore::new(config_dir.clone())
        .set_skill_enabled(&repo_dir, &disabled_id, false)
        .unwrap();
    ProjectConfigStore::new(config_dir.clone())
        .add_project_with_agents(
            project_dir.to_string_lossy().as_ref(),
            "Project",
            BTreeMap::from([(
                "codex".to_string(),
                ProjectAgentAssignment {
                    selected_skill_ids: vec![disabled_id],
                    selected_scene_ids: Vec::new(),
                    excluded_skill_ids: Vec::new(),
                },
            )]),
        )
        .unwrap();

    let response =
        apply_project_assignments(&config_dir, &repo_dir, &test_system_dirs(temp.path())).unwrap();

    let agent_result = &response.results[0].results[0];
    assert_eq!(agent_result.status, AgentApplyStatus::Partial);
    assert!(agent_result
        .message
        .contains("globally disabled: custom:disabled"));
    assert!(!project_dir.join(".codex/skills/disabled").exists());
}

#[test]
fn project_sync_uses_saved_copy_mode() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");
    let project_dir = temp.path().join("project");
    fs::create_dir_all(&project_dir).unwrap();
    let skill_id = create_skill(&repo_dir, "custom/alpha", "Alpha");
    SettingsStore::new(config_dir.clone())
        .save_agent_sync_mode(AgentSyncMode::Copy)
        .unwrap();

    ProjectConfigStore::new(config_dir.clone())
        .add_project(
            project_dir.to_string_lossy().as_ref(),
            "Project",
            vec![skill_id],
            vec!["codex".to_string()],
        )
        .unwrap();

    apply_project_assignments(&config_dir, &repo_dir, &test_system_dirs(temp.path())).unwrap();

    let metadata = fs::symlink_metadata(project_dir.join(".codex/skills/alpha")).unwrap();
    assert!(metadata.is_dir());
    assert!(!metadata.file_type().is_symlink());
}

#[cfg(not(windows))]
#[test]
fn project_sync_applies_saved_symlink_mode() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");
    let project_dir = temp.path().join("project");
    fs::create_dir_all(&project_dir).unwrap();
    let skill_id = create_skill(&repo_dir, "custom/alpha", "Alpha");
    SettingsStore::new(config_dir.clone())
        .save_agent_sync_mode(AgentSyncMode::Symlink)
        .unwrap();
    ProjectConfigStore::new(config_dir.clone())
        .add_project(
            project_dir.to_string_lossy().as_ref(),
            "Project",
            vec![skill_id],
            vec!["codex".to_string()],
        )
        .unwrap();

    apply_project_assignments(&config_dir, &repo_dir, &test_system_dirs(temp.path())).unwrap();

    let metadata = fs::symlink_metadata(project_dir.join(".codex/skills/alpha")).unwrap();
    assert!(metadata.file_type().is_symlink());
}

#[test]
fn project_sync_keeps_cursor_and_claude_targets_independent() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");
    let project_dir = temp.path().join("project");
    fs::create_dir_all(&project_dir).unwrap();
    let skill_id = create_skill(&repo_dir, "custom/alpha", "Alpha");

    ProjectConfigStore::new(config_dir.clone())
        .add_project(
            project_dir.to_string_lossy().as_ref(),
            "Project",
            vec![skill_id],
            vec!["claude_code".to_string(), "cursor".to_string()],
        )
        .unwrap();

    let response = apply_project_assignments(
        &config_dir,
        &repo_dir,
        &AgentSystemDirs {
            home_dir: temp.path().join("home"),
            config_dir: Some(temp.path().join("xdg")),
        },
    )
    .unwrap();

    assert_eq!(response.results[0].results.len(), 2);
    assert!(project_dir.join(".claude/skills/alpha/SKILL.md").exists());
    assert!(project_dir.join(".cursor/skills/alpha/SKILL.md").exists());
}

#[test]
fn project_sync_cleans_legacy_target_when_project_rule_changes() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");
    let project_dir = temp.path().join("project");
    let old_target = project_dir.join(".config/opencode/skills");
    fs::create_dir_all(&old_target).unwrap();
    let skill_id = create_skill(&repo_dir, "custom/alpha", "Alpha");
    let old_desired = build_desired_skill_entries(&[SkillSummary {
        id: skill_id.clone(),
        name: "Alpha".to_string(),
        description: String::new(),
        source_type: "custom".to_string(),
        relative_path: "custom/alpha".to_string(),
        directory_path: repo_dir.join("custom/alpha").to_string_lossy().to_string(),
        skill_document_path: repo_dir
            .join("custom/alpha/SKILL.md")
            .to_string_lossy()
            .to_string(),
        managed_source: None,
    }]);
    apply_desired_entries(&old_target, "opencode", &old_desired, SyncMode::Copy).unwrap();

    ProjectConfigStore::new(config_dir.clone())
        .add_project(
            project_dir.to_string_lossy().as_ref(),
            "Project",
            vec![skill_id],
            vec!["opencode".to_string()],
        )
        .unwrap();
    let ledger = ProjectSyncLedger {
        assignments: BTreeMap::from([(
            ledger_key(
                &project_dir.to_string_lossy().replace('\\', "/"),
                "opencode",
            ),
            ProjectSyncLedgerEntry {
                agent_key: "opencode".to_string(),
                target_dir: old_target.to_string_lossy().to_string(),
            },
        )]),
    };
    save_ledger(&config_dir, &ledger).unwrap();

    apply_project_assignments(
        &config_dir,
        &repo_dir,
        &AgentSystemDirs {
            home_dir: temp.path().join("home"),
            config_dir: Some(temp.path().join("xdg")),
        },
    )
    .unwrap();

    assert!(!old_target.join("alpha").exists());
    assert!(project_dir.join(".opencode/skills/alpha/SKILL.md").exists());
}

#[cfg(windows)]
#[test]
fn legacy_target_comparison_normalizes_windows_backslashes() {
    assert_eq!(
        normalize_legacy_target_for_comparison(r"C:\Users\test\project\.opencode\skills"),
        "C:/Users/test/project/.opencode/skills"
    );
}

#[cfg(not(windows))]
#[test]
fn legacy_target_comparison_preserves_non_windows_backslashes() {
    assert_eq!(
        normalize_legacy_target_for_comparison("/Users/test/project\\name/.opencode/skills"),
        "/Users/test/project\\name/.opencode/skills"
    );
}
