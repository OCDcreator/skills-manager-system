use std::fs;
use std::path::{Path, PathBuf};

use tempfile::tempdir;

use crate::core::skills::scan::SkillSummary;

use super::target_inventory::scan_target_skill_entries;
use super::target_management::{
    delete_target_skill_entry, import_unmanaged_target_skill, take_over_unmanaged_target_skill,
};
use super::target_sync::{apply_desired_entries, build_desired_skill_entries, SyncMode};

fn custom_skill(repo_root: &Path, name: &str) -> SkillSummary {
    let relative_path = format!("custom/{name}");
    let skill_dir = repo_root.join(&relative_path);
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), format!("# {name}")).unwrap();
    fs::write(skill_dir.join("marker.txt"), format!("repo:{name}")).unwrap();

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

fn manual_target_skill(target_dir: &Path, entry_name: &str, marker: &str) -> PathBuf {
    let skill_dir = target_dir.join(entry_name);
    fs::create_dir_all(skill_dir.join("assets")).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        format!("---\nname: {entry_name}\n---\n# {entry_name}"),
    )
    .unwrap();
    fs::write(skill_dir.join("assets/marker.txt"), marker).unwrap();
    skill_dir
}

#[test]
fn taking_over_unmanaged_entry_keeps_existing_contents_when_sync_would_conflict() {
    let repo_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();
    let alpha = custom_skill(repo_dir.path(), "alpha");

    let unmanaged_dir = manual_target_skill(target_dir.path(), "custom--alpha", "manual:alpha");

    take_over_unmanaged_target_skill(target_dir.path(), "codex", "custom--alpha").unwrap();

    let desired_entries = build_desired_skill_entries(&[alpha]);
    let stats = apply_desired_entries(target_dir.path(), "codex", &desired_entries, SyncMode::Copy)
        .unwrap();

    assert_eq!(stats.conflict_count, 1);
    assert_eq!(
        fs::read_to_string(unmanaged_dir.join("assets/marker.txt")).unwrap(),
        "manual:alpha"
    );

    let entries = scan_target_skill_entries(target_dir.path(), "codex").unwrap();
    let entry = entries
        .iter()
        .find(|item| item.entry_name == "custom--alpha")
        .unwrap();
    assert!(entry.managed);
}

#[test]
fn deleting_requested_entry_preserves_other_target_entries() {
    let repo_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();
    let alpha = custom_skill(repo_dir.path(), "alpha");
    let beta = custom_skill(repo_dir.path(), "beta");

    let desired_entries = build_desired_skill_entries(&[alpha, beta]);
    apply_desired_entries(target_dir.path(), "codex", &desired_entries, SyncMode::Copy).unwrap();

    let unmanaged_one = manual_target_skill(target_dir.path(), "manual-one", "manual:one");
    let unmanaged_two = manual_target_skill(target_dir.path(), "manual-two", "manual:two");

    delete_target_skill_entry(target_dir.path(), "codex", "manual-one").unwrap();
    assert!(!unmanaged_one.exists());
    assert!(unmanaged_two.exists());
    assert!(target_dir.path().join("custom--alpha").exists());
    assert!(target_dir.path().join("custom--beta").exists());

    delete_target_skill_entry(target_dir.path(), "codex", "custom--alpha").unwrap();
    assert!(!target_dir.path().join("custom--alpha").exists());
    assert!(target_dir.path().join("custom--beta").exists());
    assert!(unmanaged_two.exists());

    let entries = scan_target_skill_entries(target_dir.path(), "codex").unwrap();
    assert!(entries.iter().any(|entry| entry.entry_name == "manual-two"));
    assert!(entries.iter().any(|entry| entry.entry_name == "custom--beta"));
    assert!(!entries.iter().any(|entry| entry.entry_name == "custom--alpha"));
  }

#[test]
fn importing_unmanaged_entry_copies_into_repo_and_only_deletes_when_requested() {
    let repo_dir = tempdir().unwrap();
    fs::create_dir_all(repo_dir.path().join("custom")).unwrap();
    let target_dir = tempdir().unwrap();

    let manual_keep = manual_target_skill(target_dir.path(), "manual-keep", "keep");
    let manual_delete = manual_target_skill(target_dir.path(), "manual-delete", "delete");

    let imported_keep = import_unmanaged_target_skill(
        repo_dir.path(),
        target_dir.path(),
        "codex",
        "manual-keep",
        false,
    )
    .unwrap();
    assert_eq!(imported_keep.relative_path, "custom/manual-keep");
    assert!(
        repo_dir
            .path()
            .join("custom/manual-keep/assets/marker.txt")
            .exists()
    );
    assert!(manual_keep.exists());

    let imported_delete = import_unmanaged_target_skill(
        repo_dir.path(),
        target_dir.path(),
        "codex",
        "manual-delete",
        true,
    )
    .unwrap();
    assert_eq!(imported_delete.relative_path, "custom/manual-delete");
    assert!(
        repo_dir
            .path()
            .join("custom/manual-delete/assets/marker.txt")
            .exists()
    );
    assert!(!manual_delete.exists());
}
