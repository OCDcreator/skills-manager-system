use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::core::external_sources::{
    ExternalSourcesSnapshot, ImportedExternalSkillRecord, ManagedSkillMirrorManifest,
};

use super::scan::{ManagedSourceInfo, SkillSummary};

pub fn enrich_managed_external_skills(
    skills: &mut [SkillSummary],
    warnings: &mut Vec<String>,
    snapshot: &ExternalSourcesSnapshot,
) {
    for skill in skills {
        if skill.source_type != "external" {
            continue;
        }

        let manifest_path = Path::new(&skill.directory_path).join(".skills-manager-source.json");
        if !manifest_path.exists() {
            continue;
        }

        match read_managed_manifest(&manifest_path) {
            Ok(manifest) => {
                skill.managed_source = Some(build_managed_source_info(
                    skill,
                    &manifest,
                    snapshot
                        .imports
                        .iter()
                        .find(|record| record.skill_id == skill.id),
                ));
            }
            Err(error) => warnings.push(format!(
                "Failed to parse managed mirror manifest at {}: {error}",
                manifest_path.display()
            )),
        }
    }
}

fn read_managed_manifest(path: &Path) -> Result<ManagedSkillMirrorManifest> {
    let raw = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

fn build_managed_source_info(
    skill: &SkillSummary,
    manifest: &ManagedSkillMirrorManifest,
    import_record: Option<&ImportedExternalSkillRecord>,
) -> ManagedSourceInfo {
    let Some(import_record) = import_record else {
        return mismatch_managed_source(manifest);
    };

    if !manifest.managed
        || manifest.skill_id != skill.id
        || manifest.mirror_relative_path != skill.relative_path
        || manifest.import_id != import_record.import_id
        || manifest.source_id != import_record.external_source_id
        || manifest.agent_key != import_record.agent_key
        || manifest.variant_path != import_record.upstream_variant_path
        || manifest.pinned_commit != import_record.pinned_commit
    {
        return mismatch_managed_source(manifest);
    }

    ManagedSourceInfo {
        kind: "github_import".to_string(),
        import_id: import_record.import_id.clone(),
        repo_url: manifest.repo_url.clone(),
        pinned_commit: import_record.pinned_commit.clone(),
        agent_key: import_record.agent_key.clone(),
        update_available: import_record.update_available,
        integrity: None,
    }
}

fn mismatch_managed_source(manifest: &ManagedSkillMirrorManifest) -> ManagedSourceInfo {
    ManagedSourceInfo {
        kind: "github_import".to_string(),
        import_id: manifest.import_id.clone(),
        repo_url: manifest.repo_url.clone(),
        pinned_commit: manifest.pinned_commit.clone(),
        agent_key: manifest.agent_key.clone(),
        update_available: false,
        integrity: Some("mismatch".to_string()),
    }
}
