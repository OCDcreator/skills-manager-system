use std::path::PathBuf;
use std::process::{Command, Output};
use std::{env, io};

const MACOS_COMMAND_DIRS: &[&str] = &[
    "/opt/homebrew/bin",
    "/usr/local/bin",
    "/usr/bin",
    "/bin",
    "/opt/homebrew/sbin",
    "/usr/local/sbin",
    "/usr/sbin",
    "/sbin",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandResolution {
    command_name: String,
    candidates: Vec<PathBuf>,
    resolved_path: Option<PathBuf>,
}

impl CommandResolution {
    pub fn attempted_candidates(&self) -> &[PathBuf] {
        &self.candidates
    }

    pub fn resolved_path(&self) -> Option<&PathBuf> {
        self.resolved_path.as_ref()
    }

    pub fn spawn_error_message(&self, error: &std::io::Error) -> String {
        let candidates = self
            .candidates
            .iter()
            .map(|candidate| candidate.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "Failed to execute {}: {}. Tried candidates: {}",
            self.command_name, error, candidates
        )
    }
}

fn resolution_for(command_name: &str) -> CommandResolution {
    CommandResolution {
        command_name: command_name.to_string(),
        candidates: candidate_executable_paths(command_name),
        resolved_path: None,
    }
}

pub fn candidate_executable_paths(command_name: &str) -> Vec<PathBuf> {
    command_candidate_strings(command_name)
        .into_iter()
        .map(PathBuf::from)
        .collect()
}

pub fn command_candidate_strings(command_name: &str) -> Vec<String> {
    if cfg!(windows) {
        return vec![
            format!("{command_name}.cmd"),
            format!("{command_name}.exe"),
            command_name.to_string(),
        ];
    }

    let mut candidates = vec![command_name.to_string()];
    if cfg!(target_os = "macos") {
        candidates.extend(
            MACOS_COMMAND_DIRS
                .iter()
                .map(|directory| format!("{directory}/{command_name}")),
        );
    }
    candidates
}

pub fn resolved_command(command_name: &str) -> (Command, CommandResolution) {
    let candidates = candidate_executable_paths(command_name);
    let resolved_path = candidates
        .iter()
        .find(|candidate| !candidate.is_absolute() && path_can_resolve(candidate))
        .cloned()
        .or_else(|| {
            candidates
                .iter()
                .find(|candidate| candidate.is_absolute() && candidate.exists())
                .cloned()
        });
    let executable = resolved_path
        .as_ref()
        .cloned()
        .unwrap_or_else(|| candidates[0].clone());

    (
        Command::new(executable),
        CommandResolution {
            command_name: command_name.to_string(),
            candidates,
            resolved_path,
        },
    )
}

pub fn git_command() -> (Command, CommandResolution) {
    resolved_command("git")
}

pub fn command_output<F>(command_name: &str, configure: F) -> io::Result<Output>
where
    F: Fn(&mut Command),
{
    let resolution = resolution_for(command_name);
    let mut last_error = None;

    for (index, candidate) in resolution.attempted_candidates().iter().enumerate() {
        let mut command = Command::new(candidate);
        configure(&mut command);

        match command.output() {
            Ok(output) => return Ok(output),
            Err(error)
                if error.kind() == io::ErrorKind::NotFound
                    && index + 1 < resolution.attempted_candidates().len() =>
            {
                last_error = Some(error);
            }
            Err(error) => {
                return Err(io::Error::new(
                    error.kind(),
                    resolution.spawn_error_message(&error),
                ));
            }
        }
    }

    let error = last_error.unwrap_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("{command_name} was not found"),
        )
    });
    Err(io::Error::new(
        error.kind(),
        resolution.spawn_error_message(&error),
    ))
}

fn path_can_resolve(command_name: &PathBuf) -> bool {
    env::var_os("PATH")
        .map(|path| {
            env::split_paths(&path)
                .map(|directory| directory.join(command_name))
                .any(|candidate| candidate.exists())
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn windows_candidates_prefer_cmd_then_exe_then_bare_name() {
        assert_eq!(
            command_candidate_strings("git"),
            vec![
                "git.cmd".to_string(),
                "git.exe".to_string(),
                "git".to_string()
            ]
        );
    }

    #[cfg(all(not(windows), not(target_os = "macos")))]
    #[test]
    fn non_windows_non_macos_candidates_keep_bare_name_only() {
        assert_eq!(command_candidate_strings("git"), vec!["git".to_string()]);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn macos_candidates_keep_bare_name_before_common_gui_missing_path_locations() {
        assert_eq!(
            command_candidate_strings("git"),
            vec![
                "git".to_string(),
                "/opt/homebrew/bin/git".to_string(),
                "/usr/local/bin/git".to_string(),
                "/usr/bin/git".to_string(),
                "/bin/git".to_string(),
                "/opt/homebrew/sbin/git".to_string(),
                "/usr/local/sbin/git".to_string(),
                "/usr/sbin/git".to_string(),
                "/sbin/git".to_string(),
            ]
        );
    }

    #[test]
    fn resolved_command_falls_back_to_bare_command_when_no_absolute_candidate_exists() {
        let (_command, resolution) = resolved_command("definitely-missing-skills-manager-command");

        assert!(resolution.resolved_path().is_none());
        assert!(resolution.attempted_candidates().iter().any(|candidate| {
            candidate == &PathBuf::from("definitely-missing-skills-manager-command")
        }));
    }

    #[test]
    fn diagnostics_include_command_error_and_candidates() {
        let error = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let message = resolved_command("definitely-missing-skills-manager-command")
            .1
            .spawn_error_message(&error);

        assert!(message.contains("Failed to execute definitely-missing-skills-manager-command"));
        assert!(message.contains("not found"));
        assert!(message.contains("Tried candidates:"));
        assert!(message.contains("definitely-missing-skills-manager-command"));
    }

    #[test]
    fn command_output_reports_candidate_diagnostics_for_missing_command() {
        let error =
            command_output("definitely-missing-skills-manager-command", |_| {}).unwrap_err();
        let message = error.to_string();

        assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
        assert!(message.contains("Failed to execute definitely-missing-skills-manager-command"));
        assert!(message.contains("Tried candidates:"));
    }
}
