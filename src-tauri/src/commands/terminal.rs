use tauri::State;

use crate::core::terminal::{
    TerminalDrainResponse, TerminalLaunchInput, TerminalSessionSnapshot,
};
use crate::core::terminal::session::TerminalState;

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
