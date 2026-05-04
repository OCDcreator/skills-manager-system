use super::*;
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
        snapshot.projects[&project_path].skill_ids,
        vec!["custom:skill"]
    );
    assert_eq!(store.load().unwrap(), snapshot);
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

    assert!(snapshot.projects.contains_key("C:/Users/test/workspace/app"));
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

    assert!(error.contains(
        "Duplicate project entries normalize to 'C:/Users/test/workspace/app'"
    ));
}
