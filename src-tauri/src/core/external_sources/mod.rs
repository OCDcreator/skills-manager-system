pub mod detect;
mod generated_agent_detection;
mod generic_skill_detection;
mod git_command;
mod git_export;
pub mod git_repo;
mod git_tree;
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
mod variant_fingerprint;

pub use detect::{detect_external_source_variants, DetectedExternalVariant, DetectionResult};
pub use git_repo::normalize_github_repo_url;
pub use imports::{
    import_variant_into_repo, remove_imported_variant_from_repo, ImportRemovalResult,
    ImportVariantInput, ImportVariantResult,
};
pub use models::{
    ExternalSourceRecord, ExternalSourceWarning, ExternalSourcesSnapshot,
    ImportedExternalSkillRecord, ManagedSkillMirrorManifest,
};
pub use reference_check::{find_skill_references, BlockingReferences};
pub use service::{
    add_external_source, add_external_source_with_input, fetch_external_source,
    import_external_variant, list_external_sources, remove_external_source, repair_external_import,
    update_external_import, AddExternalSourceInput, ExternalSourceSnapshotItem,
    ExternalSourcesListResponse, ExternalVariantSnapshot,
};
pub use store::ExternalSourcesStore;
