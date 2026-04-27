use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum SceneSkillSelectionMode {
    #[default]
    AllExceptDisabled,
    OnlySelected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SceneEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub skill_selection_mode: SceneSkillSelectionMode,
    pub disabled_skill_ids: Vec<String>,
    #[serde(default)]
    pub selected_skill_ids: Vec<String>,
    pub enabled_agent_keys: Vec<String>,
    #[serde(default)]
    pub skill_order: Vec<String>,
}

impl SceneEntry {
    pub fn includes_skill(&self, skill_id: &str) -> bool {
        match self.skill_selection_mode {
            SceneSkillSelectionMode::AllExceptDisabled => {
                !self.disabled_skill_ids.iter().any(|id| id == skill_id)
            }
            SceneSkillSelectionMode::OnlySelected => {
                self.selected_skill_ids.iter().any(|id| id == skill_id)
            }
        }
    }

    pub fn disabled_skill_count<'a, I>(&self, skill_ids: I) -> usize
    where
        I: IntoIterator<Item = &'a str>,
    {
        skill_ids
            .into_iter()
            .filter(|skill_id| !self.includes_skill(skill_id))
            .count()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SceneConfigSnapshot {
    pub scenes: BTreeMap<String, SceneEntry>,
    pub active_scene_id: Option<String>,
}

impl Default for SceneConfigSnapshot {
    fn default() -> Self {
        Self {
            scenes: BTreeMap::new(),
            active_scene_id: None,
        }
    }
}

pub struct SceneConfigStore {
    base_dir: PathBuf,
}

impl SceneConfigStore {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn load(&self) -> Result<SceneConfigSnapshot> {
        let path = self.config_path();
        if !path.exists() {
            return Ok(SceneConfigSnapshot::default());
        }

        let raw =
            fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
        serde_json::from_str::<SceneConfigSnapshot>(&raw)
            .with_context(|| format!("Failed to parse {:?}", path))
    }

    pub fn create_scene(
        &self,
        id: &str,
        name: &str,
        description: &str,
    ) -> Result<SceneConfigSnapshot> {
        self.update(|snapshot| {
            let owned_id = id.trim().to_string();
            if owned_id.is_empty() {
                return Err(anyhow::anyhow!("Scene ID is required"));
            }
            if snapshot.scenes.contains_key(&owned_id) {
                return Err(anyhow::anyhow!("Scene '{}' already exists", owned_id));
            }

            snapshot.scenes.insert(
                owned_id.clone(),
                SceneEntry {
                    id: owned_id,
                    name: name.trim().to_string(),
                    description: description.trim().to_string(),
                    skill_selection_mode: SceneSkillSelectionMode::OnlySelected,
                    disabled_skill_ids: Vec::new(),
                    selected_skill_ids: Vec::new(),
                    enabled_agent_keys: Vec::new(),
                    skill_order: Vec::new(),
                },
            );
            Ok(())
        })
    }

    pub fn update_scene(
        &self,
        id: &str,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<SceneConfigSnapshot> {
        self.update(|snapshot| {
            let entry = snapshot
                .scenes
                .get_mut(id)
                .ok_or_else(|| anyhow::anyhow!("Scene '{}' not found", id))?;
            if let Some(name) = name {
                entry.name = name.trim().to_string();
            }
            if let Some(description) = description {
                entry.description = description.trim().to_string();
            }
            Ok(())
        })
    }

    pub fn delete_scene(&self, id: &str) -> Result<SceneConfigSnapshot> {
        self.update(|snapshot| {
            if snapshot.scenes.remove(id).is_none() {
                return Err(anyhow::anyhow!("Scene '{}' not found", id));
            }
            if snapshot.active_scene_id.as_deref() == Some(id) {
                snapshot.active_scene_id = None;
            }
            Ok(())
        })
    }

    pub fn set_active_scene(&self, id: Option<&str>) -> Result<SceneConfigSnapshot> {
        self.update(|snapshot| {
            if let Some(id) = id {
                if !snapshot.scenes.contains_key(id) {
                    return Err(anyhow::anyhow!("Scene '{}' not found", id));
                }
            }
            snapshot.active_scene_id = id.map(|s| s.to_string());
            Ok(())
        })
    }

    pub fn set_scene_skills(
        &self,
        id: &str,
        skill_ids: Vec<String>,
    ) -> Result<SceneConfigSnapshot> {
        self.update(|snapshot| {
            let entry = snapshot
                .scenes
                .get_mut(id)
                .ok_or_else(|| anyhow::anyhow!("Scene '{}' not found", id))?;
            match entry.skill_selection_mode {
                SceneSkillSelectionMode::AllExceptDisabled => {
                    entry.disabled_skill_ids = normalize_skill_ids(skill_ids);
                    entry.selected_skill_ids.clear();
                    entry
                        .skill_order
                        .retain(|skill_id| !entry.disabled_skill_ids.contains(skill_id));
                }
                SceneSkillSelectionMode::OnlySelected => {
                    entry.selected_skill_ids = normalize_skill_ids(skill_ids);
                    entry.disabled_skill_ids.clear();
                    entry
                        .skill_order
                        .retain(|skill_id| entry.selected_skill_ids.contains(skill_id));
                }
            }
            Ok(())
        })
    }

    pub fn set_scene_agents(
        &self,
        id: &str,
        enabled_agent_keys: Vec<String>,
    ) -> Result<SceneConfigSnapshot> {
        self.update(|snapshot| {
            let entry = snapshot
                .scenes
                .get_mut(id)
                .ok_or_else(|| anyhow::anyhow!("Scene '{}' not found", id))?;
            entry.enabled_agent_keys = enabled_agent_keys;
            Ok(())
        })
    }

    pub fn set_scene_skill_order(
        &self,
        id: &str,
        skill_order: Vec<String>,
    ) -> Result<SceneConfigSnapshot> {
        self.update(|snapshot| {
            let entry = snapshot
                .scenes
                .get_mut(id)
                .ok_or_else(|| anyhow::anyhow!("Scene '{}' not found", id))?;
            entry.skill_order = skill_order;
            Ok(())
        })
    }

    fn update<F>(&self, mutator: F) -> Result<SceneConfigSnapshot>
    where
        F: FnOnce(&mut SceneConfigSnapshot) -> Result<()>,
    {
        let mut snapshot = self.load()?;
        mutator(&mut snapshot)?;
        self.save(&snapshot)?;
        Ok(snapshot)
    }

    fn save(&self, snapshot: &SceneConfigSnapshot) -> Result<()> {
        fs::create_dir_all(&self.base_dir)
            .with_context(|| format!("Failed to create {:?}", self.base_dir))?;
        let json = serde_json::to_string_pretty(snapshot)?;
        fs::write(self.config_path(), json).context("Failed to write scene-config.json")?;
        Ok(())
    }

    fn config_path(&self) -> PathBuf {
        self.base_dir.join("scene-config.json")
    }
}

fn normalize_skill_ids(ids: Vec<String>) -> Vec<String> {
    ids.into_iter()
        .map(|id| id.trim().to_string())
        .filter(|id| !id.is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn load_returns_defaults_when_missing() {
        let dir = tempdir().unwrap();
        let store = SceneConfigStore::new(dir.path().to_path_buf());
        let snapshot = store.load().unwrap();
        assert!(snapshot.scenes.is_empty());
        assert_eq!(snapshot.active_scene_id, None);
    }

    #[test]
    fn create_scene_round_trips() {
        let dir = tempdir().unwrap();
        let store = SceneConfigStore::new(dir.path().to_path_buf());
        let snapshot = store.create_scene("work", "Work", "Work scene").unwrap();

        assert!(snapshot.scenes.contains_key("work"));
        assert_eq!(snapshot.scenes["work"].name, "Work");
        assert_eq!(snapshot.scenes["work"].selected_skill_ids, Vec::<String>::new());
        assert_eq!(
            snapshot.scenes["work"].skill_selection_mode,
            SceneSkillSelectionMode::OnlySelected
        );
    }

    #[test]
    fn create_scene_rejects_duplicate() {
        let dir = tempdir().unwrap();
        let store = SceneConfigStore::new(dir.path().to_path_buf());
        store.create_scene("work", "Work", "").unwrap();
        assert!(store.create_scene("work", "Work 2", "").is_err());
    }

    #[test]
    fn delete_scene_clears_active() {
        let dir = tempdir().unwrap();
        let store = SceneConfigStore::new(dir.path().to_path_buf());
        store.create_scene("work", "Work", "").unwrap();
        store.set_active_scene(Some("work")).unwrap();
        let snapshot = store.delete_scene("work").unwrap();
        assert_eq!(snapshot.active_scene_id, None);
    }

    #[test]
    fn set_scene_skills_persists() {
        let dir = tempdir().unwrap();
        let store = SceneConfigStore::new(dir.path().to_path_buf());
        store.create_scene("work", "Work", "").unwrap();
        let snapshot = store
            .set_scene_skills("work", vec!["external:foo".to_string()])
            .unwrap();
        assert_eq!(
            snapshot.scenes["work"].selected_skill_ids,
            vec!["external:foo"]
        );
        assert!(snapshot.scenes["work"].disabled_skill_ids.is_empty());
    }

    #[test]
    fn legacy_scene_without_selection_mode_stays_all_except_disabled() {
        let dir = tempdir().unwrap();
        let store = SceneConfigStore::new(dir.path().to_path_buf());
        fs::create_dir_all(dir.path()).unwrap();
        fs::write(
            dir.path().join("scene-config.json"),
            r#"{
  "scenes": {
    "legacy": {
      "id": "legacy",
      "name": "Legacy",
      "description": "",
      "disabledSkillIds": ["custom:beta"],
      "enabledAgentKeys": [],
      "skillOrder": []
    }
  },
  "activeSceneId": null
}"#,
        )
        .unwrap();

        let snapshot = store.load().unwrap();
        assert_eq!(
            snapshot.scenes["legacy"].skill_selection_mode,
            SceneSkillSelectionMode::AllExceptDisabled
        );
    }
}
