use serde_json::{json, Value};
use std::path::Path;

use crate::app_runtime::{
    AppRuntimeContext, CliCommandError, CliExitStatus, CliRunResult, CliWarning,
};
use crate::core::agents::discovery::AgentSystemDirs;
use crate::core::agents::sync::{apply_agent_sync, AgentApplyStatus, ApplyAgentSyncResponse};
use crate::core::agents::target_sync::SyncMode;
use crate::core::settings::AgentSyncMode;

pub(super) fn run(context: &AppRuntimeContext, raw_sync_mode: Option<&str>) -> CliRunResult {
    let repo_path = match context.require_repo_path() {
        Ok(repo_path) => repo_path,
        Err(_) => {
            return CliRunResult::error(
                "agents sync",
                CliCommandError::missing_configuration(
                    "repo_path_not_configured",
                    "Repository path is not configured.",
                ),
                context,
                None,
            );
        }
    };
    let mode = match resolve_sync_mode(context, raw_sync_mode) {
        Ok(mode) => mode,
        Err(error) => {
            return CliRunResult::error("agents sync", error, context, Some(repo_path.as_path()));
        }
    };
    let system_dirs = match AgentSystemDirs::current() {
        Ok(system_dirs) => system_dirs,
        Err(error) => {
            return CliRunResult::error(
                "agents sync",
                CliCommandError::filesystem(
                    "agent_system_dirs_unavailable",
                    format!("Failed to determine agent system directories: {error}"),
                ),
                context,
                Some(repo_path.as_path()),
            );
        }
    };

    sync_with_system_dirs(context, &repo_path, &system_dirs, mode)
}

pub(super) fn sync_with_system_dirs(
    context: &AppRuntimeContext,
    repo_path: &Path,
    system_dirs: &AgentSystemDirs,
    mode: SyncMode,
) -> CliRunResult {
    let _guard = match context.acquire_config_lock() {
        Ok(guard) => guard,
        Err(error) => {
            return CliRunResult::error(
                "agents sync",
                CliCommandError::from_config_lock(error),
                context,
                Some(repo_path),
            );
        }
    };

    match apply_agent_sync(&context.config_dir, repo_path, system_dirs, mode) {
        Ok(response) => sync_response_to_result(context, repo_path, response),
        Err(error) => CliRunResult::error(
            "agents sync",
            sync_failure_error(&error.to_string(), None),
            context,
            Some(repo_path),
        ),
    }
}

fn sync_response_to_result(
    context: &AppRuntimeContext,
    repo_path: &Path,
    response: ApplyAgentSyncResponse,
) -> CliRunResult {
    if let Some(failed) = response
        .results
        .iter()
        .find(|result| result.status == AgentApplyStatus::Failed)
    {
        return CliRunResult::error(
            "agents sync",
            sync_failure_error(&failed.message, Some(json!({ "result": failed }))),
            context,
            Some(repo_path),
        );
    }

    let warnings = response
        .results
        .iter()
        .filter(|result| result.status == AgentApplyStatus::Partial)
        .map(|result| {
            let mut warning = CliWarning::new(
                "agent_sync_conflicts",
                result.message.clone(),
                Some(result.key.clone()),
            );
            warning.details = Some(json!({ "conflictCount": result.conflict_count }));
            warning
        })
        .collect::<Vec<_>>();

    if warnings.is_empty() {
        CliRunResult::success(
            "agents sync",
            json!({ "sync": response }),
            context,
            Some(repo_path),
        )
    } else {
        CliRunResult::partial(
            "agents sync",
            json!({ "sync": response }),
            warnings,
            context,
            Some(repo_path),
        )
    }
}

fn resolve_sync_mode(
    context: &AppRuntimeContext,
    raw_sync_mode: Option<&str>,
) -> Result<SyncMode, CliCommandError> {
    let mode = match raw_sync_mode {
        Some("copy") => AgentSyncMode::Copy,
        Some("symlink") => AgentSyncMode::Symlink,
        Some(other) => return Err(CliCommandError::invalid_sync_mode(other)),
        None => {
            context
                .load_settings()
                .map_err(|error| {
                    CliCommandError::filesystem(
                        "config_read_failed",
                        format!("Failed to load settings: {error}"),
                    )
                })?
                .agent_sync_mode
        }
    };

    Ok(match mode {
        AgentSyncMode::Copy => SyncMode::Copy,
        AgentSyncMode::Symlink => SyncMode::Symlink,
    })
}

pub(super) fn sync_failure_error(message: &str, details: Option<Value>) -> CliCommandError {
    if is_windows_symlink_privilege_message(message) {
        return CliCommandError::new(
            CliExitStatus::FilesystemFailure,
            "windows_symlink_privilege_required",
            "Windows symlink creation requires Developer Mode or administrator privileges.",
            details,
        );
    }

    CliCommandError::new(
        CliExitStatus::FilesystemFailure,
        "agent_sync_failed",
        format!("Failed to sync agent skills: {message}"),
        details,
    )
}

fn is_windows_symlink_privilege_message(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    lower.contains("developer mode")
        || lower.contains("symbolic link privilege")
        || lower.contains("privilege not held")
}
