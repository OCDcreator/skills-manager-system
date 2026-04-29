use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use super::git_repo::{read_default_branch, read_head_commit};
use super::git_tree::{
    list_direct_child_skill_dirs_at_ref, list_recursive_skill_dirs_at_ref,
};
use super::models::ExternalSourceWarning;
use crate::core::skills::identity::canonicalize_repo_relative_path;

const GENERATED_AGENT_BUNDLE_KIND: &str = "generated_agent_bundle";
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

pub fn detect_external_source_variants(repo_dir: &Path, source_id: &str) -> Result<DetectionResult> {
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
    detect_with_listing(
        source_id,
        |root| list_direct_child_skill_dirs_at_ref(repo_dir, git_ref, root),
        |root| list_recursive_skill_dirs_at_ref(repo_dir, git_ref, root),
    )
}

fn detect_external_source_variants_from_worktree(
    repo_dir: &Path,
    source_id: &str,
) -> Result<DetectionResult> {
    detect_with_listing(
        source_id,
        |root| collect_direct_skill_dirs_from_worktree(repo_dir, root),
        |root| collect_recursive_skill_dirs_from_worktree(repo_dir, root),
    )
}

fn detect_with_listing<Direct, Recursive>(
    source_id: &str,
    list_direct_skill_dirs: Direct,
    list_recursive_skill_dirs: Recursive,
) -> Result<DetectionResult>
where
    Direct: Fn(&str) -> Result<Vec<String>>,
    Recursive: Fn(&str) -> Result<Vec<String>>,
{
    let source_skill_dirs = list_direct_skill_dirs("source/skills")?;
    let source_skill_index = build_source_skill_index(&source_skill_dirs);
    let variants = collect_supported_variants(&list_direct_skill_dirs, &source_skill_index)?;
    let warnings = detect_unknown_agent_variants(source_id, &list_recursive_skill_dirs)?;

    Ok(DetectionResult {
        kind: if variants.is_empty() {
            UNSUPPORTED_KIND.to_string()
        } else {
            GENERATED_AGENT_BUNDLE_KIND.to_string()
        },
        variants,
        warnings,
    })
}

fn collect_supported_variants<Direct>(
    list_direct_skill_dirs: &Direct,
    source_skill_index: &BTreeMap<String, String>,
) -> Result<Vec<DetectedExternalVariant>>
where
    Direct: Fn(&str) -> Result<Vec<String>>,
{
    let mut variants = Vec::new();
    for rule in GENERATED_RULES {
        for variant_path in list_direct_skill_dirs(rule.variant_root)? {
            let skill_name = Path::new(&variant_path)
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid variant path: {variant_path}"))?
                .to_string();
            variants.push(DetectedExternalVariant {
                agent_key: rule.agent_key.to_string(),
                metadata_path: Some(format!("{variant_path}/SKILL.md")),
                source_of_truth_path: source_skill_index.get(&skill_name).cloned(),
                variant_path,
            });
        }
    }

    Ok(variants)
}

fn build_source_skill_index(source_skill_dirs: &[String]) -> BTreeMap<String, String> {
    let mut index = BTreeMap::new();
    for path in source_skill_dirs {
        if let Some(name) = Path::new(path).file_name().and_then(|name| name.to_str()) {
            index.insert(name.to_string(), path.clone());
        }
    }
    index
}

fn collect_direct_skill_dirs_from_worktree(repo_dir: &Path, root: &str) -> Result<Vec<String>> {
    let root = canonicalize_repo_relative_path(root)?;
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
            path_relative_to(repo_dir, &skill_dir)?.to_string_lossy().as_ref(),
        )?);
    }

    Ok(skill_dirs.into_iter().collect())
}

fn collect_recursive_skill_dirs_from_worktree(repo_dir: &Path, root: &str) -> Result<Vec<String>> {
    let root = canonicalize_repo_relative_path(root)?;
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
            path_relative_to(repo_dir, skill_dir)?.to_string_lossy().as_ref(),
        )?);
    }

    Ok(skill_dirs.into_iter().collect())
}

fn detect_unknown_agent_variants<Recursive>(
    source_id: &str,
    list_recursive_skill_dirs: &Recursive,
) -> Result<Vec<ExternalSourceWarning>>
where
    Recursive: Fn(&str) -> Result<Vec<String>>,
{
    let mut warnings = Vec::new();
    let mut warned_paths = BTreeSet::new();
    for scan_root in generated_scan_roots() {
        for relative_skill_dir in list_recursive_skill_dirs(scan_root)? {
            if GENERATED_RULES
                .iter()
                .any(|rule| is_supported_variant_dir(rule, &relative_skill_dir))
            {
                continue;
            }
            if !warned_paths.insert(relative_skill_dir.clone()) {
                continue;
            }

            warnings.push(ExternalSourceWarning {
                code: "unsupported_agent_variant".to_string(),
                severity: "warning".to_string(),
                message: format!(
                    "External source {source_id} exposes an unsupported generated agent layout at {relative_skill_dir}"
                ),
            });
        }
    }

    Ok(warnings)
}

fn generated_scan_roots() -> Vec<&'static str> {
    let mut roots = BTreeSet::new();
    for rule in GENERATED_RULES {
        roots.insert(rule.scan_root);
    }
    roots.into_iter().collect()
}

fn is_supported_variant_dir(rule: &GeneratedRule, relative_skill_dir: &str) -> bool {
    let Some(remainder) = relative_skill_dir
        .strip_prefix(&format!("{}/", rule.variant_root))
    else {
        return false;
    };

    !remainder.is_empty() && !remainder.contains('/')
}

fn path_relative_to<'a>(repo_dir: &Path, child: &'a Path) -> Result<&'a Path> {
    child
        .strip_prefix(repo_dir)
        .with_context(|| format!("{} is not under {}", child.display(), repo_dir.display()))
}

struct GeneratedRule {
    agent_key: &'static str,
    scan_root: &'static str,
    variant_root: &'static str,
}

const GENERATED_RULES: &[GeneratedRule] = &[
    GeneratedRule {
        agent_key: "codex",
        scan_root: "dist/agents",
        variant_root: "dist/agents/.agents/skills",
    },
    GeneratedRule {
        agent_key: "claude_code",
        scan_root: "dist/agents",
        variant_root: "dist/agents/.claude/skills",
    },
    GeneratedRule {
        agent_key: "opencode",
        scan_root: "dist/agents",
        variant_root: "dist/agents/.opencode/skills",
    },
    GeneratedRule {
        agent_key: "codex",
        scan_root: ".agents",
        variant_root: ".agents/skills",
    },
    GeneratedRule {
        agent_key: "cursor",
        scan_root: "dist/cursor",
        variant_root: "dist/cursor/.cursor/skills",
    },
    GeneratedRule {
        agent_key: "cursor",
        scan_root: ".cursor",
        variant_root: ".cursor/skills",
    },
    GeneratedRule {
        agent_key: "claude_code",
        scan_root: ".claude",
        variant_root: ".claude/skills",
    },
    GeneratedRule {
        agent_key: "gemini_cli",
        scan_root: "dist/gemini",
        variant_root: "dist/gemini/.gemini/skills",
    },
    GeneratedRule {
        agent_key: "gemini_cli",
        scan_root: ".gemini",
        variant_root: ".gemini/skills",
    },
    GeneratedRule {
        agent_key: "github_copilot",
        scan_root: "dist/github",
        variant_root: "dist/github/.github/skills",
    },
    GeneratedRule {
        agent_key: "github_copilot",
        scan_root: ".github",
        variant_root: ".github/skills",
    },
    GeneratedRule {
        agent_key: "kilo_code",
        scan_root: "dist/kiro",
        variant_root: "dist/kiro/.kiro/skills",
    },
    // Upstream repositories like impeccable publish Kiro under `.kiro`, while this app
    // currently exposes the corresponding managed target as `kilo_code`.
    GeneratedRule {
        agent_key: "kilo_code",
        scan_root: ".kiro",
        variant_root: ".kiro/skills",
    },
    GeneratedRule {
        agent_key: "opencode",
        scan_root: ".opencode",
        variant_root: ".opencode/skills",
    },
];

#[cfg(test)]
#[path = "detect_tests.rs"]
mod tests;
