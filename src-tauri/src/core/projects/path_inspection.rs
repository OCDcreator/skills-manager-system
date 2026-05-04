use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::core::agents::catalog::{agent_catalog, find_agent, project_skills_dir_rule};
use crate::core::platform_paths::portable_path_string;

use super::project_paths::normalize_project_path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPathInspectionAgentResult {
    pub agent_key: String,
    pub display_name: String,
    pub marker_dir: String,
    pub target_dir: String,
    pub marker_exists: bool,
    pub target_exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPathInspection {
    pub normalized_path: String,
    pub agents: Vec<ProjectPathInspectionAgentResult>,
    pub unsupported_agent_keys: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn inspect_project_assignment_path(
    project_path: &str,
    agent_keys: &[String],
) -> Result<ProjectPathInspection> {
    let normalized_path = normalize_project_path(project_path)?;
    let inspect_keys = if agent_keys.is_empty() {
        agent_catalog()
            .iter()
            .map(|entry| entry.key.to_string())
            .collect::<Vec<_>>()
    } else {
        agent_keys.to_vec()
    };

    let mut agents = Vec::new();
    let mut unsupported_agent_keys = Vec::new();

    for agent_key in inspect_keys {
        let Some(agent) = find_agent(&agent_key) else {
            unsupported_agent_keys.push(agent_key);
            continue;
        };

        let marker_dir = Path::new(&normalized_path).join(agent.detect_dir_rule);
        let target_dir = Path::new(&normalized_path).join(project_skills_dir_rule(agent));

        agents.push(ProjectPathInspectionAgentResult {
            agent_key: agent.key.to_string(),
            display_name: agent.display_name.to_string(),
            marker_dir: portable_path_string(&marker_dir),
            target_dir: portable_path_string(&target_dir),
            marker_exists: marker_dir.exists(),
            target_exists: target_dir.exists(),
        });
    }

    Ok(ProjectPathInspection {
        normalized_path,
        agents,
        unsupported_agent_keys,
        warnings: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn inspect_path_reports_marker_and_target_existence() {
        let temp = tempdir().unwrap();
        let project_path = temp.path().join("app");
        fs::create_dir_all(project_path.join(".codex/skills")).unwrap();

        let result = inspect_project_assignment_path(
            project_path.to_string_lossy().as_ref(),
            &["codex".to_string()],
        )
        .unwrap();

        assert_eq!(
            result.normalized_path,
            project_path.to_string_lossy().replace('\\', "/")
        );
        assert_eq!(result.unsupported_agent_keys, Vec::<String>::new());
        assert_eq!(result.agents.len(), 1);
        assert!(result.agents[0].marker_exists);
        assert!(result.agents[0].target_exists);
    }

    #[test]
    fn inspect_path_reports_unsupported_agent_keys() {
        let temp = tempdir().unwrap();
        let project_path = temp.path().join("app");
        fs::create_dir_all(&project_path).unwrap();

        let result = inspect_project_assignment_path(
            project_path.to_string_lossy().as_ref(),
            &["codex".to_string(), "unknown_agent".to_string()],
        )
        .unwrap();

        assert_eq!(result.unsupported_agent_keys, vec!["unknown_agent".to_string()]);
    }

    #[test]
    fn inspect_path_uses_project_local_agent_rules() {
        let temp = tempdir().unwrap();
        let project_path = temp.path().join("app");
        fs::create_dir_all(&project_path).unwrap();

        let result = inspect_project_assignment_path(
            project_path.to_string_lossy().as_ref(),
            &["opencode".to_string(), "cursor".to_string()],
        )
        .unwrap();

        assert!(result.agents[0].target_dir.ends_with("/app/.opencode/skills"));
        assert!(result.agents[1].target_dir.ends_with("/app/.cursor/skills"));
    }
}
