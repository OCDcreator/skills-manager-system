use std::path::Path;

use tauri::Manager;

use crate::core::settings::{AgentSyncMode, SettingsStore};

fn app_config_dir(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    app.path()
        .app_config_dir()
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_repo_path(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let config_dir = app_config_dir(&app)?;
    SettingsStore::new(config_dir)
        .load()
        .map(|settings| settings.repo_path)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_repo_path(app: tauri::AppHandle, path: String) -> Result<Option<String>, String> {
    let trimmed = path.trim();
    let normalized = if trimmed.is_empty() {
        None
    } else {
        Some(Path::new(trimmed))
    };

    let config_dir = app_config_dir(&app)?;
    SettingsStore::new(config_dir)
        .save_repo_path(normalized)
        .map(|settings| settings.repo_path)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_agent_sync_mode(app: tauri::AppHandle) -> Result<AgentSyncMode, String> {
    let config_dir = app_config_dir(&app)?;
    SettingsStore::new(config_dir)
        .load()
        .map(|settings| settings.agent_sync_mode)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_agent_sync_mode(
    app: tauri::AppHandle,
    sync_mode: AgentSyncMode,
) -> Result<AgentSyncMode, String> {
    let config_dir = app_config_dir(&app)?;
    SettingsStore::new(config_dir)
        .save_agent_sync_mode(sync_mode)
        .map(|settings| settings.agent_sync_mode)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_agent_order(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let config_dir = app_config_dir(&app)?;
    SettingsStore::new(config_dir)
        .load()
        .map(|settings| settings.agent_order)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_agent_order(
    app: tauri::AppHandle,
    agent_order: Vec<String>,
) -> Result<Vec<String>, String> {
    let config_dir = app_config_dir(&app)?;
    SettingsStore::new(config_dir)
        .save_agent_order(&agent_order)
        .map(|settings| settings.agent_order)
        .map_err(|error| error.to_string())
}
