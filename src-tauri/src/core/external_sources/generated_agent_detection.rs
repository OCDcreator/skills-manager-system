use anyhow::Result;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::detect::DetectedExternalVariant;
use super::models::ExternalSourceWarning;
use crate::core::skills::identity::canonicalize_repo_relative_path;

pub(super) const GENERATED_AGENT_BUNDLE_KIND: &str = "generated_agent_bundle";

pub(super) fn collect_supported_generated_variants<Direct>(
    subpath: Option<&str>,
    list_direct_skill_dirs: &Direct,
) -> Result<Vec<DetectedExternalVariant>>
where
    Direct: Fn(&str) -> Result<Vec<String>>,
{
    let source_skill_dirs = list_direct_skill_dirs(&scoped_path(subpath, "source/skills")?)?;
    let source_skill_index = build_source_skill_index(&source_skill_dirs);
    let mut variants = Vec::new();
    for rule in GENERATED_RULES {
        let variant_root = scoped_path(subpath, rule.variant_root)?;
        for variant_path in list_direct_skill_dirs(&variant_root)? {
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

pub(super) fn detect_unknown_generated_agent_variants<Recursive>(
    source_id: &str,
    subpath: Option<&str>,
    list_recursive_skill_dirs: &Recursive,
) -> Result<Vec<ExternalSourceWarning>>
where
    Recursive: Fn(&str) -> Result<Vec<String>>,
{
    let mut warnings = Vec::new();
    let mut warned_paths = BTreeSet::new();
    for scan_root in generated_scan_roots() {
        let scoped_scan_root = scoped_path(subpath, scan_root)?;
        for relative_skill_dir in list_recursive_skill_dirs(&scoped_scan_root)? {
            if GENERATED_RULES.iter().any(|rule| {
                scoped_path(subpath, rule.variant_root)
                    .map(|variant_root| {
                        is_supported_variant_dir(&variant_root, &relative_skill_dir)
                    })
                    .unwrap_or(false)
            }) {
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

pub(super) fn scoped_path(subpath: Option<&str>, relative: &str) -> Result<String> {
    let relative = relative.trim_matches('/');
    match (subpath, relative.is_empty()) {
        (Some(root), true) => canonicalize_repo_relative_path(root),
        (Some(root), false) => canonicalize_repo_relative_path(&format!("{root}/{relative}")),
        (None, true) => Ok(".".to_string()),
        (None, false) => canonicalize_repo_relative_path(relative),
    }
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

fn generated_scan_roots() -> Vec<&'static str> {
    let mut roots = BTreeSet::new();
    for rule in GENERATED_RULES {
        roots.insert(rule.scan_root);
    }
    roots.into_iter().collect()
}

fn is_supported_variant_dir(variant_root: &str, relative_skill_dir: &str) -> bool {
    let Some(remainder) = relative_skill_dir.strip_prefix(&format!("{variant_root}/")) else {
        return false;
    };

    !remainder.is_empty() && !remainder.contains('/')
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
