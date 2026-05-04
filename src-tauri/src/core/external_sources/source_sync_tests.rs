use std::cell::RefCell;
use std::path::{Path, PathBuf};

use anyhow::Result;
use tempfile::tempdir;

use super::{fetch_source_with_runtime, upsert_source, SourceFetchRuntime, SourceUpsertInput};
use crate::app_runtime::config_lock::acquire_config_lock;
use crate::core::external_sources::detect::DetectionResult;
use crate::core::external_sources::models::{ExternalSourceRecord, ExternalSourcesSnapshot};
use crate::core::external_sources::ExternalSourcesStore;

#[test]
fn upsert_source_updates_branch_and_subpath_for_existing_repo_url_identity() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");

    let first_id = upsert_source(
        &config_dir,
        SourceUpsertInput {
            repo_url: "https://github.com/OCDcreator/example.git",
            branch: Some("main"),
            subpath: Some("packages/skills"),
        },
    )
    .unwrap();
    let second_id = upsert_source(
        &config_dir,
        SourceUpsertInput {
            repo_url: "git@github.com:OCDcreator/example.git",
            branch: Some("release"),
            subpath: Some(r".\dist\agents"),
        },
    )
    .unwrap();

    assert_eq!(second_id, first_id);
    let snapshot = ExternalSourcesStore::new(config_dir).load().unwrap();
    assert_eq!(snapshot.sources.len(), 1);
    assert_eq!(
        snapshot.sources[0].repo_url,
        "git@github.com:OCDcreator/example.git"
    );
    assert_eq!(snapshot.sources[0].branch.as_deref(), Some("release"));
    assert_eq!(snapshot.sources[0].subpath.as_deref(), Some("dist/agents"));
}

#[test]
fn upsert_source_rejects_traversing_subpath() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");

    let error = upsert_source(
        &config_dir,
        SourceUpsertInput {
            repo_url: "https://github.com/OCDcreator/example",
            branch: None,
            subpath: Some("../outside"),
        },
    )
    .unwrap_err();

    assert!(error.to_string().contains("Invalid relative path"));
    let snapshot = ExternalSourcesStore::new(config_dir).load().unwrap();
    assert!(snapshot.sources.is_empty());
}

#[test]
fn fetch_source_reads_requested_branch_commit_and_stores_default_branch() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let store = ExternalSourcesStore::new(config_dir.clone());
    let guard = acquire_config_lock(&config_dir).unwrap();
    store
        .save(
            &guard,
            &ExternalSourcesSnapshot {
                schema_version: 1,
                sources: vec![ExternalSourceRecord {
                    id: "src_example".to_string(),
                    repo_url: "https://github.com/example/repo".to_string(),
                    branch: Some("release".to_string()),
                    ..ExternalSourceRecord::default()
                }],
                imports: Vec::new(),
            },
        )
        .unwrap();
    drop(guard);
    let runtime = RecordingFetchRuntime::new(temp.path().join("repo"));

    fetch_source_with_runtime(&config_dir, "src_example", &runtime).unwrap();

    assert_eq!(runtime.head_branches.borrow().as_slice(), ["release"]);
    let snapshot = store.load().unwrap();
    let source = &snapshot.sources[0];
    assert_eq!(source.default_branch.as_deref(), Some("main"));
    assert_eq!(
        source.last_fetched_commit.as_deref(),
        Some("release_commit")
    );
    assert_eq!(source.status.as_deref(), Some("warning"));
    assert!(source
        .warnings
        .iter()
        .any(|warning| warning.code == "no_importable_skills"));
}

#[test]
fn fetch_source_passes_configured_subpath_to_detection() {
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let store = ExternalSourcesStore::new(config_dir.clone());
    let guard = acquire_config_lock(&config_dir).unwrap();
    store
        .save(
            &guard,
            &ExternalSourcesSnapshot {
                schema_version: 1,
                sources: vec![ExternalSourceRecord {
                    id: "src_example".to_string(),
                    repo_url: "https://github.com/example/repo".to_string(),
                    subpath: Some("packages/skills".to_string()),
                    ..ExternalSourceRecord::default()
                }],
                imports: Vec::new(),
            },
        )
        .unwrap();
    drop(guard);
    let runtime = RecordingFetchRuntime::new(temp.path().join("repo"));

    fetch_source_with_runtime(&config_dir, "src_example", &runtime).unwrap();

    assert_eq!(
        runtime.detect_subpaths.borrow().as_slice(),
        [Some("packages/skills".to_string())]
    );
    let snapshot = store.load().unwrap();
    assert!(snapshot.sources[0].warnings.iter().any(|warning| {
        warning
            .message
            .contains("configured subpath 'packages/skills'")
    }));
}

struct RecordingFetchRuntime {
    repo_dir: PathBuf,
    head_branches: RefCell<Vec<String>>,
    detect_subpaths: RefCell<Vec<Option<String>>>,
}

impl RecordingFetchRuntime {
    fn new(repo_dir: PathBuf) -> Self {
        Self {
            repo_dir,
            head_branches: RefCell::new(Vec::new()),
            detect_subpaths: RefCell::new(Vec::new()),
        }
    }
}

impl SourceFetchRuntime for RecordingFetchRuntime {
    fn ensure_cached_repo(
        &self,
        _cache_root: &Path,
        _source_id: &str,
        _repo_url: &str,
    ) -> Result<PathBuf> {
        Ok(self.repo_dir.clone())
    }

    fn read_default_branch(&self, _repo_dir: &Path) -> Result<String> {
        Ok("main".to_string())
    }

    fn read_head_commit(&self, _repo_dir: &Path, branch: &str) -> Result<String> {
        self.head_branches.borrow_mut().push(branch.to_string());
        Ok(format!("{branch}_commit"))
    }

    fn detect_variants(
        &self,
        _repo_dir: &Path,
        _git_ref: &str,
        _source_id: &str,
        subpath: Option<&str>,
    ) -> Result<DetectionResult> {
        self.detect_subpaths
            .borrow_mut()
            .push(subpath.map(ToOwned::to_owned));
        Ok(DetectionResult {
            kind: "unsupported".to_string(),
            variants: Vec::new(),
            warnings: Vec::new(),
        })
    }

    fn fingerprint_variant_at_ref(
        &self,
        _repo_dir: &Path,
        _git_ref: &str,
        _variant_path: &str,
    ) -> Result<Option<String>> {
        Ok(None)
    }
}
