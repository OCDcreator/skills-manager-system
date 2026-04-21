use serde_json::json;

use crate::app_runtime::AppRuntimeContext;
use crate::app_runtime::{normalize_output_path, CliCommandError, CliRunResult};
use crate::cli::args::SettingsCommand;

pub fn run(context: &AppRuntimeContext, command: &SettingsCommand) -> CliRunResult {
    match command {
        SettingsCommand::GetRepoPath => get_repo_path(context),
        SettingsCommand::GetSyncMode => get_sync_mode(context),
    }
}

fn get_repo_path(context: &AppRuntimeContext) -> CliRunResult {
    match context.current_repo_path() {
        Ok(repo_path) => CliRunResult::success(
            "settings get-repo-path",
            json!({
                "repoPath": repo_path.as_deref().map(normalize_output_path),
            }),
            context,
            repo_path.as_deref(),
        ),
        Err(error) => CliRunResult::error(
            "settings get-repo-path",
            CliCommandError::filesystem(
                "config_read_failed",
                format!("Failed to resolve the configured repository path: {error}"),
            ),
            context,
            None,
        ),
    }
}

fn get_sync_mode(context: &AppRuntimeContext) -> CliRunResult {
    let repo_path = match context.current_repo_path() {
        Ok(repo_path) => repo_path,
        Err(error) => {
            return CliRunResult::error(
                "settings get-sync-mode",
                CliCommandError::filesystem(
                    "config_read_failed",
                    format!("Failed to resolve the configured repository path: {error}"),
                ),
                context,
                None,
            );
        }
    };

    match context.load_settings() {
        Ok(settings) => CliRunResult::success(
            "settings get-sync-mode",
            json!({
                "agentSyncMode": settings.agent_sync_mode,
            }),
            context,
            repo_path.as_deref(),
        ),
        Err(error) => CliRunResult::error(
            "settings get-sync-mode",
            CliCommandError::filesystem(
                "config_read_failed",
                format!("Failed to load settings: {error}"),
            ),
            context,
            repo_path.as_deref(),
        ),
    }
}
