use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use super::models::ExternalSourcesSnapshot;
use crate::app_runtime::{write_text_atomic, ConfigLockGuard};

pub struct ExternalSourcesStore {
    config_dir: PathBuf,
}

impl ExternalSourcesStore {
    pub fn new(config_dir: PathBuf) -> Self {
        Self { config_dir }
    }

    pub fn load(&self) -> Result<ExternalSourcesSnapshot> {
        let path = self.snapshot_path();
        if !path.exists() {
            return Ok(ExternalSourcesSnapshot::default());
        }

        let raw =
            fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
        serde_json::from_str(&raw).with_context(|| format!("Failed to parse {:?}", path))
    }

    pub fn save(&self, _guard: &ConfigLockGuard, snapshot: &ExternalSourcesSnapshot) -> Result<()> {
        fs::create_dir_all(&self.config_dir)
            .with_context(|| format!("Failed to create {:?}", self.config_dir))?;
        let json = serde_json::to_string_pretty(snapshot)?;
        write_text_atomic(&self.snapshot_path(), &json)
            .context("Failed to write external-sources.json")?;
        Ok(())
    }

    fn snapshot_path(&self) -> PathBuf {
        self.config_dir.join("external-sources.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_runtime::config_lock::acquire_config_lock;
    use crate::core::external_sources::models::{
        ExternalSourceRecord, ExternalSourceWarning, ImportedExternalSkillRecord,
    };
    use tempfile::tempdir;

    #[test]
    fn load_returns_empty_schema_when_file_is_missing() {
        let dir = tempdir().unwrap();
        let store = ExternalSourcesStore::new(dir.path().to_path_buf());

        let snapshot = store.load().unwrap();

        assert_eq!(snapshot.schema_version, 1);
        assert!(snapshot.sources.is_empty());
        assert!(snapshot.imports.is_empty());
    }

    #[test]
    fn save_round_trips_source_and_import_records() {
        let dir = tempdir().unwrap();
        let store = ExternalSourcesStore::new(dir.path().to_path_buf());
        let guard = acquire_config_lock(dir.path()).unwrap();
        let snapshot = ExternalSourcesSnapshot {
            schema_version: 1,
            sources: vec![ExternalSourceRecord {
                id: "src_01".to_string(),
                repo_url: "github.com/ocdcreator/example".to_string(),
                branch: Some("release".to_string()),
                subpath: Some("packages/skills".to_string()),
                default_branch: Some("main".to_string()),
                cached_repo_path: Some("external-sources/src_01/repo".to_string()),
                detected_kind: Some("generated_agent_bundle".to_string()),
                last_fetched_commit: Some("abcdef123456".to_string()),
                last_fetched_at: Some("2026-04-29T00:00:00Z".to_string()),
                status: Some("ok".to_string()),
                warnings: vec![ExternalSourceWarning {
                    code: "example_warning".to_string(),
                    severity: "warning".to_string(),
                    message: "Example warning".to_string(),
                }],
            }],
            imports: vec![ImportedExternalSkillRecord {
                import_id: "imp_01".to_string(),
                external_source_id: "src_01".to_string(),
                agent_key: "codex".to_string(),
                upstream_variant_path: "dist/agents/.agents/skills/example".to_string(),
                pinned_commit: "abcdef123456".to_string(),
                pinned_variant_fingerprint: Some("sha256:abcd1234".to_string()),
                skill_id: "external:managed/github/ocdcreator__example/codex/example".to_string(),
                mirror_relative_path: "external/managed/github/ocdcreator__example/codex/example"
                    .to_string(),
                last_checked_commit: Some("abcdef123456".to_string()),
                imported_at: Some("2026-04-29T00:05:00Z".to_string()),
                warnings: Vec::new(),
                update_available: false,
            }],
        };

        store.save(&guard, &snapshot).unwrap();

        assert_eq!(store.load().unwrap(), snapshot);
    }

    #[test]
    fn load_defaults_branch_and_subpath_for_legacy_source_records() {
        let dir = tempdir().unwrap();
        fs::write(
            dir.path().join("external-sources.json"),
            r#"{
              "schemaVersion": 1,
              "sources": [
                {
                  "id": "src_legacy",
                  "repoUrl": "https://github.com/example/repo",
                  "defaultBranch": "main",
                  "status": "ok",
                  "warnings": []
                }
              ],
              "imports": []
            }"#,
        )
        .unwrap();
        let store = ExternalSourcesStore::new(dir.path().to_path_buf());

        let snapshot = store.load().unwrap();

        assert_eq!(snapshot.sources.len(), 1);
        assert_eq!(snapshot.sources[0].branch, None);
        assert_eq!(snapshot.sources[0].subpath, None);
    }

    #[test]
    fn save_uses_caller_owned_config_lock() {
        let dir = tempdir().unwrap();
        let store = ExternalSourcesStore::new(dir.path().to_path_buf());
        let guard = acquire_config_lock(dir.path()).unwrap();

        store
            .save(&guard, &ExternalSourcesSnapshot::default())
            .unwrap();

        assert!(dir.path().join("external-sources.json").exists());
    }
}
