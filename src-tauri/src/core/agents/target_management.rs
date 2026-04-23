use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use super::target_manifest::{
    is_manifest_file_name, load_manifest, save_or_remove_manifest, AgentTargetManifestEntry,
};
use super::target_sync::remove_target;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImportTargetSkillResult {
    pub relative_path: String,
    pub absolute_path: String,
    pub deleted_source: bool,
}

pub fn take_over_unmanaged_target_skill(
    target_dir: &Path,
    agent_key: &str,
    entry_name: &str,
) -> Result<()> {
    validate_entry_name(entry_name)?;
    require_existing_target_path(target_dir, entry_name)?;
    let mut manifest = load_manifest(target_dir, agent_key)?;

    if manifest.entries.contains_key(entry_name) {
        return Err(anyhow!("Target entry is already managed"));
    }

    manifest.entries.insert(
        entry_name.to_string(),
        AgentTargetManifestEntry {
            skill_id: adopted_skill_id(agent_key, entry_name),
            relative_path: entry_name.to_string(),
            preserve_existing: true,
        },
    );
    save_or_remove_manifest(target_dir, agent_key, &manifest)
}

pub fn delete_target_skill_entry(
    target_dir: &Path,
    agent_key: &str,
    entry_name: &str,
) -> Result<()> {
    validate_entry_name(entry_name)?;
    let mut manifest = load_manifest(target_dir, agent_key)?;
    let target_path = target_dir.join(entry_name);

    if !target_path.exists() && !manifest.entries.contains_key(entry_name) {
        return Err(anyhow!("Target entry does not exist"));
    }

    remove_target(&target_path)?;
    manifest.entries.remove(entry_name);
    save_or_remove_manifest(target_dir, agent_key, &manifest)
}

pub fn import_unmanaged_target_skill(
    repo_root: &Path,
    target_dir: &Path,
    agent_key: &str,
    entry_name: &str,
    delete_source_after_import: bool,
) -> Result<ImportTargetSkillResult> {
    validate_entry_name(entry_name)?;
    let source_dir = require_existing_target_path(target_dir, entry_name)?;
    let manifest = load_manifest(target_dir, agent_key)?;

    if manifest.entries.contains_key(entry_name) {
        return Err(anyhow!(
            "Managed target entries cannot be imported as unmanaged skills"
        ));
    }
    if !source_dir.is_dir() {
        return Err(anyhow!("Only directory-based target skills can be imported"));
    }
    if !source_dir.join("SKILL.md").exists() {
        return Err(anyhow!("Target entry does not contain SKILL.md"));
    }

    let custom_root = repo_root.join("custom");
    fs::create_dir_all(&custom_root)
        .with_context(|| format!("Failed to create {:?}", custom_root))?;
    let folder_name = next_available_import_dir_name(&custom_root, entry_name);
    let destination_dir = custom_root.join(&folder_name);

    copy_dir_recursive(&source_dir, &destination_dir)?;

    if delete_source_after_import {
        delete_target_skill_entry(target_dir, agent_key, entry_name)?;
    }

    Ok(ImportTargetSkillResult {
        relative_path: format!("custom/{folder_name}"),
        absolute_path: normalize_path(&destination_dir),
        deleted_source: delete_source_after_import,
    })
}

fn require_existing_target_path(target_dir: &Path, entry_name: &str) -> Result<PathBuf> {
    let target_path = target_dir.join(entry_name);
    if !target_path.exists() {
        return Err(anyhow!("Target entry does not exist"));
    }
    Ok(target_path)
}

fn validate_entry_name(entry_name: &str) -> Result<()> {
    if entry_name.is_empty()
        || entry_name == "."
        || entry_name == ".."
        || entry_name.contains('/')
        || entry_name.contains('\\')
        || is_manifest_file_name(std::ffi::OsStr::new(entry_name))
    {
        return Err(anyhow!("Invalid target entry name"));
    }

    Ok(())
}

fn adopted_skill_id(agent_key: &str, entry_name: &str) -> String {
    format!("adopted:{agent_key}:{entry_name}")
}

fn next_available_import_dir_name(custom_root: &Path, entry_name: &str) -> String {
    let base_name = sanitize_import_dir_name(entry_name);
    let mut candidate = base_name.clone();
    let mut suffix = 2usize;

    while custom_root.join(&candidate).exists() {
        candidate = format!("{base_name}-{suffix}");
        suffix += 1;
    }

    candidate
}

fn sanitize_import_dir_name(entry_name: &str) -> String {
    let mut sanitized = String::new();
    let mut last_was_dash = false;

    for character in entry_name.chars() {
        if character.is_alphanumeric() || matches!(character, '-' | '_') {
            sanitized.push(character.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash {
            sanitized.push('-');
            last_was_dash = true;
        }
    }

    let trimmed = sanitized.trim_matches('-');
    if trimmed.is_empty() {
        "imported-skill".to_string()
    } else {
        trimmed.to_string()
    }
}

fn copy_dir_recursive(source_dir: &Path, destination_dir: &Path) -> Result<()> {
    fs::create_dir_all(destination_dir)
        .with_context(|| format!("Failed to create {:?}", destination_dir))?;

    for entry in
        fs::read_dir(source_dir).with_context(|| format!("Failed to read {:?}", source_dir))?
    {
        let entry = entry?;
        let file_name = entry.file_name();
        if file_name == ".git" {
            continue;
        }

        let destination_path = destination_dir.join(&file_name);
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &destination_path)?;
        } else {
            fs::copy(entry.path(), &destination_path)
                .with_context(|| format!("Failed to copy into {:?}", destination_path))?;
        }
    }

    Ok(())
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
