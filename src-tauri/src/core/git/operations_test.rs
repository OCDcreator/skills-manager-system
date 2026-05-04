use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

use super::operations::{git_commit, git_diff, git_log, git_remote_url, git_status};

fn run_git(dir: &Path, args: &[&str]) {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("LC_ALL", "C")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_repo(dir: &Path) {
    run_git(dir, &["init"]);
    run_git(dir, &["config", "user.email", "test@test.com"]);
    run_git(dir, &["config", "user.name", "Test"]);
}

#[test]
fn git_status_clean_on_fresh_repo() {
    let dir = tempdir().unwrap();
    init_repo(dir.path());

    let status = git_status(dir.path()).unwrap();
    assert!(status.is_clean);
    assert!(status.staged.is_empty());
}

#[test]
fn git_status_detects_untracked() {
    let dir = tempdir().unwrap();
    init_repo(dir.path());
    fs::write(dir.path().join("hello.txt"), "world").unwrap();

    let status = git_status(dir.path()).unwrap();
    assert!(!status.is_clean);
    assert_eq!(status.untracked.len(), 1);
    assert_eq!(status.untracked[0].path, "hello.txt");
}

#[test]
fn git_commit_creates_commit() {
    let dir = tempdir().unwrap();
    init_repo(dir.path());
    fs::write(dir.path().join("hello.txt"), "world").unwrap();

    let result = git_commit(dir.path(), "initial commit");
    assert!(result.success);

    let log = git_log(dir.path(), 5).unwrap();
    assert_eq!(log.entries.len(), 1);
    assert_eq!(log.entries[0].message, "initial commit");
}

#[test]
fn git_log_returns_entries_after_commits() {
    let dir = tempdir().unwrap();
    init_repo(dir.path());

    fs::write(dir.path().join("a.txt"), "a").unwrap();
    git_commit(dir.path(), "first");
    fs::write(dir.path().join("b.txt"), "b").unwrap();
    git_commit(dir.path(), "second");

    let log = git_log(dir.path(), 10).unwrap();
    assert_eq!(log.entries.len(), 2);
    assert_eq!(log.entries[0].message, "second");
    assert_eq!(log.entries[1].message, "first");
}

#[test]
fn git_diff_staged_returns_empty_when_nothing_staged() {
    let dir = tempdir().unwrap();
    init_repo(dir.path());
    fs::write(dir.path().join("new.txt"), "content").unwrap();

    let diff = git_diff(dir.path(), true).unwrap();
    assert!(diff.diff.is_empty());
}

#[test]
fn git_remote_url_returns_none_without_remote() {
    let dir = tempdir().unwrap();
    init_repo(dir.path());

    let url = git_remote_url(dir.path()).unwrap();
    assert_eq!(url, None);
}
