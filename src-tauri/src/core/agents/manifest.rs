use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::skills::scan::SkillSummary;

const APP_ID: &str = "skills-manager-system";
const MANIFEST_FILE_NAME: &str = ".skills-manager-system-manifest.json";
const LEDGER_FILE_NAME: &str = "agent-sync-ledger.json";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SyncMode {
    #[default]
    Copy,
    Symlink,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentSyncLedger {
    pub agents: BTreeMap<String, AgentSyncLedgerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentSyncLedgerEntry {
    pub last_applied_target_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct AgentTargetManifest {
    app_id: String,
    agent_key: String,
    entries: BTreeMap<String, AgentTargetManifestEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct AgentTargetManifestEntry {
    skill_id: String,
    relative_path: String,
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
    let mut manifest = load_manifest(target_dir, agent_key)?;
    let managed_entry_names = manifest
        .entries
        .keys()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let stale_entry_names: Vec<String> = manifest
        .entries
        .keys()
        .filter(|entry_name| !desired_entries.contains_key(*entry_name))
        .cloned()
        .collect();
    let mut stats = TargetApplyStats::default();

    for entry_name in stale_entry_names {
        remove_target(&target_dir.join(&entry_name))?;
        manifest.entries.remove(&entry_name);
        stats.removed_count += 1;
    }

    let mut next_entries = BTreeMap::new();
    for (entry_name, desired_entry) in desired_entries {
        let target_path = target_dir.join(entry_name);
        if target_path.exists() && !managed_entry_names.contains(entry_name) {
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
            },
        );
    }

    if next_entries.is_empty() {
        remove_manifest(target_dir)?;
    } else {
        fs::create_dir_all(target_dir)
            .with_context(|| format!("Failed to create {:?}", target_dir))?;
        manifest.entries = next_entries;
        save_manifest(target_dir, &manifest)?;
    }

    Ok(stats)
}

pub(crate) fn cleanup_managed_entries(target_dir: &Path, agent_key: &str) -> Result<usize> {
    if !manifest_path(target_dir).exists() {
        return Ok(0);
    }

    let manifest = load_manifest(target_dir, agent_key)?;
    let mut removed_count = 0;

    for entry_name in manifest.entries.keys() {
        remove_target(&target_dir.join(entry_name))?;
        removed_count += 1;
    }

    remove_manifest(target_dir)?;
    Ok(removed_count)
}

pub(crate) fn load_ledger(config_dir: &Path) -> Result<AgentSyncLedger> {
    let path = ledger_path(config_dir);
    if !path.exists() {
        return Ok(AgentSyncLedger::default());
    }

    let raw = fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
    serde_json::from_str::<AgentSyncLedger>(&raw)
        .with_context(|| format!("Failed to parse {:?}", path))
}

pub(crate) fn save_ledger(config_dir: &Path, ledger: &AgentSyncLedger) -> Result<()> {
    fs::create_dir_all(config_dir).with_context(|| format!("Failed to create {:?}", config_dir))?;
    let json = serde_json::to_string_pretty(ledger)?;
    fs::write(ledger_path(config_dir), json).context("Failed to write agent-sync-ledger.json")?;
    Ok(())
}

fn load_manifest(target_dir: &Path, agent_key: &str) -> Result<AgentTargetManifest> {
    let path = manifest_path(target_dir);
    if !path.exists() {
        return Ok(AgentTargetManifest {
            app_id: APP_ID.to_string(),
            agent_key: agent_key.to_string(),
            entries: BTreeMap::new(),
        });
    }

    let raw = fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
    let manifest = serde_json::from_str::<AgentTargetManifest>(&raw)
        .with_context(|| format!("Failed to parse {:?}", path))?;

    if manifest.app_id != APP_ID {
        return Err(anyhow!(
            "Managed manifest at {:?} belongs to a different app",
            path
        ));
    }
    if manifest.agent_key != agent_key {
        return Err(anyhow!(
            "Managed manifest at {:?} belongs to a different agent",
            path
        ));
    }

    Ok(manifest)
}

fn save_manifest(target_dir: &Path, manifest: &AgentTargetManifest) -> Result<()> {
    let json = serde_json::to_string_pretty(manifest)?;
    fs::write(manifest_path(target_dir), json).context("Failed to write target manifest")?;
    Ok(())
}

fn remove_manifest(target_dir: &Path) -> Result<()> {
    let path = manifest_path(target_dir);
    if path.exists() {
        fs::remove_file(&path).with_context(|| format!("Failed to remove {:?}", path))?;
    }
    Ok(())
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

fn remove_target(target: &Path) -> Result<()> {
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

fn ledger_path(config_dir: &Path) -> PathBuf {
    config_dir.join(LEDGER_FILE_NAME)
}

fn manifest_path(target_dir: &Path) -> PathBuf {
    target_dir.join(MANIFEST_FILE_NAME)
}

fn managed_entry_name(skill_id: &str) -> String {
    skill_id.replace(':', "--").replace(['/', '\\'], "--")
}
