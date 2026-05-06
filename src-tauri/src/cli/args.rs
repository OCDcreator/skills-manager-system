use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub use super::command_groups::{
    AgentsCommand, GitCommand, ProjectsCommand, ScenesCommand, SettingsCommand, SkillsCommand,
};

#[derive(Debug, Parser, PartialEq, Eq)]
#[command(
    name = "skills-manager",
    about = "Machine-first CLI for skills-manager-system",
    version,
    disable_help_subcommand = true
)]
pub struct CliArgs {
    #[arg(long)]
    pub json: bool,

    #[arg(long, conflicts_with = "json")]
    pub pretty: bool,

    #[arg(long)]
    pub quiet: bool,

    #[arg(long)]
    pub no_color: bool,

    #[arg(long, value_name = "PATH")]
    pub config_dir: Option<PathBuf>,

    #[arg(long, value_name = "PATH")]
    pub repo: Option<PathBuf>,

    #[command(subcommand)]
    pub command: RootCommand,
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum RootCommand {
    Settings {
        #[command(subcommand)]
        command: SettingsCommand,
    },
    Skills {
        #[command(subcommand)]
        command: SkillsCommand,
    },
    Agents {
        #[command(subcommand)]
        command: AgentsCommand,
    },
    Scenes {
        #[command(subcommand)]
        command: ScenesCommand,
    },
    Projects {
        #[command(subcommand)]
        command: ProjectsCommand,
    },
    Git {
        #[command(subcommand)]
        command: GitCommand,
    },
}

impl RootCommand {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Settings { command } => command.label(),
            Self::Skills { command } => command.label(),
            Self::Agents { command } => command.label(),
            Self::Scenes { command } => command.label(),
            Self::Projects { command } => command.label(),
            Self::Git { command } => command.label(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_global_pretty_and_settings_subcommand() {
        let args =
            CliArgs::try_parse_from(["skills-manager", "--pretty", "settings", "get-repo-path"])
                .unwrap();

        assert!(args.pretty);
        assert_eq!(
            args.command,
            RootCommand::Settings {
                command: SettingsCommand::GetRepoPath,
            }
        );
    }

    #[test]
    fn parses_skills_list_with_repo_override() {
        let args =
            CliArgs::try_parse_from(["skills-manager", "--repo", "/tmp/repo", "skills", "list"])
                .unwrap();

        assert_eq!(args.repo, Some(PathBuf::from("/tmp/repo")));
        assert_eq!(
            args.command,
            RootCommand::Skills {
                command: SkillsCommand::List,
            }
        );
    }

    #[test]
    fn parses_agents_list_without_mutation_flags() {
        let args = CliArgs::try_parse_from(["skills-manager", "agents", "list"]).unwrap();

        assert_eq!(
            args.command,
            RootCommand::Agents {
                command: AgentsCommand::List,
            }
        );
        assert!(!args.pretty);
        assert!(!args.quiet);
    }

    #[test]
    fn parses_mutation_and_extended_command_groups() {
        let settings =
            CliArgs::try_parse_from(["skills-manager", "settings", "set-sync-mode", "copy"])
                .unwrap();
        assert_eq!(
            settings.command,
            RootCommand::Settings {
                command: SettingsCommand::SetSyncMode {
                    sync_mode: "copy".to_string(),
                },
            }
        );

        let scenes = CliArgs::try_parse_from([
            "skills-manager",
            "scenes",
            "set-skills",
            "focus",
            "custom:one",
            "external:two",
        ])
        .unwrap();
        assert_eq!(
            scenes.command,
            RootCommand::Scenes {
                command: ScenesCommand::SetSkills {
                    id: "focus".to_string(),
                    skill_ids: vec!["custom:one".to_string(), "external:two".to_string()],
                },
            }
        );

        let git =
            CliArgs::try_parse_from(["skills-manager", "git", "commit", "-m", "sync"]).unwrap();
        assert_eq!(
            git.command,
            RootCommand::Git {
                command: GitCommand::Commit {
                    message: "sync".to_string(),
                },
            }
        );
    }

    #[test]
    fn projects_apply_rejects_path_argument() {
        let error = CliArgs::try_parse_from([
            "skills-manager",
            "projects",
            "apply",
            "/tmp/should-not-be-accepted",
        ])
        .unwrap_err();

        assert_eq!(error.kind(), clap::error::ErrorKind::UnknownArgument);
    }

    #[test]
    fn parses_project_layer_flags() {
        let args = CliArgs::try_parse_from([
            "skills-manager",
            "projects",
            "add",
            "/tmp/app",
            "--agent",
            "codex",
            "--skill",
            "custom:legacy",
            "--agent-scene",
            "codex=focus",
            "--agent-skill",
            "opencode=custom:direct",
            "--agent-exclude",
            "codex=custom:legacy",
        ])
        .unwrap();

        assert_eq!(
            args.command,
            RootCommand::Projects {
                command: ProjectsCommand::Add {
                    project_path: PathBuf::from("/tmp/app"),
                    display_name: String::new(),
                    skill_ids: vec!["custom:legacy".to_string()],
                    agent_keys: vec!["codex".to_string()],
                    agent_skill_ids: vec!["opencode=custom:direct".to_string()],
                    agent_scene_ids: vec!["codex=focus".to_string()],
                    agent_excluded_skill_ids: vec!["codex=custom:legacy".to_string()],
                },
            }
        );
    }
}
