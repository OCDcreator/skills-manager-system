pub mod detect;
mod git_export;
pub mod git_repo;
mod hash;
mod import_paths;
pub mod imports;
mod mirror_fs;
pub mod models;
pub mod reference_check;
pub mod service;
mod source_snapshot;
mod source_sync;
pub mod store;

pub use detect::{DetectedExternalVariant, DetectionResult, detect_external_source_variants};
pub use git_repo::normalize_github_repo_url;
pub use imports::{
    ImportRemovalResult, ImportVariantInput, ImportVariantResult, import_variant_into_repo,
    remove_imported_variant_from_repo,
};
pub use models::{
    ExternalSourceRecord, ExternalSourceWarning, ExternalSourcesSnapshot,
    ImportedExternalSkillRecord, ManagedSkillMirrorManifest,
};
pub use reference_check::{BlockingReferences, find_skill_references};
pub use service::{
    ExternalSourceSnapshotItem, ExternalSourcesListResponse, ExternalVariantSnapshot,
    add_external_source, fetch_external_source, import_external_variant, list_external_sources,
    remove_external_source, repair_external_import, update_external_import,
};
pub use store::ExternalSourcesStore;
