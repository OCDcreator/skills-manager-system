use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::Command;

pub(crate) fn git_cmd(repo_path: &Path) -> Command {
    let (mut cmd, _) = crate::core::command_resolution::git_command();
    cmd.arg("-C")
        .arg(repo_path)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("LC_ALL", "C");
    cmd
}

pub(crate) fn run_git(cmd: &mut Command) -> Result<String> {
    let output = cmd.output().map_err(git_spawn_error)?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout)
            .trim_end()
            .to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr)
            .trim_end()
            .to_string();
        Err(anyhow!("{}", readable_git_error(&stderr)))
    }
}

pub(crate) fn run_git_bytes(cmd: &mut Command) -> Result<Vec<u8>> {
    let output = cmd.output().map_err(git_spawn_error)?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr)
            .trim_end()
            .to_string();
        Err(anyhow!("{}", readable_git_error(&stderr)))
    }
}

fn git_spawn_error(error: std::io::Error) -> anyhow::Error {
    anyhow!(
        "Git executable was not found or could not be launched. {}",
        crate::core::command_resolution::git_command()
            .1
            .spawn_error_message(&error)
    )
}

pub(crate) fn readable_git_error(stderr: &str) -> String {
    let detail = sanitize_git_stderr(stderr);
    let lower = detail.to_ascii_lowercase();
    let summary = if lower.contains("permission denied (publickey)")
        || lower.contains("could not read from remote repository")
        || lower.contains("authentication failed")
    {
        "Git authentication failed. Check repository access, SSH keys, or private repo permissions."
    } else if lower.contains("repository not found") {
        "Git repository was not found. Check the owner, repository name, and remote URL."
    } else if lower.contains("couldn't find remote ref")
        || lower.contains("could not find remote ref")
        || lower.contains("unknown revision")
        || lower.contains("ambiguous argument")
        || lower.contains("needed a single revision")
    {
        "Git branch, tag, or commit was not found. Check the configured branch/ref."
    } else if lower.contains("failed to connect")
        || lower.contains("could not resolve host")
        || lower.contains("unable to access")
        || lower.contains("operation timed out")
        || lower.contains("network is unreachable")
        || lower.contains("ssl")
    {
        "Git network request failed. Check network connectivity, proxy, and GitHub availability."
    } else {
        "Git command failed."
    };

    if detail.is_empty() {
        summary.to_string()
    } else {
        format!("{summary} Detail: {detail}")
    }
}

fn sanitize_git_stderr(stderr: &str) -> String {
    let mut detail = stderr
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .take(4)
        .collect::<Vec<_>>()
        .join(" ");
    detail = mask_credentials_in_urls(&detail);
    for prefix in ["ghp_", "gho_", "ghu_", "ghs_", "ghr_", "github_pat_"] {
        detail = mask_token_prefixes(&detail, prefix);
    }
    if detail.len() > 600 {
        detail.truncate(600);
        detail.push_str("...");
    }
    detail
}

fn mask_credentials_in_urls(input: &str) -> String {
    let mut output = String::new();
    let mut rest = input;
    while let Some(index) = rest.find("://") {
        let (before, after) = rest.split_at(index + 3);
        output.push_str(before);
        let at = after.find('@');
        let slash = after.find('/').unwrap_or(after.len());
        if let Some(at_index) = at {
            if at_index < slash {
                output.push_str("***@");
                rest = &after[at_index + 1..];
                continue;
            }
        }
        rest = after;
    }
    output.push_str(rest);
    output
}

fn mask_token_prefixes(input: &str, prefix: &str) -> String {
    let mut output = String::new();
    let mut rest = input;
    while let Some(index) = rest.find(prefix) {
        output.push_str(&rest[..index]);
        output.push_str(prefix);
        output.push_str("***");
        let token_tail = &rest[index + prefix.len()..];
        let end = token_tail
            .find(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
            .unwrap_or(token_tail.len());
        rest = &token_tail[end..];
    }
    output.push_str(rest);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readable_git_error_explains_missing_repository() {
        let message =
            readable_git_error("remote: Repository not found.\nfatal: repository not found");

        assert!(message.contains("Git repository was not found"));
        assert!(message.contains("Repository not found"));
    }

    #[test]
    fn readable_git_error_explains_ssh_auth_failure() {
        let message = readable_git_error(
            "git@github.com: Permission denied (publickey).\nfatal: Could not read from remote repository.",
        );

        assert!(message.contains("Git authentication failed"));
        assert!(message.contains("SSH keys"));
    }

    #[test]
    fn readable_git_error_explains_missing_branch() {
        let message = readable_git_error("fatal: couldn't find remote ref release");

        assert!(message.contains("branch, tag, or commit was not found"));
    }

    #[test]
    fn readable_git_error_masks_credentials_and_tokens() {
        let message = readable_git_error(
            "fatal: unable to access 'https://user:secret@github.com/OCDcreator/repo.git/': The requested URL returned error: 403 ghp_abcdefghijklmnopqrstuvwxyz",
        );

        assert!(message.contains("https://***@github.com"));
        assert!(message.contains("ghp_***"));
        assert!(!message.contains("secret"));
        assert!(!message.contains("abcdefghijklmnopqrstuvwxyz"));
    }
}
