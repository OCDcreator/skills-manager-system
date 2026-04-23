use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::skills::scan::SkillSummary;

use super::target_manifest::{
    default_manifest, load_manifest, remove_manifest, save_manifest, AgentTargetManifestEntry,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SyncMode {
    #[default]
    Copy,
    Symlink,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManagedTargetEntrySnapshot {
    pub skill_id: String,
    pub relative_path: String,
    pub preserve_existing: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct DesiredSkillEntry {
    pub skill_id: String,
    pub relative_path: String,
    pub source_dir: PathBuf,
}

#[derive(Debug, Default)]
pub(crate) struct TargetApplyStats {
    pub written_count: usize,
    pub removed_count: usize,
    pub conflict_count: usize,
}

pub(crate) fn build_desired_skill_entries(
    skills: &[SkillSummary],
) -> BTreeMap<String, DesiredSkillEntry> {
    skills
        .iter()
        .map(|skill| {
            (
                managed_entry_name(&skill.id),
                DesiredSkillEntry {
                    skill_id: skill.id.clone(),
                    relative_path: skill.relative_path.clone(),
                    source_dir: PathBuf::from(&skill.directory_path),
                },
            )
        })
        .collect()
}

pub(crate) fn apply_desired_entries(
    target_dir: &Path,
    agent_key: &str,
    desired_entries: &BTreeMap<String, DesiredSkillEntry>,
    mode: SyncMode,
) -> Result<TargetApplyStats> {
    let previous_entries = load_manifest(target_dir, agent_key)?.entries;
    let managed_entry_names = previous_entries
        .keys()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let mut stats = TargetApplyStats::default();
    let mut next_entries = BTreeMap::new();

    for (entry_name, existing_entry) in &previous_entries {
        let target_path = target_dir.join(entry_name);
        if desired_entries.contains_key(entry_name) {
            if existing_entry.preserve_existing && target_path.exists() {
                next_entries.insert(entry_name.clone(), existing_entry.clone());
            }
            continue;
        }

        if existing_entry.preserve_existing {
            if target_path.exists() {
                next_entries.insert(entry_name.clone(), existing_entry.clone());
            }
            continue;
        }

        remove_target(&target_path)?;
        stats.removed_count += 1;
    }

    for (entry_name, desired_entry) in desired_entries {
        let target_path = target_dir.join(entry_name);
        if target_path.exists() && !managed_entry_names.contains(entry_name) {
            stats.conflict_count += 1;
            continue;
        }
        if previous_entries
            .get(entry_name)
            .is_some_and(|entry| entry.preserve_existing && target_path.exists())
        {
            stats.conflict_count += 1;
            continue;
        }

        remove_target(&target_path)?;
        deploy_skill(&desired_entry.source_dir, &target_path, mode)?;
        stats.written_count += 1;
        next_entries.insert(
            entry_name.clone(),
            AgentTargetManifestEntry {
                skill_id: desired_entry.skill_id.clone(),
                relative_path: desired_entry.relative_path.clone(),
                preserve_existing: false,
            },
        );
    }

    if next_entries.is_empty() {
        remove_manifest(target_dir)?;
    } else {
        fs::create_dir_all(target_dir)
            .with_context(|| format!("Failed to create {:?}", target_dir))?;
        let mut manifest = default_manifest();
        manifest.entries = next_entries;
        save_manifest(target_dir, agent_key, &manifest)?;
    }

    Ok(stats)
}

pub(crate) fn cleanup_managed_entries(target_dir: &Path, agent_key: &str) -> Result<usize> {
    let manifest = load_manifest(target_dir, agent_key)?;
    if manifest.entries.is_empty() {
        return Ok(0);
    }

    let mut removed_count = 0;
    for entry_name in manifest.entries.keys() {
        remove_target(&target_dir.join(entry_name))?;
        removed_count += 1;
    }

    remove_manifest(target_dir)?;
    Ok(removed_count)
}

pub(crate) fn load_managed_entry_snapshots(
    target_dir: &Path,
    agent_key: &str,
) -> Result<BTreeMap<String, ManagedTargetEntrySnapshot>> {
    let manifest = load_manifest(target_dir, agent_key)?;
    Ok(manifest
        .entries
        .into_iter()
        .map(|(entry_name, entry)| {
            (
                entry_name,
                ManagedTargetEntrySnapshot {
                    skill_id: entry.skill_id,
                    relative_path: entry.relative_path,
                    preserve_existing: entry.preserve_existing,
                },
            )
        })
        .collect())
}

fn deploy_skill(source_dir: &Path, target_dir: &Path, mode: SyncMode) -> Result<()> {
    match mode {
        SyncMode::Symlink => symlink_dir_contents(source_dir, target_dir),
        SyncMode::Copy => copy_dir_recursive(source_dir, target_dir),
    }
}

fn symlink_dir_contents(source_dir: &Path, target_dir: &Path) -> Result<()> {
    fs::create_dir_all(target_dir).with_context(|| format!("Failed to create {:?}", target_dir))?;

    for entry in
        fs::read_dir(source_dir).with_context(|| format!("Failed to read {:?}", source_dir))?
    {
        let entry = entry?;
        let file_name = entry.file_name();
        if file_name == ".git" {
            continue;
        }

        let link_path = target_dir.join(&file_name);
        let original = entry.path();

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&original, &link_path)
                .with_context(|| format!("Failed to symlink {:?} -> {:?}", original, link_path))?;
        }

        #[cfg(windows)]
        {
            if entry.file_type()?.is_dir() {
                std::os::windows::fs::symlink_dir(&original, &link_path).with_context(|| {
                    format!(
                        "Failed to symlink dir {:?} -> {:?} (developer mode may be required)",
                        original, link_path
                    )
                })?;
            } else {
                std::os::windows::fs::symlink_file(&original, &link_path).with_context(|| {
                    format!("Failed to symlink {:?} -> {:?}", original, link_path)
                })?;
            }
        }
    }

    Ok(())
}

fn copy_dir_recursive(source_dir: &Path, target_dir: &Path) -> Result<()> {
    fs::create_dir_all(target_dir).with_context(|| format!("Failed to create {:?}", target_dir))?;

    for entry in
        fs::read_dir(source_dir).with_context(|| format!("Failed to read {:?}", source_dir))?
    {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let file_name = entry.file_name();
        if file_name == ".git" {
            continue;
        }

        let destination_path = target_dir.join(&file_name);
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &destination_path)?;
        } else {
            fs::copy(entry.path(), &destination_path)
                .with_context(|| format!("Failed to copy into {:?}", destination_path))?;
        }
    }

    Ok(())
}

pub(crate) fn remove_target(target: &Path) -> Result<()> {
    if !target.exists() {
        return Ok(());
    }

    let metadata = fs::symlink_metadata(target)?;
    if metadata.file_type().is_symlink() || metadata.is_file() {
        fs::remove_file(target).with_context(|| format!("Failed to remove {:?}", target))?;
    } else if metadata.is_dir() {
        fs::remove_dir_all(target).with_context(|| format!("Failed to remove {:?}", target))?;
    }

    Ok(())
}

pub(crate) fn managed_entry_name(skill_id: &str) -> String {
    skill_id.replace(':', "--").replace(['/', '\\'], "--")
}
