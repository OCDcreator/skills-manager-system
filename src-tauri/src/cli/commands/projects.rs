use serde_json::json;
use std::path::Path;

use crate::app_runtime::{
    AppRuntimeContext, CliCommandError, CliExitStatus, CliRunResult, CliWarning,
};
use crate::cli::args::ProjectsCommand;
use crate::core::agents::discovery::AgentSystemDirs;
use crate::core::agents::sync::AgentApplyStatus;
use crate::core::projects::store::{ProjectConfigSnapshot, ProjectConfigStore};
use crate::core::projects::sync::{apply_project_assignments, ApplyProjectAssignmentsResponse};

use super::project_agent_args::build_layered_agent_map;

pub fn run(context: &AppRuntimeContext, command: &ProjectsCommand) -> CliRunResult {
    match command {
        ProjectsCommand::List => list(context),
        ProjectsCommand::Add {
            project_path,
            display_name,
            skill_ids,
            agent_keys,
            agent_skill_ids,
            agent_scene_ids,
            agent_excluded_skill_ids,
        } => mutate_config(
            context,
            "projects add",
            Some(project_path.as_path()),
            |store| match build_layered_agent_map(
                agent_keys,
                skill_ids,
                agent_skill_ids,
                agent_scene_ids,
                agent_excluded_skill_ids,
            )? {
                Some(agents) => store.add_project_with_agents(
                    &project_path.to_string_lossy(),
                    display_name,
                    agents,
                ),
                None => store.add_project(
                    &project_path.to_string_lossy(),
                    display_name,
                    skill_ids.clone(),
                    agent_keys.clone(),
                ),
            },
        ),
        ProjectsCommand::Update {
            project_path,
            display_name,
            skill_ids,
            agent_keys,
            agent_skill_ids,
            agent_scene_ids,
            agent_excluded_skill_ids,
        } => mutate_config(
            context,
            "projects update",
            Some(project_path.as_path()),
            |store| match build_layered_agent_map(
                agent_keys,
                skill_ids,
                agent_skill_ids,
                agent_scene_ids,
                agent_excluded_skill_ids,
            )? {
                Some(agents) => store.update_project_agents(
                    &project_path.to_string_lossy(),
                    display_name.as_deref(),
                    Some(agents),
                ),
                None => store.update_project(
                    &project_path.to_string_lossy(),
                    display_name.as_deref(),
                    (!skill_ids.is_empty()).then(|| skill_ids.clone()),
                    (!agent_keys.is_empty()).then(|| agent_keys.clone()),
                ),
            },
        ),
        ProjectsCommand::Remove { project_path } => mutate_config(
            context,
            "projects remove",
            Some(project_path.as_path()),
            |store| store.remove_project(&project_path.to_string_lossy()),
        ),
        ProjectsCommand::Apply => apply(context),
    }
}

fn list(context: &AppRuntimeContext) -> CliRunResult {
    let repo_path = context.current_repo_path().ok().flatten();
    match ProjectConfigStore::new(context.config_dir.clone()).load() {
        Ok(snapshot) => CliRunResult::success(
            "projects list",
            json!({ "projectConfig": snapshot }),
            context,
            repo_path.as_deref(),
        ),
        Err(error) => CliRunResult::error(
            "projects list",
            CliCommandError::filesystem(
                "config_read_failed",
                format!("Failed to load project configuration: {error}"),
            ),
            context,
            repo_path.as_deref(),
        ),
    }
}

fn mutate_config(
    context: &AppRuntimeContext,
    command: &str,
    project_path: Option<&Path>,
    mutation: impl FnOnce(&ProjectConfigStore) -> anyhow::Result<ProjectConfigSnapshot>,
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

    match mutation(&ProjectConfigStore::new(context.config_dir.clone())) {
        Ok(snapshot) => CliRunResult::success(
            command,
            json!({ "projectConfig": snapshot }),
            context,
            repo_path.as_deref(),
        ),
        Err(error) => CliRunResult::error(
            command,
            map_project_error(project_path, error),
            context,
            repo_path.as_deref(),
        ),
    }
}

fn apply(context: &AppRuntimeContext) -> CliRunResult {
    let repo_path = match context.require_repo_path() {
        Ok(repo_path) => repo_path,
        Err(_) => {
            return CliRunResult::error(
                "projects apply",
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
                "projects apply",
                CliCommandError::filesystem(
                    "agent_system_dirs_unavailable",
                    format!("Failed to determine agent system directories: {error}"),
                ),
                context,
                Some(repo_path.as_path()),
            );
        }
    };

    apply_with_system_dirs(context, &repo_path, &system_dirs)
}

pub(super) fn apply_with_system_dirs(
    context: &AppRuntimeContext,
    repo_path: &Path,
    system_dirs: &AgentSystemDirs,
) -> CliRunResult {
    let _guard = match context.acquire_config_lock() {
        Ok(guard) => guard,
        Err(error) => {
            return CliRunResult::error(
                "projects apply",
                CliCommandError::from_config_lock(error),
                context,
                Some(repo_path),
            );
        }
    };

    match apply_project_assignments(&context.config_dir, repo_path, system_dirs) {
        Ok(response) => apply_response_to_result(context, repo_path, response),
        Err(error) => CliRunResult::error(
            "projects apply",
            map_project_error(None, error),
            context,
            Some(repo_path),
        ),
    }
}

fn apply_response_to_result(
    context: &AppRuntimeContext,
    repo_path: &Path,
    response: ApplyProjectAssignmentsResponse,
) -> CliRunResult {
    if let Some((project_path, failed)) = response.results.iter().find_map(|project| {
        project
            .results
            .iter()
            .find(|result| result.status == AgentApplyStatus::Failed)
            .map(|result| (project.project_path.as_str(), result))
    }) {
        return CliRunResult::error(
            "projects apply",
            CliCommandError::new(
                CliExitStatus::FilesystemFailure,
                "project_apply_failed",
                failed.message.clone(),
                Some(json!({ "projectPath": project_path, "result": failed })),
            ),
            context,
            Some(repo_path),
        );
    }

    let warnings = response
        .results
        .iter()
        .flat_map(|project| {
            project.results.iter().filter_map(|result| {
                (result.status == AgentApplyStatus::Partial).then(|| {
                    let mut warning = CliWarning::new(
                        "project_sync_conflicts",
                        result.message.clone(),
                        Some(project.project_path.clone()),
                    );
                    warning.details = Some(json!({
                        "agentKey": result.key,
                        "conflictCount": result.conflict_count,
                    }));
                    warning
                })
            })
        })
        .collect::<Vec<_>>();

    if warnings.is_empty() {
        CliRunResult::success(
            "projects apply",
            json!({ "projects": response }),
            context,
            Some(repo_path),
        )
    } else {
        CliRunResult::partial(
            "projects apply",
            json!({ "projects": response }),
            warnings,
            context,
            Some(repo_path),
        )
    }
}

fn map_project_error(project_path: Option<&Path>, error: anyhow::Error) -> CliCommandError {
    let message = error.to_string();
    let details = project_path.map(|path| json!({ "projectPath": path.to_string_lossy() }));

    if message.contains("not found") {
        CliCommandError::target_not_found("project_not_found", message, details)
    } else if message.contains("already exists") {
        CliCommandError::state_conflict("project_already_exists", message, details)
    } else if message.contains("must be absolute")
        || message.contains("is required")
        || message.contains("AGENT=VALUE")
        || message.contains("agent key cannot be empty")
        || message.contains("value cannot be empty")
    {
        CliCommandError::new(
            CliExitStatus::ArgumentError,
            "invalid_project_path",
            message,
            details,
        )
    } else {
        CliCommandError::config_write_failed(
            format!("Failed to update project configuration: {message}"),
            details,
        )
    }
}
