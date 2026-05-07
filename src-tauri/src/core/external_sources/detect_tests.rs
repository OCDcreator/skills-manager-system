use super::*;
use crate::core::external_sources::git_repo::{
    ensure_cached_repo, read_default_branch, read_head_commit,
};
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

#[path = "detect_tests/fetched_ref_scan.rs"]
mod fetched_ref_scan;
#[path = "detect_tests/generated_layouts.rs"]
mod generated_layouts;
#[path = "detect_tests/generic_fallback.rs"]
mod generic_fallback;

fn create_skill_dir(repo_dir: &Path, relative_path: &str) {
    let skill_dir = repo_dir.join(relative_path);
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), "# Test skill\n").unwrap();
}

fn git_cmd(repo_dir: &Path) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(repo_dir);
    cmd
}

fn run_git(cmd: &mut Command) -> Result<String> {
    let output = cmd.output().context("Failed to execute git")?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout)
            .trim_end()
            .to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr)
            .trim_end()
            .to_string();
        Err(anyhow!(stderr))
    }
}
