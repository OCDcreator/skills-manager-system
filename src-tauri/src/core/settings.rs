use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub repo_path: Option<String>,
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
        fs::create_dir_all(&self.base_dir)
            .with_context(|| format!("Failed to create {:?}", self.base_dir))?;

        let settings = AppSettings {
            repo_path: repo_path.map(|value| value.to_string_lossy().to_string()),
        };

        let json = serde_json::to_string_pretty(&settings)?;
        fs::write(self.settings_path(), json).context("Failed to write settings.json")?;
        Ok(settings)
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
    fn load_returns_none_when_file_does_not_exist() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        assert_eq!(store.load().unwrap().repo_path, None);
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
    }
}
