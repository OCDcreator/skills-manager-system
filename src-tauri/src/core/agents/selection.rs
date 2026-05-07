use anyhow::Result;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::discovery::AgentInventoryItem;
use crate::core::projects::store::ProjectAgentAssignment;
use crate::core::scenes::config::{SceneConfigStore, SceneEntry, SceneSkillSelectionMode};
use crate::core::skills::scan::{scan_repo_skills, SkillSummary};
use crate::core::skills::state::SkillStateStore;

pub(crate) struct SkillSelectionContext {
    skills: Vec<SkillSummary>,
    globally_disabled_skill_ids: BTreeSet<String>,
    scenes: BTreeMap<String, SceneEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum SkillSourceLabel {
    GlobalDirect,
    GlobalScene {
        scene_id: String,
        scene_name: String,
    },
    ProjectDirect,
    ProjectScene {
        scene_id: String,
        scene_name: String,
    },
}

impl SkillSourceLabel {
    #[cfg(test)]
    fn stable_label(&self) -> &'static str {
        match self {
            Self::GlobalDirect => "globalDirect",
            Self::GlobalScene { .. } => "globalScene",
            Self::ProjectDirect => "projectDirect",
            Self::ProjectScene { .. } => "projectScene",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedSkillEntry {
    pub skill: SkillSummary,
    pub sources: Vec<SkillSourceLabel>,
    pub excluded: bool,
    pub globally_disabled: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SkillResolutionDiagnostics {
    pub missing_scene_ids: Vec<String>,
    pub missing_skill_ids: Vec<String>,
    pub globally_disabled_references: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SkillResolutionResult {
    pub entries: Vec<ResolvedSkillEntry>,
    pub diagnostics: SkillResolutionDiagnostics,
}

impl SkillResolutionResult {
    #[cfg(test)]
    pub(crate) fn source_labels_for(&self, skill_id: &str) -> Vec<&'static str> {
        self.entries
            .iter()
            .find(|entry| entry.skill.id == skill_id)
            .map(|entry| {
                entry
                    .sources
                    .iter()
                    .map(SkillSourceLabel::stable_label)
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl SkillSelectionContext {
    pub(crate) fn available_skill_count(&self) -> usize {
        self.skills
            .iter()
            .filter(|skill| !self.globally_disabled_skill_ids.contains(&skill.id))
            .count()
    }
}

pub(crate) fn load_skill_selection_context(
    config_dir: &Path,
    repo_path: &Path,
) -> Result<SkillSelectionContext> {
    let scan_result = scan_repo_skills(repo_path)?;
    let disabled_skill_ids = SkillStateStore::new(config_dir.to_path_buf())
        .load_for_repo(repo_path)?
        .disabled_skill_ids
        .into_iter()
        .collect::<BTreeSet<_>>();
    let scenes = SceneConfigStore::new(config_dir.to_path_buf())
        .load()?
        .scenes;

    Ok(SkillSelectionContext {
        skills: scan_result.skills,
        globally_disabled_skill_ids: disabled_skill_ids,
        scenes,
    })
}

pub(crate) fn resolve_agent_skills(
    agent: &AgentInventoryItem,
    context: &SkillSelectionContext,
) -> Vec<SkillSummary> {
    resolve_agent_skill_selection(agent, context)
        .entries
        .into_iter()
        .filter(|entry| !entry.excluded && !entry.globally_disabled)
        .map(|entry| entry.skill)
        .collect()
}

pub(crate) fn resolve_agent_skill_selection(
    agent: &AgentInventoryItem,
    context: &SkillSelectionContext,
) -> SkillResolutionResult {
    let skill_lookup = skill_lookup(context);
    let excluded_skill_ids = id_set(&agent.excluded_skill_ids);
    let mut entries = BTreeMap::new();
    let mut diagnostics = SkillResolutionDiagnostics::default();

    for skill_id in &agent.selected_skill_ids {
        add_skill_reference(
            skill_id,
            SkillSourceLabel::GlobalDirect,
            context,
            &skill_lookup,
            &excluded_skill_ids,
            &mut entries,
            &mut diagnostics,
        );
    }

    for scene_id in &agent.selected_scene_ids {
        if let Some(scene) = context.scenes.get(scene_id) {
            add_scene_skill_references(
                scene,
                SkillSourceLabel::GlobalScene {
                    scene_id: scene.id.clone(),
                    scene_name: scene.name.clone(),
                },
                context,
                &skill_lookup,
                &excluded_skill_ids,
                &mut entries,
                &mut diagnostics,
            );
        } else {
            diagnostics.missing_scene_ids.push(scene_id.clone());
        }
    }

    add_missing_exclusion_diagnostics(&agent.excluded_skill_ids, &skill_lookup, &mut diagnostics);
    ordered_result(context, entries, diagnostics)
}

pub(crate) fn resolve_project_agent_skill_selection(
    global_result: &SkillResolutionResult,
    project_agent: &ProjectAgentAssignment,
    context: &SkillSelectionContext,
) -> SkillResolutionResult {
    let skill_lookup = skill_lookup(context);
    let excluded_skill_ids = id_set(&project_agent.excluded_skill_ids);
    let mut diagnostics = global_result.diagnostics.clone();
    let mut entries = BTreeMap::new();

    for entry in &global_result.entries {
        if entry.excluded || entry.globally_disabled {
            continue;
        }

        let mut layered_entry = entry.clone();
        layered_entry.excluded = excluded_skill_ids.contains(&entry.skill.id);
        entries.insert(layered_entry.skill.id.clone(), layered_entry);
    }

    for skill_id in &project_agent.selected_skill_ids {
        add_skill_reference(
            skill_id,
            SkillSourceLabel::ProjectDirect,
            context,
            &skill_lookup,
            &excluded_skill_ids,
            &mut entries,
            &mut diagnostics,
        );
    }

    for scene_id in &project_agent.selected_scene_ids {
        if let Some(scene) = context.scenes.get(scene_id) {
            add_scene_skill_references(
                scene,
                SkillSourceLabel::ProjectScene {
                    scene_id: scene.id.clone(),
                    scene_name: scene.name.clone(),
                },
                context,
                &skill_lookup,
                &excluded_skill_ids,
                &mut entries,
                &mut diagnostics,
            );
        } else {
            diagnostics.missing_scene_ids.push(scene_id.clone());
        }
    }

    add_missing_exclusion_diagnostics(
        &project_agent.excluded_skill_ids,
        &skill_lookup,
        &mut diagnostics,
    );
    ordered_result(context, entries, diagnostics)
}

fn add_scene_skill_references(
    scene: &SceneEntry,
    source: SkillSourceLabel,
    context: &SkillSelectionContext,
    skill_lookup: &BTreeMap<&str, &SkillSummary>,
    excluded_skill_ids: &BTreeSet<String>,
    entries: &mut BTreeMap<String, ResolvedSkillEntry>,
    diagnostics: &mut SkillResolutionDiagnostics,
) {
    for skill in &context.skills {
        if scene.includes_skill(&skill.id) {
            add_skill_reference(
                &skill.id,
                source.clone(),
                context,
                skill_lookup,
                excluded_skill_ids,
                entries,
                diagnostics,
            );
        }
    }

    let stale_reference_ids = match scene.skill_selection_mode {
        SceneSkillSelectionMode::AllExceptDisabled => &scene.disabled_skill_ids,
        SceneSkillSelectionMode::OnlySelected => &scene.selected_skill_ids,
    };

    for skill_id in stale_reference_ids {
        if !skill_lookup.contains_key(skill_id.as_str()) {
            diagnostics.missing_skill_ids.push(skill_id.clone());
        }
    }
}

fn add_skill_reference(
    skill_id: &str,
    source: SkillSourceLabel,
    context: &SkillSelectionContext,
    skill_lookup: &BTreeMap<&str, &SkillSummary>,
    excluded_skill_ids: &BTreeSet<String>,
    entries: &mut BTreeMap<String, ResolvedSkillEntry>,
    diagnostics: &mut SkillResolutionDiagnostics,
) {
    let Some(skill) = skill_lookup.get(skill_id) else {
        diagnostics.missing_skill_ids.push(skill_id.to_string());
        return;
    };

    if context.globally_disabled_skill_ids.contains(skill_id) {
        diagnostics
            .globally_disabled_references
            .push(skill_id.to_string());
    }

    let entry = entries
        .entry(skill_id.to_string())
        .or_insert_with(|| ResolvedSkillEntry {
            skill: (*skill).clone(),
            sources: Vec::new(),
            excluded: excluded_skill_ids.contains(skill_id),
            globally_disabled: context.globally_disabled_skill_ids.contains(skill_id),
        });

    if !entry.sources.contains(&source) {
        entry.sources.push(source);
    }
}

fn normalize_diagnostics(diagnostics: &mut SkillResolutionDiagnostics) {
    diagnostics.missing_scene_ids.sort();
    diagnostics.missing_scene_ids.dedup();
    diagnostics.missing_skill_ids.sort();
    diagnostics.missing_skill_ids.dedup();
    diagnostics.globally_disabled_references.sort();
    diagnostics.globally_disabled_references.dedup();
}

fn skill_lookup(context: &SkillSelectionContext) -> BTreeMap<&str, &SkillSummary> {
    context
        .skills
        .iter()
        .map(|skill| (skill.id.as_str(), skill))
        .collect()
}

fn id_set(ids: &[String]) -> BTreeSet<String> {
    ids.iter().cloned().collect()
}

fn add_missing_exclusion_diagnostics(
    excluded_ids: &[String],
    skill_lookup: &BTreeMap<&str, &SkillSummary>,
    diagnostics: &mut SkillResolutionDiagnostics,
) {
    diagnostics.missing_skill_ids.extend(
        excluded_ids
            .iter()
            .filter(|skill_id| !skill_lookup.contains_key(skill_id.as_str()))
            .cloned(),
    );
}

fn ordered_result(
    context: &SkillSelectionContext,
    mut entries: BTreeMap<String, ResolvedSkillEntry>,
    mut diagnostics: SkillResolutionDiagnostics,
) -> SkillResolutionResult {
    normalize_diagnostics(&mut diagnostics);
    let entries = context
        .skills
        .iter()
        .filter_map(|skill| entries.remove(&skill.id))
        .collect();
    SkillResolutionResult {
        entries,
        diagnostics,
    }
}
