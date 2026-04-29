use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};
use serde::Serialize;

use crate::app_runtime::config_lock::acquire_config_lock;
use crate::core::external_sources::imports::{
    import_variant_into_repo as import_variant_into_repo_core,
    preflight_remove_imported_variant, remove_imported_variant_from_repo, ImportVariantInput,
    ImportVariantResult,
};
use crate::core::external_sources::models::{
    ExternalSourceRecord, ImportedExternalSkillRecord,
};
use crate::core::external_sources::ExternalSourcesStore;

use super::source_snapshot::{load_variants_for_source, runtime_integrity_warning};
use super::source_sync::{fetch_source, set_source_failure, upsert_source};

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
    let snapshot = ExternalSourcesStore::new(config_dir.to_path_buf()).load()?;
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
    let cache_root = config_dir.join("external-sources").join(source_id);
    if cache_root.exists() {
        fs::remove_dir_all(&cache_root)?;
    }

    let mut snapshot = store.load()?;
    snapshot.sources.retain(|item| item.id != source_id);
    snapshot
        .imports
        .retain(|item| item.external_source_id != source_id);
    store.save(&guard, &snapshot)?;

    list_external_sources(config_dir, repo_root)
}

#[cfg(test)]
#[path = "service_tests.rs"]
mod tests;

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
