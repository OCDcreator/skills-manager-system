use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use tempfile::tempdir;

use crate::core::skills::scan::SkillSummary;

use super::target_inventory::scan_target_skill_entries;
use super::target_sync::{apply_desired_entries, build_desired_skill_entries, SyncMode};

fn custom_skill(repo_root: &Path, name: &str) -> SkillSummary {
    let relative_path = format!("custom/{name}");
    let skill_dir = repo_root.join(&relative_path);
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), format!("# {name}")).unwrap();

    SkillSummary {
        id: format!("custom:{name}"),
        name: name.to_string(),
        description: String::new(),
        source_type: "custom".to_string(),
        relative_path,
        directory_path: skill_dir.to_string_lossy().replace('\\', "/"),
        skill_document_path: skill_dir.join("SKILL.md").to_string_lossy().replace('\\', "/"),
    }
}

#[test]
fn target_inventory_classifies_managed_and_unmanaged_entries() {
    let repo_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();
    let alpha = custom_skill(repo_dir.path(), "alpha");

    let desired_entries = build_desired_skill_entries(&[alpha]);
    apply_desired_entries(target_dir.path(), "codex", &desired_entries, SyncMode::Copy).unwrap();

    let manual_dir = target_dir.path().join("manual-skill");
    fs::create_dir_all(&manual_dir).unwrap();
    fs::write(
        manual_dir.join("SKILL.md"),
        "---\nname: Manual Skill\n---\n# Manual",
    )
    .unwrap();

    let entries = scan_target_skill_entries(target_dir.path(), "codex").unwrap();

    let managed = entries
        .iter()
        .find(|entry| entry.entry_name == "custom--alpha")
        .unwrap();
    assert!(managed.managed);
    assert_eq!(managed.skill_id.as_deref(), Some("custom:alpha"));

    let unmanaged = entries
        .iter()
        .find(|entry| entry.entry_name == "manual-skill")
        .unwrap();
    assert!(!unmanaged.managed);
    assert_eq!(unmanaged.display_name, "Manual Skill");
}

#[test]
fn target_inventory_ignores_manifest_and_keeps_unmanaged_after_cleanup() {
    let repo_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();
    let alpha = custom_skill(repo_dir.path(), "alpha");
    let manual_dir = target_dir.path().join("manual-skill");
    fs::create_dir_all(&manual_dir).unwrap();

    let desired_entries = build_desired_skill_entries(&[alpha]);
    apply_desired_entries(target_dir.path(), "codex", &desired_entries, SyncMode::Copy).unwrap();
    apply_desired_entries(
        target_dir.path(),
        "codex",
        &BTreeMap::new(),
        SyncMode::Copy,
    )
    .unwrap();

    let entries = scan_target_skill_entries(target_dir.path(), "codex").unwrap();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].entry_name, "manual-skill");
    assert!(!entries[0].managed);
}
