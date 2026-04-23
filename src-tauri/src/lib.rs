pub mod app_runtime;
#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "desktop")]
mod commands;
pub mod core;

#[cfg(feature = "desktop")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::agent_targets::take_over_agent_target_skill,
            commands::agent_targets::delete_agent_target_skill,
            commands::agent_targets::import_agent_target_skill,
            commands::agents::get_agent_inventory,
            commands::agents::set_agent_enabled,
            commands::agents::set_agent_path_override,
            commands::agents::clear_agent_path_override,
            commands::agents::set_agent_configuration,
            commands::agents::apply_agent_sync,
            commands::git::git_status,
            commands::git::git_diff,
            commands::git::git_log,
            commands::git::git_pull,
            commands::git::git_push,
            commands::git::git_commit,
            commands::git::git_fetch,
            commands::projects::get_project_config,
            commands::projects::add_project,
            commands::projects::update_project,
            commands::projects::remove_project,
            commands::projects::apply_project_assignments,
            commands::git::run_sync_script,
            commands::scenes::get_scene_config,
            commands::scenes::create_scene,
            commands::scenes::update_scene,
            commands::scenes::delete_scene,
            commands::scenes::set_active_scene,
            commands::scenes::set_scene_skills,
            commands::scenes::set_scene_agents,
            commands::scenes::set_scene_skill_order,
            commands::scenes::apply_scene,
            commands::settings::get_repo_path,
            commands::settings::set_repo_path,
            commands::settings::get_agent_sync_mode,
            commands::settings::set_agent_sync_mode,
            commands::skills::scan_skills,
            commands::skills::get_skill_document,
            commands::skills::get_skill_state,
            commands::skills::set_skill_enabled
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
