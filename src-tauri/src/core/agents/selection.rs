use anyhow::Result;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::core::scenes::config::{SceneConfigStore, SceneEntry};
use crate::core::skills::scan::{scan_repo_skills, SkillSummary};
use crate::core::skills::state::SkillStateStore;

use super::discovery::AgentInventoryItem;

pub(crate) struct SkillSelectionContext {
    skills: Vec<SkillSummary>,
    globally_disabled_skill_ids: BTreeSet<String>,
    scenes: BTreeMap<String, SceneEntry>,
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
    let scenes = SceneConfigStore::new(config_dir.to_path_buf()).load()?.scenes;

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
    let mut selected_skill_ids = BTreeSet::new();

    for skill_id in &agent.selected_skill_ids {
        if is_globally_available(skill_id, context) {
            selected_skill_ids.insert(skill_id.clone());
        }
    }

    for scene_id in &agent.selected_scene_ids {
        if let Some(scene) = context.scenes.get(scene_id) {
            add_scene_skill_ids(scene, context, &mut selected_skill_ids);
        }
    }

    for skill_id in &agent.excluded_skill_ids {
        selected_skill_ids.remove(skill_id);
    }

    context
        .skills
        .iter()
        .filter(|skill| selected_skill_ids.contains(&skill.id))
        .cloned()
        .collect()
}

fn add_scene_skill_ids(
    scene: &SceneEntry,
    context: &SkillSelectionContext,
    selected_skill_ids: &mut BTreeSet<String>,
) {
    let scene_disabled_skill_ids = scene
        .disabled_skill_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();

    for skill in &context.skills {
        if !context.globally_disabled_skill_ids.contains(&skill.id)
            && !scene_disabled_skill_ids.contains(skill.id.as_str())
        {
            selected_skill_ids.insert(skill.id.clone());
        }
    }
}

fn is_globally_available(skill_id: &str, context: &SkillSelectionContext) -> bool {
    !context.globally_disabled_skill_ids.contains(skill_id)
        && context.skills.iter().any(|skill| skill.id == skill_id)
}
