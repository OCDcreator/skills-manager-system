use anyhow::{anyhow, Result};
use std::path::{Component, Path};

pub fn canonicalize_repo_relative_path(raw: &str) -> Result<String> {
    let normalized = raw.replace('\\', "/");
    if has_drive_qualified_prefix(&normalized) {
        return Err(anyhow!("Invalid relative path"));
    }
    let path = Path::new(&normalized);
    let mut parts = Vec::new();

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(value) => parts.push(value.to_string_lossy().to_string()),
            Component::ParentDir | Component::Prefix(_) | Component::RootDir => {
                return Err(anyhow!("Invalid relative path"));
            }
        }
    }

    if parts.is_empty() {
        return Err(anyhow!("Invalid relative path"));
    }

    Ok(parts.join("/"))
}

fn has_drive_qualified_prefix(path: &str) -> bool {
    let bytes = path.as_bytes();
    bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && bytes[2] == b'/'
}

pub fn resolve_source_type(relative_path: &str) -> Result<&'static str> {
    let canonical_relative_path = canonicalize_repo_relative_path(relative_path)?;

    if canonical_relative_path == "custom" || canonical_relative_path.starts_with("custom/") {
        Ok("custom")
    } else if canonical_relative_path == "external"
        || canonical_relative_path.starts_with("external/")
    {
        Ok("external")
    } else {
        Err(anyhow!("Invalid skill source path"))
    }
}

pub fn build_skill_id(source_type: &str, relative_path: &str) -> String {
    let source_relative = relative_path
        .strip_prefix("custom/")
        .or_else(|| relative_path.strip_prefix("external/"))
        .unwrap_or(relative_path);
    format!("{source_type}:{source_relative}")
}

pub fn build_skill_id_from_relative_path(relative_path: &str) -> Result<String> {
    let canonical_relative_path = canonicalize_repo_relative_path(relative_path)?;
    let source_type = resolve_source_type(&canonical_relative_path)?;
    Ok(build_skill_id(source_type, &canonical_relative_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonicalize_repo_relative_path_normalizes_separators_and_curdir() {
        let path = canonicalize_repo_relative_path(r".\external\managed\github\owner__repo\codex\skill\.")
            .unwrap();

        assert_eq!(path, "external/managed/github/owner__repo/codex/skill");
    }

    #[test]
    fn canonicalize_repo_relative_path_rejects_parent_traversal() {
        let error = canonicalize_repo_relative_path("external/managed/../skill").unwrap_err();

        assert!(error.to_string().contains("Invalid relative path"));
    }

    #[test]
    fn canonicalize_repo_relative_path_rejects_drive_qualified_paths() {
        let forward_slash_error = canonicalize_repo_relative_path("C:/repo/skill").unwrap_err();
        let backslash_error = canonicalize_repo_relative_path(r"C:\repo\skill").unwrap_err();

        assert!(forward_slash_error
            .to_string()
            .contains("Invalid relative path"));
        assert!(backslash_error
            .to_string()
            .contains("Invalid relative path"));
    }

    #[test]
    fn resolve_source_type_accepts_external_managed_paths() {
        let source_type =
            resolve_source_type("external/managed/github/owner__repo/codex/skill").unwrap();

        assert_eq!(source_type, "external");
    }

    #[test]
    fn build_skill_id_from_relative_path_builds_external_managed_ids() {
        let skill_id =
            build_skill_id_from_relative_path("external/managed/github/owner__repo/codex/skill")
                .unwrap();

        assert_eq!(skill_id, "external:managed/github/owner__repo/codex/skill");
    }
}
