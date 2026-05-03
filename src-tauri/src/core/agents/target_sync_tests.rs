use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use tempfile::tempdir;

use crate::core::skills::scan::SkillSummary;

use super::target_sync::{
    apply_desired_entries, build_desired_skill_entries, managed_entry_name, remove_target, SyncMode,
};

fn create_skill(repo_root: &Path, relative_path: &str) -> SkillSummary {
    let skill_dir = repo_root.join(relative_path);
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), format!("# {relative_path}")).unwrap();

    SkillSummary {
        id: relative_path
            .replace('/', ":")
            .replacen("custom:", "custom:", 1),
        name: relative_path.to_string(),
        description: String::new(),
        source_type: "custom".to_string(),
        relative_path: relative_path.to_string(),
        directory_path: skill_dir.to_string_lossy().replace('\\', "/"),
        skill_document_path: skill_dir
            .join("SKILL.md")
            .to_string_lossy()
            .replace('\\', "/"),
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
fn managed_entry_names_prefer_skill_directory_name_without_source_prefix() {
    let repo_dir = tempdir().unwrap();
    let open_source_skill = custom_skill(repo_dir.path(), "open-source-project");
    let managed_external_skill = create_skill(
        repo_dir.path(),
        "external/managed/github/owner__repo/opencode/open-source-project",
    );

    let custom_entries = build_desired_skill_entries(&[open_source_skill]);
    assert!(custom_entries.contains_key("open-source-project"));
    assert!(!custom_entries.contains_key("custom--open-source-project"));

    let external_entries = build_desired_skill_entries(&[managed_external_skill]);
    assert!(external_entries.contains_key("open-source-project"));
    assert!(!external_entries
        .contains_key("external--managed--github--owner__repo--opencode--open-source-project"));
}

#[test]
fn duplicate_skill_directory_names_are_disambiguated_by_relative_path() {
    let repo_dir = tempdir().unwrap();
    let custom = custom_skill(repo_dir.path(), "shared");
    let external = create_skill(repo_dir.path(), "external/vendor/shared");

    let entries = build_desired_skill_entries(&[custom, external]);

    assert!(entries.contains_key("custom--shared"));
    assert!(entries.contains_key("external--vendor--shared"));
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

    apply_desired_entries(target_dir.path(), "codex", &BTreeMap::new(), SyncMode::Copy).unwrap();

    assert!(manual_dir.join("SKILL.md").exists());
    assert!(!target_dir.path().join("alpha").exists());
    assert!(!target_dir.path().join("custom--alpha").exists());
}

#[test]
#[cfg_attr(
    windows,
    ignore = "requires Developer Mode or administrator symlink privilege"
)]
fn symlink_mode_links_the_skill_directory_entry_instead_of_each_document() {
    let repo_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();
    let alpha = custom_skill(repo_dir.path(), "alpha");
    let asset_dir = repo_dir.path().join("custom/alpha/assets");
    fs::create_dir_all(&asset_dir).unwrap();
    fs::write(asset_dir.join("prompt.txt"), "asset").unwrap();

    let desired_entries = build_desired_skill_entries(&[alpha]);
    apply_desired_entries(
        target_dir.path(),
        "opencode",
        &desired_entries,
        SyncMode::Symlink,
    )
    .unwrap();

    let target_entry = target_dir.path().join("alpha");
    let target_entry_metadata = fs::symlink_metadata(&target_entry).unwrap();
    assert!(target_entry_metadata.file_type().is_symlink());
    assert!(target_entry.join("SKILL.md").exists());
    assert!(target_entry.join("assets/prompt.txt").exists());

    let skill_document_metadata = fs::symlink_metadata(target_entry.join("SKILL.md")).unwrap();
    assert!(!skill_document_metadata.file_type().is_symlink());

    #[cfg(unix)]
    assert_eq!(
        fs::read_link(target_entry).unwrap(),
        repo_dir.path().join("custom/alpha")
    );
}

#[test]
#[cfg(windows)]
fn remove_target_deletes_windows_directory_symlink_without_deleting_source() {
    let root = tempdir().unwrap();
    let source_dir = root.path().join("source-skill");
    let target_link = root.path().join("target-skill");
    fs::create_dir_all(&source_dir).unwrap();
    fs::write(source_dir.join("SKILL.md"), "# source").unwrap();

    if std::os::windows::fs::symlink_dir(&source_dir, &target_link).is_err() {
        return;
    }

    remove_target(&target_link).unwrap();

    assert!(!target_link.exists());
    assert!(source_dir.join("SKILL.md").exists());
}

#[test]
fn managed_entry_name_sanitizes_relative_path_when_disambiguation_is_needed() {
    assert_eq!(
        managed_entry_name("external/vendor/shared"),
        "external--vendor--shared"
    );
}
