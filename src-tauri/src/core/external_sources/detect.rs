use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use super::generated_agent_detection::{
    collect_supported_generated_variants, detect_unknown_generated_agent_variants, scoped_path,
    GENERATED_AGENT_BUNDLE_KIND,
};
use super::generic_skill_detection::{
    detect_generic_skill_variants, GENERIC_SKILL_REPOSITORY_KIND,
};
use super::git_repo::{read_default_branch, read_head_commit};
use super::git_tree::{
    list_direct_child_skill_dirs_at_ref, list_recursive_skill_dirs_at_ref, skill_dir_exists_at_ref,
};
use super::models::ExternalSourceWarning;
use crate::core::skills::identity::canonicalize_repo_relative_path;

const UNSUPPORTED_KIND: &str = "unsupported";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedExternalVariant {
    pub agent_key: String,
    pub variant_path: String,
    pub source_of_truth_path: Option<String>,
    pub metadata_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectionResult {
    pub kind: String,
    pub variants: Vec<DetectedExternalVariant>,
    pub warnings: Vec<ExternalSourceWarning>,
}

pub fn detect_external_source_variants(
    repo_dir: &Path,
    source_id: &str,
) -> Result<DetectionResult> {
    if let Ok(default_branch) = read_default_branch(repo_dir) {
        if let Ok(head_commit) = read_head_commit(repo_dir, &default_branch) {
            return detect_external_source_variants_at_ref(repo_dir, &head_commit, source_id);
        }
    }

    detect_external_source_variants_from_worktree(repo_dir, source_id)
}

pub(crate) fn detect_external_source_variants_at_ref(
    repo_dir: &Path,
    git_ref: &str,
    source_id: &str,
) -> Result<DetectionResult> {
    detect_external_source_variants_at_ref_in_root(repo_dir, git_ref, source_id, None)
}

pub(crate) fn detect_external_source_variants_at_ref_in_root(
    repo_dir: &Path,
    git_ref: &str,
    source_id: &str,
    subpath: Option<&str>,
) -> Result<DetectionResult> {
    detect_with_listing(
        source_id,
        subpath,
        |root| skill_dir_exists_at_ref(repo_dir, git_ref, root),
        |root| list_direct_child_skill_dirs_at_ref(repo_dir, git_ref, root),
        |root| list_recursive_skill_dirs_at_ref(repo_dir, git_ref, root),
    )
}

pub(crate) fn detect_external_source_variants_from_worktree(
    repo_dir: &Path,
    source_id: &str,
) -> Result<DetectionResult> {
    detect_external_source_variants_from_worktree_in_root(repo_dir, source_id, None)
}

pub(crate) fn detect_external_source_variants_from_worktree_in_root(
    repo_dir: &Path,
    source_id: &str,
    subpath: Option<&str>,
) -> Result<DetectionResult> {
    detect_with_listing(
        source_id,
        subpath,
        |root| skill_dir_exists_from_worktree(repo_dir, root),
        |root| collect_direct_skill_dirs_from_worktree(repo_dir, root),
        |root| collect_recursive_skill_dirs_from_worktree(repo_dir, root),
    )
}

fn detect_with_listing<HasSkillDir, Direct, Recursive>(
    source_id: &str,
    subpath: Option<&str>,
    has_skill_dir: HasSkillDir,
    list_direct_skill_dirs: Direct,
    list_recursive_skill_dirs: Recursive,
) -> Result<DetectionResult>
where
    HasSkillDir: Fn(&str) -> Result<bool>,
    Direct: Fn(&str) -> Result<Vec<String>>,
    Recursive: Fn(&str) -> Result<Vec<String>>,
{
    let scan_root = scoped_path(subpath, "")?;
    let variants = collect_supported_generated_variants(subpath, &list_direct_skill_dirs)?;
    let mut warnings =
        detect_unknown_generated_agent_variants(source_id, subpath, &list_recursive_skill_dirs)?;
    if !variants.is_empty() {
        return Ok(DetectionResult {
            kind: GENERATED_AGENT_BUNDLE_KIND.to_string(),
            variants,
            warnings,
        });
    }

    let (variants, generic_warnings) =
        detect_generic_skill_variants(&scan_root, has_skill_dir, list_direct_skill_dirs)?;
    warnings.extend(generic_warnings);

    Ok(DetectionResult {
        kind: if variants.is_empty() {
            UNSUPPORTED_KIND.to_string()
        } else {
            GENERIC_SKILL_REPOSITORY_KIND.to_string()
        },
        variants,
        warnings,
    })
}

fn collect_direct_skill_dirs_from_worktree(repo_dir: &Path, root: &str) -> Result<Vec<String>> {
    let root = canonicalize_scan_root(root)?;
    let target_root = repo_dir.join(&root);
    if !target_root.is_dir() {
        return Ok(Vec::new());
    }

    let mut skill_dirs = BTreeSet::new();
    for entry in fs::read_dir(&target_root)
        .with_context(|| format!("Failed to read {}", target_root.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let skill_dir = entry.path();
        if !skill_dir.join("SKILL.md").is_file() {
            continue;
        }
        skill_dirs.insert(canonicalize_repo_relative_path(
            path_relative_to(repo_dir, &skill_dir)?
                .to_string_lossy()
                .as_ref(),
        )?);
    }

    Ok(skill_dirs.into_iter().collect())
}

fn collect_recursive_skill_dirs_from_worktree(repo_dir: &Path, root: &str) -> Result<Vec<String>> {
    let root = canonicalize_scan_root(root)?;
    let target_root = repo_dir.join(&root);
    if !target_root.is_dir() {
        return Ok(Vec::new());
    }

    let mut skill_dirs = BTreeSet::new();
    for skill_file in WalkDir::new(&target_root)
        .min_depth(2)
        .into_iter()
        .filter_map(|item| item.ok())
        .filter(|item| item.file_type().is_file() && item.file_name() == "SKILL.md")
    {
        let skill_dir = skill_file
            .path()
            .parent()
            .with_context(|| format!("Missing parent for {}", skill_file.path().display()))?;
        skill_dirs.insert(canonicalize_repo_relative_path(
            path_relative_to(repo_dir, skill_dir)?
                .to_string_lossy()
                .as_ref(),
        )?);
    }

    Ok(skill_dirs.into_iter().collect())
}

fn skill_dir_exists_from_worktree(repo_dir: &Path, root: &str) -> Result<bool> {
    let root = canonicalize_scan_root(root)?;
    Ok(repo_dir.join(root).join("SKILL.md").is_file())
}

fn canonicalize_scan_root(root: &str) -> Result<String> {
    if root == "." {
        Ok(".".to_string())
    } else {
        canonicalize_repo_relative_path(root)
    }
}

fn path_relative_to<'a>(repo_dir: &Path, child: &'a Path) -> Result<&'a Path> {
    child
        .strip_prefix(repo_dir)
        .with_context(|| format!("{} is not under {}", child.display(), repo_dir.display()))
}

#[cfg(test)]
#[path = "detect_tests.rs"]
mod tests;
