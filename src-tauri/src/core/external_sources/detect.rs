use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
    let mut variants = Vec::new();
    let mut warnings = Vec::new();
    let source_skill_dirs = collect_source_skill_dirs(repo_dir)?;

    for rule in GENERATED_RULES {
        let target_root = repo_dir.join(rule.built_root);
        if !target_root.exists() {
            continue;
        }

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

            let skill_name = entry.file_name().to_string_lossy().to_string();
            let variant_path = canonicalize_repo_relative_path(
                path_relative_to(repo_dir, &skill_dir)?.to_string_lossy().as_ref(),
            )?;
            let metadata_path = format!("{variant_path}/SKILL.md");
            let source_of_truth_path = source_skill_dirs
                .iter()
                .find(|path| path.file_name().and_then(|name| name.to_str()) == Some(skill_name.as_str()))
                .map(|path| {
                    canonicalize_repo_relative_path(
                        path_relative_to(repo_dir, path).unwrap().to_string_lossy().as_ref(),
                    )
                    .unwrap()
                });

            variants.push(DetectedExternalVariant {
                agent_key: rule.agent_key.to_string(),
                variant_path,
                source_of_truth_path,
                metadata_path: Some(metadata_path),
            });
        }
    }

    warnings.extend(detect_unknown_agent_variants(repo_dir, source_id)?);

    let kind = if variants.is_empty() {
        UNSUPPORTED_KIND.to_string()
    } else {
        GENERATED_AGENT_BUNDLE_KIND.to_string()
    };

    Ok(DetectionResult {
        kind,
        variants,
        warnings,
    })
}

fn collect_source_skill_dirs(repo_dir: &Path) -> Result<Vec<PathBuf>> {
    let source_root = repo_dir.join("source/skills");
    if !source_root.is_dir() {
        return Ok(Vec::new());
    }

    let mut skill_dirs = Vec::new();
    for entry in fs::read_dir(&source_root)
        .with_context(|| format!("Failed to read {}", source_root.display()))?
    {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            skill_dirs.push(entry.path());
        }
    }
    Ok(skill_dirs)
}

fn detect_unknown_agent_variants(repo_dir: &Path, source_id: &str) -> Result<Vec<ExternalSourceWarning>> {
    let dist_agents_root = repo_dir.join("dist/agents");
    if !dist_agents_root.is_dir() {
        return Ok(Vec::new());
    }

    let mut warnings = Vec::new();
    let mut warned_paths = BTreeSet::new();
    for skill_file in WalkDir::new(&dist_agents_root)
        .min_depth(2)
        .into_iter()
        .filter_map(|item| item.ok())
        .filter(|item| item.file_type().is_file() && item.file_name() == "SKILL.md")
    {
        let skill_dir = skill_file
            .path()
            .parent()
            .with_context(|| format!("Missing parent for {}", skill_file.path().display()))?;
        let relative_skill_dir = canonicalize_repo_relative_path(
            path_relative_to(repo_dir, skill_dir)?.to_string_lossy().as_ref(),
        )?;
        if GENERATED_RULES
            .iter()
            .any(|rule| is_supported_built_variant_dir(rule, &relative_skill_dir))
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

    Ok(warnings)
}

fn is_supported_built_variant_dir(rule: &GeneratedRule, relative_skill_dir: &str) -> bool {
    let Some(remainder) = relative_skill_dir
        .strip_prefix(&format!("{}/", rule.built_root))
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
    built_root: &'static str,
}

const GENERATED_RULES: &[GeneratedRule] = &[
    GeneratedRule {
        agent_key: "codex",
        built_root: "dist/agents/.agents/skills",
    },
    GeneratedRule {
        agent_key: "claude_code",
        built_root: "dist/agents/.claude/skills",
    },
    GeneratedRule {
        agent_key: "opencode",
        built_root: "dist/agents/.opencode/skills",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn detect_generated_agent_bundle_prefers_built_agent_target() {
        let temp = tempdir().unwrap();
        let repo_dir = temp.path();

        create_skill_dir(repo_dir, "source/skills/impeccable");
        create_skill_dir(repo_dir, "dist/agents/.agents/skills/impeccable");

        let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

        assert_eq!(result.kind, "generated_agent_bundle");
        assert_eq!(result.variants.len(), 1);
        assert_eq!(result.variants[0].agent_key, "codex");
        assert_eq!(
            result.variants[0].variant_path,
            "dist/agents/.agents/skills/impeccable"
        );
        assert_eq!(
            result.variants[0].source_of_truth_path.as_deref(),
            Some("source/skills/impeccable")
        );
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn detect_unknown_agent_variant_emits_warning() {
        let temp = tempdir().unwrap();
        let repo_dir = temp.path();

        create_skill_dir(repo_dir, "source/skills/impeccable");
        create_skill_dir(repo_dir, "dist/agents/.mystery/skills/impeccable");

        let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

        assert_eq!(result.kind, "unsupported");
        assert!(result.variants.is_empty());
        assert!(result
            .warnings
            .iter()
            .any(|warning| warning.code == "unsupported_agent_variant"));
    }

    #[test]
    fn detect_unknown_agent_variant_emits_warning_under_known_agent_root() {
        let temp = tempdir().unwrap();
        let repo_dir = temp.path();

        create_skill_dir(repo_dir, "source/skills/impeccable");
        create_skill_dir(repo_dir, "dist/agents/.agents/skills/impeccable");
        create_skill_dir(repo_dir, "dist/agents/.agents/skills/group/impeccable");

        let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

        assert_eq!(result.kind, "generated_agent_bundle");
        assert_eq!(result.variants.len(), 1);
        assert!(result.warnings.iter().any(|warning| {
            warning.code == "unsupported_agent_variant"
                && warning
                    .message
                    .contains("dist/agents/.agents/skills/group/impeccable")
        }));
    }

    fn create_skill_dir(repo_dir: &Path, relative_path: &str) {
        let skill_dir = repo_dir.join(relative_path);
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(skill_dir.join("SKILL.md"), "# Test skill\n").unwrap();
    }
}
