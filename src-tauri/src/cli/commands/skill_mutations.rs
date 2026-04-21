use serde_json::json;
use std::path::Path;

use crate::app_runtime::{AppRuntimeContext, CliCommandError, CliRunResult};
use crate::core::skills::scan::scan_repo_skills;
use crate::core::skills::state::SkillStateStore;

pub(super) fn set_enabled(
    context: &AppRuntimeContext,
    skill_id: &str,
    enabled: bool,
) -> CliRunResult {
    let command = if enabled {
        "skills enable"
    } else {
        "skills disable"
    };
    let repo_path = match context.require_repo_path() {
        Ok(repo_path) => repo_path,
        Err(_) => {
            return CliRunResult::error(
                command,
                CliCommandError::missing_configuration(
                    "repo_path_not_configured",
                    "Repository path is not configured.",
                ),
                context,
                None,
            );
        }
    };

    if let Err(result) = ensure_skill_exists(command, &repo_path, skill_id) {
        return result;
    }

    let _guard = match context.acquire_config_lock() {
        Ok(guard) => guard,
        Err(error) => {
            return CliRunResult::error(
                command,
                CliCommandError::from_config_lock(error),
                context,
                Some(repo_path.as_path()),
            );
        }
    };

    match SkillStateStore::new(context.config_dir.clone())
        .set_skill_enabled(&repo_path, skill_id, enabled)
    {
        Ok(snapshot) => CliRunResult::success(
            command,
            json!({ "state": snapshot }),
            context,
            Some(repo_path.as_path()),
        ),
        Err(error) => CliRunResult::error(
            command,
            CliCommandError::config_write_failed(
                format!("Failed to update skill state: {error}"),
                Some(json!({ "skillId": skill_id })),
            ),
            context,
            Some(repo_path.as_path()),
        ),
    }
}

fn ensure_skill_exists(
    command: &str,
    repo_path: &Path,
    skill_id: &str,
) -> Result<(), CliRunResult> {
    match scan_repo_skills(repo_path) {
        Ok(response) if response.skills.iter().any(|skill| skill.id == skill_id) => Ok(()),
        Ok(_) => Err(CliRunResult::bootstrap_error(
            command,
            CliCommandError::target_not_found(
                "skill_not_found",
                format!("Skill '{skill_id}' does not exist."),
                Some(json!({ "skillId": skill_id })),
            ),
            None,
            Some(repo_path),
        )),
        Err(error) => Err(CliRunResult::bootstrap_error(
            command,
            CliCommandError::filesystem(
                "skill_scan_failed",
                format!("Failed to scan skills: {error}"),
            ),
            None,
            Some(repo_path),
        )),
    }
}
