use serde_json::json;

use crate::app_runtime::{AppRuntimeContext, CliCommandError, CliRunResult};
use crate::cli::args::AgentsCommand;
use crate::core::agents::catalog::find_agent;
use crate::core::agents::config::AgentConfigStore;
use crate::core::agents::discovery::{load_agent_inventory, AgentSystemDirs};

pub fn run(context: &AppRuntimeContext, command: &AgentsCommand) -> CliRunResult {
    match command {
        AgentsCommand::List => list(context),
        AgentsCommand::Enable { key } => set_enabled(context, key, true),
        AgentsCommand::Disable { key } => set_enabled(context, key, false),
        AgentsCommand::SetPath { key, path } => set_path(context, key, path),
        AgentsCommand::ClearPath { key } => clear_path(context, key),
        AgentsCommand::Sync { sync_mode } => super::agent_sync::run(context, sync_mode.as_deref()),
    }
}

fn list(context: &AppRuntimeContext) -> CliRunResult {
    let system_dirs = match AgentSystemDirs::current() {
        Ok(system_dirs) => system_dirs,
        Err(error) => {
            return CliRunResult::error(
                "agents list",
                CliCommandError::filesystem(
                    "agent_system_dirs_unavailable",
                    format!("Failed to determine agent system directories: {error}"),
                ),
                context,
                context.current_repo_path().ok().flatten().as_deref(),
            );
        }
    };

    list_with_system_dirs(context, &system_dirs)
}

pub(super) fn list_with_system_dirs(
    context: &AppRuntimeContext,
    system_dirs: &AgentSystemDirs,
) -> CliRunResult {
    let repo_path = context.current_repo_path().ok().flatten();

    match load_agent_inventory(&context.config_dir, system_dirs) {
        Ok(snapshot) => CliRunResult::success(
            "agents list",
            json!({ "agents": snapshot.agents }),
            context,
            repo_path.as_deref(),
        ),
        Err(error) => CliRunResult::error(
            "agents list",
            CliCommandError::filesystem(
                "agent_inventory_read_failed",
                format!("Failed to load the agent inventory: {error}"),
            ),
            context,
            repo_path.as_deref(),
        ),
    }
}

fn set_enabled(context: &AppRuntimeContext, key: &str, enabled: bool) -> CliRunResult {
    let command = if enabled {
        "agents enable"
    } else {
        "agents disable"
    };
    mutate_agent_config(context, command, key, |store| {
        store.set_agent_enabled(key, enabled)
    })
}

fn set_path(context: &AppRuntimeContext, key: &str, path: &str) -> CliRunResult {
    mutate_agent_config(context, "agents set-path", key, |store| {
        store.set_agent_path_override(key, path)
    })
}

fn clear_path(context: &AppRuntimeContext, key: &str) -> CliRunResult {
    mutate_agent_config(context, "agents clear-path", key, |store| {
        store.clear_agent_path_override(key)
    })
}

fn mutate_agent_config(
    context: &AppRuntimeContext,
    command: &str,
    key: &str,
    mutation: impl FnOnce(
        &AgentConfigStore,
    ) -> anyhow::Result<crate::core::agents::config::AgentConfigSnapshot>,
) -> CliRunResult {
    let repo_path = context.current_repo_path().ok().flatten();
    if find_agent(key).is_none() {
        return CliRunResult::error(
            command,
            CliCommandError::target_not_found(
                "agent_not_found",
                format!("Agent '{key}' does not exist."),
                Some(json!({ "agentKey": key })),
            ),
            context,
            repo_path.as_deref(),
        );
    }

    let _guard = match context.acquire_config_lock() {
        Ok(guard) => guard,
        Err(error) => {
            return CliRunResult::error(
                command,
                CliCommandError::from_config_lock(error),
                context,
                repo_path.as_deref(),
            );
        }
    };

    match mutation(&AgentConfigStore::new(context.config_dir.clone())) {
        Ok(snapshot) => CliRunResult::success(
            command,
            json!({ "agentConfig": snapshot }),
            context,
            repo_path.as_deref(),
        ),
        Err(error) => CliRunResult::error(
            command,
            CliCommandError::config_write_failed(
                format!("Failed to update agent configuration: {error}"),
                Some(json!({ "agentKey": key })),
            ),
            context,
            repo_path.as_deref(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_runtime::AppRuntimeOptions;
    use serde_json::Value;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn agents_list_returns_inventory_payload() {
        let temp_root = tempdir().unwrap();
        let home_dir = temp_root.path().join("home");
        let config_dir = temp_root.path().join("config");
        std::fs::create_dir_all(home_dir.join(".codex/skills")).unwrap();

        let context = AppRuntimeContext::from_options_with_parts(
            AppRuntimeOptions {
                config_dir_override: Some(config_dir.clone()),
                ..AppRuntimeOptions::default()
            },
            PathBuf::from("/unused"),
            crate::app_runtime::DEVELOPMENT_APP_IDENTIFIER,
        );

        let result = list_with_system_dirs(
            &context,
            &AgentSystemDirs {
                home_dir,
                config_dir: Some(temp_root.path().join("xdg-config")),
            },
        );
        let agents = result
            .response
            .data
            .unwrap()
            .get("agents")
            .and_then(Value::as_array)
            .unwrap()
            .to_vec();

        assert_eq!(
            result.response.status,
            crate::app_runtime::CliStatus::Success
        );
        assert!(agents.iter().any(|agent| {
            agent.get("key").and_then(Value::as_str) == Some("codex")
                && agent.get("pathMode").and_then(Value::as_str) == Some("detected")
        }));
    }
}
