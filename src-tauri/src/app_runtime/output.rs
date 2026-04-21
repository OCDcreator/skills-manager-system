use serde::Serialize;
use serde_json::{json, Value};
use std::path::Path;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use super::context::{normalize_output_path, AppRuntimeContext};
use super::output_errors::{CliCommandError, CliErrorBody, CliExitStatus};

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CliStatus {
    Success,
    Partial,
    Error,
}

impl CliStatus {
    fn headline(self) -> &'static str {
        match self {
            Self::Success => "SUCCESS",
            Self::Partial => "PARTIAL",
            Self::Error => "ERROR",
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CliMeta {
    pub config_dir: String,
    pub repo_path: Option<String>,
    pub timestamp: String,
    pub version: String,
}

impl CliMeta {
    pub fn from_context(context: &AppRuntimeContext, repo_path: Option<&Path>) -> Self {
        Self::from_paths(Some(context.config_dir.as_path()), repo_path)
    }

    pub fn from_paths(config_dir: Option<&Path>, repo_path: Option<&Path>) -> Self {
        Self {
            config_dir: config_dir.map(normalize_output_path).unwrap_or_default(),
            repo_path: repo_path.map(normalize_output_path),
            timestamp: OffsetDateTime::now_utc()
                .format(&Rfc3339)
                .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string()),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CliWarning {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

impl CliWarning {
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
        target: Option<String>,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            target,
            details: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CliResponse {
    pub ok: bool,
    pub status: CliStatus,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<CliErrorBody>,
    pub warnings: Vec<CliWarning>,
    pub meta: CliMeta,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CliRunResult {
    pub response: CliResponse,
    pub exit_status: CliExitStatus,
}

impl CliRunResult {
    pub fn success(
        command: &str,
        data: Value,
        context: &AppRuntimeContext,
        repo_path: Option<&Path>,
    ) -> Self {
        Self::ok(
            command,
            CliStatus::Success,
            CliExitStatus::Success,
            data,
            Vec::new(),
            CliMeta::from_context(context, repo_path),
        )
    }

    pub fn ok_with_warnings(
        command: &str,
        data: Value,
        warnings: Vec<CliWarning>,
        context: &AppRuntimeContext,
        repo_path: Option<&Path>,
    ) -> Self {
        if warnings.is_empty() {
            Self::success(command, data, context, repo_path)
        } else {
            Self::ok(
                command,
                CliStatus::Partial,
                CliExitStatus::PartialSuccess,
                data,
                warnings,
                CliMeta::from_context(context, repo_path),
            )
        }
    }

    pub fn error(
        command: &str,
        error: CliCommandError,
        context: &AppRuntimeContext,
        repo_path: Option<&Path>,
    ) -> Self {
        Self::error_with_meta(command, error, CliMeta::from_context(context, repo_path))
    }

    pub fn bootstrap_error(
        command: &str,
        error: CliCommandError,
        config_dir: Option<&Path>,
        repo_path: Option<&Path>,
    ) -> Self {
        Self::error_with_meta(command, error, CliMeta::from_paths(config_dir, repo_path))
    }

    pub fn render(&self, pretty: bool) -> String {
        if pretty {
            self.render_pretty()
        } else {
            serde_json::to_string(&self.response).unwrap_or_else(|_| {
                json!({
                    "ok": false,
                    "status": "error",
                    "command": self.response.command,
                    "error": {
                        "code": "serialization_error",
                        "message": "Failed to serialize CLI response."
                    },
                    "warnings": [],
                    "meta": self.response.meta
                })
                .to_string()
            })
        }
    }

    pub fn code(&self) -> i32 {
        self.exit_status.code()
    }

    fn ok(
        command: &str,
        status: CliStatus,
        exit_status: CliExitStatus,
        data: Value,
        warnings: Vec<CliWarning>,
        meta: CliMeta,
    ) -> Self {
        Self {
            response: CliResponse {
                ok: true,
                status,
                command: command.to_string(),
                data: Some(data),
                error: None,
                warnings,
                meta,
            },
            exit_status,
        }
    }

    fn error_with_meta(command: &str, error: CliCommandError, meta: CliMeta) -> Self {
        Self {
            response: CliResponse {
                ok: false,
                status: CliStatus::Error,
                command: command.to_string(),
                data: None,
                error: Some(CliErrorBody {
                    code: error.code,
                    message: error.message,
                    details: error.details,
                }),
                warnings: Vec::new(),
                meta,
            },
            exit_status: error.exit_status,
        }
    }

    fn render_pretty(&self) -> String {
        let mut sections = vec![format!(
            "{} {}",
            self.response.status.headline(),
            self.response.command
        )];

        if let Some(error) = &self.response.error {
            sections.push(format!("{} ({})", error.message, error.code));
        }

        if let Some(data) = &self.response.data {
            sections.push(serde_json::to_string_pretty(data).unwrap_or_else(|_| "{}".to_string()));
        }

        if !self.response.warnings.is_empty() {
            let warnings = self
                .response
                .warnings
                .iter()
                .map(|warning| match &warning.target {
                    Some(target) => format!("- {} [{}]", warning.message, target),
                    None => format!("- {}", warning.message),
                })
                .collect::<Vec<_>>()
                .join("\n");
            sections.push(format!("Warnings:\n{warnings}"));
        }

        sections.push(format!("Config: {}", self.response.meta.config_dir));

        sections.join("\n\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warning_object_serializes_with_stable_shape() {
        let warning = CliWarning::new(
            "agent_target_missing",
            "No detected default path or override is configured for this agent.",
            Some("cursor".to_string()),
        );

        assert_eq!(
            serde_json::to_value(&warning).unwrap(),
            json!({
                "code": "agent_target_missing",
                "message": "No detected default path or override is configured for this agent.",
                "target": "cursor"
            })
        );
    }
}
