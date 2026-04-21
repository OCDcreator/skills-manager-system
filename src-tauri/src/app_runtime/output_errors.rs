use serde::Serialize;
use serde_json::Value;

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

    #[test]
    fn partial_status_maps_to_exit_code_8() {
        assert_eq!(CliExitStatus::PartialSuccess.code(), 8);
    }
}
