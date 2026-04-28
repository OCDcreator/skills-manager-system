use std::path::PathBuf;

use tauri::Manager;

use crate::core::external_sources::{
    add_external_source as add_external_source_core,
    fetch_external_source as fetch_external_source_core,
    import_external_variant as import_external_variant_core,
    list_external_sources as list_external_sources_core,
    remove_external_source as remove_external_source_core,
    repair_external_import as repair_external_import_core,
    update_external_import as update_external_import_core,
    ExternalSourcesListResponse, ImportVariantResult,
};
use crate::core::settings::SettingsStore;

fn app_config_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path().app_config_dir().map_err(|error| error.to_string())
}

fn load_repo_path(app: &tauri::AppHandle) -> Result<Option<PathBuf>, String> {
    let settings = SettingsStore::new(app_config_dir(app)?)
        .load()
        .map_err(|error| error.to_string())?;
    Ok(settings.repo_path.map(PathBuf::from))
}

fn require_repo_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    load_repo_path(app)?
        .ok_or_else(|| "Repository path is not configured".to_string())
}

#[tauri::command]
pub fn list_external_sources(app: tauri::AppHandle) -> Result<ExternalSourcesListResponse, String> {
    let config_dir = app_config_dir(&app)?;
    let repo_root = load_repo_path(&app)?;
    list_external_sources_core(&config_dir, repo_root.as_deref()).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn add_external_source(
    app: tauri::AppHandle,
    repo_url: String,
) -> Result<ExternalSourcesListResponse, String> {
    let config_dir = app_config_dir(&app)?;
    let repo_root = load_repo_path(&app)?;
    add_external_source_core(&config_dir, repo_root.as_deref(), &repo_url)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn fetch_external_source(
    app: tauri::AppHandle,
    source_id: String,
) -> Result<ExternalSourcesListResponse, String> {
    let config_dir = app_config_dir(&app)?;
    let repo_root = load_repo_path(&app)?;
    fetch_external_source_core(&config_dir, repo_root.as_deref(), &source_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn import_external_variant(
    app: tauri::AppHandle,
    source_id: String,
    agent_key: String,
    variant_path: String,
) -> Result<ImportVariantResult, String> {
    let config_dir = app_config_dir(&app)?;
    let repo_root = require_repo_path(&app)?;
    import_external_variant_core(&config_dir, &repo_root, &source_id, &agent_key, &variant_path)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn update_external_import(
    app: tauri::AppHandle,
    import_id: String,
) -> Result<ImportVariantResult, String> {
    let config_dir = app_config_dir(&app)?;
    let repo_root = require_repo_path(&app)?;
    update_external_import_core(&config_dir, &repo_root, &import_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn remove_external_source(
    app: tauri::AppHandle,
    source_id: String,
    remove_imports: bool,
) -> Result<ExternalSourcesListResponse, String> {
    let config_dir = app_config_dir(&app)?;
    let repo_root = load_repo_path(&app)?;
    remove_external_source_core(&config_dir, repo_root.as_deref(), &source_id, remove_imports)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn repair_external_import(
    app: tauri::AppHandle,
    import_id: String,
) -> Result<ImportVariantResult, String> {
    let config_dir = app_config_dir(&app)?;
    let repo_root = require_repo_path(&app)?;
    repair_external_import_core(&config_dir, &repo_root, &import_id)
        .map_err(|error| error.to_string())
}
