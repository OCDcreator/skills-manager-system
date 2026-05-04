use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct ExternalSourcesSnapshot {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    pub sources: Vec<ExternalSourceRecord>,
    pub imports: Vec<ImportedExternalSkillRecord>,
}

impl ExternalSourcesSnapshot {
    pub const SCHEMA_VERSION: u32 = 1;
}

impl Default for ExternalSourcesSnapshot {
    fn default() -> Self {
        Self {
            schema_version: Self::SCHEMA_VERSION,
            sources: Vec::new(),
            imports: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct ExternalSourceRecord {
    pub id: String,
    pub repo_url: String,
    pub branch: Option<String>,
    pub subpath: Option<String>,
    pub default_branch: Option<String>,
    pub cached_repo_path: Option<String>,
    pub detected_kind: Option<String>,
    pub last_fetched_commit: Option<String>,
    pub last_fetched_at: Option<String>,
    pub status: Option<String>,
    pub warnings: Vec<ExternalSourceWarning>,
}

impl Default for ExternalSourceRecord {
    fn default() -> Self {
        Self {
            id: String::new(),
            repo_url: String::new(),
            branch: None,
            subpath: None,
            default_branch: None,
            cached_repo_path: None,
            detected_kind: None,
            last_fetched_commit: None,
            last_fetched_at: None,
            status: None,
            warnings: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct ImportedExternalSkillRecord {
    pub import_id: String,
    pub external_source_id: String,
    pub agent_key: String,
    pub upstream_variant_path: String,
    pub pinned_commit: String,
    pub pinned_variant_fingerprint: Option<String>,
    pub skill_id: String,
    pub mirror_relative_path: String,
    pub last_checked_commit: Option<String>,
    pub imported_at: Option<String>,
    pub warnings: Vec<ExternalSourceWarning>,
    pub update_available: bool,
}

impl Default for ImportedExternalSkillRecord {
    fn default() -> Self {
        Self {
            import_id: String::new(),
            external_source_id: String::new(),
            agent_key: String::new(),
            upstream_variant_path: String::new(),
            pinned_commit: String::new(),
            pinned_variant_fingerprint: None,
            skill_id: String::new(),
            mirror_relative_path: String::new(),
            last_checked_commit: None,
            imported_at: None,
            warnings: Vec::new(),
            update_available: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct ManagedSkillMirrorManifest {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    pub managed: bool,
    pub import_id: String,
    pub source_id: String,
    pub repo_url: String,
    pub agent_key: String,
    pub variant_path: String,
    pub mirror_relative_path: String,
    pub skill_id: String,
    pub pinned_commit: String,
}

impl Default for ManagedSkillMirrorManifest {
    fn default() -> Self {
        Self {
            schema_version: ExternalSourcesSnapshot::SCHEMA_VERSION,
            managed: true,
            import_id: String::new(),
            source_id: String::new(),
            repo_url: String::new(),
            agent_key: String::new(),
            variant_path: String::new(),
            mirror_relative_path: String::new(),
            skill_id: String::new(),
            pinned_commit: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalSourceWarning {
    pub code: String,
    pub severity: String,
    pub message: String,
}

fn default_schema_version() -> u32 {
    ExternalSourcesSnapshot::SCHEMA_VERSION
}
