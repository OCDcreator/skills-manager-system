use std::collections::BTreeSet;

use anyhow::Result;

use super::detect::{is_noise_skill_dir, DetectedExternalVariant};
use super::models::ExternalSourceWarning;

pub(super) const GENERIC_SKILL_REPOSITORY_KIND: &str = "skill_repository";
pub(super) const GENERIC_SKILL_AGENT_KEY: &str = "skill_repository";

pub(super) fn detect_generic_skill_variants<HasSkillDir, Recursive>(
    scan_root: &str,
    has_skill_dir: HasSkillDir,
    list_recursive_skill_dirs: Recursive,
    excluded_paths: &BTreeSet<String>,
) -> Result<(Vec<DetectedExternalVariant>, Vec<ExternalSourceWarning>)>
where
    HasSkillDir: Fn(&str) -> Result<bool>,
    Recursive: Fn(&str) -> Result<Vec<String>>,
{
    let mut candidate_paths = BTreeSet::new();
    if has_skill_dir(scan_root)? {
        candidate_paths.insert(scan_root.to_string());
    }
    for skill_dir in list_recursive_skill_dirs(scan_root)? {
        candidate_paths.insert(skill_dir);
    }

    let variants = candidate_paths
        .into_iter()
        .filter(|skill_dir| !excluded_paths.contains(skill_dir))
        .filter(|skill_dir| !is_noise_skill_dir(skill_dir))
        .filter(|skill_dir| {
            skill_dir != "source/skills" && !skill_dir.starts_with("source/skills/")
        })
        .map(|skill_dir| variant_for_skill_dir(&skill_dir))
        .collect();

    Ok((variants, Vec::new()))
}

fn variant_for_skill_dir(skill_dir: &str) -> DetectedExternalVariant {
    let variant_path = skill_dir.to_string();
    let metadata_path = if skill_dir == "." {
        "SKILL.md".to_string()
    } else {
        format!("{skill_dir}/SKILL.md")
    };

    DetectedExternalVariant {
        agent_key: GENERIC_SKILL_AGENT_KEY.to_string(),
        variant_path,
        source_of_truth_path: None,
        metadata_path: Some(metadata_path),
        detection_class: "generic_discovered".to_string(),
        suggested_target_agents: Vec::new(),
        detected_agent_hint: None,
    }
}
