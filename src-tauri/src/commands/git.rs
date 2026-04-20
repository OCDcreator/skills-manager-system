use std::path::Path;

use tauri::Manager;

use crate::core::git::operations::{
    git_commit as git_commit_core, git_diff as git_diff_core, git_fetch as git_fetch_core,
    git_log as git_log_core, git_pull as git_pull_core, git_push as git_push_core,
    git_status as git_status_core, run_sync_script as run_sync_script_core, GitDiffResponse,
    GitLogResponse, GitOperationResult, GitStatusResponse,
};
use crate::core::settings::SettingsStore;

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
pub fn git_status(app: tauri::AppHandle) -> Result<GitStatusResponse, String> {
    let repo_path = load_repo_path(&app)?;
    git_status_core(Path::new(&repo_path)).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn git_diff(app: tauri::AppHandle, staged: bool) -> Result<GitDiffResponse, String> {
    let repo_path = load_repo_path(&app)?;
    git_diff_core(Path::new(&repo_path), staged).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn git_log(app: tauri::AppHandle, max_count: Option<usize>) -> Result<GitLogResponse, String> {
    let repo_path = load_repo_path(&app)?;
    git_log_core(Path::new(&repo_path), max_count.unwrap_or(20)).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn git_pull(app: tauri::AppHandle) -> Result<GitOperationResult, String> {
    let repo_path = load_repo_path(&app)?;
    Ok(git_pull_core(Path::new(&repo_path)))
}

#[tauri::command]
pub fn git_push(app: tauri::AppHandle) -> Result<GitOperationResult, String> {
    let repo_path = load_repo_path(&app)?;
    Ok(git_push_core(Path::new(&repo_path)))
}

#[tauri::command]
pub fn git_commit(app: tauri::AppHandle, message: String) -> Result<GitOperationResult, String> {
    let trimmed = message.trim();
    if trimmed.is_empty() {
        return Err("Commit message is required".to_string());
    }
    let repo_path = load_repo_path(&app)?;
    Ok(git_commit_core(Path::new(&repo_path), trimmed))
}

#[tauri::command]
pub fn git_fetch(app: tauri::AppHandle) -> Result<GitOperationResult, String> {
    let repo_path = load_repo_path(&app)?;
    Ok(git_fetch_core(Path::new(&repo_path)))
}

#[tauri::command]
pub fn run_sync_script(app: tauri::AppHandle) -> Result<GitOperationResult, String> {
    let repo_path = load_repo_path(&app)?;
    Ok(run_sync_script_core(Path::new(&repo_path)))
}
