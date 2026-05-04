use super::*;
use crate::core::projects::store::ProjectConfigStore;
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
    assert!(!project_dir.join(".config/opencode/skills/alpha/SKILL.md").exists());
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
    apply_desired_entries(&old_target, "opencode", &old_desired, SyncMode::Copy)
        .unwrap();

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
            ledger_key(&project_dir.to_string_lossy().replace('\\', "/"), "opencode"),
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
