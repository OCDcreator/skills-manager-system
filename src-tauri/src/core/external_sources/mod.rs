pub mod detect;
pub mod git_repo;
mod hash;
pub mod imports;
pub mod models;
pub mod reference_check;
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
pub use store::ExternalSourcesStore;
