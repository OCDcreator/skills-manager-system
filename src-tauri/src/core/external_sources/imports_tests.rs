use super::*;
use crate::app_runtime::config_lock::acquire_config_lock;
use crate::core::external_sources::models::{ExternalSourceRecord, ExternalSourcesSnapshot};
use crate::core::external_sources::store::ExternalSourcesStore;
use crate::core::scenes::config::SceneConfigStore;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::tempdir;

#[test]
fn import_variant_writes_manifest_and_updates_record() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_root = temp.path().join("my-skills");
    let cached_repo = temp.path().join("cached-repo");
    fs::create_dir_all(repo_root.join("external")).unwrap();
    initialize_repo_with_variant(
        &cached_repo,
        "dist/agents/.agents/skills/impeccable",
        "---\nname: impeccable\ndescription: imported\n---\n# Impeccable\n",
    );
    let pinned_commit = git_output(&cached_repo, ["rev-parse", "HEAD"]);

    let store = ExternalSourcesStore::new(config_dir.clone());
    let guard = acquire_config_lock(&config_dir).unwrap();
    store
        .save(
            &guard,
            &ExternalSourcesSnapshot {
                schema_version: 1,
                sources: vec![ExternalSourceRecord {
                    id: "src_01".to_string(),
                    repo_url: "git@github.com:OCDcreator/Impeccable.git".to_string(),
                    cached_repo_path: Some(cached_repo.to_string_lossy().to_string()),
                    ..ExternalSourceRecord::default()
                }],
                imports: Vec::new(),
            },
        )
        .unwrap();
    drop(guard);

    let result = import_variant_into_repo(
        &config_dir,
        &repo_root,
        ImportVariantInput {
            external_source_id: "src_01".to_string(),
            agent_key: "codex".to_string(),
            upstream_variant_path: "dist/agents/.agents/skills/impeccable".to_string(),
            pinned_commit: pinned_commit.clone(),
        },
    )
    .unwrap();

    assert_eq!(
        result.skill_id,
        "external:managed/github/ocdcreator__impeccable/codex/impeccable"
    );
    assert_eq!(
        result.mirror_relative_path,
        "external/managed/github/ocdcreator__impeccable/codex/impeccable"
    );
    assert!(
        repo_root
            .join(&result.mirror_relative_path)
            .join(".skills-manager-source.json")
            .exists()
    );

    let snapshot = store.load().unwrap();
    assert_eq!(snapshot.imports.len(), 1);
    assert_eq!(snapshot.imports[0].skill_id, result.skill_id);
    assert_eq!(snapshot.imports[0].pinned_commit, pinned_commit);
}

#[test]
fn remove_import_blocks_when_skill_is_referenced() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_root = temp.path().join("my-skills");
    let cached_repo = temp.path().join("cached-repo");
    fs::create_dir_all(repo_root.join("external")).unwrap();
    initialize_repo_with_variant(
        &cached_repo,
        "dist/agents/.agents/skills/impeccable",
        "---\nname: impeccable\n---\n# Impeccable\n",
    );
    let pinned_commit = git_output(&cached_repo, ["rev-parse", "HEAD"]);

    let store = ExternalSourcesStore::new(config_dir.clone());
    let guard = acquire_config_lock(&config_dir).unwrap();
    store
        .save(
            &guard,
            &ExternalSourcesSnapshot {
                schema_version: 1,
                sources: vec![ExternalSourceRecord {
                    id: "src_01".to_string(),
                    repo_url: "git@github.com:OCDcreator/Impeccable.git".to_string(),
                    cached_repo_path: Some(cached_repo.to_string_lossy().to_string()),
                    ..ExternalSourceRecord::default()
                }],
                imports: Vec::new(),
            },
        )
        .unwrap();
    drop(guard);

    let imported = import_variant_into_repo(
        &config_dir,
        &repo_root,
        ImportVariantInput {
            external_source_id: "src_01".to_string(),
            agent_key: "codex".to_string(),
            upstream_variant_path: "dist/agents/.agents/skills/impeccable".to_string(),
            pinned_commit,
        },
    )
    .unwrap();

    let scene_store = SceneConfigStore::new(config_dir.clone());
    scene_store.create_scene("focus", "Focus", "").unwrap();
    scene_store
        .set_scene_skills("focus", vec![imported.skill_id.clone()])
        .unwrap();

    let error =
        remove_imported_variant_from_repo(&config_dir, &repo_root, &imported.skill_id).unwrap_err();

    assert!(error.to_string().contains("referenced"));
    assert!(
        store
            .load()
            .unwrap()
            .imports
            .iter()
            .any(|record| record.skill_id == imported.skill_id)
    );
    assert!(repo_root.join(imported.mirror_relative_path).exists());
}

#[test]
fn remove_import_succeeds_without_source_snapshot_entry() {
    let temp = tempdir().unwrap();
    let (config_dir, repo_root, store, imported) = import_fixture(&temp);

    let guard = acquire_config_lock(&config_dir).unwrap();
    let mut snapshot = store.load().unwrap();
    snapshot.sources.clear();
    store.save(&guard, &snapshot).unwrap();
    drop(guard);

    let result =
        remove_imported_variant_from_repo(&config_dir, &repo_root, &imported.skill_id).unwrap();

    assert_eq!(result.skill_id, imported.skill_id);
    assert!(store.load().unwrap().imports.is_empty());
    assert!(!repo_root.join(imported.mirror_relative_path).exists());
}

#[test]
fn remove_import_succeeds_when_source_repo_url_drifts() {
    let temp = tempdir().unwrap();
    let (config_dir, repo_root, store, imported) = import_fixture(&temp);

    let guard = acquire_config_lock(&config_dir).unwrap();
    let mut snapshot = store.load().unwrap();
    snapshot.sources[0].repo_url = "git@github.com:someone-else/drifted.git".to_string();
    store.save(&guard, &snapshot).unwrap();
    drop(guard);

    let result =
        remove_imported_variant_from_repo(&config_dir, &repo_root, &imported.skill_id).unwrap();

    assert_eq!(result.skill_id, imported.skill_id);
    assert!(store.load().unwrap().imports.is_empty());
    assert!(!repo_root.join(imported.mirror_relative_path).exists());
}

fn import_fixture(
    temp: &tempfile::TempDir,
) -> (PathBuf, PathBuf, ExternalSourcesStore, ImportVariantResult) {
    let config_dir = temp.path().join("config");
    let repo_root = temp.path().join("my-skills");
    let cached_repo = temp.path().join("cached-repo");
    fs::create_dir_all(repo_root.join("external")).unwrap();
    initialize_repo_with_variant(
        &cached_repo,
        "dist/agents/.agents/skills/impeccable",
        "---\nname: impeccable\n---\n# Impeccable\n",
    );
    let pinned_commit = git_output(&cached_repo, ["rev-parse", "HEAD"]);

    let store = ExternalSourcesStore::new(config_dir.clone());
    let guard = acquire_config_lock(&config_dir).unwrap();
    store
        .save(
            &guard,
            &ExternalSourcesSnapshot {
                schema_version: 1,
                sources: vec![ExternalSourceRecord {
                    id: "src_01".to_string(),
                    repo_url: "git@github.com:OCDcreator/Impeccable.git".to_string(),
                    cached_repo_path: Some(cached_repo.to_string_lossy().to_string()),
                    ..ExternalSourceRecord::default()
                }],
                imports: Vec::new(),
            },
        )
        .unwrap();
    drop(guard);

    let imported = import_variant_into_repo(
        &config_dir,
        &repo_root,
        ImportVariantInput {
            external_source_id: "src_01".to_string(),
            agent_key: "codex".to_string(),
            upstream_variant_path: "dist/agents/.agents/skills/impeccable".to_string(),
            pinned_commit,
        },
    )
    .unwrap();

    (config_dir, repo_root, store, imported)
}

fn initialize_repo_with_variant(repo_dir: &Path, variant_path: &str, skill_md: &str) {
    run_git(Command::new("git").arg("init").arg(repo_dir));
    run_git(git_cmd(repo_dir).args(["config", "user.email", "test@example.com"]));
    run_git(git_cmd(repo_dir).args(["config", "user.name", "Test User"]));
    let skill_path = repo_dir.join(variant_path);
    fs::create_dir_all(&skill_path).unwrap();
    fs::write(skill_path.join("SKILL.md"), skill_md).unwrap();
    fs::write(skill_path.join("notes.txt"), "hello\n").unwrap();
    run_git(git_cmd(repo_dir).args(["add", "."]));
    run_git(git_cmd(repo_dir).args(["commit", "-m", "initial"]));
}

fn git_cmd(repo_dir: &Path) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(repo_dir);
    cmd
}

fn git_output<const N: usize>(repo_dir: &Path, args: [&str; N]) -> String {
    let output = git_cmd(repo_dir).args(args).output().unwrap();
    assert!(output.status.success(), "git command failed");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn run_git(cmd: &mut Command) {
    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
