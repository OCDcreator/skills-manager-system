use std::collections::BTreeMap;
use std::path::Path;

use tauri::Manager;

use crate::core::agents::discovery::AgentSystemDirs;
use crate::core::projects::path_inspection::{
    inspect_project_assignment_path as inspect_project_assignment_path_core, ProjectPathInspection,
};
use crate::core::projects::store::{
    ProjectAgentAssignment, ProjectConfigSnapshot, ProjectConfigStore,
};
use crate::core::projects::sync::{
    apply_project_assignments as apply_project_assignments_core, ApplyProjectAssignmentsResponse,
    attach_project_apply_statuses,
};
use crate::core::settings::SettingsStore;

fn app_config_dir(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    app.path()
        .app_config_dir()
        .map_err(|error| error.to_string())
}

fn load_repo_path(app: &tauri::AppHandle) -> Result<String, String> {
    let settings = SettingsStore::new(app_config_dir(app)?)
        .load()
        .map_err(|error| error.to_string())?;

    settings
        .repo_path
        .ok_or_else(|| "Repository path is not configured".to_string())
}

fn project_store(app: &tauri::AppHandle) -> Result<ProjectConfigStore, String> {
    Ok(ProjectConfigStore::new(app_config_dir(app)?))
}

fn with_apply_statuses(
    app: &tauri::AppHandle,
    snapshot: ProjectConfigSnapshot,
) -> Result<ProjectConfigSnapshot, String> {
    let Ok(repo_path) = load_repo_path(app) else {
        return Ok(snapshot);
    };
    let config_dir = app_config_dir(app)?;
    let system_dirs = AgentSystemDirs::current().map_err(|error| error.to_string())?;

    attach_project_apply_statuses(&config_dir, Path::new(&repo_path), &system_dirs, snapshot)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_project_config(app: tauri::AppHandle) -> Result<ProjectConfigSnapshot, String> {
    let snapshot = project_store(&app)?
        .load()
        .map_err(|error| error.to_string())?;
    with_apply_statuses(&app, snapshot)
}

#[tauri::command]
pub fn add_project(
    app: tauri::AppHandle,
    project_path: String,
    display_name: String,
    skill_ids: Vec<String>,
    agent_keys: Vec<String>,
) -> Result<ProjectConfigSnapshot, String> {
    let snapshot = project_store(&app)?
        .add_project(&project_path, &display_name, skill_ids, agent_keys)
        .map_err(|error| error.to_string())?;
    with_apply_statuses(&app, snapshot)
}

#[tauri::command]
pub fn add_project_with_agents(
    app: tauri::AppHandle,
    project_path: String,
    display_name: String,
    agents: BTreeMap<String, ProjectAgentAssignment>,
) -> Result<ProjectConfigSnapshot, String> {
    let snapshot = project_store(&app)?
        .add_project_with_agents(&project_path, &display_name, agents)
        .map_err(|error| error.to_string())?;
    with_apply_statuses(&app, snapshot)
}

#[tauri::command]
pub fn update_project(
    app: tauri::AppHandle,
    project_path: String,
    display_name: Option<String>,
    skill_ids: Option<Vec<String>>,
    agent_keys: Option<Vec<String>>,
) -> Result<ProjectConfigSnapshot, String> {
    let snapshot = project_store(&app)?
        .update_project(
            &project_path,
            display_name.as_deref(),
            skill_ids,
            agent_keys,
        )
        .map_err(|error| error.to_string())?;
    with_apply_statuses(&app, snapshot)
}

#[tauri::command]
pub fn update_project_with_agents(
    app: tauri::AppHandle,
    project_path: String,
    display_name: Option<String>,
    agents: Option<BTreeMap<String, ProjectAgentAssignment>>,
) -> Result<ProjectConfigSnapshot, String> {
    let snapshot = project_store(&app)?
        .update_project_agents(&project_path, display_name.as_deref(), agents)
        .map_err(|error| error.to_string())?;
    with_apply_statuses(&app, snapshot)
}

#[tauri::command]
pub fn remove_project(
    app: tauri::AppHandle,
    project_path: String,
) -> Result<ProjectConfigSnapshot, String> {
    let snapshot = project_store(&app)?
        .remove_project(&project_path)
        .map_err(|error| error.to_string())?;
    with_apply_statuses(&app, snapshot)
}

#[tauri::command]
pub fn inspect_project_assignment_path(
    project_path: String,
    agent_keys: Vec<String>,
) -> Result<ProjectPathInspection, String> {
    inspect_project_assignment_path_core(&project_path, &agent_keys)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn apply_project_assignments(
    app: tauri::AppHandle,
) -> Result<ApplyProjectAssignmentsResponse, String> {
    let repo_path = load_repo_path(&app)?;
    let config_dir = app_config_dir(&app)?;
    let system_dirs = AgentSystemDirs::current().map_err(|error| error.to_string())?;

    apply_project_assignments_core(&config_dir, Path::new(&repo_path), &system_dirs)
        .map_err(|error| error.to_string())
}
