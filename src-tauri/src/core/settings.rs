use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::platform_paths::portable_path_string;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AgentSyncMode {
    #[default]
    Copy,
    Symlink,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct AppSettings {
    pub repo_path: Option<String>,
    pub agent_sync_mode: AgentSyncMode,
    pub agent_order: Vec<String>,
    pub assistant_working_directory: Option<String>,
}

pub struct SettingsStore {
    base_dir: PathBuf,
}

impl SettingsStore {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn load(&self) -> Result<AppSettings> {
        let path = self.settings_path();
        if !path.exists() {
            return Ok(AppSettings::default());
        }

        let raw =
            fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
        let settings = serde_json::from_str::<AppSettings>(&raw)
            .with_context(|| format!("Failed to parse {:?}", path))?;
        Ok(settings)
    }

    pub fn save_repo_path(&self, repo_path: Option<&Path>) -> Result<AppSettings> {
        let mut settings = self.load()?;
        settings.repo_path = repo_path.map(portable_path_string);
        self.save(&settings)
    }

    pub fn save_agent_sync_mode(&self, agent_sync_mode: AgentSyncMode) -> Result<AppSettings> {
        let mut settings = self.load()?;
        settings.agent_sync_mode = agent_sync_mode;
        self.save(&settings)
    }

    pub fn save_agent_order(&self, agent_order: &[String]) -> Result<AppSettings> {
        let mut settings = self.load()?;
        settings.agent_order = agent_order.to_vec();
        self.save(&settings)
    }

    pub fn save_assistant_working_directory(
        &self,
        assistant_working_directory: Option<&Path>,
    ) -> Result<AppSettings> {
        let mut settings = self.load()?;
        settings.assistant_working_directory =
            assistant_working_directory.map(portable_path_string);
        self.save(&settings)
    }

    fn save(&self, settings: &AppSettings) -> Result<AppSettings> {
        fs::create_dir_all(&self.base_dir)
            .with_context(|| format!("Failed to create {:?}", self.base_dir))?;

        let json = serde_json::to_string_pretty(settings)?;
        fs::write(self.settings_path(), json).context("Failed to write settings.json")?;
        Ok(settings.clone())
    }

    fn settings_path(&self) -> PathBuf {
        self.base_dir.join("settings.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn load_returns_defaults_when_file_does_not_exist() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        let settings = store.load().unwrap();
        assert_eq!(settings.repo_path, None);
        assert_eq!(settings.agent_sync_mode, AgentSyncMode::Copy);
        assert!(settings.agent_order.is_empty());
        assert_eq!(settings.assistant_working_directory, None);
    }

    #[test]
    fn load_backfills_missing_sync_mode() {
        let dir = tempdir().unwrap();
        fs::write(
            dir.path().join("settings.json"),
            r#"{"repoPath":"/tmp/my-skills"}"#,
        )
        .unwrap();

        let settings = SettingsStore::new(dir.path().to_path_buf()).load().unwrap();
        assert_eq!(settings.repo_path.as_deref(), Some("/tmp/my-skills"));
        assert_eq!(settings.agent_sync_mode, AgentSyncMode::Copy);
        assert!(settings.agent_order.is_empty());
        assert_eq!(settings.assistant_working_directory, None);
    }

    #[test]
    fn load_backfills_missing_agent_order() {
        let dir = tempdir().unwrap();
        fs::write(
            dir.path().join("settings.json"),
            r#"{"repoPath":"/tmp/my-skills","agentSyncMode":"copy"}"#,
        )
        .unwrap();

        let settings = SettingsStore::new(dir.path().to_path_buf()).load().unwrap();
        assert!(settings.agent_order.is_empty());
        assert_eq!(settings.assistant_working_directory, None);
    }

    #[test]
    fn save_repo_path_round_trips() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        store
            .save_repo_path(Some(Path::new(
                "C:/Users/test/Desktop/Write/custom-project/my-skills",
            )))
            .unwrap();

        assert_eq!(
            store.load().unwrap().repo_path.as_deref(),
            Some("C:/Users/test/Desktop/Write/custom-project/my-skills")
        );
        assert!(store.load().unwrap().agent_order.is_empty());
        assert_eq!(store.load().unwrap().assistant_working_directory, None);
    }

    #[cfg(windows)]
    #[test]
    fn save_repo_path_persists_portable_separators() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        store
            .save_repo_path(Some(Path::new(
                r"C:\Users\test\Desktop\Write\custom-project\my-skills\",
            )))
            .unwrap();

        assert_eq!(
            store.load().unwrap().repo_path.as_deref(),
            Some("C:/Users/test/Desktop/Write/custom-project/my-skills")
        );
    }

    #[test]
    fn save_agent_sync_mode_round_trips() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        store.save_agent_sync_mode(AgentSyncMode::Symlink).unwrap();

        assert_eq!(
            store.load().unwrap().agent_sync_mode,
            AgentSyncMode::Symlink
        );
        assert!(store.load().unwrap().agent_order.is_empty());
        assert_eq!(store.load().unwrap().assistant_working_directory, None);
    }

    #[test]
    fn save_agent_order_round_trips() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        store
            .save_agent_order(&[
                "codex".to_string(),
                "claude_code".to_string(),
                "opencode".to_string(),
            ])
            .unwrap();

        assert_eq!(
            store.load().unwrap().agent_order,
            vec![
                "codex".to_string(),
                "claude_code".to_string(),
                "opencode".to_string(),
            ]
        );
    }

    #[test]
    fn save_assistant_working_directory_round_trips() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        store
            .save_assistant_working_directory(Some(Path::new(
                "C:/Users/test/AppData/Roaming/com.ocdcreator.skills-manager-system.dev",
            )))
            .unwrap();

        assert_eq!(
            store.load().unwrap().assistant_working_directory.as_deref(),
            Some("C:/Users/test/AppData/Roaming/com.ocdcreator.skills-manager-system.dev")
        );
    }

    #[cfg(windows)]
    #[test]
    fn save_assistant_working_directory_persists_portable_separators() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        store
            .save_assistant_working_directory(Some(Path::new(
                r"C:\Users\test\AppData\Roaming\skills-manager-system\",
            )))
            .unwrap();

        assert_eq!(
            store.load().unwrap().assistant_working_directory.as_deref(),
            Some("C:/Users/test/AppData/Roaming/skills-manager-system")
        );
    }

    #[test]
    fn save_repo_path_preserves_agent_order() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        store
            .save_agent_order(&["codex".to_string(), "cursor".to_string()])
            .unwrap();
        store
            .save_repo_path(Some(Path::new("C:/tmp/my-skills")))
            .unwrap();

        assert_eq!(
            store.load().unwrap().agent_order,
            vec!["codex".to_string(), "cursor".to_string()]
        );
    }

    #[test]
    fn save_assistant_working_directory_preserves_agent_order() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        store
            .save_agent_order(&["codex".to_string(), "cursor".to_string()])
            .unwrap();
        store
            .save_assistant_working_directory(Some(Path::new("C:/tmp/assistant")))
            .unwrap();

        let settings = store.load().unwrap();
        assert_eq!(
            settings.agent_order,
            vec!["codex".to_string(), "cursor".to_string()]
        );
        assert_eq!(
            settings.assistant_working_directory.as_deref(),
            Some("C:/tmp/assistant")
        );
    }

    #[test]
    fn save_agent_sync_mode_preserves_agent_order() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        store
            .save_agent_order(&["codex".to_string(), "cursor".to_string()])
            .unwrap();
        store.save_agent_sync_mode(AgentSyncMode::Symlink).unwrap();

        assert_eq!(
            store.load().unwrap().agent_order,
            vec!["codex".to_string(), "cursor".to_string()]
        );
    }
}
