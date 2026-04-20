use std::path::Path;

use tauri::Manager;

use crate::core::settings::SettingsStore;

#[tauri::command]
pub fn get_repo_path(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|error| error.to_string())?;
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

    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|error| error.to_string())?;
    SettingsStore::new(config_dir)
        .save_repo_path(normalized)
        .map(|settings| settings.repo_path)
        .map_err(|error| error.to_string())
}
