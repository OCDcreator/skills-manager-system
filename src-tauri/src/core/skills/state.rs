use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkillStateSnapshot {
    pub disabled_skill_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PersistedSkillStateFile {
    pub repos: BTreeMap<String, PersistedRepoSkillState>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PersistedRepoSkillState {
    pub repo_path: String,
    pub disabled_skill_ids: Vec<String>,
}

pub struct SkillStateStore {
    base_dir: PathBuf,
}

impl SkillStateStore {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn load_for_repo(&self, repo_path: &Path) -> Result<SkillStateSnapshot> {
        let repo_key = build_repo_state_key(repo_path)?;
        let file = self.load_file()?;

        Ok(file
            .repos
            .get(&repo_key)
            .map(|repo_state| SkillStateSnapshot {
                disabled_skill_ids: normalize_disabled_skill_ids(&repo_state.disabled_skill_ids),
            })
            .unwrap_or_default())
    }

    pub fn set_skill_enabled(
        &self,
        repo_path: &Path,
        skill_id: &str,
        enabled: bool,
    ) -> Result<SkillStateSnapshot> {
        fs::create_dir_all(&self.base_dir)
            .with_context(|| format!("Failed to create {:?}", self.base_dir))?;

        let repo_key = build_repo_state_key(repo_path)?;
        let normalized_repo_path = normalize_repo_path(repo_path);
        let mut file = self.load_file()?;

        let repo_state = file
            .repos
            .entry(repo_key)
            .or_insert_with(|| PersistedRepoSkillState {
                repo_path: normalized_repo_path.clone(),
                disabled_skill_ids: Vec::new(),
            });

        let mut disabled_ids = repo_state
            .disabled_skill_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();

        if enabled {
            disabled_ids.remove(skill_id);
        } else {
            disabled_ids.insert(skill_id.to_string());
        }

        repo_state.repo_path = normalized_repo_path;
        repo_state.disabled_skill_ids = disabled_ids.into_iter().collect();

        let snapshot = SkillStateSnapshot {
            disabled_skill_ids: repo_state.disabled_skill_ids.clone(),
        };

        self.save_file(&file)?;

        Ok(snapshot)
    }

    fn load_file(&self) -> Result<PersistedSkillStateFile> {
        let path = self.state_path();
        if !path.exists() {
            return Ok(PersistedSkillStateFile::default());
        }

        let raw = fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
        serde_json::from_str::<PersistedSkillStateFile>(&raw)
            .with_context(|| format!("Failed to parse {:?}", path))
    }

    fn save_file(&self, file: &PersistedSkillStateFile) -> Result<()> {
        let json = serde_json::to_string_pretty(file)?;
        fs::write(self.state_path(), json).context("Failed to write skill-state.json")?;
        Ok(())
    }

    fn state_path(&self) -> PathBuf {
        self.base_dir.join("skill-state.json")
    }
}

pub fn build_repo_state_key(repo_path: &Path) -> Result<String> {
    let canonical = repo_path
        .canonicalize()
        .unwrap_or_else(|_| repo_path.to_path_buf());
    Ok(normalize_repo_path(&canonical))
}

pub fn normalize_repo_path(path: &Path) -> String {
    let normalized = path.to_string_lossy().replace('\\', "/");

    if cfg!(windows) {
        normalized.to_lowercase()
    } else {
        normalized
    }
}

fn normalize_disabled_skill_ids(ids: &[String]) -> Vec<String> {
    ids.iter()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn load_for_repo_returns_empty_state_when_store_does_not_exist() {
        let dir = tempdir().unwrap();
        let repo = tempdir().unwrap();
        let store = SkillStateStore::new(dir.path().to_path_buf());

        let snapshot = store.load_for_repo(repo.path()).unwrap();

        assert!(snapshot.disabled_skill_ids.is_empty());
    }

    #[test]
    fn set_skill_enabled_persists_disabled_ids_for_one_repo() {
        let dir = tempdir().unwrap();
        let repo = tempdir().unwrap();
        let store = SkillStateStore::new(dir.path().to_path_buf());

        let snapshot = store
            .set_skill_enabled(repo.path(), "custom:searxng", false)
            .unwrap();

        assert_eq!(snapshot.disabled_skill_ids, vec!["custom:searxng".to_string()]);
        assert_eq!(
            store.load_for_repo(repo.path()).unwrap().disabled_skill_ids,
            vec!["custom:searxng".to_string()]
        );
    }

    #[test]
    fn enabling_skill_removes_it_from_disabled_ids() {
        let dir = tempdir().unwrap();
        let repo = tempdir().unwrap();
        let store = SkillStateStore::new(dir.path().to_path_buf());

        store
            .set_skill_enabled(repo.path(), "custom:searxng", false)
            .unwrap();
        let snapshot = store
            .set_skill_enabled(repo.path(), "custom:searxng", true)
            .unwrap();

        assert!(snapshot.disabled_skill_ids.is_empty());
    }

    #[test]
    fn state_is_scoped_per_repo_path() {
        let dir = tempdir().unwrap();
        let repo_a = tempdir().unwrap();
        let repo_b = tempdir().unwrap();
        let store = SkillStateStore::new(dir.path().to_path_buf());

        store
            .set_skill_enabled(repo_a.path(), "custom:searxng", false)
            .unwrap();

        assert_eq!(
            store.load_for_repo(repo_a.path()).unwrap().disabled_skill_ids,
            vec!["custom:searxng".to_string()]
        );
        assert!(store
            .load_for_repo(repo_b.path())
            .unwrap()
            .disabled_skill_ids
            .is_empty());
    }

    #[test]
    fn repeated_disable_writes_remain_sorted_and_deduped() {
        let dir = tempdir().unwrap();
        let repo = tempdir().unwrap();
        let store = SkillStateStore::new(dir.path().to_path_buf());

        store
            .set_skill_enabled(repo.path(), "external:z-skill", false)
            .unwrap();
        store
            .set_skill_enabled(repo.path(), "custom:a-skill", false)
            .unwrap();
        let snapshot = store
            .set_skill_enabled(repo.path(), "external:z-skill", false)
            .unwrap();

        assert_eq!(
            snapshot.disabled_skill_ids,
            vec![
                "custom:a-skill".to_string(),
                "external:z-skill".to_string()
            ]
        );
    }

    #[test]
    fn load_for_repo_tolerates_unknown_disabled_ids() {
        let dir = tempdir().unwrap();
        let repo = tempdir().unwrap();
        let store = SkillStateStore::new(dir.path().to_path_buf());
        let repo_key = build_repo_state_key(repo.path()).unwrap();

        fs::create_dir_all(dir.path()).unwrap();
        fs::write(
            dir.path().join("skill-state.json"),
            format!(
                "{{\"repos\":{{\"{repo_key}\":{{\"repoPath\":\"{}\",\"disabledSkillIds\":[\"external:missing-skill\"]}}}}}}",
                normalize_repo_path(repo.path())
            ),
        )
        .unwrap();

        let snapshot = store.load_for_repo(repo.path()).unwrap();

        assert_eq!(
            snapshot.disabled_skill_ids,
            vec!["external:missing-skill".to_string()]
        );
    }
}
