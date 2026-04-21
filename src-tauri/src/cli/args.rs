use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
}

impl RootCommand {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Settings { command } => command.label(),
            Self::Skills { command } => command.label(),
            Self::Agents { command } => command.label(),
        }
    }
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum SettingsCommand {
    GetRepoPath,
    GetSyncMode,
}

impl SettingsCommand {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::GetRepoPath => "settings get-repo-path",
            Self::GetSyncMode => "settings get-sync-mode",
        }
    }
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum SkillsCommand {
    Scan,
    State,
    List,
    Doc { target: String },
}

impl SkillsCommand {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Scan => "skills scan",
            Self::State => "skills state",
            Self::List => "skills list",
            Self::Doc { .. } => "skills doc",
        }
    }
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum AgentsCommand {
    List,
}

impl AgentsCommand {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::List => "agents list",
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
}
