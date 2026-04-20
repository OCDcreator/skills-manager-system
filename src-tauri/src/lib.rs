mod commands;
mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_repo_path,
            commands::settings::set_repo_path,
            commands::skills::scan_skills,
            commands::skills::get_skill_document
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
