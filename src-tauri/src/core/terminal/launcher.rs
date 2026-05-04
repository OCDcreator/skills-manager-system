use std::path::PathBuf;

use anyhow::{bail, Result};

use crate::core::command_resolution::command_candidate_strings;

use super::model::{CliKey, TerminalLaunchInput};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalLaunchSpec {
    pub cli_key: CliKey,
    pub working_directory: PathBuf,
    pub cols: u16,
    pub rows: u16,
    pub program_candidates: Vec<String>,
}

fn command_candidates(base_names: &[&str]) -> Vec<String> {
    base_names
        .iter()
        .flat_map(|name| command_candidate_strings(name))
        .collect()
}

fn program_candidates_for(cli_key: CliKey) -> Vec<String> {
    match cli_key {
        CliKey::Codex => command_candidates(&["codex"]),
        CliKey::Opencode => command_candidates(&["opencode"]),
        CliKey::ClaudeCode => command_candidates(&["claude", "claude-code"]),
        CliKey::Kimi => command_candidates(&["kimi", "kimi-code"]),
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
        #[cfg(windows)]
        assert_eq!(
            spec.program_candidates,
            vec![
                "opencode.cmd".to_string(),
                "opencode.exe".to_string(),
                "opencode".to_string()
            ]
        );
        #[cfg(not(windows))]
        {
            assert_eq!(spec.program_candidates[0], "opencode");
            #[cfg(target_os = "macos")]
            assert!(spec
                .program_candidates
                .contains(&"/opt/homebrew/bin/opencode".to_string()));
        }
        assert_eq!(spec.cols, 100);
        assert_eq!(spec.rows, 28);
    }

    #[test]
    fn codex_prefers_windows_launchers_before_bare_shim() {
        let temp = tempdir().unwrap();
        let spec = build_launch_spec(TerminalLaunchInput {
            cli_key: CliKey::Codex,
            working_directory: temp.path().to_string_lossy().to_string(),
            cols: 120,
            rows: 32,
        })
        .unwrap();

        #[cfg(windows)]
        assert_eq!(
            spec.program_candidates,
            vec![
                "codex.cmd".to_string(),
                "codex.exe".to_string(),
                "codex".to_string()
            ]
        );
        #[cfg(not(windows))]
        {
            assert_eq!(spec.program_candidates[0], "codex");
            #[cfg(target_os = "macos")]
            assert!(spec
                .program_candidates
                .contains(&"/opt/homebrew/bin/codex".to_string()));
        }
    }

    #[cfg(not(windows))]
    #[test]
    fn non_windows_launcher_keeps_bare_candidate_first() {
        let candidates = command_candidates(&["codex"]);

        assert_eq!(candidates[0], "codex");
        #[cfg(target_os = "macos")]
        assert!(candidates.contains(&"/usr/local/bin/codex".to_string()));
    }
}
