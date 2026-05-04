use anyhow::{anyhow, bail, Context, Result};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

use super::git_export::export_variant_from_git;
use super::git_repo::normalize_github_repo_url;
use super::import_paths::{
    determine_mirror_relative_path, resolve_cached_repo_path, stable_import_id,
};
use super::mirror_fs::{
    create_operation_dir, create_sibling_path, current_timestamp_string,
    remove_empty_managed_parents, restore_previous_target, upsert_import_record,
    validate_live_mirror_for_removal, validate_mirror_dir, validate_target_path_for_import,
    write_manifest,
};
use super::models::{
    ExternalSourceRecord, ExternalSourcesSnapshot, ImportedExternalSkillRecord,
    ManagedSkillMirrorManifest,
};
use super::reference_check::find_skill_references;
use super::store::ExternalSourcesStore;
use crate::app_runtime::config_lock::acquire_config_lock;
use crate::core::skills::identity::{
    build_skill_id_from_relative_path, canonicalize_repo_relative_path,
};

#[derive(Debug, Clone)]
pub struct ImportVariantInput {
    pub external_source_id: String,
    pub agent_key: String,
    pub upstream_variant_path: String,
    pub pinned_commit: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImportVariantResult {
    pub import_id: String,
    pub skill_id: String,
    pub mirror_relative_path: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImportRemovalResult {
    pub import_id: String,
    pub skill_id: String,
    pub mirror_relative_path: String,
}

pub fn import_variant_into_repo(
    config_dir: &Path,
    repo_root: &Path,
    input: ImportVariantInput,
) -> Result<ImportVariantResult> {
    let guard = acquire_config_lock(config_dir)?;
    let store = ExternalSourcesStore::new(config_dir.to_path_buf());
    let snapshot = store.load()?;
    let source = find_source(&snapshot, &input.external_source_id)?;
    let variant_path = normalize_variant_path(&input.upstream_variant_path)?;
    let cached_repo_path =
        resolve_cached_repo_path(config_dir, source.cached_repo_path.as_deref())?;
    let normalized_repo_url = normalize_github_repo_url(&source.repo_url)?;
    let existing_import = snapshot.imports.iter().find(|record| {
        record.external_source_id == input.external_source_id
            && record.agent_key == input.agent_key
            && record.upstream_variant_path == variant_path
    });
    let mirror_relative_path = determine_mirror_relative_path(
        repo_root,
        &snapshot,
        source,
        &input.agent_key,
        &variant_path,
        &normalized_repo_url,
        existing_import,
    )?;
    let skill_id = build_skill_id_from_relative_path(&mirror_relative_path)?;
    let import_id = existing_import
        .map(|record| record.import_id.clone())
        .unwrap_or_else(|| {
            stable_import_id(
                &input.external_source_id,
                &input.agent_key,
                &mirror_relative_path,
            )
        });
    let manifest = ManagedSkillMirrorManifest {
        schema_version: ExternalSourcesSnapshot::SCHEMA_VERSION,
        managed: true,
        import_id: import_id.clone(),
        source_id: input.external_source_id.clone(),
        repo_url: source.repo_url.clone(),
        agent_key: input.agent_key.clone(),
        variant_path: variant_path.clone(),
        mirror_relative_path: mirror_relative_path.clone(),
        skill_id: skill_id.clone(),
        pinned_commit: input.pinned_commit.clone(),
    };
    let target_dir = repo_root.join(&mirror_relative_path);

    validate_target_path_for_import(&target_dir, &manifest, &snapshot)?;

    let stage_dir = create_operation_dir(repo_root, "stage", &import_id)?;
    let mut warnings = Vec::new();
    let import_result = (|| -> Result<ImportVariantResult> {
        let fingerprint = export_variant_from_git(
            &cached_repo_path,
            &input.pinned_commit,
            &variant_path,
            &stage_dir,
        )?;
        write_manifest(&stage_dir, &manifest)?;
        validate_mirror_dir(&stage_dir, &manifest)?;

        let backup_dir = if target_dir.exists() {
            let backup = create_sibling_path(&target_dir, "backup");
            fs::rename(&target_dir, &backup).with_context(|| {
                format!(
                    "Failed to move existing managed mirror {} into backup {}",
                    target_dir.display(),
                    backup.display()
                )
            })?;
            Some(backup)
        } else {
            None
        };

        if let Some(parent) = target_dir.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create {}", parent.display()))?;
        }

        if let Err(error) = fs::rename(&stage_dir, &target_dir) {
            restore_previous_target(&target_dir, backup_dir.as_deref())?;
            return Err(error).with_context(|| {
                format!(
                    "Failed to move staged mirror {} into {}",
                    stage_dir.display(),
                    target_dir.display()
                )
            });
        }

        if let Err(error) = validate_mirror_dir(&target_dir, &manifest) {
            restore_previous_target(&target_dir, backup_dir.as_deref())?;
            return Err(error).context("Post-swap managed mirror validation failed");
        }

        let imported_at = current_timestamp_string();
        let mut updated_snapshot = snapshot.clone();
        upsert_import_record(
            &mut updated_snapshot,
            ImportedExternalSkillRecord {
                import_id: import_id.clone(),
                external_source_id: input.external_source_id.clone(),
                agent_key: input.agent_key.clone(),
                upstream_variant_path: variant_path.clone(),
                pinned_commit: input.pinned_commit.clone(),
                pinned_variant_fingerprint: Some(fingerprint),
                skill_id: skill_id.clone(),
                mirror_relative_path: mirror_relative_path.clone(),
                last_checked_commit: Some(input.pinned_commit.clone()),
                imported_at: Some(imported_at),
                warnings: Vec::new(),
                update_available: false,
            },
        );

        if let Err(error) = store.save(&guard, &updated_snapshot) {
            restore_previous_target(&target_dir, backup_dir.as_deref())?;
            return Err(error).context("Failed to persist external import record");
        }

        if let Some(backup_dir) = backup_dir {
            if let Err(error) = fs::remove_dir_all(&backup_dir) {
                warnings.push(format!(
                    "Imported mirror but failed to clean backup {}: {error}",
                    backup_dir.display()
                ));
            }
        }

        Ok(ImportVariantResult {
            import_id,
            skill_id,
            mirror_relative_path,
            warnings: warnings.clone(),
        })
    })();

    if stage_dir.exists() {
        let _ = fs::remove_dir_all(&stage_dir);
    }

    import_result
}

pub fn remove_imported_variant_from_repo(
    config_dir: &Path,
    repo_root: &Path,
    skill_id: &str,
) -> Result<ImportRemovalResult> {
    let (import_record, target_dir) =
        preflight_remove_imported_variant(config_dir, repo_root, skill_id)?;
    let guard = acquire_config_lock(config_dir)?;
    let store = ExternalSourcesStore::new(config_dir.to_path_buf());
    let snapshot = store.load()?;

    let backup_dir = create_sibling_path(&target_dir, "remove");
    fs::rename(&target_dir, &backup_dir).with_context(|| {
        format!(
            "Failed to move managed mirror {} into {} for removal",
            target_dir.display(),
            backup_dir.display()
        )
    })?;

    let mut updated_snapshot = snapshot.clone();
    updated_snapshot
        .imports
        .retain(|record| record.skill_id != import_record.skill_id);

    if let Err(error) = store.save(&guard, &updated_snapshot) {
        restore_previous_target(&target_dir, Some(&backup_dir))?;
        return Err(error).context("Failed to remove import record");
    }

    fs::remove_dir_all(&backup_dir)
        .with_context(|| format!("Failed to remove {}", backup_dir.display()))?;
    remove_empty_managed_parents(target_dir.parent(), repo_root)?;

    Ok(ImportRemovalResult {
        import_id: import_record.import_id,
        skill_id: import_record.skill_id,
        mirror_relative_path: import_record.mirror_relative_path,
    })
}

pub fn preflight_remove_imported_variant(
    config_dir: &Path,
    repo_root: &Path,
    skill_id: &str,
) -> Result<(ImportedExternalSkillRecord, PathBuf)> {
    let store = ExternalSourcesStore::new(config_dir.to_path_buf());
    let snapshot = store.load()?;
    let import_record = snapshot
        .imports
        .iter()
        .find(|record| record.skill_id == skill_id)
        .cloned()
        .ok_or_else(|| anyhow!("Imported skill '{}' was not found", skill_id))?;
    let references = find_skill_references(config_dir, skill_id)?;
    if references.has_any() {
        bail!(
            "Imported skill '{}' is referenced by scenes={:?}, agents={:?}, projects={:?}",
            skill_id,
            references.scenes,
            references.agents,
            references.projects
        );
    }

    let target_dir = repo_root.join(&import_record.mirror_relative_path);
    validate_live_mirror_for_removal(&target_dir, &import_record)?;
    Ok((import_record, target_dir))
}

fn find_source<'a>(
    snapshot: &'a ExternalSourcesSnapshot,
    source_id: &str,
) -> Result<&'a ExternalSourceRecord> {
    snapshot
        .sources
        .iter()
        .find(|source| source.id == source_id)
        .ok_or_else(|| anyhow!("External source '{}' was not found", source_id))
}

fn normalize_variant_path(raw: &str) -> Result<String> {
    if raw.trim() == "." {
        Ok(".".to_string())
    } else {
        canonicalize_repo_relative_path(raw)
    }
}

#[cfg(test)]
#[path = "imports_tests.rs"]
mod tests;
