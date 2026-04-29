use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use super::models::{
    ExternalSourcesSnapshot, ImportedExternalSkillRecord, ManagedSkillMirrorManifest,
};
use crate::core::skills::identity::build_skill_id_from_relative_path;

pub(super) fn validate_target_path_for_import(
    target_dir: &Path,
    expected_manifest: &ManagedSkillMirrorManifest,
    snapshot: &ExternalSourcesSnapshot,
) -> Result<()> {
    if !target_dir.exists() {
        return Ok(());
    }
    if !target_dir.is_dir() {
        bail!(
            "Managed mirror target exists but is not a directory: {}",
            target_dir.display()
        );
    }

    let manifest_path = target_dir.join(".skills-manager-source.json");
    if !manifest_path.exists() {
        bail!(
            "Managed mirror target already exists and is unmanaged: {}",
            target_dir.display()
        );
    }

    let live_manifest = read_manifest(&manifest_path)?;
    let Some(record) = snapshot
        .imports
        .iter()
        .find(|record| record.import_id == live_manifest.import_id)
    else {
        bail!(
            "Managed mirror target {} has a manifest without a matching import record; repair is required",
            target_dir.display()
        );
    };

    if !manifest_matches_record(&live_manifest, record) {
        bail!(
            "Managed mirror target {} has inconsistent manifest metadata; repair is required",
            target_dir.display()
        );
    }

    if live_manifest.import_id != expected_manifest.import_id
        || live_manifest.skill_id != expected_manifest.skill_id
    {
        bail!(
            "Managed mirror target {} belongs to a different import",
            target_dir.display()
        );
    }

    Ok(())
}

pub(super) fn validate_live_mirror_for_removal(
    target_dir: &Path,
    import_record: &ImportedExternalSkillRecord,
) -> Result<ManagedSkillMirrorManifest> {
    if !target_dir.join("SKILL.md").is_file() {
        bail!("Managed mirror is missing SKILL.md at {}", target_dir.display());
    }

    let manifest_path = target_dir.join(".skills-manager-source.json");
    let live_manifest = read_manifest(&manifest_path)?;
    if !manifest_matches_record(&live_manifest, import_record) {
        bail!(
            "Managed mirror manifest does not match the import record at {}",
            manifest_path.display()
        );
    }

    let computed_skill_id = build_skill_id_from_relative_path(&live_manifest.mirror_relative_path)?;
    if computed_skill_id != live_manifest.skill_id {
        bail!("Managed mirror manifest skillId does not match its mirror path");
    }

    Ok(live_manifest)
}

pub(super) fn validate_mirror_dir(
    target_dir: &Path,
    manifest: &ManagedSkillMirrorManifest,
) -> Result<()> {
    if !target_dir.join("SKILL.md").is_file() {
        bail!("Managed mirror is missing SKILL.md at {}", target_dir.display());
    }
    let manifest_path = target_dir.join(".skills-manager-source.json");
    let live_manifest = read_manifest(&manifest_path)?;
    if &live_manifest != manifest {
        bail!(
            "Managed mirror manifest does not match expected metadata at {}",
            manifest_path.display()
        );
    }
    let computed_skill_id = build_skill_id_from_relative_path(&manifest.mirror_relative_path)?;
    if computed_skill_id != manifest.skill_id {
        bail!("Managed mirror manifest skillId does not match its mirror path");
    }
    Ok(())
}

pub(super) fn write_manifest(target_dir: &Path, manifest: &ManagedSkillMirrorManifest) -> Result<()> {
    fs::create_dir_all(target_dir)
        .with_context(|| format!("Failed to create {}", target_dir.display()))?;
    let json = serde_json::to_string_pretty(manifest)?;
    fs::write(target_dir.join(".skills-manager-source.json"), json)
        .context("Failed to write managed mirror manifest")?;
    Ok(())
}

pub(super) fn upsert_import_record(
    snapshot: &mut ExternalSourcesSnapshot,
    new_record: ImportedExternalSkillRecord,
) {
    if let Some(existing_record) = snapshot
        .imports
        .iter_mut()
        .find(|record| record.import_id == new_record.import_id || record.skill_id == new_record.skill_id)
    {
        *existing_record = new_record;
    } else {
        snapshot.imports.push(new_record);
        snapshot
            .imports
            .sort_by(|left, right| left.skill_id.cmp(&right.skill_id));
    }
}

pub(super) fn create_operation_dir(repo_root: &Path, purpose: &str, import_id: &str) -> Result<PathBuf> {
    let dir = repo_root
        .join(".tmp-skills")
        .join("external-sources")
        .join(format!("{purpose}-{import_id}-{}", unique_nonce()));
    fs::create_dir_all(&dir).with_context(|| format!("Failed to create {}", dir.display()))?;
    Ok(dir)
}

pub(super) fn create_sibling_path(target_dir: &Path, purpose: &str) -> PathBuf {
    let name = target_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("mirror");
    target_dir.with_file_name(format!(".{name}.{purpose}.{}", unique_nonce()))
}

pub(super) fn restore_previous_target(target_dir: &Path, backup_dir: Option<&Path>) -> Result<()> {
    if target_dir.exists() {
        fs::remove_dir_all(target_dir).with_context(|| {
            format!("Failed to remove incomplete mirror {}", target_dir.display())
        })?;
    }
    if let Some(backup_dir) = backup_dir {
        fs::rename(backup_dir, target_dir).with_context(|| {
            format!(
                "Failed to restore backup mirror {} into {}",
                backup_dir.display(),
                target_dir.display()
            )
        })?;
    }
    Ok(())
}

pub(super) fn remove_empty_managed_parents(
    mut current: Option<&Path>,
    repo_root: &Path,
) -> Result<()> {
    let stop = repo_root.join("external");
    while let Some(dir) = current {
        if dir == stop || dir == repo_root {
            break;
        }
        match fs::remove_dir(dir) {
            Ok(()) => current = dir.parent(),
            Err(error) if error.kind() == std::io::ErrorKind::DirectoryNotEmpty => break,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                current = dir.parent();
            }
            Err(error) => {
                return Err(error).with_context(|| format!("Failed to remove {}", dir.display()));
            }
        }
    }
    Ok(())
}

pub(super) fn current_timestamp_string() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("unix:{}", duration.as_secs()),
        Err(_) => "unix:0".to_string(),
    }
}

fn read_manifest(path: &Path) -> Result<ManagedSkillMirrorManifest> {
    let raw = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

fn manifest_matches_record(
    manifest: &ManagedSkillMirrorManifest,
    record: &ImportedExternalSkillRecord,
) -> bool {
    manifest.managed
        && manifest.import_id == record.import_id
        && manifest.source_id == record.external_source_id
        && manifest.agent_key == record.agent_key
        && manifest.variant_path == record.upstream_variant_path
        && manifest.mirror_relative_path == record.mirror_relative_path
        && manifest.skill_id == record.skill_id
        && manifest.pinned_commit == record.pinned_commit
}

fn unique_nonce() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("{}-{}", std::process::id(), duration.as_nanos()),
        Err(_) => format!("{}-0", std::process::id()),
    }
}
