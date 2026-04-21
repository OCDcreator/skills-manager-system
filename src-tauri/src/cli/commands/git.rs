use serde_json::json;
use std::path::{Path, PathBuf};

use crate::app_runtime::{AppRuntimeContext, CliCommandError, CliExitStatus, CliRunResult};
use crate::cli::args::GitCommand;
use crate::core::git::operations::{
    git_commit, git_diff, git_fetch, git_log, git_pull, git_push, git_status, run_sync_script,
};
use crate::core::git::types::GitOperationResult;

pub fn run(context: &AppRuntimeContext, command: &GitCommand) -> CliRunResult {
    match command {
        GitCommand::Status => status(context),
        GitCommand::Diff { staged } => diff(context, *staged),
        GitCommand::Log { max_count } => log(context, *max_count),
        GitCommand::Fetch => operation(context, "git fetch", "fetch", |repo| git_fetch(repo)),
        GitCommand::Pull => operation(context, "git pull", "pull", |repo| git_pull(repo)),
        GitCommand::Push => operation(context, "git push", "push", |repo| git_push(repo)),
        GitCommand::Commit { message } => commit(context, message),
        GitCommand::SyncExternal => {
            operation(context, "git sync-external", "sync-external", |repo| {
                run_sync_script(repo)
            })
        }
    }
}

fn status(context: &AppRuntimeContext) -> CliRunResult {
    let repo_path = match require_repo_path("git status", context) {
        Ok(repo_path) => repo_path,
        Err(result) => return result,
    };

    match git_status(&repo_path) {
        Ok(status) => CliRunResult::success(
            "git status",
            json!({ "status": status }),
            context,
            Some(repo_path.as_path()),
        ),
        Err(error) => git_read_error("git status", "status", error, context, &repo_path),
    }
}

fn diff(context: &AppRuntimeContext, staged: bool) -> CliRunResult {
    let repo_path = match require_repo_path("git diff", context) {
        Ok(repo_path) => repo_path,
        Err(result) => return result,
    };

    match git_diff(&repo_path, staged) {
        Ok(diff) => CliRunResult::success(
            "git diff",
            json!({ "diff": diff }),
            context,
            Some(repo_path.as_path()),
        ),
        Err(error) => git_read_error("git diff", "diff", error, context, &repo_path),
    }
}

fn log(context: &AppRuntimeContext, max_count: usize) -> CliRunResult {
    let repo_path = match require_repo_path("git log", context) {
        Ok(repo_path) => repo_path,
        Err(result) => return result,
    };

    match git_log(&repo_path, max_count) {
        Ok(log) => CliRunResult::success(
            "git log",
            json!({ "log": log }),
            context,
            Some(repo_path.as_path()),
        ),
        Err(error) => git_read_error("git log", "log", error, context, &repo_path),
    }
}

fn commit(context: &AppRuntimeContext, message: &str) -> CliRunResult {
    if message.trim().is_empty() {
        return CliRunResult::error(
            "git commit",
            CliCommandError::new(
                CliExitStatus::ArgumentError,
                "commit_message_required",
                "Commit message is required.",
                None,
            ),
            context,
            context.current_repo_path().ok().flatten().as_deref(),
        );
    }

    operation(context, "git commit", "commit", |repo| {
        git_commit(repo, message)
    })
}

fn operation(
    context: &AppRuntimeContext,
    command: &str,
    operation_name: &str,
    run_operation: impl FnOnce(&Path) -> GitOperationResult,
) -> CliRunResult {
    let repo_path = match require_repo_path(command, context) {
        Ok(repo_path) => repo_path,
        Err(result) => return result,
    };
    let result = run_operation(&repo_path);

    if result.success {
        CliRunResult::success(
            command,
            json!({ "operation": result }),
            context,
            Some(repo_path.as_path()),
        )
    } else {
        CliRunResult::error(
            command,
            git_operation_error(operation_name, &result),
            context,
            Some(repo_path.as_path()),
        )
    }
}

fn require_repo_path(command: &str, context: &AppRuntimeContext) -> Result<PathBuf, CliRunResult> {
    context.require_repo_path().map_err(|_| {
        CliRunResult::error(
            command,
            CliCommandError::missing_configuration(
                "repo_path_not_configured",
                "Repository path is not configured.",
            ),
            context,
            None,
        )
    })
}

fn git_read_error(
    command: &str,
    operation_name: &str,
    error: anyhow::Error,
    context: &AppRuntimeContext,
    repo_path: &Path,
) -> CliRunResult {
    CliRunResult::error(
        command,
        CliCommandError::new(
            CliExitStatus::ExternalCommandFailure,
            "git_command_failed",
            format!("git {operation_name} failed"),
            Some(json!({
                "operation": operation_name,
                "stderr": error.to_string(),
            })),
        ),
        context,
        Some(repo_path),
    )
}

fn git_operation_error(operation_name: &str, result: &GitOperationResult) -> CliCommandError {
    CliCommandError::new(
        CliExitStatus::ExternalCommandFailure,
        "git_command_failed",
        format!("git {operation_name} failed"),
        Some(json!({
            "operation": operation_name,
            "stdout": result.stdout,
            "stderr": result.stderr,
            "exitCode": result.exit_code,
        })),
    )
}
