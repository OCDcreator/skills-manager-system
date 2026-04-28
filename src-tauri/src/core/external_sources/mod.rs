pub mod detect;
pub mod git_repo;
mod hash;
pub mod models;
pub mod store;

pub use detect::{DetectedExternalVariant, DetectionResult, detect_external_source_variants};
pub use git_repo::normalize_github_repo_url;
pub use models::{
    ExternalSourceRecord, ExternalSourceWarning, ExternalSourcesSnapshot,
    ImportedExternalSkillRecord,
};
pub use store::ExternalSourcesStore;
