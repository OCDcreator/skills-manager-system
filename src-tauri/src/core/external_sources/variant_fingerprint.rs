use std::fs;
use std::path::Path;

use anyhow::Result;
use walkdir::WalkDir;

use super::git_command::{git_cmd, run_git};
use super::hash::sha256_hex;

pub(super) fn load_variant_fingerprint(
    repo_dir: &Path,
    git_ref: Option<&str>,
    variant_path: &str,
) -> Option<String> {
    if let Some(git_ref) = git_ref {
        if let Ok(fingerprint) = fingerprint_variant_tree_at_ref(repo_dir, git_ref, variant_path) {
            return fingerprint;
        }
    }

    fingerprint_variant_from_worktree(repo_dir, variant_path)
        .ok()
        .flatten()
}

fn fingerprint_variant_tree_at_ref(
    repo_dir: &Path,
    git_ref: &str,
    variant_path: &str,
) -> Result<Option<String>> {
    let is_root_variant = variant_path == ".";
    let output = run_git(git_cmd(repo_dir).args([
        "ls-tree",
        "-r",
        "--full-tree",
        "--format=%(objectname) %(path)",
        git_ref,
        "--",
        variant_path,
    ]))?;

    let mut entries = Vec::new();
    for line in output.lines().filter(|line| !line.trim().is_empty()) {
        let Some((object_id, full_path)) = line.split_once(' ') else {
            continue;
        };
        let relative_path = if is_root_variant {
            full_path.to_string()
        } else if full_path == variant_path {
            Path::new(full_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(full_path)
                .to_string()
        } else {
            full_path
                .strip_prefix(&format!("{variant_path}/"))
                .unwrap_or(full_path)
                .to_string()
        };
        if relative_path == ".skills-manager-source.json" {
            continue;
        }
        entries.push((relative_path, object_id.to_string()));
    }

    if entries.is_empty() {
        return Ok(None);
    }
    entries.sort_by(|left, right| left.0.cmp(&right.0));

    let mut serialized = Vec::new();
    for (relative_path, object_id) in entries {
        serialized.extend_from_slice(relative_path.as_bytes());
        serialized.push(b'\n');
        serialized.extend_from_slice(object_id.as_bytes());
        serialized.push(b'\n');
    }

    Ok(Some(format!("git-tree:{}", sha256_hex(&serialized))))
}

fn fingerprint_variant_from_worktree(
    repo_dir: &Path,
    variant_path: &str,
) -> Result<Option<String>> {
    let variant_dir = if variant_path == "." {
        repo_dir.to_path_buf()
    } else {
        repo_dir.join(variant_path)
    };
    if !variant_dir.is_dir() {
        return Ok(None);
    }

    let mut entries = Vec::new();
    for entry in WalkDir::new(&variant_dir)
        .into_iter()
        .filter_map(|item| item.ok())
        .filter(|item| item.file_type().is_file())
    {
        let relative_path = entry.path().strip_prefix(&variant_dir)?.to_string_lossy();
        let relative_path = relative_path.replace('\\', "/");
        if relative_path == ".skills-manager-source.json" || relative_path.starts_with(".git/") {
            continue;
        }
        let bytes = fs::read(entry.path())?;
        entries.push((relative_path, sha256_hex(&bytes)));
    }

    if entries.is_empty() {
        return Ok(None);
    }
    entries.sort_by(|left, right| left.0.cmp(&right.0));

    let mut serialized = Vec::new();
    for (relative_path, file_hash) in entries {
        serialized.extend_from_slice(relative_path.as_bytes());
        serialized.push(b'\n');
        serialized.extend_from_slice(file_hash.as_bytes());
        serialized.push(b'\n');
    }

    Ok(Some(format!("sha256:{}", sha256_hex(&serialized))))
}
