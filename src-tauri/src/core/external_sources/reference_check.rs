use anyhow::Result;
use serde::Serialize;
use std::path::Path;

use crate::core::agents::config::AgentConfigStore;
use crate::core::projects::store::ProjectConfigStore;
use crate::core::scenes::config::SceneConfigStore;

#[derive(Debug, Default, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlockingReferences {
    pub scenes: Vec<String>,
    pub agents: Vec<String>,
    pub projects: Vec<String>,
}

impl BlockingReferences {
    pub fn has_any(&self) -> bool {
        !self.scenes.is_empty() || !self.agents.is_empty() || !self.projects.is_empty()
    }
}

pub fn find_skill_references(config_dir: &Path, skill_id: &str) -> Result<BlockingReferences> {
    let mut references = BlockingReferences::default();

    let scene_snapshot = SceneConfigStore::new(config_dir.to_path_buf()).load()?;
    for scene in scene_snapshot.scenes.values() {
        if contains(&scene.selected_skill_ids, skill_id)
            || contains(&scene.disabled_skill_ids, skill_id)
            || contains(&scene.skill_order, skill_id)
        {
            references.scenes.push(scene.id.clone());
        }
    }

    let agent_snapshot = AgentConfigStore::new(config_dir.to_path_buf()).load()?;
    for (agent_key, entry) in agent_snapshot.agents {
        if contains(&entry.selected_skill_ids, skill_id)
            || contains(&entry.excluded_skill_ids, skill_id)
        {
            references.agents.push(agent_key);
        }
    }

    let project_snapshot = ProjectConfigStore::new(config_dir.to_path_buf()).load()?;
    for project in project_snapshot.projects.values() {
        if contains(&project.skill_ids, skill_id) {
            references.projects.push(project.project_path.clone());
        }
    }

    Ok(references)
}

fn contains(values: &[String], needle: &str) -> bool {
    values.iter().any(|value| value == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocking_references_has_any_only_when_any_bucket_is_non_empty() {
        assert!(!BlockingReferences::default().has_any());
        assert!(BlockingReferences {
            scenes: vec!["focus".to_string()],
            agents: Vec::new(),
            projects: Vec::new(),
        }
        .has_any());
    }
}
