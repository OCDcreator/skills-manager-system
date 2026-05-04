use serde::Serialize;
use serde_json::{json, Value};

use super::config_lock::{ConfigLockError, ConfigLockErrorKind};
use super::context::normalize_output_path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliExitStatus {
    Success,
    GeneralFailure,
    ArgumentError,
    MissingConfiguration,
    TargetNotFound,
    ExternalCommandFailure,
    FilesystemFailure,
    StateConflict,
    PartialSuccess,
}

impl CliExitStatus {
    pub const fn code(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::GeneralFailure => 1,
            Self::ArgumentError => 2,
            Self::MissingConfiguration => 3,
            Self::TargetNotFound => 4,
            Self::ExternalCommandFailure => 5,
            Self::FilesystemFailure => 6,
            Self::StateConflict => 7,
            Self::PartialSuccess => 8,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CliErrorBody {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct CliCommandError {
    pub exit_status: CliExitStatus,
    pub code: String,
    pub message: String,
    pub details: Option<Value>,
}

impl CliCommandError {
    pub fn new(
        exit_status: CliExitStatus,
        code: impl Into<String>,
        message: impl Into<String>,
        details: Option<Value>,
    ) -> Self {
        Self {
            exit_status,
            code: code.into(),
            message: message.into(),
            details,
        }
    }

    pub fn missing_configuration(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(CliExitStatus::MissingConfiguration, code, message, None)
    }

    pub fn target_not_found(
        code: impl Into<String>,
        message: impl Into<String>,
        details: Option<Value>,
    ) -> Self {
        Self::new(CliExitStatus::TargetNotFound, code, message, details)
    }

    pub fn filesystem(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(CliExitStatus::FilesystemFailure, code, message, None)
    }

    pub fn state_conflict(
        code: impl Into<String>,
        message: impl Into<String>,
        details: Option<Value>,
    ) -> Self {
        Self::new(CliExitStatus::StateConflict, code, message, details)
    }

    pub fn invalid_sync_mode(raw_sync_mode: &str) -> Self {
        Self::new(
            CliExitStatus::ArgumentError,
            "invalid_sync_mode",
            format!("Unsupported sync mode '{raw_sync_mode}'. Use 'copy' or 'symlink'."),
            Some(json!({
                "syncMode": raw_sync_mode,
                "supportedValues": ["copy", "symlink"],
            })),
        )
    }

    pub fn config_write_failed(message: impl Into<String>, details: Option<Value>) -> Self {
        Self::new(
            CliExitStatus::FilesystemFailure,
            "config_write_failed",
            message,
            details,
        )
    }

    pub fn from_config_lock(error: ConfigLockError) -> Self {
        let details = Some(json!({
            "lockPath": normalize_output_path(error.lock_path()),
        }));

        match error.kind() {
            ConfigLockErrorKind::AlreadyHeld => Self::state_conflict(
                "config_lock_held",
                "Configuration is already locked by another writer.",
                details,
            ),
            ConfigLockErrorKind::Io => Self::config_write_failed(
                format!("Failed to acquire configuration lock: {error}"),
                details,
            ),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(
            CliExitStatus::GeneralFailure,
            "internal_error",
            message,
            None,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn partial_status_maps_to_exit_code_8() {
        assert_eq!(CliExitStatus::PartialSuccess.code(), 8);
    }

    #[test]
    fn error_body_serializes_with_stable_shape() {
        let body = CliErrorBody {
            code: "config_write_failed".to_string(),
            message: "Failed to save settings.".to_string(),
            details: Some(json!({
                "path": "C:/Users/lt/AppData/Roaming/settings.json",
            })),
        };

        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({
                "code": "config_write_failed",
                "message": "Failed to save settings.",
                "details": {
                    "path": "C:/Users/lt/AppData/Roaming/settings.json"
                }
            })
        );
    }

    #[test]
    fn invalid_sync_mode_uses_stable_argument_error() {
        let error = CliCommandError::invalid_sync_mode("hardlink");

        assert_eq!(error.exit_status, CliExitStatus::ArgumentError);
        assert_eq!(error.code, "invalid_sync_mode");
        assert_eq!(
            error.details,
            Some(json!({
                "syncMode": "hardlink",
                "supportedValues": ["copy", "symlink"],
            }))
        );
    }

    #[cfg(windows)]
    #[test]
    fn config_lock_conflict_maps_to_state_conflict() {
        let error = CliCommandError::from_config_lock(ConfigLockError::already_held(
            PathBuf::from(r"C:\Users\lt\AppData\Roaming\skills\.skills-manager-system.lock"),
        ));

        assert_eq!(error.exit_status, CliExitStatus::StateConflict);
        assert_eq!(error.code, "config_lock_held");
        assert_eq!(
            error.details,
            Some(json!({
                "lockPath": "C:/Users/lt/AppData/Roaming/skills/.skills-manager-system.lock"
            }))
        );
    }
}
