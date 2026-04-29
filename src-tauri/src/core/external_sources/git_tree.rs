use anyhow::{Context, Result, anyhow};
use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use crate::core::skills::identity::canonicalize_repo_relative_path;

pub(crate) fn list_direct_child_skill_dirs_at_ref(
    repo_dir: &Path,
    git_ref: &str,
    root: &str,
) -> Result<Vec<String>> {
    list_skill_dirs_at_ref(repo_dir, git_ref, root, true)
}

pub(crate) fn list_recursive_skill_dirs_at_ref(
    repo_dir: &Path,
    git_ref: &str,
    root: &str,
) -> Result<Vec<String>> {
    list_skill_dirs_at_ref(repo_dir, git_ref, root, false)
}

pub(crate) fn read_text_file_at_ref(repo_dir: &Path, git_ref: &str, relative_path: &str) -> Result<String> {
    let relative_path = canonicalize_repo_relative_path(relative_path)?;
    let output = run_git_bytes(git_cmd(repo_dir).args([
        "show",
        &format!("{git_ref}:{relative_path}"),
    ]))
    .with_context(|| format!("Failed to read {git_ref}:{relative_path}"))?;

    String::from_utf8(output).context("Git file content was not valid UTF-8")
}

fn list_skill_dirs_at_ref(
    repo_dir: &Path,
    git_ref: &str,
    root: &str,
    direct_only: bool,
) -> Result<Vec<String>> {
    let root = canonicalize_repo_relative_path(root)?;
    let output = run_git_bytes(git_cmd(repo_dir).args([
        "ls-tree",
        "-r",
        "--full-tree",
        "-z",
        git_ref,
        "--",
        &root,
    ]))
    .with_context(|| format!("Failed to list tree for {git_ref}:{root}"))?;

    let mut dirs = BTreeSet::new();
    let root_prefix = format!("{root}/");
    for chunk in output.split(|byte| *byte == 0) {
        if chunk.is_empty() {
            continue;
        }

        let tab_index = chunk
            .iter()
            .position(|byte| *byte == b'\t')
            .ok_or_else(|| anyhow!("Malformed ls-tree output"))?;
        let path = std::str::from_utf8(&chunk[tab_index + 1..]).context("Malformed tree path")?;
        let Some(parent) = path.strip_suffix("/SKILL.md") else {
            continue;
        };
        let Some(remainder) = parent.strip_prefix(&root_prefix) else {
            continue;
        };
        if remainder.is_empty() {
            continue;
        }
        if direct_only && remainder.contains('/') {
            continue;
        }

        dirs.insert(canonicalize_repo_relative_path(parent)?);
    }

    Ok(dirs.into_iter().collect())
}

fn git_cmd(repo_dir: &Path) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(repo_dir)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("LC_ALL", "C");
    cmd
}

fn run_git_bytes(cmd: &mut Command) -> Result<Vec<u8>> {
    let output = cmd.output().context("Failed to execute git")?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(anyhow!(
            "{}",
            String::from_utf8_lossy(&output.stderr).trim_end()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn list_direct_child_skill_dirs_at_ref_filters_nested_matches() {
        let temp = tempdir().unwrap();
        let repo_dir = temp.path().join("repo");

        run_git(Command::new("git").arg("init").arg(&repo_dir)).unwrap();
        run_git(git_cmd(&repo_dir).args(["config", "user.email", "test@example.com"])).unwrap();
        run_git(git_cmd(&repo_dir).args(["config", "user.name", "Test User"])).unwrap();

        fs::create_dir_all(repo_dir.join(".agents/skills/impeccable")).unwrap();
        fs::write(
            repo_dir.join(".agents/skills/impeccable/SKILL.md"),
            "# Impeccable\n",
        )
        .unwrap();
        fs::create_dir_all(repo_dir.join(".agents/skills/group/impeccable")).unwrap();
        fs::write(
            repo_dir.join(".agents/skills/group/impeccable/SKILL.md"),
            "# Nested\n",
        )
        .unwrap();
        run_git(git_cmd(&repo_dir).args(["add", "."])).unwrap();
        run_git(git_cmd(&repo_dir).args(["commit", "-m", "initial"])).unwrap();

        let head = run_git(git_cmd(&repo_dir).args(["rev-parse", "HEAD"])).unwrap();
        let direct = list_direct_child_skill_dirs_at_ref(&repo_dir, &head, ".agents/skills").unwrap();
        let recursive = list_recursive_skill_dirs_at_ref(&repo_dir, &head, ".agents").unwrap();

        assert_eq!(direct, vec![".agents/skills/impeccable".to_string()]);
        assert!(recursive.contains(&".agents/skills/impeccable".to_string()));
        assert!(recursive.contains(&".agents/skills/group/impeccable".to_string()));
    }

    fn run_git(cmd: &mut Command) -> Result<String> {
        let output = cmd.output().context("Failed to execute git")?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout)
                .trim_end()
                .to_string())
        } else {
            Err(anyhow!(
                "{}",
                String::from_utf8_lossy(&output.stderr).trim_end()
            ))
        }
    }
}
