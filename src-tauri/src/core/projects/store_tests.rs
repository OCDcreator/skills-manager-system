use super::*;
use std::collections::BTreeMap;
use tempfile::tempdir;

#[test]
fn load_returns_defaults_when_missing() {
    let dir = tempdir().unwrap();
    let store = ProjectConfigStore::new(dir.path().to_path_buf());
    let snapshot = store.load().unwrap();
    assert!(snapshot.projects.is_empty());
}

#[test]
fn add_project_round_trips() {
    let dir = tempdir().unwrap();
    let store = ProjectConfigStore::new(dir.path().to_path_buf());
    let project_path = dir.path().join("workspace/app");
    let project_path = normalize_project_path(project_path.to_string_lossy().as_ref()).unwrap();

    let snapshot = store
        .add_project(
            &project_path,
            "App",
            vec!["custom:skill".to_string()],
            vec!["codex".to_string()],
        )
        .unwrap();

    assert_eq!(snapshot.projects[&project_path].display_name, "App");
    assert_eq!(
        snapshot.projects[&project_path].agents["codex"].selected_skill_ids,
        vec!["custom:skill"]
    );
    assert_eq!(store.load().unwrap(), snapshot);
}

#[test]
fn loads_legacy_flat_project_as_per_agent_entries() {
    let temp = tempfile::tempdir().unwrap();
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(
        config_dir.join("project-config.json"),
        r#"{
  "projects": {
    "C:/repo/app": {
      "projectPath": "C:/repo/app",
      "displayName": "App",
      "skillIds": ["custom:alpha"],
      "agentKeys": ["codex", "unknown-agent"]
    }
  }
}"#,
    )
    .unwrap();

    let snapshot = ProjectConfigStore::new(config_dir).load().unwrap();
    let project = snapshot.projects.get("C:/repo/app").unwrap();

    assert_eq!(
        project.agents["codex"].selected_skill_ids,
        vec!["custom:alpha"]
    );
    assert_eq!(
        project.agents["codex"].selected_scene_ids,
        Vec::<String>::new()
    );
    assert_eq!(
        project.agents["codex"].excluded_skill_ids,
        Vec::<String>::new()
    );
    assert_eq!(project.unsupported_agent_keys, vec!["unknown-agent"]);
}

#[test]
fn saves_per_agent_project_entries_sorted_and_deduped() {
    let temp = tempfile::tempdir().unwrap();
    let store = ProjectConfigStore::new(temp.path().join("config"));

    store
        .add_project_with_agents(
            "C:/repo/app",
            "App",
            BTreeMap::from([(
                "codex".to_string(),
                ProjectAgentAssignment {
                    selected_skill_ids: vec![
                        "custom:beta".into(),
                        "custom:alpha".into(),
                        "custom:alpha".into(),
                    ],
                    selected_scene_ids: vec!["focus".into()],
                    excluded_skill_ids: vec!["custom:beta".into()],
                },
            )]),
        )
        .unwrap();

    let entry = &store.load().unwrap().projects["C:/repo/app"].agents["codex"];
    assert_eq!(
        entry.selected_skill_ids,
        vec!["custom:alpha", "custom:beta"]
    );
    assert_eq!(entry.selected_scene_ids, vec!["focus"]);
    assert_eq!(entry.excluded_skill_ids, vec!["custom:beta"]);
}

#[test]
fn loads_new_format_project_without_flat_compat_fields() {
    let temp = tempfile::tempdir().unwrap();
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(
        config_dir.join("project-config.json"),
        r#"{
  "projects": {
    "C:/repo/app": {
      "projectPath": "C:/repo/app",
      "displayName": "App",
      "agents": {
        "codex": {
          "selectedSkillIds": ["custom:alpha"],
          "selectedSceneIds": ["focus"],
          "excludedSkillIds": []
        }
      },
      "unsupportedAgentKeys": []
    }
  }
}"#,
    )
    .unwrap();

    let snapshot = ProjectConfigStore::new(config_dir).load().unwrap();
    let project = snapshot.projects.get("C:/repo/app").unwrap();

    assert_eq!(project.skill_ids, vec!["custom:alpha"]);
    assert_eq!(project.agent_keys, vec!["codex"]);
}

#[test]
fn serializes_flat_compat_fields_for_current_readers() {
    let temp = tempfile::tempdir().unwrap();
    let store = ProjectConfigStore::new(temp.path().join("config"));

    store
        .add_project_with_agents(
            "C:/repo/app",
            "App",
            BTreeMap::from([(
                "codex".to_string(),
                ProjectAgentAssignment {
                    selected_skill_ids: vec!["custom:beta".into(), "custom:alpha".into()],
                    selected_scene_ids: vec!["focus".into()],
                    excluded_skill_ids: Vec::new(),
                },
            )]),
        )
        .unwrap();

    let value = serde_json::to_value(store.load().unwrap()).unwrap();
    let project = &value["projects"]["C:/repo/app"];

    assert_eq!(
        project["skillIds"],
        serde_json::json!(["custom:alpha", "custom:beta"])
    );
    assert_eq!(project["agentKeys"], serde_json::json!(["codex"]));
    assert!(project["agents"].is_object());
}

#[test]
fn update_project_agents_preserves_unsupported_legacy_agent_keys() {
    let temp = tempfile::tempdir().unwrap();
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(
        config_dir.join("project-config.json"),
        r#"{
  "projects": {
    "C:/repo/app": {
      "projectPath": "C:/repo/app",
      "displayName": "App",
      "skillIds": ["custom:alpha"],
      "agentKeys": ["codex", "unknown-agent"]
    }
  }
}"#,
    )
    .unwrap();

    let store = ProjectConfigStore::new(config_dir);
    store
        .update_project_agents(
            "C:/repo/app",
            None,
            Some(BTreeMap::from([(
                "codex".to_string(),
                ProjectAgentAssignment {
                    selected_skill_ids: vec!["custom:beta".into()],
                    selected_scene_ids: Vec::new(),
                    excluded_skill_ids: Vec::new(),
                },
            )])),
        )
        .unwrap();

    let project = &store.load().unwrap().projects["C:/repo/app"];
    assert_eq!(project.unsupported_agent_keys, vec!["unknown-agent"]);
    assert_eq!(project.agent_keys, vec!["codex", "unknown-agent"]);
}

#[test]
fn add_project_rejects_duplicate_path() {
    let dir = tempdir().unwrap();
    let store = ProjectConfigStore::new(dir.path().to_path_buf());
    let project_path = dir.path().join("workspace/app");
    let project_path = project_path.to_string_lossy().to_string();

    store
        .add_project(&project_path, "App", Vec::new(), Vec::new())
        .unwrap();

    assert!(store
        .add_project(&project_path, "App Again", Vec::new(), Vec::new())
        .is_err());
}

#[test]
fn remove_project_deletes_entry() {
    let dir = tempdir().unwrap();
    let store = ProjectConfigStore::new(dir.path().to_path_buf());
    let project_path = dir.path().join("workspace/app");
    let project_path = normalize_project_path(project_path.to_string_lossy().as_ref()).unwrap();

    store
        .add_project(&project_path, "App", Vec::new(), Vec::new())
        .unwrap();

    let snapshot = store.remove_project(&project_path).unwrap();
    assert!(!snapshot.projects.contains_key(&project_path));
}

#[test]
fn normalize_project_path_accepts_trimmed_absolute_path() {
    let temp = tempdir().unwrap();
    let project_path = format!("  {}  ", temp.path().join("workspace/app").display());

    let normalized = normalize_project_path(&project_path).unwrap();

    assert_eq!(
        normalized,
        temp.path()
            .join("workspace/app")
            .to_string_lossy()
            .replace('\\', "/")
    );
}

#[cfg(windows)]
#[test]
fn load_normalizes_legacy_windows_project_keys() {
    let dir = tempdir().unwrap();
    let legacy_path = r"C:\Users\test\workspace\app";
    fs::write(
        dir.path().join("project-config.json"),
        format!(
            r#"{{
  "projects": {{
    "{escaped}": {{
      "projectPath": "{escaped}",
      "displayName": "App",
      "skillIds": ["custom:alpha"],
      "agentKeys": ["codex"]
    }}
  }}
}}"#,
            escaped = legacy_path.replace('\\', "\\\\")
        ),
    )
    .unwrap();

    let store = ProjectConfigStore::new(dir.path().to_path_buf());
    let snapshot = store.load().unwrap();

    assert!(snapshot
        .projects
        .contains_key("C:/Users/test/workspace/app"));
    assert_eq!(
        snapshot.projects["C:/Users/test/workspace/app"].project_path,
        "C:/Users/test/workspace/app"
    );
    store
        .update_project("C:/Users/test/workspace/app/", Some("Renamed"), None, None)
        .unwrap();
    assert_eq!(
        store.load().unwrap().projects["C:/Users/test/workspace/app"].display_name,
        "Renamed"
    );
}

#[cfg(windows)]
#[test]
fn load_rejects_legacy_project_keys_that_normalize_to_the_same_path() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("project-config.json"),
        r#"{
  "projects": {
    "C:\\Users\\test\\workspace\\app": {
      "projectPath": "C:\\Users\\test\\workspace\\app",
      "displayName": "Backslash App",
      "skillIds": ["custom:alpha"],
      "agentKeys": ["codex"]
    },
    "C:/Users/test/workspace/app/": {
      "projectPath": "C:/Users/test/workspace/app/",
      "displayName": "Slash App",
      "skillIds": ["custom:beta"],
      "agentKeys": ["opencode"]
    }
  }
}"#,
    )
    .unwrap();

    let store = ProjectConfigStore::new(dir.path().to_path_buf());
    let error = store.load().unwrap_err().to_string();

    assert!(error.contains("Duplicate project entries normalize to 'C:/Users/test/workspace/app'"));
}
