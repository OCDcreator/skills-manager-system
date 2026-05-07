use std::path::Path;

use tauri::Manager;

use crate::core::agents::catalog::find_agent;
use crate::core::agents::discovery::{
    load_agent_inventory, AgentInventorySnapshot, AgentSystemDirs,
};
use crate::core::agents::target_management::{
    delete_target_skill_entry, import_unmanaged_target_skill, take_over_unmanaged_target_skill,
    ImportTargetSkillResult,
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

fn resolve_agent_target_dir(app: &tauri::AppHandle, agent_key: &str) -> Result<String, String> {
    if find_agent(agent_key).is_none() {
        return Err(format!("Unsupported agent key: {agent_key}"));
    }

    let inventory = load_inventory(app)?;
    let agent = inventory
        .agents
        .into_iter()
        .find(|item| item.key == agent_key)
        .ok_or_else(|| format!("Unknown agent key: {agent_key}"))?;

    agent
        .effective_skills_dir
        .ok_or_else(|| "No effective target directory is available for this agent".to_string())
}

#[tauri::command]
pub fn take_over_agent_target_skill(
    app: tauri::AppHandle,
    agent_key: String,
    entry_name: String,
) -> Result<(), String> {
    let target_dir = resolve_agent_target_dir(&app, &agent_key)?;
    take_over_unmanaged_target_skill(Path::new(&target_dir), &agent_key, &entry_name)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn delete_agent_target_skill(
    app: tauri::AppHandle,
    agent_key: String,
    entry_name: String,
) -> Result<(), String> {
    let target_dir = resolve_agent_target_dir(&app, &agent_key)?;
    delete_target_skill_entry(Path::new(&target_dir), &agent_key, &entry_name)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn import_agent_target_skill(
    app: tauri::AppHandle,
    agent_key: String,
    entry_name: String,
    delete_source_after_import: bool,
) -> Result<ImportTargetSkillResult, String> {
    let target_dir = resolve_agent_target_dir(&app, &agent_key)?;
    let repo_path = load_repo_path(&app)?;
    import_unmanaged_target_skill(
        Path::new(&repo_path),
        Path::new(&target_dir),
        &agent_key,
        &entry_name,
        delete_source_after_import,
    )
    .map_err(|error| error.to_string())
}
