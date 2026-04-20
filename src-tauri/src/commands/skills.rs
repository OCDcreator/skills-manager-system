use tauri::Manager;

use crate::core::settings::SettingsStore;
use crate::core::skills::documents::{read_skill_document, SkillDocument};
use crate::core::skills::scan::{scan_repo_skills, ScanSkillsResponse};

#[tauri::command]
pub fn scan_skills(app: tauri::AppHandle) -> Result<ScanSkillsResponse, String> {
    let config_dir = app.path().app_config_dir().map_err(|error| error.to_string())?;
    let settings = SettingsStore::new(config_dir)
        .load()
        .map_err(|error| error.to_string())?;
    let repo_path = settings
        .repo_path
        .ok_or_else(|| "Repository path is not configured".to_string())?;

    scan_repo_skills(std::path::Path::new(&repo_path)).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_skill_document(app: tauri::AppHandle, relative_path: String) -> Result<SkillDocument, String> {
    let config_dir = app.path().app_config_dir().map_err(|error| error.to_string())?;
    let settings = SettingsStore::new(config_dir)
        .load()
        .map_err(|error| error.to_string())?;
    let repo_path = settings
        .repo_path
        .ok_or_else(|| "Repository path is not configured".to_string())?;

    read_skill_document(std::path::Path::new(&repo_path), &relative_path)
        .map_err(|error| error.to_string())
}
