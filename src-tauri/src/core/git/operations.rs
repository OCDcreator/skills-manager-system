use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::Command;

use super::types::{
    GitDiffResponse, GitLogEntry, GitLogResponse, GitOperationResult, GitStatusEntry,
    GitStatusResponse,
};

// ---------- helpers ----------

fn git_cmd(repo_path: &Path) -> Command {
    let (mut cmd, _) = crate::core::command_resolution::git_command();
    cmd.arg("-C")
        .arg(repo_path)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("LC_ALL", "C");
    cmd
}

fn run_git(cmd: &mut Command) -> Result<String> {
    let output = cmd.output().map_err(|error| {
        anyhow!(
            "{}",
            crate::core::command_resolution::git_command()
                .1
                .spawn_error_message(&error)
        )
    })?;
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

fn run_git_result(cmd: &mut Command) -> GitOperationResult {
    match cmd.output() {
        Ok(output) => build_git_result(
            output.status.success(),
            &output.stdout,
            &output.stderr,
            output.status.code(),
        ),
        Err(error) => {
            let message = crate::core::command_resolution::git_command()
                .1
                .spawn_error_message(&error);
            GitOperationResult {
                success: false,
                message: message.clone(),
                stdout: None,
                stderr: Some(message),
                exit_code: None,
            }
        }
    }
}

// ---------- public operations ----------

pub fn git_status(repo_path: &Path) -> Result<GitStatusResponse> {
    let porcelain = run_git(git_cmd(repo_path).args(["status", "--porcelain=v2", "--branch"]))?;

    let mut branch: Option<String> = None;
    let mut ahead_behind: Option<(usize, usize)> = Option::None;
    let mut staged = Vec::new();
    let mut unstaged = Vec::new();
    let mut untracked = Vec::new();

    for line in porcelain.lines() {
        if let Some(rest) = line.strip_prefix("# branch.head ") {
            branch = Some(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("# branch.ab ") {
            let parts: Vec<&str> = rest.trim().split_whitespace().collect();
            if parts.len() == 2 {
                let ahead = parts[0]
                    .trim_start_matches('+')
                    .parse::<usize>()
                    .unwrap_or(0);
                let behind = parts[1]
                    .trim_start_matches('-')
                    .parse::<usize>()
                    .unwrap_or(0);
                ahead_behind = Some((ahead, behind));
            }
        } else if line.starts_with("1 ") || line.starts_with("2 ") || line.starts_with("u ") {
            let entry = parse_indexed_entry(line);
            if entry.is_untracked {
                untracked.push(entry);
            } else {
                let has_staged = entry.x != "." && entry.x != " ";
                let has_unstaged = entry.y != "." && entry.y != " ";
                if has_staged {
                    staged.push(GitStatusEntry {
                        is_untracked: false,
                        ..entry.clone()
                    });
                }
                if has_unstaged {
                    unstaged.push(GitStatusEntry {
                        is_untracked: false,
                        ..entry.clone()
                    });
                }
                if !has_staged && !has_unstaged {
                    staged.push(entry);
                }
            }
        } else if line.starts_with("? ") {
            untracked.push(GitStatusEntry {
                path: line[2..].to_string(),
                x: "?".to_string(),
                y: "?".to_string(),
                is_untracked: true,
            });
        }
    }

    let remote_url = git_remote_url(repo_path).ok().flatten();

    let is_clean = staged.is_empty() && unstaged.is_empty() && untracked.is_empty();

    Ok(GitStatusResponse {
        branch,
        remote_url,
        ahead_behind,
        staged,
        unstaged,
        untracked,
        is_clean,
    })
}

pub fn git_diff(repo_path: &Path, staged: bool) -> Result<GitDiffResponse> {
    let mut cmd = git_cmd(repo_path);
    cmd.args(["diff", "--stat=80"]);
    if staged {
        cmd.arg("--staged");
    }
    let stat = run_git(&mut cmd)?;

    let mut cmd = git_cmd(repo_path);
    cmd.arg("diff");
    if staged {
        cmd.arg("--staged");
    }
    let diff = run_git(&mut cmd)?;

    Ok(GitDiffResponse { diff, stat })
}

pub fn git_log(repo_path: &Path, max_count: usize) -> Result<GitLogResponse> {
    let delimiter = "---COMMIT-DELIMITER---";
    let pretty = format!("%H%n%h%n%an%n%ai%n%s%n{delimiter}");
    let max_count_arg = format!("--max-count={max_count}");
    let pretty_arg = format!("--pretty=format:{pretty}");

    let output = run_git(git_cmd(repo_path).args(["log", &max_count_arg, &pretty_arg]))?;

    let entries = output
        .split(delimiter)
        .filter_map(|block| {
            let lines: Vec<&str> = block.trim().lines().collect();
            if lines.len() < 5 {
                return None;
            }
            Some(GitLogEntry {
                hash: lines[0].to_string(),
                short_hash: lines[1].to_string(),
                author: lines[2].to_string(),
                date: lines[3].to_string(),
                message: lines[4].to_string(),
            })
        })
        .collect();

    Ok(GitLogResponse { entries })
}

pub fn git_pull(repo_path: &Path) -> GitOperationResult {
    run_git_result(git_cmd(repo_path).arg("pull"))
}

pub fn git_push(repo_path: &Path) -> GitOperationResult {
    run_git_result(git_cmd(repo_path).arg("push"))
}

pub fn git_commit(repo_path: &Path, message: &str) -> GitOperationResult {
    let add_result = run_git_result(git_cmd(repo_path).args(["add", "-A"]));
    if !add_result.success {
        return add_result;
    }
    run_git_result(git_cmd(repo_path).args(["commit", "-m", message]))
}

pub fn git_fetch(repo_path: &Path) -> GitOperationResult {
    run_git_result(git_cmd(repo_path).arg("fetch"))
}

pub fn git_remote_url(repo_path: &Path) -> Result<Option<String>> {
    let output = run_git(git_cmd(repo_path).args(["remote", "get-url", "origin"]));
    match output {
        Ok(url) => Ok(Some(url)),
        Err(_) => Ok(None),
    }
}

pub fn run_sync_script(repo_path: &Path) -> GitOperationResult {
    let script_name = if cfg!(windows) {
        "update.bat"
    } else {
        "update.sh"
    };
    let script_path = repo_path.join(script_name);

    if !script_path.exists() {
        return GitOperationResult {
            success: false,
            message: format!("Sync script not found: {}", script_path.display()),
            stdout: None,
            stderr: Some(format!("Sync script not found: {}", script_path.display())),
            exit_code: None,
        };
    }

    let result = if cfg!(windows) {
        Command::new("cmd")
            .args(["/C", &script_path.to_string_lossy()])
            .current_dir(repo_path)
            .output()
    } else {
        Command::new("sh")
            .arg(&script_path)
            .current_dir(repo_path)
            .output()
    };

    match result {
        Ok(output) => build_git_result(
            output.status.success(),
            &output.stdout,
            &output.stderr,
            output.status.code(),
        ),
        Err(error) => GitOperationResult {
            success: false,
            message: error.to_string(),
            stdout: None,
            stderr: Some(error.to_string()),
            exit_code: None,
        },
    }
}

// ---------- parsing ----------

fn parse_indexed_entry(line: &str) -> GitStatusEntry {
    // porcelain v2: "1 <xy> <sub> <mH> <mI> <mW> <hH> <hI> <path>"
    let fields: Vec<&str> = line.splitn(9, ' ').collect();
    if fields.len() < 9 {
        return GitStatusEntry {
            path: line.to_string(),
            x: String::new(),
            y: String::new(),
            is_untracked: false,
        };
    }
    let xy = fields[1];
    let x = xy.chars().next().unwrap_or(' ').to_string();
    let y = xy.chars().nth(1).unwrap_or(' ').to_string();
    let path = fields[8].to_string();

    GitStatusEntry {
        path,
        x,
        y,
        is_untracked: false,
    }
}

fn build_git_result(
    success: bool,
    stdout: &[u8],
    stderr: &[u8],
    exit_code: Option<i32>,
) -> GitOperationResult {
    let stdout = trimmed_text(stdout);
    let stderr = trimmed_text(stderr);
    let message = if success {
        stdout
            .clone()
            .or_else(|| stderr.clone())
            .unwrap_or_else(|| "Command completed successfully.".to_string())
    } else {
        stderr
            .clone()
            .or_else(|| stdout.clone())
            .unwrap_or_else(|| "Git command failed.".to_string())
    };

    GitOperationResult {
        success,
        message,
        stdout,
        stderr,
        exit_code,
    }
}

fn trimmed_text(bytes: &[u8]) -> Option<String> {
    let text = String::from_utf8_lossy(bytes).trim_end().to_string();
    (!text.is_empty()).then_some(text)
}
