use std::fs;
use std::path::Path;
use std::time::SystemTime;

use tempfile::tempdir;

use super::cache::{
    load_cached_repo_skills, scan_repo_skills_cached, scan_repo_skills_cached_with_external_sources,
};

#[test]
fn cached_scan_reuses_unchanged_entries_and_scans_new_skills() {
    let root = tempdir().unwrap();
    let repo_root = root.path().join("repo");
    let config_dir = root.path().join("config");
    fs::create_dir_all(repo_root.join("custom/alpha")).unwrap();
    fs::write(
        repo_root.join("custom/alpha/SKILL.md"),
        "---\nname: alpha\ndescription: cached\n---\n# Alpha\n",
    )
    .unwrap();

    let first = scan_repo_skills_cached(&repo_root, &config_dir).unwrap();
    assert_eq!(first.skills.len(), 1);
    assert_eq!(first.skills[0].name, "alpha");

    let alpha_path = repo_root.join("custom/alpha/SKILL.md");
    let alpha_modified = fs::metadata(&alpha_path).unwrap().modified().unwrap();
    let original_len = fs::metadata(&alpha_path).unwrap().len();
    fs::write(&alpha_path, "#".repeat(original_len as usize)).unwrap();
    assert_eq!(fs::metadata(&alpha_path).unwrap().len(), original_len);
    restore_modified_time(&alpha_path, alpha_modified);
    fs::create_dir_all(repo_root.join("custom/beta")).unwrap();
    fs::write(
        repo_root.join("custom/beta/SKILL.md"),
        "---\nname: beta\ndescription: new\n---\n# Beta\n",
    )
    .unwrap();

    let second = scan_repo_skills_cached(&repo_root, &config_dir).unwrap();
    let names: Vec<_> = second
        .skills
        .iter()
        .map(|skill| skill.name.as_str())
        .collect();

    assert_eq!(names, vec!["alpha", "beta"]);
    assert_eq!(
        second
            .skills
            .iter()
            .find(|skill| skill.name == "alpha")
            .unwrap()
            .description,
        "cached"
    );
}

#[test]
fn cached_scan_rebuilds_changed_entries() {
    let root = tempdir().unwrap();
    let repo_root = root.path().join("repo");
    let config_dir = root.path().join("config");
    fs::create_dir_all(repo_root.join("custom/alpha")).unwrap();
    fs::write(
        repo_root.join("custom/alpha/SKILL.md"),
        "---\nname: alpha\ndescription: old\n---\n# Alpha\n",
    )
    .unwrap();

    scan_repo_skills_cached(&repo_root, &config_dir).unwrap();
    fs::write(
        repo_root.join("custom/alpha/SKILL.md"),
        "---\nname: alpha\ndescription: new\n---\n# Alpha\n",
    )
    .unwrap();

    let response = scan_repo_skills_cached(&repo_root, &config_dir).unwrap();

    assert_eq!(response.skills[0].description, "new");
}

#[test]
fn load_cached_repo_skills_returns_previous_snapshot_without_validation() {
    let root = tempdir().unwrap();
    let repo_root = root.path().join("repo");
    let config_dir = root.path().join("config");
    fs::create_dir_all(repo_root.join("custom/alpha")).unwrap();
    fs::write(
        repo_root.join("custom/alpha/SKILL.md"),
        "---\nname: alpha\ndescription: cached\n---\n# Alpha\n",
    )
    .unwrap();
    scan_repo_skills_cached(&repo_root, &config_dir).unwrap();
    fs::remove_dir_all(repo_root.join("custom/alpha")).unwrap();

    let response = load_cached_repo_skills(&repo_root, &config_dir)
        .unwrap()
        .unwrap();

    assert_eq!(response.skills.len(), 1);
    assert_eq!(response.skills[0].name, "alpha");
}

#[test]
fn cached_scan_with_external_sources_rewrites_cache_file() {
    let root = tempdir().unwrap();
    let repo_root = root.path().join("repo");
    let config_dir = root.path().join("config");
    fs::create_dir_all(repo_root.join("custom/alpha")).unwrap();
    fs::write(
        repo_root.join("custom/alpha/SKILL.md"),
        "---\nname: alpha\n---\n# Alpha\n",
    )
    .unwrap();

    let response = scan_repo_skills_cached_with_external_sources(&repo_root, &config_dir).unwrap();

    assert_eq!(response.skills.len(), 1);
    assert!(super::cache::cache_path(&config_dir).exists());
}

fn restore_modified_time(path: &Path, modified: SystemTime) {
    let file = fs::OpenOptions::new().write(true).open(path).unwrap();
    file.set_times(std::fs::FileTimes::new().set_modified(modified))
        .unwrap();
}
