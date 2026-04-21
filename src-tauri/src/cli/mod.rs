pub mod args;

pub mod commands {
    pub mod agents;
    pub mod settings;
    pub mod skills;
    #[cfg(test)]
    mod skills_tests;
}

use crate::app_runtime::{AppRuntimeContext, AppRuntimeOptions, CliCommandError, CliRunResult};

pub use args::CliArgs;
use args::RootCommand;

pub fn run(args: CliArgs) -> CliRunResult {
    let command_name = args.command.label();
    let config_dir_override = args.config_dir.clone();
    let repo_override = args.repo.clone();

    let context = match AppRuntimeContext::from_options(AppRuntimeOptions {
        config_dir_override: config_dir_override.clone(),
        repo_override: repo_override.clone(),
        pretty: args.pretty,
        quiet: args.quiet,
        no_color: args.no_color,
    }) {
        Ok(context) => context,
        Err(error) => {
            return CliRunResult::bootstrap_error(
                command_name,
                CliCommandError::filesystem(
                    "config_dir_unavailable",
                    format!("Failed to resolve the application config directory: {error}"),
                ),
                config_dir_override.as_deref(),
                repo_override.as_deref(),
            );
        }
    };

    match &args.command {
        RootCommand::Settings { command } => commands::settings::run(&context, command),
        RootCommand::Skills { command } => commands::skills::run(&context, command),
        RootCommand::Agents { command } => commands::agents::run(&context, command),
    }
}
