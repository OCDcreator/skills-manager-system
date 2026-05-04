use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::project_paths::normalize_project_path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectAssignment {
    pub project_path: String,
    pub display_name: String,
    pub skill_ids: Vec<String>,
    pub agent_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfigSnapshot {
    pub projects: BTreeMap<String, ProjectAssignment>,
}

pub struct ProjectConfigStore {
    base_dir: PathBuf,
}

impl ProjectConfigStore {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn load(&self) -> Result<ProjectConfigSnapshot> {
        let path = self.config_path();
        if !path.exists() {
            return Ok(ProjectConfigSnapshot::default());
        }

        let raw =
            fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
        let snapshot = serde_json::from_str::<ProjectConfigSnapshot>(&raw)
            .with_context(|| format!("Failed to parse {:?}", path))?;
        normalize_snapshot(snapshot)
    }

    pub fn add_project(
        &self,
        project_path: &str,
        display_name: &str,
        skill_ids: Vec<String>,
        agent_keys: Vec<String>,
    ) -> Result<ProjectConfigSnapshot> {
        self.update(|snapshot| {
            let project_path = normalize_project_path(project_path)?;
            if snapshot.projects.contains_key(&project_path) {
                return Err(anyhow!("Project '{}' already exists", project_path));
            }

            snapshot.projects.insert(
                project_path.clone(),
                ProjectAssignment {
                    display_name: normalize_display_name(&project_path, display_name),
                    project_path: project_path.clone(),
                    skill_ids,
                    agent_keys,
                },
            );
            Ok(())
        })
    }

    pub fn update_project(
        &self,
        project_path: &str,
        display_name: Option<&str>,
        skill_ids: Option<Vec<String>>,
        agent_keys: Option<Vec<String>>,
    ) -> Result<ProjectConfigSnapshot> {
        self.update(|snapshot| {
            let project_path = normalize_project_path(project_path)?;
            let entry = snapshot
                .projects
                .get_mut(&project_path)
                .ok_or_else(|| anyhow!("Project '{}' not found", project_path))?;

            if let Some(display_name) = display_name {
                entry.display_name = normalize_display_name(&project_path, display_name);
            }
            if let Some(skill_ids) = skill_ids {
                entry.skill_ids = skill_ids;
            }
            if let Some(agent_keys) = agent_keys {
                entry.agent_keys = agent_keys;
            }

            Ok(())
        })
    }

    pub fn remove_project(&self, project_path: &str) -> Result<ProjectConfigSnapshot> {
        self.update(|snapshot| {
            let project_path = normalize_project_path(project_path)?;
            if snapshot.projects.remove(&project_path).is_none() {
                return Err(anyhow!("Project '{}' not found", project_path));
            }
            Ok(())
        })
    }

    fn update<F>(&self, mutator: F) -> Result<ProjectConfigSnapshot>
    where
        F: FnOnce(&mut ProjectConfigSnapshot) -> Result<()>,
    {
        let mut snapshot = self.load()?;
        mutator(&mut snapshot)?;
        self.save(&snapshot)?;
        Ok(snapshot)
    }

    fn save(&self, snapshot: &ProjectConfigSnapshot) -> Result<()> {
        fs::create_dir_all(&self.base_dir)
            .with_context(|| format!("Failed to create {:?}", self.base_dir))?;
        let json = serde_json::to_string_pretty(snapshot)?;
        fs::write(self.config_path(), json).context("Failed to write project-config.json")?;
        Ok(())
    }

    fn config_path(&self) -> PathBuf {
        self.base_dir.join("project-config.json")
    }
}

fn normalize_display_name(project_path: &str, display_name: &str) -> String {
    let display_name = display_name.trim();
    if !display_name.is_empty() {
        return display_name.to_string();
    }

    Path::new(project_path)
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| project_path.to_string())
}

fn normalize_snapshot(snapshot: ProjectConfigSnapshot) -> Result<ProjectConfigSnapshot> {
    let mut projects = BTreeMap::new();

    for (stored_key, mut project) in snapshot.projects {
        let path_seed = if project.project_path.trim().is_empty() {
            stored_key.as_str()
        } else {
            project.project_path.as_str()
        };
        let normalized_path = normalize_project_path(path_seed)
            .or_else(|_| normalize_project_path(&stored_key))?;

        project.project_path = normalized_path.clone();
        project.display_name = normalize_display_name(&normalized_path, &project.display_name);
        if projects.contains_key(&normalized_path) {
            return Err(anyhow!(
                "Duplicate project entries normalize to '{}'; remove or merge legacy keys before continuing",
                normalized_path
            ));
        }
        projects.insert(normalized_path, project);
    }

    Ok(ProjectConfigSnapshot { projects })
}

#[cfg(test)]
#[path = "store_tests.rs"]
mod tests;
