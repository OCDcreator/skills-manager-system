mod commands;
mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::agents::get_agent_inventory,
            commands::agents::set_agent_enabled,
            commands::agents::set_agent_path_override,
            commands::agents::clear_agent_path_override,
            commands::agents::apply_agent_sync,
            commands::git::git_status,
            commands::git::git_diff,
            commands::git::git_log,
            commands::git::git_pull,
            commands::git::git_push,
            commands::git::git_commit,
            commands::git::git_fetch,
            commands::git::run_sync_script,
            commands::scenes::get_scene_config,
            commands::scenes::create_scene,
            commands::scenes::update_scene,
            commands::scenes::delete_scene,
            commands::scenes::set_active_scene,
            commands::scenes::set_scene_skills,
            commands::scenes::set_scene_agents,
            commands::scenes::apply_scene,
            commands::settings::get_repo_path,
            commands::settings::set_repo_path,
            commands::skills::scan_skills,
            commands::skills::get_skill_document,
            commands::skills::get_skill_state,
            commands::skills::set_skill_enabled
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
