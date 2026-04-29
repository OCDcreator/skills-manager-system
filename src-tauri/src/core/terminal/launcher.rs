use std::path::PathBuf;

use anyhow::{bail, Result};

use super::model::{CliKey, TerminalLaunchInput};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalLaunchSpec {
    pub cli_key: CliKey,
    pub working_directory: PathBuf,
    pub cols: u16,
    pub rows: u16,
    pub program_candidates: Vec<String>,
}

fn program_candidates_for(cli_key: CliKey) -> Vec<String> {
    match cli_key {
        CliKey::Codex => vec!["codex".into()],
        CliKey::Opencode => vec!["opencode".into()],
        CliKey::ClaudeCode => vec!["claude".into(), "claude-code".into()],
        CliKey::Kimi => vec!["kimi".into(), "kimi-code".into()],
    }
}

pub fn build_launch_spec(input: TerminalLaunchInput) -> Result<TerminalLaunchSpec> {
    let working_directory = PathBuf::from(&input.working_directory);
    if !working_directory.is_dir() {
        bail!("Selected working directory is missing or not a directory");
    }
    if input.cols == 0 || input.rows == 0 {
        bail!("Terminal dimensions must be greater than zero");
    }

    Ok(TerminalLaunchSpec {
        cli_key: input.cli_key,
        working_directory,
        cols: input.cols,
        rows: input.rows,
        program_candidates: program_candidates_for(input.cli_key),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::terminal::model::CliKey;
    use tempfile::tempdir;

    #[test]
    fn launch_spec_requires_existing_directory() {
        let error = build_launch_spec(TerminalLaunchInput {
            cli_key: CliKey::Codex,
            working_directory: "Z:/missing-terminal-dir".into(),
            cols: 120,
            rows: 32,
        })
        .unwrap_err();

        assert!(error.to_string().contains("working directory"));
    }

    #[test]
    fn launch_spec_maps_known_cli_to_candidates() {
        let temp = tempdir().unwrap();
        let spec = build_launch_spec(TerminalLaunchInput {
            cli_key: CliKey::Opencode,
            working_directory: temp.path().to_string_lossy().to_string(),
            cols: 100,
            rows: 28,
        })
        .unwrap();

        assert_eq!(spec.cli_key, CliKey::Opencode);
        assert_eq!(spec.program_candidates, vec!["opencode".to_string()]);
        assert_eq!(spec.cols, 100);
        assert_eq!(spec.rows, 28);
    }
}
