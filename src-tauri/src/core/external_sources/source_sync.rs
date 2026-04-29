use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::app_runtime::config_lock::acquire_config_lock;
use crate::core::external_sources::detect::DetectedExternalVariant;
use crate::core::external_sources::git_repo::{
    ensure_cached_repo, fingerprint_variant_at_ref, read_default_branch, read_head_commit,
};
use crate::core::external_sources::models::{
    ExternalSourceRecord, ExternalSourceWarning, ImportedExternalSkillRecord,
};
use crate::core::external_sources::{normalize_github_repo_url, ExternalSourcesStore};

pub(super) fn upsert_source(config_dir: &Path, repo_url: &str) -> Result<String> {
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

pub(super) fn set_source_failure(config_dir: &Path, source_id: &str, message: &str) -> Result<()> {
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

pub(super) fn fetch_source(config_dir: &Path, source_id: &str) -> Result<()> {
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
    let detection = super::detect_external_source_variants(&repo_dir, source_id)?;
    let mut warnings = detection.warnings;
    warnings.extend(refresh_imports_for_source(
        &repo_dir,
        &head_commit,
        source_id,
        &detection.variants,
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

pub(super) fn cache_repo_absolute_path(config_dir: &Path, source_id: &str) -> PathBuf {
    config_dir.join("external-sources").join(source_id).join("repo")
}

fn cache_repo_relative_path(source_id: &str) -> String {
    format!("external-sources/{source_id}/repo")
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

fn refresh_imports_for_source(
    repo_dir: &Path,
    head_commit: &str,
    source_id: &str,
    variants: &[DetectedExternalVariant],
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

fn warning(code: &str, severity: &str, message: impl Into<String>) -> ExternalSourceWarning {
    ExternalSourceWarning {
        code: code.to_string(),
        severity: severity.to_string(),
        message: message.into(),
    }
}
