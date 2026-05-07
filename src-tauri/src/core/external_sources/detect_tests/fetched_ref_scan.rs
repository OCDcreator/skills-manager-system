use super::*;

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
