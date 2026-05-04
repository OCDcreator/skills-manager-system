use anyhow::{anyhow, bail, Context, Result};
use std::fs;
use std::path::Path;

use super::git_command::{git_cmd, run_git_bytes};
use super::hash::sha256_hex;

pub(super) fn export_variant_from_git(
    repo_dir: &Path,
    git_ref: &str,
    variant_path: &str,
    destination_dir: &Path,
) -> Result<String> {
    let entries = list_variant_blob_entries(repo_dir, git_ref, variant_path)?;
    if entries.is_empty() {
        bail!(
            "Variant '{}' was not found at commit {}",
            variant_path,
            git_ref
        );
    }

    let mut fingerprint_entries = Vec::new();
    for entry in entries {
        let bytes = run_git_bytes(
            git_cmd(repo_dir).args(["show", &format!("{git_ref}:{}", entry.full_path)]),
        )
        .with_context(|| format!("Failed to read {}", entry.full_path))?;
        let target_path = destination_dir.join(&entry.relative_path);
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create {}", parent.display()))?;
        }
        fs::write(&target_path, &bytes)
            .with_context(|| format!("Failed to write {}", target_path.display()))?;
        if entry.relative_path != ".skills-manager-source.json" {
            fingerprint_entries.push((entry.relative_path, sha256_hex(&bytes)));
        }
    }

    fingerprint_entries.sort_by(|left, right| left.0.cmp(&right.0));
    let mut serialized = Vec::new();
    for (relative_path, file_hash) in fingerprint_entries {
        serialized.extend_from_slice(relative_path.as_bytes());
        serialized.push(b'\n');
        serialized.extend_from_slice(file_hash.as_bytes());
        serialized.push(b'\n');
    }

    Ok(format!("sha256:{}", sha256_hex(&serialized)))
}

fn list_variant_blob_entries(
    repo_dir: &Path,
    git_ref: &str,
    variant_path: &str,
) -> Result<Vec<VariantBlobEntry>> {
    let is_root_variant = variant_path == ".";
    let output = run_git_bytes(git_cmd(repo_dir).args([
        "ls-tree",
        "-r",
        "--full-tree",
        "-z",
        git_ref,
        "--",
        variant_path,
    ]))?;

    let mut entries = Vec::new();
    for chunk in output.split(|byte| *byte == 0) {
        if chunk.is_empty() {
            continue;
        }
        let tab_index = chunk
            .iter()
            .position(|byte| *byte == b'\t')
            .ok_or_else(|| anyhow!("Malformed ls-tree output"))?;
        let header = std::str::from_utf8(&chunk[..tab_index]).context("Malformed tree header")?;
        let full_path =
            std::str::from_utf8(&chunk[tab_index + 1..]).context("Malformed tree path")?;
        let mut header_parts = header.split_whitespace();
        let _mode = header_parts.next();
        let object_type = header_parts.next();
        if object_type != Some("blob") {
            continue;
        }

        let relative_path = if is_root_variant {
            full_path.to_string()
        } else if full_path == variant_path {
            Path::new(full_path)
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| anyhow!("Variant path is invalid"))?
                .to_string()
        } else {
            full_path
                .strip_prefix(&format!("{variant_path}/"))
                .ok_or_else(|| anyhow!("Variant entry escaped its source path"))?
                .to_string()
        };

        entries.push(VariantBlobEntry {
            full_path: full_path.to_string(),
            relative_path,
        });
    }

    Ok(entries)
}

struct VariantBlobEntry {
    full_path: String,
    relative_path: String,
}
