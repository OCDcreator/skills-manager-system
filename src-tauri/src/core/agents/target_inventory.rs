use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use super::target_manifest::is_manifest_file_name;
use super::target_sync::load_managed_entry_snapshots;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AgentTargetSkillEntryKind {
    Directory,
    Symlink,
    File,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgentTargetSkillEntry {
    pub entry_name: String,
    pub display_name: String,
    pub absolute_path: String,
    pub symlink_target_path: Option<String>,
    pub managed: bool,
    pub preserve_existing: bool,
    pub skill_id: Option<String>,
    pub relative_path: Option<String>,
    pub has_skill_document: bool,
    pub entry_kind: AgentTargetSkillEntryKind,
}

pub fn scan_target_skill_entries(
    target_dir: &Path,
    agent_key: &str,
) -> Result<Vec<AgentTargetSkillEntry>> {
    if !target_dir.exists() {
        return Ok(Vec::new());
    }

    let managed_entries = load_managed_entry_snapshots(target_dir, agent_key)?;
    let mut entries = Vec::new();

    for entry in
        fs::read_dir(target_dir).with_context(|| format!("Failed to read {target_dir:?}"))?
    {
        let entry = entry?;
        let entry_name = entry.file_name();
        if is_manifest_file_name(&entry_name) {
            continue;
        }

        let entry_name = entry_name.to_string_lossy().to_string();
        let path = entry.path();
        let managed = managed_entries.get(&entry_name);
        let metadata = fs::symlink_metadata(&path)?;
        let entry_kind = entry_kind_from(&metadata);
        let symlink_target_path = read_symlink_target_path(&path, &entry_kind);
        let has_skill_document = path.join("SKILL.md").exists();

        if !should_report_entry(
            &entry_name,
            managed.is_some(),
            has_skill_document,
            &entry_kind,
        ) {
            continue;
        }

        entries.push(AgentTargetSkillEntry {
            display_name: managed
                .map(|item| item.relative_path.clone())
                .or_else(|| read_skill_display_name(&path))
                .unwrap_or_else(|| entry_name.clone()),
            entry_name: entry_name.clone(),
            absolute_path: normalize_path(path),
            symlink_target_path,
            managed: managed.is_some(),
            preserve_existing: managed.is_some_and(|item| item.preserve_existing),
            skill_id: managed.map(|item| item.skill_id.clone()),
            relative_path: managed.map(|item| item.relative_path.clone()),
            has_skill_document,
            entry_kind,
        });
    }

    entries.sort_by(|left, right| {
        left.managed
            .cmp(&right.managed)
            .reverse()
            .then_with(|| left.display_name.cmp(&right.display_name))
    });
    Ok(entries)
}

fn entry_kind_from(metadata: &fs::Metadata) -> AgentTargetSkillEntryKind {
    if metadata.file_type().is_symlink() {
        AgentTargetSkillEntryKind::Symlink
    } else if metadata.is_dir() {
        AgentTargetSkillEntryKind::Directory
    } else if metadata.is_file() {
        AgentTargetSkillEntryKind::File
    } else {
        AgentTargetSkillEntryKind::Other
    }
}

fn should_report_entry(
    entry_name: &str,
    is_managed: bool,
    has_skill_document: bool,
    entry_kind: &AgentTargetSkillEntryKind,
) -> bool {
    is_managed
        || has_skill_document
        || (!entry_name.starts_with('.')
            && matches!(
                entry_kind,
                AgentTargetSkillEntryKind::Directory | AgentTargetSkillEntryKind::Symlink
            ))
}

fn read_skill_display_name(path: &Path) -> Option<String> {
    let raw = fs::read_to_string(path.join("SKILL.md")).ok()?;
    read_frontmatter_name(&raw).or_else(|| read_heading_name(&raw))
}

fn read_symlink_target_path(path: &Path, entry_kind: &AgentTargetSkillEntryKind) -> Option<String> {
    if !matches!(entry_kind, AgentTargetSkillEntryKind::Symlink) {
        return None;
    }

    let raw_target = fs::read_link(path).ok()?;
    let resolved_target = if raw_target.is_absolute() {
        raw_target
    } else {
        path.parent().unwrap_or(Path::new("")).join(raw_target)
    };

    Some(normalize_path(resolved_target))
}

fn read_frontmatter_name(raw: &str) -> Option<String> {
    let mut lines = raw.lines();
    if lines.next()? != "---" {
        return None;
    }

    for line in lines {
        if line == "---" {
            break;
        }
        if let Some(name) = line.strip_prefix("name:") {
            return Some(name.trim().trim_matches('"').trim_matches('\'').to_string())
                .filter(|value| !value.is_empty());
        }
    }

    None
}

fn read_heading_name(raw: &str) -> Option<String> {
    raw.lines()
        .find_map(|line| line.strip_prefix("# "))
        .map(str::trim)
        .map(str::to_string)
        .filter(|value| !value.is_empty())
}

fn normalize_path(path: PathBuf) -> String {
    path.to_string_lossy().replace('\\', "/")
}
