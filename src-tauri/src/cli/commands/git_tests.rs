use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::tempdir;

use super::git::run;
use crate::app_runtime::{AppRuntimeContext, AppRuntimeOptions, CliStatus};
use crate::cli::args::GitCommand;

fn test_context(config_dir: PathBuf, repo_dir: PathBuf) -> AppRuntimeContext {
    AppRuntimeContext::from_options_with_parts(
        AppRuntimeOptions {
            config_dir_override: Some(config_dir),
            repo_override: Some(repo_dir),
            ..AppRuntimeOptions::default()
        },
        PathBuf::from("/unused"),
        crate::app_runtime::DEVELOPMENT_APP_IDENTIFIER,
    )
}

fn git(dir: &Path, args: &[&str]) {
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
    git(dir, &["init"]);
    git(dir, &["config", "user.email", "test@example.com"]);
    git(dir, &["config", "user.name", "Test User"]);
}

#[test]
fn git_status_commit_diff_and_log_commands_work() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path().join("repo");
    std::fs::create_dir_all(&repo_dir).unwrap();
    init_repo(&repo_dir);
    std::fs::write(repo_dir.join("note.txt"), "hello").unwrap();
    let context = test_context(temp.path().join("config"), repo_dir.clone());

    let status = run(&context, &GitCommand::Status);
    assert_eq!(status.response.status, CliStatus::Success);
    assert!(!status.response.data.unwrap()["status"]["isClean"]
        .as_bool()
        .unwrap());

    let diff = run(&context, &GitCommand::Diff { staged: false });
    assert_eq!(diff.response.status, CliStatus::Success);

    let commit = run(
        &context,
        &GitCommand::Commit {
            message: "initial".to_string(),
        },
    );
    assert_eq!(commit.response.status, CliStatus::Success);

    let log = run(&context, &GitCommand::Log { max_count: 5 });
    assert_eq!(
        log.response.data.unwrap()["log"]["entries"][0]["message"].as_str(),
        Some("initial")
    );
}

#[test]
fn git_fetch_pull_and_push_work_with_local_remote() {
    let temp = tempdir().unwrap();
    let remote_dir = temp.path().join("remote.git");
    let repo_dir = temp.path().join("repo");
    let other_dir = temp.path().join("other");
    git(
        temp.path(),
        &["init", "--bare", remote_dir.to_string_lossy().as_ref()],
    );

    std::fs::create_dir_all(&repo_dir).unwrap();
    init_repo(&repo_dir);
    git(
        &repo_dir,
        &[
            "remote",
            "add",
            "origin",
            remote_dir.to_string_lossy().as_ref(),
        ],
    );
    std::fs::write(repo_dir.join("note.txt"), "base").unwrap();
    git(&repo_dir, &["add", "-A"]);
    git(&repo_dir, &["commit", "-m", "base"]);
    git(&repo_dir, &["push", "-u", "origin", "HEAD"]);
    git(
        temp.path(),
        &[
            "clone",
            remote_dir.to_string_lossy().as_ref(),
            other_dir.to_string_lossy().as_ref(),
        ],
    );
    git(&other_dir, &["config", "user.email", "test@example.com"]);
    git(&other_dir, &["config", "user.name", "Test User"]);
    std::fs::write(other_dir.join("remote.txt"), "remote").unwrap();
    git(&other_dir, &["add", "-A"]);
    git(&other_dir, &["commit", "-m", "remote change"]);
    git(&other_dir, &["push"]);

    let context = test_context(temp.path().join("config"), repo_dir.clone());
    let fetch = run(&context, &GitCommand::Fetch);
    assert_eq!(fetch.response.status, CliStatus::Success);

    let pulled = run(&context, &GitCommand::Pull);
    assert_eq!(pulled.response.status, CliStatus::Success);

    std::fs::write(repo_dir.join("local.txt"), "local").unwrap();
    let commit = run(
        &context,
        &GitCommand::Commit {
            message: "local change".to_string(),
        },
    );
    assert_eq!(commit.response.status, CliStatus::Success);
    let pushed = run(&context, &GitCommand::Push);
    assert_eq!(pushed.response.status, CliStatus::Success);
}

#[cfg(unix)]
#[test]
fn git_sync_external_executes_update_script() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path().join("repo");
    std::fs::create_dir_all(&repo_dir).unwrap();
    init_repo(&repo_dir);
    std::fs::write(
        repo_dir.join("update.sh"),
        "#!/bin/sh\nprintf 'synced' > sync-output.txt\n",
    )
    .unwrap();
    git(&repo_dir, &["add", "-A"]);
    git(&repo_dir, &["commit", "-m", "add script"]);
    let context = test_context(temp.path().join("config"), repo_dir.clone());

    let result = run(&context, &GitCommand::SyncExternal);

    assert_eq!(result.response.status, CliStatus::Success);
    assert_eq!(
        std::fs::read_to_string(repo_dir.join("sync-output.txt")).unwrap(),
        "synced"
    );
}
