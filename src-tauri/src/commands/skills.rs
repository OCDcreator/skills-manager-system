use std::path::Path;
use tauri::Manager;

use crate::core::settings::SettingsStore;
use crate::core::skills::cache::{
    load_cached_repo_skills_with_external_sources, scan_repo_skills_cached_with_external_sources,
};
use crate::core::skills::documents::{read_skill_document, SkillDocument};
use crate::core::skills::scan::ScanSkillsResponse;
use crate::core::skills::state::{SkillStateSnapshot, SkillStateStore};

fn load_repo_path(app: &tauri::AppHandle) -> Result<String, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|error| error.to_string())?;
    let settings = SettingsStore::new(config_dir)
        .load()
        .map_err(|error| error.to_string())?;

    settings
        .repo_path
        .ok_or_else(|| "Repository path is not configured".to_string())
}

#[tauri::command]
pub fn scan_skills(app: tauri::AppHandle) -> Result<ScanSkillsResponse, String> {
    let repo_path = load_repo_path(&app)?;
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|error| error.to_string())?;

    scan_repo_skills_cached_with_external_sources(Path::new(&repo_path), &config_dir)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn load_cached_skills(app: tauri::AppHandle) -> Result<Option<ScanSkillsResponse>, String> {
    let repo_path = load_repo_path(&app)?;
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|error| error.to_string())?;

    load_cached_repo_skills_with_external_sources(Path::new(&repo_path), &config_dir)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_skill_document(
    app: tauri::AppHandle,
    relative_path: String,
) -> Result<SkillDocument, String> {
    let repo_path = load_repo_path(&app)?;

    read_skill_document(Path::new(&repo_path), &relative_path).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_skill_state(app: tauri::AppHandle) -> Result<SkillStateSnapshot, String> {
    let repo_path = load_repo_path(&app)?;
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|error| error.to_string())?;

    SkillStateStore::new(config_dir)
        .load_for_repo(Path::new(&repo_path))
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_skill_enabled(
    app: tauri::AppHandle,
    skill_id: String,
    enabled: bool,
) -> Result<SkillStateSnapshot, String> {
    if skill_id.trim().is_empty() {
        return Err("Skill id is required".to_string());
    }

    let repo_path = load_repo_path(&app)?;
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|error| error.to_string())?;

    SkillStateStore::new(config_dir)
        .set_skill_enabled(Path::new(&repo_path), &skill_id, enabled)
        .map_err(|error| error.to_string())
}
