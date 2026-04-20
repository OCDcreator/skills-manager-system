use anyhow::{anyhow, Result};
use serde::Serialize;
use std::fs;
use std::path::{Component, Path};

use super::metadata::parse_skill_metadata;
use super::scan::build_skill_id;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkillDocument {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source_type: String,
    pub relative_path: String,
    pub content: String,
}

pub fn read_skill_document(repo_root: &Path, relative_path: &str) -> Result<SkillDocument> {
    validate_relative_path(relative_path)?;

    let skill_dir = repo_root.join(relative_path);
    let skill_md_path = skill_dir.join("SKILL.md");
    let content = fs::read_to_string(&skill_md_path)?;
    let metadata = parse_skill_metadata(&skill_md_path)?;

    let source_type = if relative_path.starts_with("custom/") {
        "custom"
    } else if relative_path.starts_with("external/") {
        "external"
    } else {
        return Err(anyhow!("Invalid skill source path"));
    };

    let name = metadata.name.unwrap_or_else(|| {
        skill_dir
            .file_name()
            .map(|value| value.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown-skill".to_string())
    });

    Ok(SkillDocument {
        id: build_skill_id(source_type, relative_path),
        name,
        description: metadata.description.unwrap_or_default(),
        source_type: source_type.to_string(),
        relative_path: relative_path.to_string(),
        content,
    })
}

fn validate_relative_path(relative_path: &str) -> Result<()> {
    let path = Path::new(relative_path);
    for component in path.components() {
        if matches!(
            component,
            Component::ParentDir | Component::Prefix(_) | Component::RootDir
        ) {
            return Err(anyhow!("Invalid relative path"));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn read_skill_document_returns_content_for_valid_relative_path() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("custom/searxng")).unwrap();
        fs::write(
            repo.path().join("custom/searxng/SKILL.md"),
            "---\nname: searxng\ndescription: search helper\n---\n# Content",
        )
        .unwrap();

        let document = read_skill_document(repo.path(), "custom/searxng").unwrap();

        assert_eq!(document.id, "custom:searxng");
        assert!(document.content.contains("# Content"));
    }

    #[test]
    fn read_skill_document_rejects_parent_directory_traversal() {
        let repo = tempdir().unwrap();
        let error = read_skill_document(repo.path(), "../outside").unwrap_err();
        assert!(error.to_string().contains("Invalid relative path"));
    }
}
