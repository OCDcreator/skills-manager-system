use anyhow::{anyhow, Result};
use serde::Serialize;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use super::metadata::parse_skill_metadata;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkillSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source_type: String,
    pub relative_path: String,
    pub directory_path: String,
    pub skill_document_path: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ScanSkillsResponse {
    pub skills: Vec<SkillSummary>,
    pub warnings: Vec<String>,
}

const IGNORED_DIRS: &[&str] = &[".git", "node_modules", "dist", "target", ".tmp-skills"];

pub fn scan_repo_skills(repo_root: &Path) -> Result<ScanSkillsResponse> {
    if !repo_root.exists() {
        return Err(anyhow!("Configured repository path does not exist"));
    }

    let mut warnings = Vec::new();
    let mut skills = Vec::new();

    let custom_root = repo_root.join("custom");
    if custom_root.exists() {
        for entry in std::fs::read_dir(&custom_root)? {
            let entry = entry?;
            let skill_dir = entry.path();
            if !skill_dir.is_dir() {
                continue;
            }

            let skill_md = skill_dir.join("SKILL.md");
            if !skill_md.exists() {
                continue;
            }

            skills.push(build_skill_summary(repo_root, &skill_dir, "custom")?);
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
                skills.push(build_skill_summary(repo_root, skill_dir, "external")?);
            }
        }
    } else {
        warnings
            .push("Missing external/ directory; continuing with remaining sources.".to_string());
    }

    skills.sort_by(|left, right| left.id.cmp(&right.id));

    Ok(ScanSkillsResponse { skills, warnings })
}

pub fn build_skill_id(source_type: &str, relative_path: &str) -> String {
    let source_relative = relative_path
        .strip_prefix("custom/")
        .or_else(|| relative_path.strip_prefix("external/"))
        .unwrap_or(relative_path);
    format!("{source_type}:{source_relative}")
}

fn should_walk(entry: &DirEntry) -> bool {
    !IGNORED_DIRS
        .iter()
        .any(|ignored| entry.file_name() == *ignored)
}

fn build_skill_summary(
    repo_root: &Path,
    skill_dir: &Path,
    source_type: &str,
) -> Result<SkillSummary> {
    let relative_path = normalize_relative_path(repo_root, skill_dir)?;
    let skill_md_path = skill_dir.join("SKILL.md");
    let metadata = parse_skill_metadata(&skill_md_path)?;
    let default_name = skill_dir
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown-skill".to_string());

    Ok(SkillSummary {
        id: build_skill_id(source_type, &relative_path),
        name: metadata.name.unwrap_or(default_name),
        description: metadata.description.unwrap_or_default(),
        source_type: source_type.to_string(),
        relative_path,
        directory_path: skill_dir.to_string_lossy().to_string(),
        skill_document_path: skill_md_path.to_string_lossy().to_string(),
    })
}

fn normalize_relative_path(repo_root: &Path, skill_dir: &Path) -> Result<String> {
    let relative = skill_dir.strip_prefix(repo_root)?;
    Ok(relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn scan_custom_only_reads_first_level_directories() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("custom/searxng")).unwrap();
        fs::create_dir_all(repo.path().join("custom/group/sub-skill")).unwrap();
        fs::write(
            repo.path().join("custom/searxng/SKILL.md"),
            "---\nname: searxng\n---",
        )
        .unwrap();
        fs::write(
            repo.path().join("custom/group/sub-skill/SKILL.md"),
            "---\nname: nested\n---",
        )
        .unwrap();

        let response = scan_repo_skills(repo.path()).unwrap();
        let ids: Vec<_> = response
            .skills
            .iter()
            .map(|skill| skill.id.as_str())
            .collect();

        assert!(ids.contains(&"custom:searxng"));
        assert!(!ids.contains(&"custom:group/sub-skill"));
    }

    #[test]
    fn scan_external_recurses_and_ignores_reference_only_sources() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("external/html-ppt-skill")).unwrap();
        fs::create_dir_all(repo.path().join("external/awesome-design-md/apple")).unwrap();
        fs::write(
            repo.path().join("external/html-ppt-skill/SKILL.md"),
            "---\nname: html-ppt\n---",
        )
        .unwrap();
        fs::write(
            repo.path()
                .join("external/awesome-design-md/apple/README.md"),
            "# reference",
        )
        .unwrap();

        let response = scan_repo_skills(repo.path()).unwrap();
        let ids: Vec<_> = response
            .skills
            .iter()
            .map(|skill| skill.id.as_str())
            .collect();

        assert!(ids.contains(&"external:html-ppt-skill"));
        assert!(!ids.iter().any(|id| id.contains("awesome-design-md")));
    }

    #[test]
    fn scan_ignores_noise_directories() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("external/node_modules/pkg")).unwrap();
        fs::create_dir_all(
            repo.path()
                .join("external/anthropics-skills/frontend-design"),
        )
        .unwrap();
        fs::write(
            repo.path().join("external/node_modules/pkg/SKILL.md"),
            "---\nname: nope\n---",
        )
        .unwrap();
        fs::write(
            repo.path()
                .join("external/anthropics-skills/frontend-design/SKILL.md"),
            "---\nname: frontend-design\n---",
        )
        .unwrap();

        let response = scan_repo_skills(repo.path()).unwrap();
        let ids: Vec<_> = response
            .skills
            .iter()
            .map(|skill| skill.id.as_str())
            .collect();

        assert!(!ids.iter().any(|id| id.contains("node_modules")));
        assert!(ids.contains(&"external:anthropics-skills/frontend-design"));
    }
}
