use serde_json::json;
use std::path::Path;

use crate::app_runtime::AppRuntimeContext;
use crate::app_runtime::{normalize_output_path, CliCommandError, CliRunResult};
use crate::cli::args::SettingsCommand;
use crate::core::settings::{AgentSyncMode, SettingsStore};

pub fn run(context: &AppRuntimeContext, command: &SettingsCommand) -> CliRunResult {
    match command {
        SettingsCommand::GetRepoPath => get_repo_path(context),
        SettingsCommand::SetRepoPath { path } => set_repo_path(context, path),
        SettingsCommand::GetSyncMode => get_sync_mode(context),
        SettingsCommand::SetSyncMode { sync_mode } => set_sync_mode(context, sync_mode),
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

fn set_repo_path(context: &AppRuntimeContext, path: &str) -> CliRunResult {
    let trimmed = path.trim();
    let repo_path = (!trimmed.is_empty()).then(|| Path::new(trimmed));
    let _guard = match context.acquire_config_lock() {
        Ok(guard) => guard,
        Err(error) => {
            return CliRunResult::error(
                "settings set-repo-path",
                CliCommandError::from_config_lock(error),
                context,
                repo_path,
            );
        }
    };

    match SettingsStore::new(context.config_dir.clone()).save_repo_path(repo_path) {
        Ok(settings) => CliRunResult::success(
            "settings set-repo-path",
            json!({ "settings": settings }),
            context,
            settings.repo_path.as_deref().map(Path::new),
        ),
        Err(error) => CliRunResult::error(
            "settings set-repo-path",
            CliCommandError::config_write_failed(
                format!("Failed to save repository path: {error}"),
                None,
            ),
            context,
            repo_path,
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

fn set_sync_mode(context: &AppRuntimeContext, raw_sync_mode: &str) -> CliRunResult {
    let sync_mode = match parse_agent_sync_mode(raw_sync_mode) {
        Ok(sync_mode) => sync_mode,
        Err(error) => {
            return CliRunResult::error("settings set-sync-mode", error, context, None);
        }
    };
    let repo_path = context.current_repo_path().ok().flatten();
    let _guard = match context.acquire_config_lock() {
        Ok(guard) => guard,
        Err(error) => {
            return CliRunResult::error(
                "settings set-sync-mode",
                CliCommandError::from_config_lock(error),
                context,
                repo_path.as_deref(),
            );
        }
    };

    match SettingsStore::new(context.config_dir.clone()).save_agent_sync_mode(sync_mode) {
        Ok(settings) => CliRunResult::success(
            "settings set-sync-mode",
            json!({ "settings": settings }),
            context,
            repo_path.as_deref(),
        ),
        Err(error) => CliRunResult::error(
            "settings set-sync-mode",
            CliCommandError::config_write_failed(
                format!("Failed to save agent sync mode: {error}"),
                None,
            ),
            context,
            repo_path.as_deref(),
        ),
    }
}

fn parse_agent_sync_mode(raw_sync_mode: &str) -> Result<AgentSyncMode, CliCommandError> {
    match raw_sync_mode {
        "copy" => Ok(AgentSyncMode::Copy),
        "symlink" => Ok(AgentSyncMode::Symlink),
        other => Err(CliCommandError::invalid_sync_mode(other)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_runtime::{AppRuntimeOptions, CliStatus};
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn test_context(config_dir: PathBuf) -> AppRuntimeContext {
        AppRuntimeContext::from_options_with_parts(
            AppRuntimeOptions {
                config_dir_override: Some(config_dir),
                ..AppRuntimeOptions::default()
            },
            PathBuf::from("/unused"),
            crate::app_runtime::DEVELOPMENT_APP_IDENTIFIER,
        )
    }

    #[test]
    fn settings_set_repo_path_writes_snapshot() {
        let dir = tempdir().unwrap();
        let repo = dir.path().join("repo");
        let context = test_context(dir.path().join("config"));

        let result = run(
            &context,
            &SettingsCommand::SetRepoPath {
                path: repo.to_string_lossy().to_string(),
            },
        );

        assert_eq!(result.response.status, CliStatus::Success);
        assert_eq!(
            SettingsStore::new(context.config_dir.clone())
                .load()
                .unwrap()
                .repo_path
                .as_deref(),
            Some(repo.to_string_lossy().as_ref())
        );
    }

    #[test]
    fn settings_set_sync_mode_rejects_invalid_value() {
        let dir = tempdir().unwrap();
        let context = test_context(dir.path().join("config"));

        let result = run(
            &context,
            &SettingsCommand::SetSyncMode {
                sync_mode: "hardlink".to_string(),
            },
        );

        assert_eq!(
            result.response.error.unwrap().code,
            "invalid_sync_mode".to_string()
        );
    }
}
