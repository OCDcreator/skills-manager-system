use std::fs;
use std::path::Path;

use crate::core::external_sources::detect::detect_external_source_variants;
use crate::core::external_sources::models::{
    ExternalSourceRecord, ExternalSourceWarning, ImportedExternalSkillRecord,
    ManagedSkillMirrorManifest,
};
use crate::core::skills::identity::build_skill_id_from_relative_path;

use super::service::ExternalVariantSnapshot;
use super::source_sync::cache_repo_absolute_path;

pub(super) fn load_variants_for_source(
    config_dir: &Path,
    record: &ExternalSourceRecord,
) -> Vec<ExternalVariantSnapshot> {
    record
        .cached_repo_path
        .as_deref()
        .and_then(|_| {
            detect_external_source_variants(&cache_repo_absolute_path(config_dir, &record.id), &record.id).ok()
        })
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
                .collect()
        })
        .unwrap_or_default()
}

pub(super) fn runtime_integrity_warning(
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

fn warning(code: &str, severity: &str, message: impl Into<String>) -> ExternalSourceWarning {
    ExternalSourceWarning {
        code: code.to_string(),
        severity: severity.to_string(),
        message: message.into(),
    }
}
