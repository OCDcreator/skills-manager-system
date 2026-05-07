use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

use super::git_repo::normalize_github_repo_url;
use super::hash::sha256_hex;
use super::models::{ExternalSourceRecord, ExternalSourcesSnapshot, ImportedExternalSkillRecord};

pub(super) fn determine_mirror_relative_path(
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
        let candidate = format!("external/managed/github/{repo_slug}/{agent_key}/{variant_key}");
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

pub(super) fn stable_import_id(
    source_id: &str,
    agent_key: &str,
    mirror_relative_path: &str,
) -> String {
    let seed = format!("{source_id}\n{agent_key}\n{mirror_relative_path}");
    format!("imp_{}", &sha256_hex(seed.as_bytes())[..12])
}

pub(super) fn resolve_cached_repo_path(
    config_dir: &Path,
    cached_repo_path: Option<&str>,
) -> Result<PathBuf> {
    let cached_repo_path = cached_repo_path
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow::anyhow!("External source is missing its cached repository path"))?;
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
            && normalize_github_repo_url(&other.repo_url).ok().as_deref()
                != Some(normalized_repo_url)
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
