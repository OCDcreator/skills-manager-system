use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::Serialize;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use walkdir::WalkDir;

const CONTEXT_SCOPE_LABEL: &str = "AGENTS.md + docs/**/*.md + package.json + src-tauri/Cargo.toml";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AssistantContextStatus {
    pub project_root: String,
    pub scope_label: String,
    pub indexed_document_count: usize,
    pub indexed_chunk_count: usize,
    pub last_indexed_at: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssistantContextDocument {
    pub path: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssistantContextBundle {
    pub status: AssistantContextStatus,
    pub documents: Vec<AssistantContextDocument>,
}

fn discover_project_root(start: &Path) -> Result<PathBuf> {
    for candidate in start.ancestors() {
        let has_agents = candidate.join("AGENTS.md").is_file();
        let has_cargo = candidate.join("src-tauri").join("Cargo.toml").is_file();
        if has_agents && has_cargo {
            return Ok(candidate.to_path_buf());
        }
    }

    bail!("Could not discover project root from {}", start.display())
}

fn collect_context_paths(root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let direct_files = [
        root.join("AGENTS.md"),
        root.join("package.json"),
        root.join("src-tauri").join("Cargo.toml"),
    ];

    for path in direct_files {
        if path.is_file() {
            paths.push(path);
        }
    }

    let docs_root = root.join("docs");
    if docs_root.is_dir() {
        for entry in WalkDir::new(&docs_root)
            .into_iter()
            .filter_map(|entry| entry.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|value| value.to_str()) == Some("md") {
                paths.push(path.to_path_buf());
            }
        }
    }

    paths.sort();
    paths.dedup();
    paths
}

fn relative_display_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn title_from_path(path: &str, content: &str) -> String {
    content
        .lines()
        .find_map(|line| line.strip_prefix("# ").map(str::trim))
        .filter(|title| !title.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| path.rsplit('/').next().unwrap_or(path).to_string())
}

pub fn load_context_from_root(root: &Path) -> Result<AssistantContextBundle> {
    let project_root = discover_project_root(root)?;
    let mut documents = Vec::new();
    let mut warnings = Vec::new();

    for path in collect_context_paths(&project_root) {
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(error) => {
                warnings.push(format!("Failed to read {}: {}", path.display(), error));
                continue;
            }
        };

        let relative_path = relative_display_path(&project_root, &path);
        documents.push(AssistantContextDocument {
            title: title_from_path(&relative_path, &content),
            path: relative_path,
            content,
        });
    }

    let indexed_chunk_count: usize = documents
        .iter()
        .map(|document| {
            document
                .content
                .split("\n\n")
                .filter(|chunk| !chunk.trim().is_empty())
                .count()
        })
        .sum();

    let last_indexed_at = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .context("format assistant context timestamp")?;

    Ok(AssistantContextBundle {
        status: AssistantContextStatus {
            project_root: project_root.to_string_lossy().into_owned(),
            scope_label: CONTEXT_SCOPE_LABEL.to_string(),
            indexed_document_count: documents.len(),
            indexed_chunk_count,
            last_indexed_at,
            warnings,
        },
        documents,
    })
}

pub fn load_workspace_context() -> Result<AssistantContextBundle> {
    let cwd = std::env::current_dir()?;
    load_context_from_root(&cwd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn assistant_context_status_counts_scannable_files() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        fs::create_dir_all(root.join("docs/modules")).unwrap();
        fs::create_dir_all(root.join("src-tauri")).unwrap();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("AGENTS.md"), "# Agent Rules").unwrap();
        fs::write(root.join("docs/README.md"), "# Docs").unwrap();
        fs::write(
            root.join("docs/modules/intro.md"),
            "Verification lives here",
        )
        .unwrap();
        fs::write(root.join("package.json"), r#"{ "name": "assistant-demo" }"#).unwrap();
        fs::write(
            root.join("src-tauri/Cargo.toml"),
            "[package]\nname = \"assistant-demo\"",
        )
        .unwrap();
        fs::write(root.join("src/main.tsx"), "ignored").unwrap();

        let bundle = load_context_from_root(root).unwrap();

        assert_eq!(bundle.status.indexed_document_count, 5);
        assert_eq!(bundle.documents.len(), 5);
        assert!(bundle.status.scope_label.contains("AGENTS.md"));
        assert!(!bundle.status.project_root.is_empty());
    }

    #[test]
    fn assistant_context_ignores_non_allowed_files() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::create_dir_all(root.join("src-tauri")).unwrap();
        fs::write(root.join("AGENTS.md"), "# Agent Rules").unwrap();
        fs::write(root.join("docs/keep.md"), "# Keep").unwrap();
        fs::write(root.join("docs/skip.txt"), "skip").unwrap();
        fs::write(root.join("package.json"), r#"{ "name": "assistant-demo" }"#).unwrap();
        fs::write(
            root.join("src-tauri/Cargo.toml"),
            "[package]\nname = \"assistant-demo\"",
        )
        .unwrap();

        let bundle = load_context_from_root(root).unwrap();

        assert!(bundle
            .documents
            .iter()
            .all(|document| !document.path.ends_with(".txt")));
        assert!(bundle
            .documents
            .iter()
            .any(|document| document.path == "docs/keep.md"));
    }
}
