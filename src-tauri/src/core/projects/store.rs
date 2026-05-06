use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::agents::catalog::find_agent;

use super::project_paths::normalize_project_path;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectAgentAssignment {
    #[serde(default)]
    pub selected_skill_ids: Vec<String>,
    #[serde(default)]
    pub selected_scene_ids: Vec<String>,
    #[serde(default)]
    pub excluded_skill_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectAssignment {
    pub project_path: String,
    pub display_name: String,
    #[serde(default)]
    pub agents: BTreeMap<String, ProjectAgentAssignment>,
    #[serde(default)]
    pub unsupported_agent_keys: Vec<String>,
    #[serde(default)]
    pub skill_ids: Vec<String>,
    #[serde(default)]
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
        let (agents, unsupported_agent_keys) = build_agents_from_flat(skill_ids, agent_keys);
        self.add_project_entry(project_path, display_name, agents, unsupported_agent_keys)
    }

    pub fn add_project_with_agents(
        &self,
        project_path: &str,
        display_name: &str,
        agents: BTreeMap<String, ProjectAgentAssignment>,
    ) -> Result<ProjectConfigSnapshot> {
        let (agents, unsupported_agent_keys) = normalize_agent_map(agents);
        self.add_project_entry(project_path, display_name, agents, unsupported_agent_keys)
    }

    fn add_project_entry(
        &self,
        project_path: &str,
        display_name: &str,
        agents: BTreeMap<String, ProjectAgentAssignment>,
        unsupported_agent_keys: Vec<String>,
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
                    agents,
                    unsupported_agent_keys,
                    skill_ids: Vec::new(),
                    agent_keys: Vec::new(),
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
        let agent_update = match (skill_ids, agent_keys) {
            (Some(skill_ids), Some(agent_keys)) => {
                Some(build_agents_from_flat(skill_ids, agent_keys))
            }
            (Some(skill_ids), None) => {
                let current = self.load()?;
                let project_path = normalize_project_path(project_path)?;
                let project = current
                    .projects
                    .get(&project_path)
                    .ok_or_else(|| anyhow!("Project '{}' not found", project_path))?;
                Some(build_agents_from_flat(
                    skill_ids,
                    project.agent_keys.clone(),
                ))
            }
            (None, Some(agent_keys)) => {
                let current = self.load()?;
                let project_path = normalize_project_path(project_path)?;
                let project = current
                    .projects
                    .get(&project_path)
                    .ok_or_else(|| anyhow!("Project '{}' not found", project_path))?;
                Some(build_agents_from_flat(
                    project.skill_ids.clone(),
                    agent_keys,
                ))
            }
            (None, None) => None,
        };

        self.update_project_entry(project_path, display_name, agent_update, false)
    }

    pub fn update_project_agents(
        &self,
        project_path: &str,
        display_name: Option<&str>,
        agents: Option<BTreeMap<String, ProjectAgentAssignment>>,
    ) -> Result<ProjectConfigSnapshot> {
        let agent_update = agents.map(normalize_agent_map);
        self.update_project_entry(project_path, display_name, agent_update, true)
    }

    fn update_project_entry(
        &self,
        project_path: &str,
        display_name: Option<&str>,
        agent_update: Option<(BTreeMap<String, ProjectAgentAssignment>, Vec<String>)>,
        preserve_unsupported_agent_keys: bool,
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
            if let Some((agents, unsupported_agent_keys)) = agent_update {
                entry.agents = agents;
                entry.unsupported_agent_keys = if preserve_unsupported_agent_keys {
                    merge_id_lists(entry.unsupported_agent_keys.clone(), unsupported_agent_keys)
                } else {
                    unsupported_agent_keys
                };
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
        let snapshot = normalize_snapshot(snapshot)?;
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
        let normalized_path =
            normalize_project_path(path_seed).or_else(|_| normalize_project_path(&stored_key))?;

        project.project_path = normalized_path.clone();
        project.display_name = normalize_display_name(&normalized_path, &project.display_name);
        normalize_project_assignment(&mut project);
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

fn build_agents_from_flat(
    skill_ids: Vec<String>,
    agent_keys: Vec<String>,
) -> (BTreeMap<String, ProjectAgentAssignment>, Vec<String>) {
    let selected_skill_ids = normalize_id_list(skill_ids);
    let agents = normalize_id_list(agent_keys)
        .into_iter()
        .map(|agent_key| {
            (
                agent_key,
                ProjectAgentAssignment {
                    selected_skill_ids: selected_skill_ids.clone(),
                    selected_scene_ids: Vec::new(),
                    excluded_skill_ids: Vec::new(),
                },
            )
        })
        .collect();

    normalize_agent_map(agents)
}

fn normalize_agent_map(
    agents: BTreeMap<String, ProjectAgentAssignment>,
) -> (BTreeMap<String, ProjectAgentAssignment>, Vec<String>) {
    let mut normalized_agents = BTreeMap::new();
    let mut unsupported_agent_keys = Vec::new();

    for (agent_key, assignment) in agents {
        let agent_key = agent_key.trim().to_string();
        if agent_key.is_empty() {
            continue;
        }
        if find_agent(&agent_key).is_none() {
            unsupported_agent_keys.push(agent_key);
            continue;
        }
        normalized_agents.insert(agent_key, normalize_agent_assignment(assignment));
    }

    (normalized_agents, normalize_id_list(unsupported_agent_keys))
}

fn normalize_agent_assignment(assignment: ProjectAgentAssignment) -> ProjectAgentAssignment {
    ProjectAgentAssignment {
        selected_skill_ids: normalize_id_list(assignment.selected_skill_ids),
        selected_scene_ids: normalize_id_list(assignment.selected_scene_ids),
        excluded_skill_ids: normalize_id_list(assignment.excluded_skill_ids),
    }
}

fn normalize_project_assignment(project: &mut ProjectAssignment) {
    if project.agents.is_empty()
        && (!project.skill_ids.is_empty() || !project.agent_keys.is_empty())
    {
        let (agents, unsupported_agent_keys) = build_agents_from_flat(
            std::mem::take(&mut project.skill_ids),
            std::mem::take(&mut project.agent_keys),
        );
        project.agents = agents;
        project
            .unsupported_agent_keys
            .extend(unsupported_agent_keys);
    }

    let mut agents = BTreeMap::new();
    let mut unsupported_agent_keys = project.unsupported_agent_keys.clone();

    for (agent_key, assignment) in std::mem::take(&mut project.agents) {
        let agent_key = agent_key.trim().to_string();
        if agent_key.is_empty() {
            continue;
        }
        if find_agent(&agent_key).is_none() {
            unsupported_agent_keys.push(agent_key);
            continue;
        }
        agents.insert(agent_key, normalize_agent_assignment(assignment));
    }

    project.agents = agents;
    project.unsupported_agent_keys = normalize_id_list(unsupported_agent_keys);
    project.agent_keys = project
        .agents
        .keys()
        .chain(project.unsupported_agent_keys.iter())
        .cloned()
        .collect();
    project.skill_ids = project
        .agents
        .values()
        .flat_map(|assignment| assignment.selected_skill_ids.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
}

fn normalize_id_list(ids: Vec<String>) -> Vec<String> {
    ids.into_iter()
        .map(|id| id.trim().to_string())
        .filter(|id| !id.is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn merge_id_lists(left: Vec<String>, right: Vec<String>) -> Vec<String> {
    left.into_iter()
        .chain(right)
        .map(|id| id.trim().to_string())
        .filter(|id| !id.is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
#[path = "store_tests.rs"]
mod tests;
