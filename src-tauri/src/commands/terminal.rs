use std::path::Path;

use tauri::{Manager, State};

use crate::core::terminal::{
    ensure_terminal_workspace, load_terminal_launcher_preferences, TerminalDrainResponse,
    TerminalLaunchInput, TerminalLauncherPreferences, TerminalSessionSnapshot,
};
use crate::core::terminal::session::TerminalState;
use crate::core::settings::SettingsStore;

fn app_config_dir(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    app.path()
        .app_config_dir()
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_terminal_launcher_preferences(
    app: tauri::AppHandle,
) -> Result<TerminalLauncherPreferences, String> {
    let config_dir = app_config_dir(&app)?;
    load_terminal_launcher_preferences(&config_dir).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_terminal_working_directory_preference(
    app: tauri::AppHandle,
    path: String,
) -> Result<Option<String>, String> {
    let config_dir = app_config_dir(&app)?;
    ensure_terminal_workspace(&config_dir).map_err(|error| error.to_string())?;

    let trimmed = path.trim();
    let normalized = if trimmed.is_empty() {
        None
    } else {
        Some(Path::new(trimmed))
    };

    SettingsStore::new(config_dir)
        .save_assistant_working_directory(normalized)
        .map(|settings| settings.assistant_working_directory)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_terminal_session(
    state: State<'_, TerminalState>,
) -> Result<Option<TerminalSessionSnapshot>, String> {
    state.snapshot().map_err(|error| error.to_string())
}

#[tauri::command]
pub fn start_terminal_session(
    state: State<'_, TerminalState>,
    input: TerminalLaunchInput,
) -> Result<TerminalSessionSnapshot, String> {
    state.start(input).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn drain_terminal_output(
    state: State<'_, TerminalState>,
) -> Result<TerminalDrainResponse, String> {
    state.drain().map_err(|error| error.to_string())
}

#[tauri::command]
pub fn write_terminal_input(
    state: State<'_, TerminalState>,
    input: String,
) -> Result<(), String> {
    state.write_input(&input).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn resize_terminal_session(
    state: State<'_, TerminalState>,
    cols: u16,
    rows: u16,
) -> Result<Option<TerminalSessionSnapshot>, String> {
    state.resize(cols, rows).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn stop_terminal_session(
    state: State<'_, TerminalState>,
) -> Result<Option<TerminalSessionSnapshot>, String> {
    state.stop().map_err(|error| error.to_string())
}
