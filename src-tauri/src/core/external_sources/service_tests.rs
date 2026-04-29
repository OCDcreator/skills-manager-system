use std::fs;
use std::path::Path;

use crate::app_runtime::config_lock::acquire_config_lock;
use crate::core::external_sources::models::{
    ExternalSourceRecord, ExternalSourcesSnapshot, ManagedSkillMirrorManifest,
};
use crate::core::scenes::config::SceneConfigStore;
use tempfile::tempdir;

use super::{remove_external_source, ExternalSourcesStore, ImportedExternalSkillRecord};

#[test]
fn remove_external_source_preflight_blocks_partial_import_deletion() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_root = temp.path().join("my-skills");
    fs::create_dir_all(repo_root.join("external")).unwrap();

    let removable = build_import_fixture(
        &repo_root,
        "imp_removable",
        "external:managed/github/example__repo/codex/removable",
        "external/managed/github/example__repo/codex/removable",
    );
    let blocked = build_import_fixture(
        &repo_root,
        "imp_blocked",
        "external:managed/github/example__repo/codex/blocked",
        "external/managed/github/example__repo/codex/blocked",
    );

    let store = ExternalSourcesStore::new(config_dir.clone());
    let guard = acquire_config_lock(&config_dir).unwrap();
    store
        .save(
            &guard,
            &ExternalSourcesSnapshot {
                schema_version: 1,
                sources: vec![ExternalSourceRecord {
                    id: "src_example".to_string(),
                    repo_url: "https://github.com/example/repo".to_string(),
                    ..ExternalSourceRecord::default()
                }],
                imports: vec![removable.clone(), blocked.clone()],
            },
        )
        .unwrap();
    drop(guard);

    let scene_store = SceneConfigStore::new(config_dir.clone());
    scene_store.create_scene("focus", "Focus", "").unwrap();
    scene_store
        .set_scene_skills("focus", vec![blocked.skill_id.clone()])
        .unwrap();

    let error = remove_external_source(&config_dir, Some(&repo_root), "src_example", true)
        .unwrap_err();

    assert!(error.to_string().contains("referenced"));
    assert!(repo_root.join(&removable.mirror_relative_path).exists());
    assert!(repo_root.join(&blocked.mirror_relative_path).exists());

    let persisted = store.load().unwrap();
    assert_eq!(persisted.sources.len(), 1);
    assert_eq!(persisted.imports.len(), 2);
    assert!(persisted
        .imports
        .iter()
        .any(|item| item.skill_id == removable.skill_id));
    assert!(persisted
        .imports
        .iter()
        .any(|item| item.skill_id == blocked.skill_id));
}

#[test]
fn remove_external_source_keeps_snapshot_when_cache_removal_fails() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let source_id = "src_example";
    let cache_root = config_dir.join("external-sources").join(source_id);
    fs::create_dir_all(cache_root.parent().unwrap()).unwrap();
    fs::write(&cache_root, "not-a-directory").unwrap();

    let store = ExternalSourcesStore::new(config_dir.clone());
    let guard = acquire_config_lock(&config_dir).unwrap();
    store
        .save(
            &guard,
            &ExternalSourcesSnapshot {
                schema_version: 1,
                sources: vec![ExternalSourceRecord {
                    id: source_id.to_string(),
                    repo_url: "https://github.com/example/repo".to_string(),
                    ..ExternalSourceRecord::default()
                }],
                imports: Vec::new(),
            },
        )
        .unwrap();
    drop(guard);

    let error = remove_external_source(&config_dir, None, source_id, false).unwrap_err();

    assert!(!error.to_string().trim().is_empty());
    assert!(cache_root.exists());

    let persisted = store.load().unwrap();
    assert_eq!(persisted.sources.len(), 1);
    assert_eq!(persisted.sources[0].id, source_id);
    assert!(persisted.imports.is_empty());
}

fn build_import_fixture(
    repo_root: &Path,
    import_id: &str,
    skill_id: &str,
    mirror_relative_path: &str,
) -> ImportedExternalSkillRecord {
    let mirror_dir = repo_root.join(mirror_relative_path);
    fs::create_dir_all(&mirror_dir).unwrap();
    fs::write(
        mirror_dir.join("SKILL.md"),
        "---\nname: fixture\ndescription: fixture\n---\n# Fixture\n",
    )
    .unwrap();

    let import = ImportedExternalSkillRecord {
        import_id: import_id.to_string(),
        external_source_id: "src_example".to_string(),
        agent_key: "codex".to_string(),
        upstream_variant_path: format!("dist/agents/.agents/skills/{import_id}"),
        pinned_commit: "abcdef123456".to_string(),
        pinned_variant_fingerprint: Some("sha256:test".to_string()),
        skill_id: skill_id.to_string(),
        mirror_relative_path: mirror_relative_path.to_string(),
        last_checked_commit: Some("abcdef123456".to_string()),
        imported_at: Some("2026-04-29T00:00:00Z".to_string()),
        warnings: Vec::new(),
        update_available: false,
    };

    let manifest = ManagedSkillMirrorManifest {
        schema_version: 1,
        managed: true,
        import_id: import.import_id.clone(),
        source_id: import.external_source_id.clone(),
        repo_url: "https://github.com/example/repo".to_string(),
        agent_key: import.agent_key.clone(),
        variant_path: import.upstream_variant_path.clone(),
        mirror_relative_path: import.mirror_relative_path.clone(),
        skill_id: import.skill_id.clone(),
        pinned_commit: import.pinned_commit.clone(),
    };
    fs::write(
        mirror_dir.join(".skills-manager-source.json"),
        serde_json::to_string_pretty(&manifest).unwrap(),
    )
    .unwrap();

    import
}
