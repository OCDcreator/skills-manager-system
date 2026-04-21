use serde_json::json;

use crate::app_runtime::{AppRuntimeContext, CliCommandError, CliRunResult};
use crate::cli::args::AgentsCommand;
use crate::core::agents::discovery::{load_agent_inventory, AgentSystemDirs};

pub fn run(context: &AppRuntimeContext, command: &AgentsCommand) -> CliRunResult {
    match command {
        AgentsCommand::List => list(context),
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

fn list_with_system_dirs(
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
