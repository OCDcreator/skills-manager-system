use anyhow::{anyhow, Context, Result};
use std::path::{Path, PathBuf};

use super::config_lock::{acquire_config_lock, ConfigLockError, ConfigLockGuard};
use crate::core::platform_paths::portable_path_string;
use crate::core::settings::{AgentSyncMode, AppSettings, SettingsStore};

pub const PRODUCTION_APP_IDENTIFIER: &str = "com.ocdcreator.skills-manager-system";
pub const DEVELOPMENT_APP_IDENTIFIER: &str = "com.ocdcreator.skills-manager-system.dev";

#[derive(Debug, Clone, Default)]
pub struct AppRuntimeOptions {
    pub config_dir_override: Option<PathBuf>,
    pub repo_override: Option<PathBuf>,
    pub pretty: bool,
    pub quiet: bool,
    pub no_color: bool,
}

#[derive(Debug, Clone)]
pub struct AppRuntimeContext {
    pub config_dir: PathBuf,
    pub repo_override: Option<PathBuf>,
    pub pretty: bool,
    pub quiet: bool,
    pub no_color: bool,
}

impl AppRuntimeContext {
    pub fn from_options(options: AppRuntimeOptions) -> Result<Self> {
        let config_root = dirs::config_dir().context("Failed to determine config directory")?;
        Ok(Self::from_options_with_parts(
            options,
            config_root,
            active_app_identifier(),
        ))
    }

    pub fn from_options_with_parts(
        options: AppRuntimeOptions,
        config_root: PathBuf,
        app_identifier: &'static str,
    ) -> Self {
        let config_dir = options
            .config_dir_override
            .clone()
            .unwrap_or_else(|| tauri_app_config_dir(&config_root, app_identifier));

        Self {
            config_dir,
            repo_override: options.repo_override,
            pretty: options.pretty,
            quiet: options.quiet,
            no_color: options.no_color,
        }
    }

    pub fn load_settings(&self) -> Result<AppSettings> {
        SettingsStore::new(self.config_dir.clone()).load()
    }

    pub fn current_repo_path(&self) -> Result<Option<PathBuf>> {
        if let Some(repo_override) = &self.repo_override {
            return Ok(Some(repo_override.clone()));
        }

        Ok(self.load_settings()?.repo_path.map(PathBuf::from))
    }

    pub fn require_repo_path(&self) -> Result<PathBuf> {
        self.current_repo_path()?
            .ok_or_else(|| anyhow!("Repository path is not configured"))
    }

    pub fn resolve_sync_mode(&self, raw: Option<&str>) -> Result<AgentSyncMode> {
        match raw {
            Some("copy") => Ok(AgentSyncMode::Copy),
            Some("symlink") => Ok(AgentSyncMode::Symlink),
            Some(other) => Err(anyhow!("Unsupported sync mode: {other}")),
            None => Ok(self.load_settings()?.agent_sync_mode),
        }
    }

    pub fn acquire_config_lock(&self) -> std::result::Result<ConfigLockGuard, ConfigLockError> {
        acquire_config_lock(&self.config_dir)
    }

    pub fn with_config_lock<T>(&self, operation: impl FnOnce() -> Result<T>) -> Result<T> {
        let _guard = self.acquire_config_lock()?;
        operation()
    }
}

pub fn active_app_identifier() -> &'static str {
    if cfg!(debug_assertions) {
        DEVELOPMENT_APP_IDENTIFIER
    } else {
        PRODUCTION_APP_IDENTIFIER
    }
}

pub fn tauri_app_config_dir(config_root: &Path, app_identifier: &str) -> PathBuf {
    config_root.join(app_identifier)
}

pub fn normalize_output_path(path: &Path) -> String {
    portable_path_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_runtime::ConfigLockErrorKind;
    use crate::core::settings::SettingsStore;
    use tempfile::tempdir;

    #[test]
    fn resolves_production_identifier_config_dir() {
        let context = AppRuntimeContext::from_options_with_parts(
            AppRuntimeOptions::default(),
            PathBuf::from("/Users/example/Library/Application Support"),
            PRODUCTION_APP_IDENTIFIER,
        );

        assert_eq!(
            context.config_dir,
            PathBuf::from(
                "/Users/example/Library/Application Support/com.ocdcreator.skills-manager-system",
            )
        );
    }

    #[test]
    fn config_dir_override_wins_over_default_resolution() {
        let config_override = PathBuf::from("/tmp/skills-manager-tests/config");
        let context = AppRuntimeContext::from_options_with_parts(
            AppRuntimeOptions {
                config_dir_override: Some(config_override.clone()),
                ..AppRuntimeOptions::default()
            },
            PathBuf::from("/Users/example/Library/Application Support"),
            DEVELOPMENT_APP_IDENTIFIER,
        );

        assert_eq!(context.config_dir, config_override);
    }

    #[test]
    fn repo_override_wins_over_saved_setting() {
        let config_dir = tempdir().unwrap();
        let saved_repo = config_dir.path().join("saved-repo");
        let override_repo = config_dir.path().join("override-repo");

        SettingsStore::new(config_dir.path().to_path_buf())
            .save_repo_path(Some(saved_repo.as_path()))
            .unwrap();

        let context = AppRuntimeContext::from_options_with_parts(
            AppRuntimeOptions {
                config_dir_override: Some(config_dir.path().to_path_buf()),
                repo_override: Some(override_repo.clone()),
                ..AppRuntimeOptions::default()
            },
            PathBuf::from("/unused"),
            DEVELOPMENT_APP_IDENTIFIER,
        );

        assert_eq!(context.current_repo_path().unwrap(), Some(override_repo));
    }

    #[cfg(windows)]
    #[test]
    fn windows_paths_normalize_to_forward_slashes() {
        let normalized = normalize_output_path(Path::new(
            r"C:\Users\lt\Desktop\Write\custom-project\my-skills",
        ));

        assert_eq!(
            normalized,
            "C:/Users/lt/Desktop/Write/custom-project/my-skills"
        );
    }

    #[cfg(not(windows))]
    #[test]
    fn unix_paths_preserve_backslash_filename_characters() {
        let normalized = normalize_output_path(Path::new("/Users/lt/app\\name/"));

        assert_eq!(normalized, "/Users/lt/app\\name");
    }

    #[test]
    fn with_config_lock_releases_after_failed_operation() {
        let config_dir = tempdir().unwrap();
        let context = AppRuntimeContext::from_options_with_parts(
            AppRuntimeOptions {
                config_dir_override: Some(config_dir.path().to_path_buf()),
                ..AppRuntimeOptions::default()
            },
            PathBuf::from("/unused"),
            DEVELOPMENT_APP_IDENTIFIER,
        );

        let failure: Result<()> = context.with_config_lock(|| Err(anyhow!("boom")));
        assert!(failure.unwrap_err().to_string().contains("boom"));

        let _guard = context.acquire_config_lock().unwrap();
    }

    #[test]
    fn context_lock_reports_typed_conflict() {
        let config_dir = tempdir().unwrap();
        let context = AppRuntimeContext::from_options_with_parts(
            AppRuntimeOptions {
                config_dir_override: Some(config_dir.path().to_path_buf()),
                ..AppRuntimeOptions::default()
            },
            PathBuf::from("/unused"),
            DEVELOPMENT_APP_IDENTIFIER,
        );

        let _guard = context.acquire_config_lock().unwrap();
        let error = context.acquire_config_lock().unwrap_err();

        assert_eq!(error.kind(), ConfigLockErrorKind::AlreadyHeld);
    }
}
