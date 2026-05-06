use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use walkdir::{DirEntry, WalkDir};

use crate::app_runtime::write_text_atomic;
use crate::core::external_sources::ExternalSourcesStore;

use super::identity::{
    build_skill_id_from_relative_path, canonicalize_repo_relative_path, resolve_source_type,
};
use super::managed_scan::enrich_managed_external_skills;
use super::metadata::parse_skill_metadata;
use super::scan::{ScanSkillsResponse, SkillSummary};
use super::state::{build_repo_state_key, normalize_repo_path};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct SkillDocumentFingerprint {
    relative_path: String,
    modified_millis: Option<u128>,
    file_len: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CachedSkillEntry {
    fingerprint: SkillDocumentFingerprint,
    summary: SkillSummary,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CachedRepoScan {
    repo_path: String,
    entries: BTreeMap<String, CachedSkillEntry>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct SkillScanCacheFile {
    schema_version: u32,
    repos: BTreeMap<String, CachedRepoScan>,
}

const CACHE_SCHEMA_VERSION: u32 = 1;
const CACHE_FILE_NAME: &str = "skill-scan-cache.json";
const IGNORED_DIRS: &[&str] = &[".git", "node_modules", "dist", "target", ".tmp-skills"];

pub fn load_cached_repo_skills(
    repo_root: &Path,
    config_dir: &Path,
) -> Result<Option<ScanSkillsResponse>> {
    if !repo_root.exists() {
        return Err(anyhow!("Configured repository path does not exist"));
    }

    let cache_file = load_cache_file(config_dir)?;
    let repo_key = build_repo_state_key(repo_root)?;
    let Some(repo_cache) = cache_file.repos.get(&repo_key) else {
        return Ok(None);
    };

    let mut skills = repo_cache
        .entries
        .values()
        .map(|entry| {
            let mut summary = entry.summary.clone();
            summary.managed_source = None;
            summary
        })
        .collect::<Vec<_>>();
    skills.sort_by(|left, right| left.id.cmp(&right.id));

    Ok(Some(ScanSkillsResponse {
        skills,
        warnings: Vec::new(),
    }))
}

pub fn load_cached_repo_skills_with_external_sources(
    repo_root: &Path,
    config_dir: &Path,
) -> Result<Option<ScanSkillsResponse>> {
    let Some(mut response) = load_cached_repo_skills(repo_root, config_dir)? else {
        return Ok(None);
    };
    enrich_from_external_sources(&mut response, config_dir)?;
    Ok(Some(response))
}

pub fn scan_repo_skills_cached(repo_root: &Path, config_dir: &Path) -> Result<ScanSkillsResponse> {
    if !repo_root.exists() {
        return Err(anyhow!("Configured repository path does not exist"));
    }

    let (fingerprints, mut warnings) = collect_skill_fingerprints(repo_root)?;
    let mut cache_file = match load_cache_file(config_dir) {
        Ok(cache_file) => cache_file,
        Err(error) => {
            warnings.push(format!(
                "Skill scan cache could not be read; rebuilding: {error}"
            ));
            SkillScanCacheFile::default()
        }
    };

    let repo_key = build_repo_state_key(repo_root)?;
    let cached_entries = cache_file
        .repos
        .get(&repo_key)
        .map(|repo_cache| &repo_cache.entries);
    let mut next_entries = BTreeMap::new();

    for fingerprint in fingerprints {
        let cached_entry = cached_entries
            .and_then(|entries| entries.get(&fingerprint.relative_path))
            .filter(|entry| entry.fingerprint == fingerprint);

        let entry = match cached_entry {
            Some(entry) => {
                let mut summary = entry.summary.clone();
                summary.managed_source = None;
                CachedSkillEntry {
                    fingerprint,
                    summary,
                }
            }
            None => CachedSkillEntry {
                summary: build_skill_summary(repo_root, &fingerprint.relative_path)?,
                fingerprint,
            },
        };

        next_entries.insert(entry.fingerprint.relative_path.clone(), entry);
    }

    let mut skills = next_entries
        .values()
        .map(|entry| entry.summary.clone())
        .collect::<Vec<_>>();
    skills.sort_by(|left, right| left.id.cmp(&right.id));

    cache_file.schema_version = CACHE_SCHEMA_VERSION;
    cache_file.repos.insert(
        repo_key,
        CachedRepoScan {
            repo_path: normalize_repo_path(repo_root),
            entries: next_entries,
        },
    );
    if let Err(error) = save_cache_file(config_dir, &cache_file) {
        warnings.push(format!("Skill scan cache could not be written: {error}"));
    }

    Ok(ScanSkillsResponse { skills, warnings })
}

pub fn scan_repo_skills_cached_with_external_sources(
    repo_root: &Path,
    config_dir: &Path,
) -> Result<ScanSkillsResponse> {
    let mut response = scan_repo_skills_cached(repo_root, config_dir)?;
    enrich_from_external_sources(&mut response, config_dir)?;
    Ok(response)
}

fn enrich_from_external_sources(
    response: &mut ScanSkillsResponse,
    config_dir: &Path,
) -> Result<()> {
    let snapshot = ExternalSourcesStore::new(config_dir.to_path_buf()).load()?;
    enrich_managed_external_skills(&mut response.skills, &mut response.warnings, &snapshot);
    Ok(())
}

fn collect_skill_fingerprints(
    repo_root: &Path,
) -> Result<(Vec<SkillDocumentFingerprint>, Vec<String>)> {
    let mut fingerprints = Vec::new();
    let mut warnings = Vec::new();

    let custom_root = repo_root.join("custom");
    if custom_root.exists() {
        for entry in fs::read_dir(&custom_root)? {
            let entry = entry?;
            let skill_dir = entry.path();
            if !skill_dir.is_dir() {
                continue;
            }
            push_skill_fingerprint(repo_root, &skill_dir, &mut fingerprints)?;
        }
    } else {
        warnings.push("Missing custom/ directory; continuing with remaining sources.".to_string());
    }

    let external_root = repo_root.join("external");
    if external_root.exists() {
        for entry in WalkDir::new(&external_root)
            .into_iter()
            .filter_entry(should_walk)
            .filter_map(Result::ok)
        {
            if entry.file_type().is_file() && entry.file_name() == "SKILL.md" {
                let skill_dir = entry.path().parent().expect("SKILL.md must have a parent");
                push_skill_fingerprint(repo_root, skill_dir, &mut fingerprints)?;
            }
        }
    } else {
        warnings
            .push("Missing external/ directory; continuing with remaining sources.".to_string());
    }

    fingerprints.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    Ok((fingerprints, warnings))
}

fn push_skill_fingerprint(
    repo_root: &Path,
    skill_dir: &Path,
    fingerprints: &mut Vec<SkillDocumentFingerprint>,
) -> Result<()> {
    let skill_md_path = skill_dir.join("SKILL.md");
    if !skill_md_path.exists() {
        return Ok(());
    }

    let relative_path = normalize_relative_path(repo_root, skill_dir)?;
    let metadata = fs::metadata(&skill_md_path)?;
    let modified_millis = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_millis());

    fingerprints.push(SkillDocumentFingerprint {
        relative_path,
        modified_millis,
        file_len: metadata.len(),
    });
    Ok(())
}

fn should_walk(entry: &DirEntry) -> bool {
    !IGNORED_DIRS
        .iter()
        .any(|ignored| entry.file_name() == *ignored)
}

fn build_skill_summary(repo_root: &Path, relative_path: &str) -> Result<SkillSummary> {
    let canonical_relative_path = canonicalize_repo_relative_path(relative_path)?;
    let skill_dir = repo_root.join(&canonical_relative_path);
    let skill_md_path = skill_dir.join("SKILL.md");
    let metadata = parse_skill_metadata(&skill_md_path)?;
    let source_type = resolve_source_type(&canonical_relative_path)?;
    let default_name = skill_dir
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown-skill".to_string());

    Ok(SkillSummary {
        id: build_skill_id_from_relative_path(&canonical_relative_path)?,
        name: metadata.name.unwrap_or(default_name),
        description: metadata.description.unwrap_or_default(),
        source_type: source_type.to_string(),
        relative_path: canonical_relative_path,
        directory_path: path_to_string(&skill_dir),
        skill_document_path: path_to_string(&skill_md_path),
        managed_source: None,
    })
}

fn normalize_relative_path(repo_root: &Path, skill_dir: &Path) -> Result<String> {
    let relative = skill_dir.strip_prefix(repo_root)?;
    canonicalize_repo_relative_path(&path_to_string(relative))
}

fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn load_cache_file(config_dir: &Path) -> Result<SkillScanCacheFile> {
    let path = cache_path(config_dir);
    if !path.exists() {
        return Ok(SkillScanCacheFile {
            schema_version: CACHE_SCHEMA_VERSION,
            repos: BTreeMap::new(),
        });
    }

    let raw = fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
    let cache_file = serde_json::from_str::<SkillScanCacheFile>(&raw)
        .with_context(|| format!("Failed to parse {:?}", path))?;
    if cache_file.schema_version != CACHE_SCHEMA_VERSION {
        return Ok(SkillScanCacheFile {
            schema_version: CACHE_SCHEMA_VERSION,
            repos: BTreeMap::new(),
        });
    }

    Ok(cache_file)
}

fn save_cache_file(config_dir: &Path, cache_file: &SkillScanCacheFile) -> Result<()> {
    fs::create_dir_all(config_dir).with_context(|| format!("Failed to create {:?}", config_dir))?;
    let json = serde_json::to_string_pretty(cache_file)?;
    write_text_atomic(&cache_path(config_dir), &json)
        .context("Failed to write skill-scan-cache.json")
}

pub(super) fn cache_path(config_dir: &Path) -> PathBuf {
    config_dir.join(CACHE_FILE_NAME)
}
