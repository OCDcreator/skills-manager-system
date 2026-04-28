use anyhow::{anyhow, Result};
use serde::Serialize;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use super::identity::{
    build_skill_id_from_relative_path, canonicalize_repo_relative_path, resolve_source_type,
};
use super::metadata::parse_skill_metadata;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ManagedSourceInfo {
    pub kind: String,
    pub import_id: String,
    pub repo_url: String,
    pub pinned_commit: String,
    pub agent_key: String,
    pub update_available: bool,
    pub integrity: Option<String>,
}

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
    pub managed_source: Option<ManagedSourceInfo>,
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

            skills.push(build_skill_summary(repo_root, &skill_dir)?);
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
                skills.push(build_skill_summary(repo_root, skill_dir)?);
            }
        }
    } else {
        warnings
            .push("Missing external/ directory; continuing with remaining sources.".to_string());
    }

    skills.sort_by(|left, right| left.id.cmp(&right.id));

    Ok(ScanSkillsResponse { skills, warnings })
}

fn should_walk(entry: &DirEntry) -> bool {
    !IGNORED_DIRS
        .iter()
        .any(|ignored| entry.file_name() == *ignored)
}

fn build_skill_summary(repo_root: &Path, skill_dir: &Path) -> Result<SkillSummary> {
    let relative_path = normalize_relative_path(repo_root, skill_dir)?;
    let skill_md_path = skill_dir.join("SKILL.md");
    let metadata = parse_skill_metadata(&skill_md_path)?;
    let source_type = resolve_source_type(&relative_path)?;
    let default_name = skill_dir
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown-skill".to_string());

    Ok(SkillSummary {
        id: build_skill_id_from_relative_path(&relative_path)?,
        name: metadata.name.unwrap_or(default_name),
        description: metadata.description.unwrap_or_default(),
        source_type: source_type.to_string(),
        relative_path,
        directory_path: path_to_string(skill_dir),
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

    #[test]
    fn scan_external_managed_path_keeps_external_source_type() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("external/managed/github/owner__repo/codex/skill"))
            .unwrap();
        fs::write(
            repo.path()
                .join("external/managed/github/owner__repo/codex/skill/SKILL.md"),
            "---\nname: managed-skill\n---",
        )
        .unwrap();

        let response = scan_repo_skills(repo.path()).unwrap();
        let skill = response
            .skills
            .iter()
            .find(|skill| skill.relative_path == "external/managed/github/owner__repo/codex/skill")
            .unwrap();

        assert_eq!(skill.source_type, "external");
        assert_eq!(skill.id, "external:managed/github/owner__repo/codex/skill");
    }
}
