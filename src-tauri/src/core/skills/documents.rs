use anyhow::Result;
use serde::Serialize;
use std::fs;
use std::path::Path;

use super::identity::{
    build_skill_id_from_relative_path, canonicalize_repo_relative_path, resolve_source_type,
};
use super::metadata::parse_skill_metadata;
use super::scan::ManagedSourceInfo;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkillDocument {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source_type: String,
    pub relative_path: String,
    pub content: String,
    pub managed_source: Option<ManagedSourceInfo>,
}

pub fn read_skill_document(repo_root: &Path, relative_path: &str) -> Result<SkillDocument> {
    let canonical_relative_path = canonicalize_repo_relative_path(relative_path)?;
    let source_type = resolve_source_type(&canonical_relative_path)?;

    let skill_dir = repo_root.join(&canonical_relative_path);
    let skill_md_path = skill_dir.join("SKILL.md");
    let content = fs::read_to_string(&skill_md_path)?;
    let metadata = parse_skill_metadata(&skill_md_path)?;

    let name = metadata.name.unwrap_or_else(|| {
        skill_dir
            .file_name()
            .map(|value| value.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown-skill".to_string())
    });

    Ok(SkillDocument {
        id: build_skill_id_from_relative_path(&canonical_relative_path)?,
        name,
        description: metadata.description.unwrap_or_default(),
        source_type: source_type.to_string(),
        relative_path: canonical_relative_path,
        content,
        managed_source: None,
    })
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
        assert_eq!(
            document.content,
            "---\nname: searxng\ndescription: search helper\n---\n# Content"
        );
    }

    #[test]
    fn read_skill_document_preserves_markdown_without_frontmatter() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("external/demo-skill")).unwrap();
        fs::write(
            repo.path().join("external/demo-skill/SKILL.md"),
            "# Demo Skill\n\n- item one\n- item two",
        )
        .unwrap();

        let document = read_skill_document(repo.path(), "external/demo-skill").unwrap();

        assert_eq!(document.content, "# Demo Skill\n\n- item one\n- item two");
    }

    #[test]
    fn read_skill_document_rejects_parent_directory_traversal() {
        let repo = tempdir().unwrap();
        let error = read_skill_document(repo.path(), "../outside").unwrap_err();
        assert!(error.to_string().contains("Invalid relative path"));
    }

    #[test]
    fn read_skill_document_accepts_external_managed_paths() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("external/managed/github/owner__repo/codex/skill"))
            .unwrap();
        fs::write(
            repo.path()
                .join("external/managed/github/owner__repo/codex/skill/SKILL.md"),
            "---\nname: managed-skill\ndescription: imported\n---\n# Managed",
        )
        .unwrap();

        let document = read_skill_document(
            repo.path(),
            r".\external\managed\github\owner__repo\codex\skill\.",
        )
        .unwrap();

        assert_eq!(document.source_type, "external");
        assert_eq!(
            document.relative_path,
            "external/managed/github/owner__repo/codex/skill"
        );
        assert_eq!(document.id, "external:managed/github/owner__repo/codex/skill");
    }
}
