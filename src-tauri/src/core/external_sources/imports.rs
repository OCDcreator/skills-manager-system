use anyhow::{anyhow, bail, Context, Result};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use super::git_repo::normalize_github_repo_url;
use super::hash::sha256_hex;
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
    let variant_path = canonicalize_repo_relative_path(&input.upstream_variant_path)?;
    let cached_repo_path = resolve_cached_repo_path(config_dir, source.cached_repo_path.as_deref())?;
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
        .unwrap_or_else(|| stable_import_id(&input.external_source_id, &input.agent_key, &mirror_relative_path));
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
        let fingerprint =
            export_variant_from_git(&cached_repo_path, &input.pinned_commit, &variant_path, &stage_dir)?;
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
    let guard = acquire_config_lock(config_dir)?;
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

fn determine_mirror_relative_path(
    repo_root: &Path,
    snapshot: &ExternalSourcesSnapshot,
    source: &ExternalSourceRecord,
    agent_key: &str,
    variant_path: &str,
    normalized_repo_url: &str,
    existing_import: Option<&ImportedExternalSkillRecord>,
) -> Result<String> {
    if let Some(existing_import) = existing_import {
        return Ok(existing_import.mirror_relative_path.clone());
    }

    let repo_slug = build_repo_slug(snapshot, source, normalized_repo_url);
    let variant_hash = sha256_hex(variant_path.as_bytes());
    let mut variant_key = sanitize_segment(
        Path::new(variant_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(""),
        &variant_hash,
        "variant",
    );
    let base_variant_key = variant_key.clone();
    let used_paths: Vec<&str> = snapshot
        .imports
        .iter()
        .map(|record| record.mirror_relative_path.as_str())
        .collect();

    let mut suffix_len = 10usize;
    loop {
        let candidate =
            format!("external/managed/github/{repo_slug}/{agent_key}/{variant_key}");
        if candidate.len() > 180 {
            bail!("Managed mirror path would exceed the safe path budget");
        }
        let candidate_path = repo_root.join(&candidate);
        if !used_paths.iter().any(|path| *path == candidate) && !candidate_path.exists() {
            return Ok(candidate);
        }

        let next_len = suffix_len.min(variant_hash.len());
        variant_key = format!("{}-{}", base_variant_key, &variant_hash[..next_len]);
        if next_len == variant_hash.len() {
            bail!("Failed to determine a unique managed mirror path");
        }
        suffix_len = (suffix_len + 4).min(variant_hash.len());
    }
}

fn build_repo_slug(
    snapshot: &ExternalSourcesSnapshot,
    source: &ExternalSourceRecord,
    normalized_repo_url: &str,
) -> String {
    let repo_path = normalized_repo_url
        .strip_prefix("github.com/")
        .unwrap_or(normalized_repo_url);
    let (owner, repo) = repo_path.split_once('/').unwrap_or(("github", repo_path));
    let repo_url_hash = sha256_hex(normalized_repo_url.as_bytes());
    let base_slug = format!(
        "{}__{}",
        sanitize_segment(owner, &repo_url_hash, "owner"),
        sanitize_segment(repo, &repo_url_hash, "repo")
    );
    let collides = snapshot.sources.iter().any(|other| {
        other.id != source.id
            && normalize_github_repo_url(&other.repo_url).ok().as_deref() != Some(normalized_repo_url)
            && format!(
                "{}__{}",
                sanitize_segment(
                    normalize_github_repo_url(&other.repo_url)
                        .ok()
                        .as_deref()
                        .and_then(|value| value.strip_prefix("github.com/"))
                        .and_then(|value| value.split_once('/'))
                        .map(|(other_owner, _)| other_owner)
                        .unwrap_or("github"),
                    &repo_url_hash,
                    "owner"
                ),
                sanitize_segment(
                    normalize_github_repo_url(&other.repo_url)
                        .ok()
                        .as_deref()
                        .and_then(|value| value.strip_prefix("github.com/"))
                        .and_then(|value| value.split_once('/'))
                        .map(|(_, other_repo)| other_repo)
                        .unwrap_or("repo"),
                    &repo_url_hash,
                    "repo"
                )
            ) == base_slug
    });

    if collides {
        format!("{}-{}", base_slug, &repo_url_hash[..10])
    } else {
        base_slug
    }
}

fn sanitize_segment(raw: &str, hash: &str, fallback_prefix: &str) -> String {
    let lowered = raw.trim().to_ascii_lowercase();
    let mut sanitized = String::new();
    let mut previous_dash = false;
    for ch in lowered.chars() {
        let keep = matches!(ch, 'a'..='z' | '0'..='9' | '.' | '_' | '-');
        let mapped = if keep { ch } else { '-' };
        if mapped == '-' {
            if previous_dash {
                continue;
            }
            previous_dash = true;
        } else {
            previous_dash = false;
        }
        sanitized.push(mapped);
    }
    sanitized = sanitized
        .trim_matches(|ch: char| matches!(ch, '-' | '_' | '.'))
        .to_string();
    if sanitized.is_empty() {
        sanitized = format!("{fallback_prefix}-{}", &hash[..10]);
    }
    if is_reserved_windows_name(&sanitized) {
        sanitized = format!("{}-{}", sanitized, &hash[..10]);
    }
    sanitized
}

fn is_reserved_windows_name(segment: &str) -> bool {
    let stem = segment.split('.').next().unwrap_or(segment);
    matches!(
        stem,
        "con"
            | "prn"
            | "aux"
            | "nul"
            | "com1"
            | "com2"
            | "com3"
            | "com4"
            | "com5"
            | "com6"
            | "com7"
            | "com8"
            | "com9"
            | "lpt1"
            | "lpt2"
            | "lpt3"
            | "lpt4"
            | "lpt5"
            | "lpt6"
            | "lpt7"
            | "lpt8"
            | "lpt9"
    )
}

fn validate_target_path_for_import(
    target_dir: &Path,
    expected_manifest: &ManagedSkillMirrorManifest,
    snapshot: &ExternalSourcesSnapshot,
) -> Result<()> {
    if !target_dir.exists() {
        return Ok(());
    }
    if !target_dir.is_dir() {
        bail!("Managed mirror target exists but is not a directory: {}", target_dir.display());
    }

    let manifest_path = target_dir.join(".skills-manager-source.json");
    if !manifest_path.exists() {
        bail!("Managed mirror target already exists and is unmanaged: {}", target_dir.display());
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

fn validate_live_mirror_for_removal(
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

fn validate_mirror_dir(target_dir: &Path, manifest: &ManagedSkillMirrorManifest) -> Result<()> {
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

fn export_variant_from_git(
    repo_dir: &Path,
    git_ref: &str,
    variant_path: &str,
    destination_dir: &Path,
) -> Result<String> {
    let entries = list_variant_blob_entries(repo_dir, git_ref, variant_path)?;
    if entries.is_empty() {
        bail!("Variant '{}' was not found at commit {}", variant_path, git_ref);
    }

    let mut fingerprint_entries = Vec::new();
    for entry in entries {
        let bytes = run_git_bytes(git_cmd(repo_dir).args([
            "show",
            &format!("{git_ref}:{}", entry.full_path),
        ]))
        .with_context(|| format!("Failed to read {}", entry.full_path))?;
        let target_path = destination_dir.join(&entry.relative_path);
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create {}", parent.display()))?;
        }
        fs::write(&target_path, &bytes)
            .with_context(|| format!("Failed to write {}", target_path.display()))?;
        if entry.relative_path != ".skills-manager-source.json" {
            fingerprint_entries.push((entry.relative_path, sha256_hex(&bytes)));
        }
    }

    fingerprint_entries.sort_by(|left, right| left.0.cmp(&right.0));
    let mut serialized = Vec::new();
    for (relative_path, file_hash) in fingerprint_entries {
        serialized.extend_from_slice(relative_path.as_bytes());
        serialized.push(b'\n');
        serialized.extend_from_slice(file_hash.as_bytes());
        serialized.push(b'\n');
    }

    Ok(format!("sha256:{}", sha256_hex(&serialized)))
}

fn list_variant_blob_entries(
    repo_dir: &Path,
    git_ref: &str,
    variant_path: &str,
) -> Result<Vec<VariantBlobEntry>> {
    let output = run_git_bytes(git_cmd(repo_dir).args([
        "ls-tree",
        "-r",
        "--full-tree",
        "-z",
        git_ref,
        "--",
        variant_path,
    ]))?;

    let mut entries = Vec::new();
    for chunk in output.split(|byte| *byte == 0) {
        if chunk.is_empty() {
            continue;
        }
        let tab_index = chunk
            .iter()
            .position(|byte| *byte == b'\t')
            .ok_or_else(|| anyhow!("Malformed ls-tree output"))?;
        let header = std::str::from_utf8(&chunk[..tab_index]).context("Malformed tree header")?;
        let full_path =
            std::str::from_utf8(&chunk[tab_index + 1..]).context("Malformed tree path")?;
        let mut header_parts = header.split_whitespace();
        let _mode = header_parts.next();
        let object_type = header_parts.next();
        if object_type != Some("blob") {
            continue;
        }

        let relative_path = if full_path == variant_path {
            Path::new(full_path)
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| anyhow!("Variant path is invalid"))?
                .to_string()
        } else {
            full_path
                .strip_prefix(&format!("{variant_path}/"))
                .ok_or_else(|| anyhow!("Variant entry escaped its source path"))?
                .to_string()
        };

        entries.push(VariantBlobEntry {
            full_path: full_path.to_string(),
            relative_path,
        });
    }

    Ok(entries)
}

fn stable_import_id(source_id: &str, agent_key: &str, mirror_relative_path: &str) -> String {
    let seed = format!("{source_id}\n{agent_key}\n{mirror_relative_path}");
    format!("imp_{}", &sha256_hex(seed.as_bytes())[..12])
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

fn resolve_cached_repo_path(config_dir: &Path, cached_repo_path: Option<&str>) -> Result<PathBuf> {
    let cached_repo_path = cached_repo_path
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow!("External source is missing its cached repository path"))?;
    let path = Path::new(cached_repo_path);
    let resolved = if path.is_absolute() {
        path.to_path_buf()
    } else {
        config_dir.join(path)
    };
    if !resolved.is_dir() {
        bail!(
            "Cached repository path does not exist or is not a directory: {}",
            resolved.display()
        );
    }
    Ok(resolved)
}

fn write_manifest(target_dir: &Path, manifest: &ManagedSkillMirrorManifest) -> Result<()> {
    fs::create_dir_all(target_dir)
        .with_context(|| format!("Failed to create {}", target_dir.display()))?;
    let json = serde_json::to_string_pretty(manifest)?;
    fs::write(target_dir.join(".skills-manager-source.json"), json)
        .context("Failed to write managed mirror manifest")?;
    Ok(())
}

fn read_manifest(path: &Path) -> Result<ManagedSkillMirrorManifest> {
    let raw = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

fn upsert_import_record(
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

fn create_operation_dir(repo_root: &Path, purpose: &str, import_id: &str) -> Result<PathBuf> {
    let dir = repo_root
        .join(".tmp-skills")
        .join("external-sources")
        .join(format!("{purpose}-{import_id}-{}", unique_nonce()));
    fs::create_dir_all(&dir).with_context(|| format!("Failed to create {}", dir.display()))?;
    Ok(dir)
}

fn create_sibling_path(target_dir: &Path, purpose: &str) -> PathBuf {
    let name = target_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("mirror");
    target_dir.with_file_name(format!(".{name}.{purpose}.{}", unique_nonce()))
}

fn restore_previous_target(target_dir: &Path, backup_dir: Option<&Path>) -> Result<()> {
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

fn remove_empty_managed_parents(mut current: Option<&Path>, repo_root: &Path) -> Result<()> {
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
            Err(error) => return Err(error).with_context(|| format!("Failed to remove {}", dir.display())),
        }
    }
    Ok(())
}

fn current_timestamp_string() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("unix:{}", duration.as_secs()),
        Err(_) => "unix:0".to_string(),
    }
}

fn unique_nonce() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("{}-{}", std::process::id(), duration.as_nanos()),
        Err(_) => format!("{}-0", std::process::id()),
    }
}

fn git_cmd(repo_dir: &Path) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(repo_dir)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("LC_ALL", "C");
    cmd
}

fn run_git_bytes(cmd: &mut Command) -> Result<Vec<u8>> {
    let output = cmd.output().context("Failed to execute git")?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(anyhow!(
            "{}",
            String::from_utf8_lossy(&output.stderr).trim_end()
        ))
    }
}

struct VariantBlobEntry {
    full_path: String,
    relative_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_runtime::config_lock::acquire_config_lock;
    use crate::core::external_sources::models::{ExternalSourceRecord, ExternalSourcesSnapshot};
    use crate::core::external_sources::store::ExternalSourcesStore;
    use crate::core::scenes::config::SceneConfigStore;
    use std::fs;
    use std::path::Path;
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
            remove_imported_variant_from_repo(&config_dir, &repo_root, &imported.skill_id)
                .unwrap_err();

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
}
