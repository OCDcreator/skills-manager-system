use super::*;
use crate::core::external_sources::git_repo::{
    ensure_cached_repo, read_default_branch, read_head_commit,
};
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn detect_generated_agent_bundle_prefers_built_agent_target() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, "source/skills/impeccable");
    create_skill_dir(repo_dir, "dist/agents/.agents/skills/impeccable");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "generated_agent_bundle");
    assert_eq!(result.variants.len(), 1);
    assert_eq!(result.variants[0].agent_key, "codex");
    assert_eq!(
        result.variants[0].variant_path,
        "dist/agents/.agents/skills/impeccable"
    );
    assert_eq!(
        result.variants[0].source_of_truth_path.as_deref(),
        Some("source/skills/impeccable")
    );
    assert!(result.warnings.is_empty());
}

#[test]
fn detect_unknown_agent_variant_emits_warning() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, "source/skills/impeccable");
    create_skill_dir(repo_dir, "dist/agents/.mystery/skills/impeccable");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "unsupported");
    assert!(result.variants.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|warning| warning.code == "unsupported_agent_variant"));
}

#[test]
fn detect_unknown_agent_variant_emits_warning_under_known_agent_root() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, "source/skills/impeccable");
    create_skill_dir(repo_dir, "dist/agents/.agents/skills/impeccable");
    create_skill_dir(repo_dir, "dist/agents/.agents/skills/group/impeccable");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "generated_agent_bundle");
    assert_eq!(result.variants.len(), 1);
    assert!(result.warnings.iter().any(|warning| {
        warning.code == "unsupported_agent_variant"
            && warning
                .message
                .contains("dist/agents/.agents/skills/group/impeccable")
    }));
}

#[test]
fn detect_generated_agent_bundle_supports_root_hidden_agent_roots() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, "source/skills/impeccable");
    create_skill_dir(repo_dir, ".agents/skills/impeccable");
    create_skill_dir(repo_dir, ".claude/skills/impeccable");
    create_skill_dir(repo_dir, ".opencode/skills/impeccable");
    create_skill_dir(repo_dir, ".cursor/skills/impeccable");
    create_skill_dir(repo_dir, ".gemini/skills/impeccable");
    create_skill_dir(repo_dir, ".github/skills/impeccable");
    create_skill_dir(repo_dir, ".kiro/skills/impeccable");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "generated_agent_bundle");
    assert_eq!(result.variants.len(), 7);
    assert!(result
        .variants
        .iter()
        .any(|variant| variant.agent_key == "codex"
            && variant.variant_path == ".agents/skills/impeccable"));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "claude_code" && variant.variant_path == ".claude/skills/impeccable"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "opencode" && variant.variant_path == ".opencode/skills/impeccable"
    }));
    assert!(result
        .variants
        .iter()
        .any(|variant| variant.agent_key == "cursor"
            && variant.variant_path == ".cursor/skills/impeccable"));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "gemini_cli" && variant.variant_path == ".gemini/skills/impeccable"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "github_copilot" && variant.variant_path == ".github/skills/impeccable"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "kilo_code" && variant.variant_path == ".kiro/skills/impeccable"
    }));
}

#[test]
fn detect_generated_agent_bundle_supports_dist_variant_roots_for_aligned_agents() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, "source/skills/impeccable");
    create_skill_dir(repo_dir, "dist/cursor/.cursor/skills/impeccable");
    create_skill_dir(repo_dir, "dist/gemini/.gemini/skills/impeccable");
    create_skill_dir(repo_dir, "dist/github/.github/skills/impeccable");
    create_skill_dir(repo_dir, "dist/kiro/.kiro/skills/impeccable");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "generated_agent_bundle");
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "cursor"
            && variant.variant_path == "dist/cursor/.cursor/skills/impeccable"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "gemini_cli"
            && variant.variant_path == "dist/gemini/.gemini/skills/impeccable"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "github_copilot"
            && variant.variant_path == "dist/github/.github/skills/impeccable"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "kilo_code"
            && variant.variant_path == "dist/kiro/.kiro/skills/impeccable"
    }));
}

#[test]
fn detect_generated_agent_bundle_reads_fetched_git_head_without_worktree_checkout() {
    let temp = tempdir().unwrap();
    let remote_dir = temp.path().join("remote.git");
    let work_dir = temp.path().join("work");
    let cache_root = temp.path().join("cache");

    run_git(
        Command::new("git")
            .arg("init")
            .arg("--bare")
            .arg(&remote_dir),
    )
    .unwrap();
    run_git(
        Command::new("git")
            .arg("clone")
            .arg(&remote_dir)
            .arg(&work_dir),
    )
    .unwrap();
    run_git(git_cmd(&work_dir).args(["checkout", "-b", "main"])).unwrap();
    run_git(git_cmd(&work_dir).args(["config", "user.email", "test@example.com"])).unwrap();
    run_git(git_cmd(&work_dir).args(["config", "user.name", "Test User"])).unwrap();
    fs::create_dir_all(work_dir.join("source/skills/impeccable")).unwrap();
    fs::write(
        work_dir.join("source/skills/impeccable/SKILL.md"),
        "# Source skill\n",
    )
    .unwrap();
    fs::create_dir_all(work_dir.join(".agents/skills/impeccable")).unwrap();
    fs::write(
        work_dir.join(".agents/skills/impeccable/SKILL.md"),
        "# Generated skill\n",
    )
    .unwrap();
    run_git(git_cmd(&work_dir).args(["add", "."])).unwrap();
    run_git(git_cmd(&work_dir).args(["commit", "-m", "initial"])).unwrap();
    run_git(git_cmd(&work_dir).args(["push", "-u", "origin", "main"])).unwrap();
    run_git(Command::new("git").arg("--git-dir").arg(&remote_dir).args([
        "symbolic-ref",
        "HEAD",
        "refs/heads/main",
    ]))
    .unwrap();

    let repo_dir = ensure_cached_repo(
        &cache_root,
        "src_test",
        remote_dir.to_string_lossy().as_ref(),
    )
    .unwrap();
    let branch = read_default_branch(&repo_dir).unwrap();
    let head = read_head_commit(&repo_dir, &branch).unwrap();

    assert_eq!(branch, "main");
    assert_eq!(head.len(), 40);
    assert!(!repo_dir.join(".agents/skills/impeccable/SKILL.md").exists());

    let result = detect_external_source_variants(&repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "generated_agent_bundle");
    assert_eq!(result.variants.len(), 1);
    assert_eq!(result.variants[0].agent_key, "codex");
    assert_eq!(result.variants[0].variant_path, ".agents/skills/impeccable");
    assert_eq!(
        result.variants[0].source_of_truth_path.as_deref(),
        Some("source/skills/impeccable")
    );
}

#[test]
fn detect_generic_skill_repo_root_exports_root_variant() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    fs::write(repo_dir.join("SKILL.md"), "# Root skill\n").unwrap();

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "skill_repository");
    assert_eq!(result.variants.len(), 1);
    assert_eq!(result.variants[0].agent_key, "skill_repository");
    assert_eq!(result.variants[0].variant_path, ".");
    assert_eq!(
        result.variants[0].metadata_path.as_deref(),
        Some("SKILL.md")
    );
    assert!(result.warnings.is_empty());
}

#[test]
fn detect_generic_skill_repo_subpath_lists_direct_child_skills() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, "packages/skills/alpha");
    create_skill_dir(repo_dir, "packages/skills/beta");
    create_skill_dir(repo_dir, "packages/skills/group/nested");

    let result = detect_external_source_variants_at_ref_in_root(
        repo_dir,
        "HEAD",
        "src_test",
        Some("packages/skills"),
    )
    .or_else(|_| {
        Ok::<_, anyhow::Error>(detect_external_source_variants_from_worktree_in_root(
            repo_dir,
            "src_test",
            Some("packages/skills"),
        )?)
    })
    .unwrap();

    assert_eq!(result.kind, "skill_repository");
    assert_eq!(
        result
            .variants
            .iter()
            .map(|variant| variant.variant_path.as_str())
            .collect::<Vec<_>>(),
        vec!["packages/skills/alpha", "packages/skills/beta"]
    );
}

#[test]
fn detect_generated_agent_bundle_remains_preferred_over_generic_skills() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    fs::write(repo_dir.join("SKILL.md"), "# Root skill\n").unwrap();
    create_skill_dir(repo_dir, ".agents/skills/generated");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "generated_agent_bundle");
    assert_eq!(result.variants.len(), 1);
    assert_eq!(result.variants[0].variant_path, ".agents/skills/generated");
}

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
