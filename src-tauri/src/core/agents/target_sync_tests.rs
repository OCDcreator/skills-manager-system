use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use tempfile::tempdir;

use crate::core::skills::scan::SkillSummary;

use super::target_sync::{apply_desired_entries, build_desired_skill_entries, SyncMode};

fn create_skill(repo_root: &Path, relative_path: &str) -> SkillSummary {
    let skill_dir = repo_root.join(relative_path);
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), format!("# {relative_path}")).unwrap();

    SkillSummary {
        id: relative_path.replace('/', ":").replacen("custom:", "custom:", 1),
        name: relative_path.to_string(),
        description: String::new(),
        source_type: "custom".to_string(),
        relative_path: relative_path.to_string(),
        directory_path: skill_dir.to_string_lossy().replace('\\', "/"),
        skill_document_path: skill_dir.join("SKILL.md").to_string_lossy().replace('\\', "/"),
        managed_source: None,
    }
}

fn custom_skill(repo_root: &Path, name: &str) -> SkillSummary {
    let relative_path = format!("custom/{name}");
    let mut skill = create_skill(repo_root, &relative_path);
    skill.id = format!("custom:{name}");
    skill
}

#[test]
fn unmanaged_entries_are_not_deleted_when_managed_entries_are_removed() {
    let repo_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();
    let alpha = custom_skill(repo_dir.path(), "alpha");

    let desired_entries = build_desired_skill_entries(&[alpha]);
    apply_desired_entries(target_dir.path(), "codex", &desired_entries, SyncMode::Copy).unwrap();

    let manual_dir = target_dir.path().join("manual-skill");
    fs::create_dir_all(&manual_dir).unwrap();
    fs::write(manual_dir.join("SKILL.md"), "# manual").unwrap();

    apply_desired_entries(
        target_dir.path(),
        "codex",
        &BTreeMap::new(),
        SyncMode::Copy,
    )
    .unwrap();

    assert!(manual_dir.join("SKILL.md").exists());
    assert!(!target_dir.path().join("custom--alpha").exists());
}
