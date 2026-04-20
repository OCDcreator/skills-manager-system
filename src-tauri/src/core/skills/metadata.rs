use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SkillMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Frontmatter {
    name: Option<String>,
    description: Option<String>,
}

pub fn parse_skill_metadata(skill_md_path: &Path) -> Result<SkillMetadata> {
    let content = fs::read_to_string(skill_md_path)?;
    let trimmed = content.trim_start();

    if !trimmed.starts_with("---") {
        return Ok(SkillMetadata::default());
    }

    let remainder = &trimmed[3..];
    let Some(end_index) = remainder.find("---") else {
        return Ok(SkillMetadata::default());
    };

    let yaml = &remainder[..end_index];
    let parsed = serde_yaml::from_str::<Frontmatter>(yaml).unwrap_or(Frontmatter {
        name: None,
        description: None,
    });

    Ok(SkillMetadata {
        name: parsed.name,
        description: parsed.description,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn parse_reads_name_and_description() {
        let dir = tempdir().unwrap();
        fs::write(
            dir.path().join("SKILL.md"),
            "---\nname: frontend-design\ndescription: polished UI systems\n---\n# body",
        )
        .unwrap();

        let metadata = parse_skill_metadata(&dir.path().join("SKILL.md")).unwrap();
        assert_eq!(metadata.name.as_deref(), Some("frontend-design"));
        assert_eq!(metadata.description.as_deref(), Some("polished UI systems"));
    }

    #[test]
    fn parse_falls_back_when_frontmatter_is_missing() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("SKILL.md"), "# plain markdown").unwrap();

        let metadata = parse_skill_metadata(&dir.path().join("SKILL.md")).unwrap();
        assert_eq!(metadata.name, None);
        assert_eq!(metadata.description, None);
    }
}
