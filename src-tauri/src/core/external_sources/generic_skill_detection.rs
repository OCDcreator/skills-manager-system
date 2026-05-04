use anyhow::Result;

use super::detect::DetectedExternalVariant;
use super::models::ExternalSourceWarning;

pub(super) const GENERIC_SKILL_REPOSITORY_KIND: &str = "skill_repository";
pub(super) const GENERIC_SKILL_AGENT_KEY: &str = "skill_repository";

pub(super) fn detect_generic_skill_variants<HasSkillDir, Direct>(
    scan_root: &str,
    has_skill_dir: HasSkillDir,
    list_direct_skill_dirs: Direct,
) -> Result<(Vec<DetectedExternalVariant>, Vec<ExternalSourceWarning>)>
where
    HasSkillDir: Fn(&str) -> Result<bool>,
    Direct: Fn(&str) -> Result<Vec<String>>,
{
    let mut variants = Vec::new();
    if has_skill_dir(scan_root)? {
        variants.push(variant_for_skill_dir(scan_root));
    } else {
        for skill_dir in list_direct_skill_dirs(scan_root)? {
            variants.push(variant_for_skill_dir(&skill_dir));
        }
    }

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
    }
}
