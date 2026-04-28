use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use serde::Serialize;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::app_runtime::config_lock::acquire_config_lock;
use crate::core::external_sources::detect::detect_external_source_variants;
use crate::core::external_sources::git_repo::{
    ensure_cached_repo, fingerprint_variant_at_ref, read_default_branch, read_head_commit,
};
use crate::core::external_sources::imports::{
    import_variant_into_repo as import_variant_into_repo_core,
    preflight_remove_imported_variant, remove_imported_variant_from_repo, ImportVariantInput,
    ImportVariantResult,
};
use crate::core::external_sources::models::{
    ExternalSourceRecord, ExternalSourceWarning, ExternalSourcesSnapshot,
    ImportedExternalSkillRecord, ManagedSkillMirrorManifest,
};
use crate::core::external_sources::{normalize_github_repo_url, ExternalSourcesStore};
use crate::core::skills::identity::build_skill_id_from_relative_path;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalVariantSnapshot {
    pub agent_key: String,
    pub variant_path: String,
    pub source_of_truth_path: Option<String>,
    pub metadata_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalSourceSnapshotItem {
    pub record: ExternalSourceRecord,
    pub variants: Vec<ExternalVariantSnapshot>,
    pub imports: Vec<ImportedExternalSkillRecord>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalSourcesListResponse {
    pub sources: Vec<ExternalSourceSnapshotItem>,
}

pub fn list_external_sources(
    config_dir: &Path,
    repo_root: Option<&Path>,
) -> Result<ExternalSourcesListResponse> {
    let snapshot: ExternalSourcesSnapshot =
        ExternalSourcesStore::new(config_dir.to_path_buf()).load()?;
    let mut sources = snapshot.sources.clone();
    sources.sort_by(|left, right| left.id.cmp(&right.id));

    Ok(ExternalSourcesListResponse {
        sources: sources
            .into_iter()
            .map(|record| {
                let variants = load_variants_for_source(config_dir, &record);
                let mut imports = snapshot
                    .imports
                    .iter()
                    .filter(|item| item.external_source_id == record.id)
                    .cloned()
                    .collect::<Vec<_>>();
                imports.sort_by(|left, right| left.skill_id.cmp(&right.skill_id));

                if let Some(root) = repo_root {
                    for import in &mut imports {
                        if let Some(warn) = runtime_integrity_warning(root, import) {
                            import.warnings.push(warn);
                        }
                    }
                }

                ExternalSourceSnapshotItem {
                    record,
                    variants,
                    imports,
                }
            })
            .collect(),
    })
}

pub fn add_external_source(
    config_dir: &Path,
    repo_root: Option<&Path>,
    repo_url: &str,
) -> Result<ExternalSourcesListResponse> {
    let source_id = upsert_source(config_dir, repo_url)?;
    if let Err(error) = fetch_source(config_dir, &source_id) {
        let _ = set_source_failure(config_dir, &source_id, &error.to_string());
    }
    list_external_sources(config_dir, repo_root)
}

pub fn fetch_external_source(
    config_dir: &Path,
    repo_root: Option<&Path>,
    source_id: &str,
) -> Result<ExternalSourcesListResponse> {
    if let Err(error) = fetch_source(config_dir, source_id) {
        let _ = set_source_failure(config_dir, source_id, &error.to_string());
    }
    list_external_sources(config_dir, repo_root)
}

pub fn import_external_variant(
    config_dir: &Path,
    repo_root: &Path,
    source_id: &str,
    agent_key: &str,
    variant_path: &str,
) -> Result<ImportVariantResult> {
    let snapshot = ExternalSourcesStore::new(config_dir.to_path_buf()).load()?;
    let source = snapshot
        .sources
        .iter()
        .find(|item| item.id == source_id)
        .ok_or_else(|| anyhow!("External source was not found"))?;
    let pinned_commit = source
        .last_fetched_commit
        .clone()
        .ok_or_else(|| anyhow!("Fetch the external source before importing a variant"))?;

    import_variant_into_repo_core(
        config_dir,
        repo_root,
        ImportVariantInput {
            external_source_id: source_id.to_string(),
            agent_key: agent_key.to_string(),
            upstream_variant_path: variant_path.to_string(),
            pinned_commit,
        },
    )
}

pub fn update_external_import(
    config_dir: &Path,
    repo_root: &Path,
    import_id: &str,
) -> Result<ImportVariantResult> {
    let snapshot = ExternalSourcesStore::new(config_dir.to_path_buf()).load()?;
    let import = snapshot
        .imports
        .iter()
        .find(|item| item.import_id == import_id)
        .ok_or_else(|| anyhow!("Imported external skill was not found"))?;
    let source = snapshot
        .sources
        .iter()
        .find(|item| item.id == import.external_source_id)
        .ok_or_else(|| anyhow!("External source for the import was not found"))?;
    let pinned_commit = source
        .last_fetched_commit
        .clone()
        .ok_or_else(|| anyhow!("Fetch the external source before updating this import"))?;

    import_variant_into_repo_core(
        config_dir,
        repo_root,
        ImportVariantInput {
            external_source_id: import.external_source_id.clone(),
            agent_key: import.agent_key.clone(),
            upstream_variant_path: import.upstream_variant_path.clone(),
            pinned_commit,
        },
    )
}

pub fn remove_external_source(
    config_dir: &Path,
    repo_root: Option<&Path>,
    source_id: &str,
    remove_imports: bool,
) -> Result<ExternalSourcesListResponse> {
    let existing = ExternalSourcesStore::new(config_dir.to_path_buf()).load()?;
    let imports = existing
        .imports
        .iter()
        .filter(|item| item.external_source_id == source_id)
        .cloned()
        .collect::<Vec<_>>();

    if !imports.is_empty() && !remove_imports {
        return Err(anyhow!(
            "This external source still has imported mirrors; confirm destructive removal to continue"
        ));
    }

    if !imports.is_empty() {
        let repo_root = repo_root.ok_or_else(|| anyhow!("Repository path is not configured"))?;
        for import in &imports {
            preflight_remove_imported_variant(config_dir, repo_root, &import.skill_id)?;
        }
        for import in imports {
            remove_imported_variant_from_repo(config_dir, repo_root, &import.skill_id)?;
        }
    }

    let guard = acquire_config_lock(config_dir)?;
    let store = ExternalSourcesStore::new(config_dir.to_path_buf());
    let mut snapshot = store.load()?;
    snapshot.sources.retain(|item| item.id != source_id);
    snapshot
        .imports
        .retain(|item| item.external_source_id != source_id);
    store.save(&guard, &snapshot)?;

    let cache_root = config_dir.join("external-sources").join(source_id);
    if cache_root.exists() {
        fs::remove_dir_all(&cache_root)?;
    }

    list_external_sources(config_dir, repo_root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_runtime::config_lock::acquire_config_lock;
    use crate::core::external_sources::models::{ExternalSourceRecord, ExternalSourcesSnapshot};
    use crate::core::scenes::config::SceneConfigStore;
    use std::path::Path;
    use tempfile::tempdir;

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

        let error =
            remove_external_source(&config_dir, Some(&repo_root), "src_example", true).unwrap_err();

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
}

pub fn repair_external_import(
    config_dir: &Path,
    repo_root: &Path,
    import_id: &str,
) -> Result<ImportVariantResult> {
    let snapshot = ExternalSourcesStore::new(config_dir.to_path_buf()).load()?;
    let import = snapshot
        .imports
        .iter()
        .find(|item| item.import_id == import_id)
        .ok_or_else(|| anyhow!("Imported external skill was not found"))?;
    let source = snapshot
        .sources
        .iter()
        .find(|item| item.id == import.external_source_id)
        .ok_or_else(|| anyhow!("External source for the import was not found"))?;
    let pinned_commit = source
        .last_fetched_commit
        .clone()
        .or_else(|| import.last_checked_commit.clone())
        .ok_or_else(|| anyhow!("Fetch the external source before repairing this import"))?;

    let mirror_dir = repo_root.join(&import.mirror_relative_path);
    let backup_dir = mirror_dir.with_file_name(format!(".repair-{}", import.import_id));
    if mirror_dir.exists() {
        if backup_dir.exists() {
            fs::remove_dir_all(&backup_dir)?;
        }
        fs::rename(&mirror_dir, &backup_dir)?;
    }

    let result = import_variant_into_repo_core(
        config_dir,
        repo_root,
        ImportVariantInput {
            external_source_id: import.external_source_id.clone(),
            agent_key: import.agent_key.clone(),
            upstream_variant_path: import.upstream_variant_path.clone(),
            pinned_commit,
        },
    );

    match result {
        Ok(value) => {
            if backup_dir.exists() {
                let _ = fs::remove_dir_all(&backup_dir);
            }
            Ok(value)
        }
        Err(error) => {
            if backup_dir.exists() && !mirror_dir.exists() {
                let _ = fs::rename(&backup_dir, &mirror_dir);
            }
            Err(error)
        }
    }
}

fn load_variants_for_source(config_dir: &Path, record: &ExternalSourceRecord) -> Vec<ExternalVariantSnapshot> {
    record
        .cached_repo_path
        .as_deref()
        .map(|_| detect_external_source_variants(&cache_repo_absolute_path(config_dir, &record.id), &record.id).ok())
        .flatten()
        .map(|detection| {
            detection
                .variants
                .into_iter()
                .map(|variant| ExternalVariantSnapshot {
                    agent_key: variant.agent_key,
                    variant_path: variant.variant_path,
                    source_of_truth_path: variant.source_of_truth_path,
                    metadata_path: variant.metadata_path,
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn cache_repo_relative_path(source_id: &str) -> String {
    format!("external-sources/{source_id}/repo")
}

fn cache_repo_absolute_path(config_dir: &Path, source_id: &str) -> PathBuf {
    config_dir.join("external-sources").join(source_id).join("repo")
}

fn now_rfc3339() -> String {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
}

fn stable_source_id(normalized_repo_url: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in normalized_repo_url.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("src_{hash:016x}")
}

fn warning(code: &str, severity: &str, message: impl Into<String>) -> ExternalSourceWarning {
    ExternalSourceWarning {
        code: code.to_string(),
        severity: severity.to_string(),
        message: message.into(),
    }
}

fn upsert_source(config_dir: &Path, repo_url: &str) -> Result<String> {
    let normalized = normalize_github_repo_url(repo_url)?;
    let source_id = stable_source_id(&normalized);
    let guard = acquire_config_lock(config_dir)?;
    let store = ExternalSourcesStore::new(config_dir.to_path_buf());
    let mut snapshot = store.load()?;
    if let Some(source) = snapshot.sources.iter_mut().find(|source| source.id == source_id) {
        source.repo_url = repo_url.trim().to_string();
    } else {
        snapshot.sources.push(ExternalSourceRecord {
            id: source_id.clone(),
            repo_url: repo_url.trim().to_string(),
            status: Some("pending".to_string()),
            ..ExternalSourceRecord::default()
        });
        snapshot.sources.sort_by(|left, right| left.id.cmp(&right.id));
    }
    store.save(&guard, &snapshot)?;
    Ok(source_id)
}

fn set_source_failure(config_dir: &Path, source_id: &str, message: &str) -> Result<()> {
    let guard = acquire_config_lock(config_dir)?;
    let store = ExternalSourcesStore::new(config_dir.to_path_buf());
    let mut snapshot = store.load()?;
    if let Some(source) = snapshot.sources.iter_mut().find(|source| source.id == source_id) {
        source.last_fetched_at = Some(now_rfc3339());
        source.status = Some("error".to_string());
        source.warnings = vec![warning("fetch_failed", "error", message)];
        store.save(&guard, &snapshot)?;
    }
    Ok(())
}

fn fetch_source(config_dir: &Path, source_id: &str) -> Result<()> {
    let guard = acquire_config_lock(config_dir)?;
    let store = ExternalSourcesStore::new(config_dir.to_path_buf());
    let mut snapshot = store.load()?;
    let source_index = snapshot
        .sources
        .iter()
        .position(|source| source.id == source_id)
        .ok_or_else(|| anyhow!("External source '{}' was not found", source_id))?;
    let repo_url = snapshot.sources[source_index].repo_url.clone();
    let repo_dir = ensure_cached_repo(&config_dir.join("external-sources"), source_id, &repo_url)?;
    let default_branch = read_default_branch(&repo_dir)?;
    let head_commit = read_head_commit(&repo_dir, &default_branch)?;
    let detection = detect_external_source_variants(&repo_dir, source_id)?;
    let variants = detection
        .variants
        .into_iter()
        .map(|variant| ExternalVariantSnapshot {
            agent_key: variant.agent_key,
            variant_path: variant.variant_path,
            source_of_truth_path: variant.source_of_truth_path,
            metadata_path: variant.metadata_path,
        })
        .collect::<Vec<_>>();
    let mut warnings = detection.warnings;
    warnings.extend(refresh_imports_for_source(
        &repo_dir,
        &head_commit,
        source_id,
        &variants,
        &mut snapshot.imports,
    )?);
    let source = &mut snapshot.sources[source_index];
    source.default_branch = Some(default_branch);
    source.cached_repo_path = Some(cache_repo_relative_path(source_id));
    source.detected_kind = Some(detection.kind);
    source.last_fetched_commit = Some(head_commit);
    source.last_fetched_at = Some(now_rfc3339());
    source.status = Some(
        if warnings.iter().any(|item| item.severity == "error") {
            "error"
        } else if warnings.is_empty() {
            "ok"
        } else {
            "warning"
        }
        .to_string(),
    );
    source.warnings = warnings;
    store.save(&guard, &snapshot)?;
    Ok(())
}

fn refresh_imports_for_source(
    repo_dir: &Path,
    head_commit: &str,
    source_id: &str,
    variants: &[ExternalVariantSnapshot],
    imports: &mut [ImportedExternalSkillRecord],
) -> Result<Vec<ExternalSourceWarning>> {
    let mut source_warnings = Vec::new();
    for import in imports.iter_mut().filter(|item| item.external_source_id == source_id) {
        import.last_checked_commit = Some(head_commit.to_string());
        let still_detected = variants.iter().any(|variant| {
            variant.agent_key == import.agent_key
                && variant.variant_path == import.upstream_variant_path
        });
        if !still_detected {
            import.update_available = false;
            import.warnings = vec![warning(
                "variant_disappeared",
                "warning",
                format!(
                    "Imported variant {} for {} is no longer detected at the fetched upstream head",
                    import.upstream_variant_path, import.agent_key
                ),
            )];
        } else {
            let fingerprint =
                fingerprint_variant_at_ref(repo_dir, head_commit, &import.upstream_variant_path)?;
            import.update_available =
                fingerprint.as_deref() != import.pinned_variant_fingerprint.as_deref();
            import.warnings.clear();
        }
        source_warnings.extend(import.warnings.iter().cloned());
    }
    Ok(source_warnings)
}

fn runtime_integrity_warning(
    repo_root: &Path,
    import: &ImportedExternalSkillRecord,
) -> Option<ExternalSourceWarning> {
    let mirror_dir = repo_root.join(&import.mirror_relative_path);
    let manifest_path = mirror_dir.join(".skills-manager-source.json");
    if !mirror_dir.join("SKILL.md").is_file() || !manifest_path.is_file() {
        return Some(warning(
            "integrity_mismatch",
            "warning",
            "Managed mirror files are missing; repair is recommended",
        ));
    }
    let manifest_raw = match fs::read_to_string(&manifest_path) {
        Ok(raw) => raw,
        Err(_) => {
            return Some(warning(
                "integrity_mismatch",
                "warning",
                "Managed mirror metadata is unreadable; repair is recommended",
            ));
        }
    };
    let manifest = match serde_json::from_str::<ManagedSkillMirrorManifest>(&manifest_raw) {
        Ok(manifest) => manifest,
        Err(_) => {
            return Some(warning(
                "integrity_mismatch",
                "warning",
                "Managed mirror metadata is invalid; repair is recommended",
            ));
        }
    };
    let computed_skill_id = match build_skill_id_from_relative_path(&manifest.mirror_relative_path) {
        Ok(skill_id) => skill_id,
        Err(_) => {
            return Some(warning(
                "integrity_mismatch",
                "warning",
                "Managed mirror path metadata is inconsistent; repair is recommended",
            ));
        }
    };
    let valid = mirror_dir.join("SKILL.md").is_file()
        && manifest.managed
        && manifest.import_id == import.import_id
        && manifest.source_id == import.external_source_id
        && manifest.agent_key == import.agent_key
        && manifest.variant_path == import.upstream_variant_path
        && manifest.mirror_relative_path == import.mirror_relative_path
        && manifest.skill_id == import.skill_id
        && manifest.pinned_commit == import.pinned_commit
        && computed_skill_id == import.skill_id;
    (!valid).then(|| {
        warning(
            "integrity_mismatch",
            "warning",
            "Managed mirror metadata is inconsistent; repair is recommended",
        )
    })
}
