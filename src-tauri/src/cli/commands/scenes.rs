use serde_json::json;
use std::path::Path;

use crate::app_runtime::{AppRuntimeContext, CliCommandError, CliRunResult};
use crate::cli::args::ScenesCommand;
use crate::core::agents::discovery::AgentSystemDirs;
use crate::core::scenes::config::{SceneConfigSnapshot, SceneConfigStore};
use crate::core::scenes::manager::apply_scene;

pub fn run(context: &AppRuntimeContext, command: &ScenesCommand) -> CliRunResult {
    match command {
        ScenesCommand::List => list(context),
        ScenesCommand::Create {
            id,
            name,
            description,
        } => mutate_config(context, "scenes create", Some(id), |store| {
            store.create_scene(id, name, description)
        }),
        ScenesCommand::Update {
            id,
            name,
            description,
        } => mutate_config(context, "scenes update", Some(id), |store| {
            store.update_scene(id, name.as_deref(), description.as_deref())
        }),
        ScenesCommand::Delete { id } => {
            mutate_config(context, "scenes delete", Some(id), |store| {
                store.delete_scene(id)
            })
        }
        ScenesCommand::SetActive { id } => {
            mutate_config(context, "scenes set-active", Some(id), |store| {
                store.set_active_scene(Some(id))
            })
        }
        ScenesCommand::ClearActive => {
            mutate_config(context, "scenes clear-active", None, |store| {
                store.set_active_scene(None)
            })
        }
        ScenesCommand::SetSkills {
            id,
            disabled_skill_ids,
        } => mutate_config(context, "scenes set-skills", Some(id), |store| {
            store.set_scene_skills(id, disabled_skill_ids.clone())
        }),
        ScenesCommand::SetAgents {
            id,
            enabled_agent_keys,
        } => mutate_config(context, "scenes set-agents", Some(id), |store| {
            store.set_scene_agents(id, enabled_agent_keys.clone())
        }),
        ScenesCommand::SetSkillOrder { id, skill_order } => {
            mutate_config(context, "scenes set-skill-order", Some(id), |store| {
                store.set_scene_skill_order(id, skill_order.clone())
            })
        }
        ScenesCommand::Apply { id } => apply(context, id),
    }
}

fn list(context: &AppRuntimeContext) -> CliRunResult {
    let repo_path = context.current_repo_path().ok().flatten();
    match SceneConfigStore::new(context.config_dir.clone()).load() {
        Ok(snapshot) => CliRunResult::success(
            "scenes list",
            json!({ "sceneConfig": snapshot }),
            context,
            repo_path.as_deref(),
        ),
        Err(error) => CliRunResult::error(
            "scenes list",
            CliCommandError::filesystem(
                "config_read_failed",
                format!("Failed to load scene configuration: {error}"),
            ),
            context,
            repo_path.as_deref(),
        ),
    }
}

fn mutate_config(
    context: &AppRuntimeContext,
    command: &str,
    scene_id: Option<&str>,
    mutation: impl FnOnce(&SceneConfigStore) -> anyhow::Result<SceneConfigSnapshot>,
) -> CliRunResult {
    let repo_path = context.current_repo_path().ok().flatten();
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

    match mutation(&SceneConfigStore::new(context.config_dir.clone())) {
        Ok(snapshot) => CliRunResult::success(
            command,
            json!({ "sceneConfig": snapshot }),
            context,
            repo_path.as_deref(),
        ),
        Err(error) => CliRunResult::error(
            command,
            map_scene_error(scene_id, error),
            context,
            repo_path.as_deref(),
        ),
    }
}

fn apply(context: &AppRuntimeContext, scene_id: &str) -> CliRunResult {
    let repo_path = match context.require_repo_path() {
        Ok(repo_path) => repo_path,
        Err(_) => {
            return CliRunResult::error(
                "scenes apply",
                CliCommandError::missing_configuration(
                    "repo_path_not_configured",
                    "Repository path is not configured.",
                ),
                context,
                None,
            );
        }
    };
    let system_dirs = match AgentSystemDirs::current() {
        Ok(system_dirs) => system_dirs,
        Err(error) => {
            return CliRunResult::error(
                "scenes apply",
                CliCommandError::filesystem(
                    "agent_system_dirs_unavailable",
                    format!("Failed to determine agent system directories: {error}"),
                ),
                context,
                Some(repo_path.as_path()),
            );
        }
    };

    apply_with_system_dirs(context, &repo_path, &system_dirs, scene_id)
}

pub(super) fn apply_with_system_dirs(
    context: &AppRuntimeContext,
    repo_path: &Path,
    system_dirs: &AgentSystemDirs,
    scene_id: &str,
) -> CliRunResult {
    let _guard = match context.acquire_config_lock() {
        Ok(guard) => guard,
        Err(error) => {
            return CliRunResult::error(
                "scenes apply",
                CliCommandError::from_config_lock(error),
                context,
                Some(repo_path),
            );
        }
    };

    match apply_scene(&context.config_dir, repo_path, system_dirs, scene_id) {
        Ok(result) => CliRunResult::success(
            "scenes apply",
            json!({ "scene": result }),
            context,
            Some(repo_path),
        ),
        Err(error) => CliRunResult::error(
            "scenes apply",
            map_scene_error(Some(scene_id), error),
            context,
            Some(repo_path),
        ),
    }
}

fn map_scene_error(scene_id: Option<&str>, error: anyhow::Error) -> CliCommandError {
    let message = error.to_string();
    let details = scene_id.map(|id| json!({ "sceneId": id }));

    if message.contains("not found") {
        CliCommandError::target_not_found("scene_not_found", message, details)
    } else if message.contains("already exists") {
        CliCommandError::state_conflict("scene_already_exists", message, details)
    } else {
        CliCommandError::config_write_failed(
            format!("Failed to update scene configuration: {message}"),
            details,
        )
    }
}
