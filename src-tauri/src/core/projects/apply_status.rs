use serde_json::json;
use std::collections::BTreeMap;
use std::path::Path;

use crate::core::agents::catalog::{find_agent, project_skills_dir_rule};
use crate::core::agents::discovery::AgentInventoryItem;
use crate::core::agents::selection::{
    resolve_agent_skill_selection, resolve_project_agent_skill_selection, SkillResolutionResult,
    SkillSelectionContext, SkillSourceLabel,
};
use crate::core::agents::target_sync::SyncMode;
use crate::core::platform_paths::portable_path_string;

use super::store::{
    ProjectAgentApplyStatus, ProjectAgentAssignment, ProjectApplyFreshness, ProjectAssignment,
};
use super::sync_ledger::{
    ledger_key, normalize_legacy_target_for_comparison, ProjectSyncLedger,
};

pub(super) fn project_apply_statuses(
    project: &ProjectAssignment,
    agents: &BTreeMap<String, AgentInventoryItem>,
    skill_context: &SkillSelectionContext,
    sync_mode: SyncMode,
    ledger: &ProjectSyncLedger,
) -> BTreeMap<String, ProjectAgentApplyStatus> {
    let mut statuses = BTreeMap::new();

    for agent_key in &project.agent_keys {
        statuses.insert(
            agent_key.clone(),
            project_agent_apply_status(project, agent_key, agents, skill_context, sync_mode, ledger),
        );
    }

    for agent_key in &project.unsupported_agent_keys {
        statuses.entry(agent_key.clone()).or_insert(ProjectAgentApplyStatus {
            apply_status: ProjectApplyFreshness::Unsupported,
            last_applied_at: None,
        });
    }

    statuses
}

fn project_agent_apply_status(
    project: &ProjectAssignment,
    agent_key: &str,
    agents: &BTreeMap<String, AgentInventoryItem>,
    skill_context: &SkillSelectionContext,
    sync_mode: SyncMode,
    ledger: &ProjectSyncLedger,
) -> ProjectAgentApplyStatus {
    let Some(agent) = find_agent(agent_key) else {
        return unsupported_status();
    };
    let Some(global_agent) = agents.get(agent_key) else {
        return unsupported_status();
    };

    let default_project_agent;
    let project_agent = match project.agents.get(agent_key) {
        Some(project_agent) => project_agent,
        None => {
            default_project_agent = ProjectAgentAssignment::default();
            &default_project_agent
        }
    };
    let global_result = resolve_agent_skill_selection(global_agent, skill_context);
    let project_result =
        resolve_project_agent_skill_selection(&global_result, project_agent, skill_context);
    let target_dir = Path::new(&project.project_path).join(project_skills_dir_rule(agent));
    let resolution_hash = project_resolution_hash(
        project,
        agent_key,
        project_agent,
        &project_result,
        project_skills_dir_rule(agent),
        sync_mode,
    );

    match ledger.assignments.get(&ledger_key(&project.project_path, agent_key)) {
        None => ProjectAgentApplyStatus {
            apply_status: ProjectApplyFreshness::NeverApplied,
            last_applied_at: None,
        },
        Some(entry)
            if normalize_legacy_target_for_comparison(&entry.target_dir)
                == portable_path_string(&target_dir)
                && entry.resolution_hash == resolution_hash =>
        {
            ProjectAgentApplyStatus {
                apply_status: ProjectApplyFreshness::Current,
                last_applied_at: Some(entry.applied_at),
            }
        }
        Some(entry) => ProjectAgentApplyStatus {
            apply_status: ProjectApplyFreshness::Stale,
            last_applied_at: if entry.applied_at == 0 { None } else { Some(entry.applied_at) },
        },
    }
}

pub(super) fn project_resolution_hash(
    project: &ProjectAssignment,
    agent_key: &str,
    project_agent: &ProjectAgentAssignment,
    project_result: &SkillResolutionResult,
    target_rule: &str,
    sync_mode: SyncMode,
) -> String {
    let resolved = project_result
        .entries
        .iter()
        .map(|entry| {
            json!({
                "skillId": entry.skill.id,
                "sources": entry.sources.iter().map(source_label).collect::<Vec<_>>(),
                "excluded": entry.excluded,
                "globallyDisabled": entry.globally_disabled,
            })
        })
        .collect::<Vec<_>>();
    let payload = json!({
        "projectPath": project.project_path,
        "agentKey": agent_key,
        "resolved": resolved,
        "exclusionIds": project_agent.excluded_skill_ids,
        "targetDir": portable_path_string(&Path::new(&project.project_path).join(target_rule)),
        "syncMode": sync_mode_label(sync_mode),
    });
    stable_hash(serde_json::to_string(&payload).unwrap_or_default().as_bytes())
}

fn unsupported_status() -> ProjectAgentApplyStatus {
    ProjectAgentApplyStatus {
        apply_status: ProjectApplyFreshness::Unsupported,
        last_applied_at: None,
    }
}

fn source_label(source: &SkillSourceLabel) -> String {
    match source {
        SkillSourceLabel::GlobalDirect => "globalDirect".to_string(),
        SkillSourceLabel::GlobalScene { scene_id, .. } => format!("globalScene:{scene_id}"),
        SkillSourceLabel::ProjectDirect => "projectDirect".to_string(),
        SkillSourceLabel::ProjectScene { scene_id, .. } => format!("projectScene:{scene_id}"),
    }
}

fn sync_mode_label(sync_mode: SyncMode) -> &'static str {
    match sync_mode {
        SyncMode::Copy => "copy",
        SyncMode::Symlink => "symlink",
    }
}

fn stable_hash(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}
