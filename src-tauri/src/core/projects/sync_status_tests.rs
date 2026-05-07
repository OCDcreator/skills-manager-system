use super::*;
use crate::core::projects::store::{
    ProjectAgentAssignment, ProjectApplyFreshness, ProjectConfigSnapshot, ProjectConfigStore,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn create_skill(repo_dir: &Path, relative_path: &str, title: &str) -> String {
    let skill_dir = repo_dir.join(relative_path);
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), format!("# {title}\n")).unwrap();
    relative_path.replace('/', ":")
}

fn test_system_dirs(root: &Path) -> AgentSystemDirs {
    AgentSystemDirs {
        home_dir: root.join("home"),
        config_dir: Some(root.join("xdg")),
    }
}

fn project_status(
    snapshot: &ProjectConfigSnapshot,
    project_dir: &Path,
    agent_key: &str,
) -> ProjectApplyFreshness {
    snapshot.projects[&project_dir.to_string_lossy().replace('\\', "/")].apply_statuses[agent_key]
        .apply_status
        .clone()
}

#[test]
fn project_apply_marks_current_then_stale_after_config_change() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");
    let project_dir = temp.path().join("project");
    fs::create_dir_all(&project_dir).unwrap();
    let alpha_id = create_skill(&repo_dir, "custom/alpha", "Alpha");
    let beta_id = create_skill(&repo_dir, "custom/beta", "Beta");

    let store = ProjectConfigStore::new(config_dir.clone());
    store
        .add_project_with_agents(
            project_dir.to_string_lossy().as_ref(),
            "Project",
            BTreeMap::from([("codex".to_string(), project_agent(vec![alpha_id.clone()]))]),
        )
        .unwrap();

    apply_project_assignments(&config_dir, &repo_dir, &test_system_dirs(temp.path())).unwrap();
    let current_snapshot = attach_project_apply_statuses(
        &config_dir,
        &repo_dir,
        &test_system_dirs(temp.path()),
        store.load().unwrap(),
    )
    .unwrap();
    assert_eq!(
        project_status(&current_snapshot, &project_dir, "codex"),
        ProjectApplyFreshness::Current
    );

    store
        .update_project_agents(
            project_dir.to_string_lossy().as_ref(),
            None,
            Some(BTreeMap::from([(
                "codex".to_string(),
                project_agent(vec![alpha_id, beta_id]),
            )])),
        )
        .unwrap();
    let stale_snapshot = attach_project_apply_statuses(
        &config_dir,
        &repo_dir,
        &test_system_dirs(temp.path()),
        store.load().unwrap(),
    )
    .unwrap();
    assert_eq!(
        project_status(&stale_snapshot, &project_dir, "codex"),
        ProjectApplyFreshness::Stale
    );
}

fn project_agent(selected_skill_ids: Vec<String>) -> ProjectAgentAssignment {
    ProjectAgentAssignment {
        selected_skill_ids,
        selected_scene_ids: Vec::new(),
        excluded_skill_ids: Vec::new(),
    }
}
