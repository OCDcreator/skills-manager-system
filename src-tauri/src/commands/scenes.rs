use tauri::Manager;

use crate::core::scenes::config::{SceneConfigSnapshot, SceneConfigStore};
use crate::core::scenes::manager::{ApplySceneResult, SCENE_APPLY_BLOCKED_MESSAGE};

fn app_config_dir(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    app.path()
        .app_config_dir()
        .map_err(|error| error.to_string())
}

fn scene_store(app: &tauri::AppHandle) -> Result<SceneConfigStore, String> {
    let config_dir = app_config_dir(app)?;
    Ok(SceneConfigStore::new(config_dir))
}

#[tauri::command]
pub fn get_scene_config(app: tauri::AppHandle) -> Result<SceneConfigSnapshot, String> {
    scene_store(&app)?.load().map_err(|error| error.to_string())
}

#[tauri::command]
pub fn create_scene(
    app: tauri::AppHandle,
    id: String,
    name: String,
    description: String,
) -> Result<SceneConfigSnapshot, String> {
    scene_store(&app)?
        .create_scene(&id, &name, &description)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn update_scene(
    app: tauri::AppHandle,
    id: String,
    name: Option<String>,
    description: Option<String>,
) -> Result<SceneConfigSnapshot, String> {
    scene_store(&app)?
        .update_scene(&id, name.as_deref(), description.as_deref())
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn delete_scene(app: tauri::AppHandle, id: String) -> Result<SceneConfigSnapshot, String> {
    scene_store(&app)?
        .delete_scene(&id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_active_scene(
    app: tauri::AppHandle,
    id: Option<String>,
) -> Result<SceneConfigSnapshot, String> {
    scene_store(&app)?
        .set_active_scene(id.as_deref())
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_scene_skills(
    app: tauri::AppHandle,
    id: String,
    skill_ids: Vec<String>,
) -> Result<SceneConfigSnapshot, String> {
    scene_store(&app)?
        .set_scene_skills(&id, skill_ids)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_scene_agents(
    app: tauri::AppHandle,
    id: String,
    enabled_agent_keys: Vec<String>,
) -> Result<SceneConfigSnapshot, String> {
    scene_store(&app)?
        .set_scene_agents(&id, enabled_agent_keys)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_scene_skill_order(
    app: tauri::AppHandle,
    id: String,
    skill_order: Vec<String>,
) -> Result<SceneConfigSnapshot, String> {
    scene_store(&app)?
        .set_scene_skill_order(&id, skill_order)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn apply_scene(_app: tauri::AppHandle, _id: String) -> Result<ApplySceneResult, String> {
    Err(SCENE_APPLY_BLOCKED_MESSAGE.to_string())
}
