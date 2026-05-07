use serde::Serialize;
use serde_json::json;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::app_runtime::{AppRuntimeContext, CliCommandError, CliRunResult, CliWarning};
use crate::cli::args::SkillsCommand;
use crate::core::skills::documents::read_skill_document;
use crate::core::skills::scan::{
    scan_repo_skills_with_external_sources, ManagedSourceInfo, SkillSummary,
};
use crate::core::skills::state::SkillStateStore;

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct SkillListItem {
    id: String,
    name: String,
    description: String,
    source_type: String,
    relative_path: String,
    directory_path: String,
    skill_document_path: String,
    managed_source: Option<ManagedSourceInfo>,
    enabled: bool,
}

pub fn run(context: &AppRuntimeContext, command: &SkillsCommand) -> CliRunResult {
    match command {
        SkillsCommand::Scan => scan(context),
        SkillsCommand::State => state(context),
        SkillsCommand::List => list(context),
        SkillsCommand::Doc { target } => doc(context, target),
        SkillsCommand::Enable { skill_id } => {
            super::skill_mutations::set_enabled(context, skill_id, true)
        }
        SkillsCommand::Disable { skill_id } => {
            super::skill_mutations::set_enabled(context, skill_id, false)
        }
    }
}

fn scan(context: &AppRuntimeContext) -> CliRunResult {
    let repo_path = match require_repo_path("skills scan", context) {
        Ok(repo_path) => repo_path,
        Err(result) => return result,
    };

    match scan_repo_skills_with_external_sources(&repo_path, &context.config_dir) {
        Ok(response) => {
            let warnings = map_scan_warnings(&response.warnings);
            CliRunResult::ok_with_warnings(
                "skills scan",
                json!({ "skills": response.skills }),
                warnings,
                context,
                Some(repo_path.as_path()),
            )
        }
        Err(error) => CliRunResult::error(
            "skills scan",
            CliCommandError::filesystem(
                "skill_scan_failed",
                format!("Failed to scan skills: {error}"),
            ),
            context,
            Some(repo_path.as_path()),
        ),
    }
}

fn state(context: &AppRuntimeContext) -> CliRunResult {
    let repo_path = match require_repo_path("skills state", context) {
        Ok(repo_path) => repo_path,
        Err(result) => return result,
    };

    match SkillStateStore::new(context.config_dir.clone()).load_for_repo(&repo_path) {
        Ok(snapshot) => CliRunResult::success(
            "skills state",
            json!({ "state": snapshot }),
            context,
            Some(repo_path.as_path()),
        ),
        Err(error) => CliRunResult::error(
            "skills state",
            CliCommandError::filesystem(
                "skill_state_read_failed",
                format!("Failed to load skill state: {error}"),
            ),
            context,
            Some(repo_path.as_path()),
        ),
    }
}

fn list(context: &AppRuntimeContext) -> CliRunResult {
    let repo_path = match require_repo_path("skills list", context) {
        Ok(repo_path) => repo_path,
        Err(result) => return result,
    };

    let scan_response =
        match scan_repo_skills_with_external_sources(&repo_path, &context.config_dir) {
            Ok(response) => response,
            Err(error) => {
                return CliRunResult::error(
                    "skills list",
                    CliCommandError::filesystem(
                        "skill_scan_failed",
                        format!("Failed to scan skills: {error}"),
                    ),
                    context,
                    Some(repo_path.as_path()),
                );
            }
        };

    let state = match SkillStateStore::new(context.config_dir.clone()).load_for_repo(&repo_path) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            return CliRunResult::error(
                "skills list",
                CliCommandError::filesystem(
                    "skill_state_read_failed",
                    format!("Failed to load skill state: {error}"),
                ),
                context,
                Some(repo_path.as_path()),
            );
        }
    };

    let disabled_skill_ids = state
        .disabled_skill_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let skills = scan_response
        .skills
        .into_iter()
        .map(|summary| build_skill_list_item(summary, &disabled_skill_ids))
        .collect::<Vec<_>>();

    CliRunResult::ok_with_warnings(
        "skills list",
        json!({ "skills": skills }),
        map_scan_warnings(&scan_response.warnings),
        context,
        Some(repo_path.as_path()),
    )
}

fn doc(context: &AppRuntimeContext, target: &str) -> CliRunResult {
    let repo_path = match require_repo_path("skills doc", context) {
        Ok(repo_path) => repo_path,
        Err(result) => return result,
    };

    let relative_path = match resolve_document_target(&repo_path, &context.config_dir, target) {
        Ok(relative_path) => relative_path,
        Err(result) => return result,
    };

    let scan_response =
        match scan_repo_skills_with_external_sources(&repo_path, &context.config_dir) {
            Ok(response) => response,
            Err(error) => {
                return CliRunResult::error(
                    "skills doc",
                    CliCommandError::filesystem(
                        "skill_scan_failed",
                        format!("Failed to scan skills while resolving '{target}': {error}"),
                    ),
                    context,
                    Some(repo_path.as_path()),
                );
            }
        };
    let managed_source = scan_response
        .skills
        .iter()
        .find(|skill| skill.relative_path == relative_path)
        .and_then(|skill| skill.managed_source.clone());

    match read_skill_document(&repo_path, &relative_path) {
        Ok(mut document) => {
            document.managed_source = managed_source;
            CliRunResult::success(
                "skills doc",
                json!({ "document": document }),
                context,
                Some(repo_path.as_path()),
            )
        }
        Err(error) => CliRunResult::error(
            "skills doc",
            CliCommandError::target_not_found(
                "skill_not_found",
                format!("Failed to load skill document '{target}': {error}"),
                Some(json!({ "target": target })),
            ),
            context,
            Some(repo_path.as_path()),
        ),
    }
}

fn require_repo_path(command: &str, context: &AppRuntimeContext) -> Result<PathBuf, CliRunResult> {
    context.require_repo_path().map_err(|_| {
        CliRunResult::error(
            command,
            CliCommandError::missing_configuration(
                "repo_path_not_configured",
                "Repository path is not configured.",
            ),
            context,
            None,
        )
    })
}

fn resolve_document_target(
    repo_path: &Path,
    config_dir: &Path,
    target: &str,
) -> Result<String, CliRunResult> {
    if !target.contains(':') {
        return Ok(target.to_string());
    }

    let scan_response =
        scan_repo_skills_with_external_sources(repo_path, config_dir).map_err(|error| {
            CliRunResult::bootstrap_error(
                "skills doc",
                CliCommandError::filesystem(
                    "skill_scan_failed",
                    format!("Failed to scan skills while resolving '{target}': {error}"),
                ),
                None,
                Some(repo_path),
            )
        })?;

    scan_response
        .skills
        .into_iter()
        .find(|skill| skill.id == target)
        .map(|skill| skill.relative_path)
        .ok_or_else(|| {
            CliRunResult::bootstrap_error(
                "skills doc",
                CliCommandError::target_not_found(
                    "skill_not_found",
                    format!("Skill '{target}' does not exist."),
                    Some(json!({ "target": target })),
                ),
                None,
                Some(repo_path),
            )
        })
}

fn build_skill_list_item(
    summary: SkillSummary,
    disabled_skill_ids: &BTreeSet<String>,
) -> SkillListItem {
    let enabled = !disabled_skill_ids.contains(&summary.id);

    SkillListItem {
        id: summary.id,
        name: summary.name,
        description: summary.description,
        source_type: summary.source_type,
        relative_path: summary.relative_path,
        directory_path: summary.directory_path,
        skill_document_path: summary.skill_document_path,
        managed_source: summary.managed_source,
        enabled,
    }
}

fn map_scan_warnings(messages: &[String]) -> Vec<CliWarning> {
    messages
        .iter()
        .map(|message| {
            let target = if message.contains("custom/") {
                Some("custom".to_string())
            } else if message.contains("external/") {
                Some("external".to_string())
            } else {
                None
            };

            CliWarning::new("skill_source_missing", message.clone(), target)
        })
        .collect()
}
