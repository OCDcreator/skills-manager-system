use std::fs;
use std::path::Path;

use crate::core::external_sources::detect::detect_external_source_variants;
use crate::core::external_sources::git_repo::{read_default_branch, read_head_commit};
use crate::core::external_sources::git_tree::read_text_file_at_ref;
use crate::core::external_sources::models::{
    ExternalSourceRecord, ExternalSourceWarning, ImportedExternalSkillRecord,
    ManagedSkillMirrorManifest,
};
use crate::core::skills::identity::build_skill_id_from_relative_path;
use crate::core::skills::metadata::{SkillMetadata, parse_skill_metadata, parse_skill_metadata_content};

use super::service::ExternalVariantSnapshot;
use super::source_sync::cache_repo_absolute_path;

pub(super) fn load_variants_for_source(
    config_dir: &Path,
    record: &ExternalSourceRecord,
) -> Vec<ExternalVariantSnapshot> {
    record.cached_repo_path.as_deref().and_then(|_| {
        let repo_dir = cache_repo_absolute_path(config_dir, &record.id);
        let git_ref = resolve_cached_head_ref(&repo_dir);
        detect_external_source_variants(&repo_dir, &record.id)
            .ok()
            .map(|detection| {
                detection
                    .variants
                    .into_iter()
                    .map(|variant| {
                        let metadata = load_variant_metadata(
                            &repo_dir,
                            git_ref.as_deref(),
                            variant.source_of_truth_path.as_deref(),
                            variant.metadata_path.as_deref(),
                        );
                        ExternalVariantSnapshot {
                            agent_key: variant.agent_key,
                            variant_path: variant.variant_path,
                            source_of_truth_path: variant.source_of_truth_path,
                            metadata_path: variant.metadata_path,
                            name: metadata.name,
                            description: metadata.description,
                        }
                    })
                    .collect()
            })
    }).unwrap_or_default()
}

fn resolve_cached_head_ref(repo_dir: &Path) -> Option<String> {
    let branch = read_default_branch(repo_dir).ok()?;
    read_head_commit(repo_dir, &branch).ok()
}

fn load_variant_metadata(
    repo_dir: &Path,
    git_ref: Option<&str>,
    source_of_truth_path: Option<&str>,
    metadata_path: Option<&str>,
) -> SkillMetadata {
    let preferred_metadata_path = source_of_truth_path
        .map(|path| format!("{path}/SKILL.md"))
        .or_else(|| metadata_path.map(ToOwned::to_owned));

    let Some(skill_md_path) = preferred_metadata_path else {
        return SkillMetadata::default();
    };

    if let Some(git_ref) = git_ref {
        if let Ok(content) = read_text_file_at_ref(repo_dir, git_ref, &skill_md_path) {
            return parse_skill_metadata_content(&content);
        }
    }

    parse_skill_metadata(&repo_dir.join(skill_md_path)).unwrap_or_default()
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
