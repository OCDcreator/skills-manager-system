use std::path::Path;

use tauri::Manager;

use crate::core::agents::config::AgentConfigStore;
use crate::core::agents::discovery::{
    load_agent_inventory, AgentInventorySnapshot, AgentSystemDirs,
};
use crate::core::agents::sync::{
    apply_agent_sync as apply_agent_sync_core, ApplyAgentSyncResponse,
};
use crate::core::settings::SettingsStore;

fn app_config_dir(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    app.path()
        .app_config_dir()
        .map_err(|error| error.to_string())
}

fn load_repo_path(app: &tauri::AppHandle) -> Result<String, String> {
    let config_dir = app_config_dir(app)?;
    let settings = SettingsStore::new(config_dir)
        .load()
        .map_err(|error| error.to_string())?;

    settings
        .repo_path
        .ok_or_else(|| "Repository path is not configured".to_string())
}

fn load_inventory(app: &tauri::AppHandle) -> Result<AgentInventorySnapshot, String> {
    let config_dir = app_config_dir(app)?;
    let system_dirs = AgentSystemDirs::current().map_err(|error| error.to_string())?;
    load_agent_inventory(&config_dir, &system_dirs).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_agent_inventory(app: tauri::AppHandle) -> Result<AgentInventorySnapshot, String> {
    load_inventory(&app)
}

#[tauri::command]
pub fn set_agent_enabled(
    app: tauri::AppHandle,
    key: String,
    enabled: bool,
) -> Result<AgentInventorySnapshot, String> {
    let config_dir = app_config_dir(&app)?;
    AgentConfigStore::new(config_dir)
        .set_agent_enabled(&key, enabled)
        .map_err(|error| error.to_string())?;
    load_inventory(&app)
}

#[tauri::command]
pub fn set_agent_path_override(
    app: tauri::AppHandle,
    key: String,
    path: String,
) -> Result<AgentInventorySnapshot, String> {
    let config_dir = app_config_dir(&app)?;
    AgentConfigStore::new(config_dir)
        .set_agent_path_override(&key, &path)
        .map_err(|error| error.to_string())?;
    load_inventory(&app)
}

#[tauri::command]
pub fn clear_agent_path_override(
    app: tauri::AppHandle,
    key: String,
) -> Result<AgentInventorySnapshot, String> {
    let config_dir = app_config_dir(&app)?;
    AgentConfigStore::new(config_dir)
        .clear_agent_path_override(&key)
        .map_err(|error| error.to_string())?;
    load_inventory(&app)
}

#[tauri::command]
pub fn apply_agent_sync(app: tauri::AppHandle) -> Result<ApplyAgentSyncResponse, String> {
    let repo_path = load_repo_path(&app)?;
    let config_dir = app_config_dir(&app)?;
    let system_dirs = AgentSystemDirs::current().map_err(|error| error.to_string())?;

    apply_agent_sync_core(&config_dir, Path::new(&repo_path), &system_dirs)
        .map_err(|error| error.to_string())
}
