use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum SettingsCommand {
    GetRepoPath,
    SetRepoPath { path: String },
    GetSyncMode,
    SetSyncMode { sync_mode: String },
}

impl SettingsCommand {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::GetRepoPath => "settings get-repo-path",
            Self::SetRepoPath { .. } => "settings set-repo-path",
            Self::GetSyncMode => "settings get-sync-mode",
            Self::SetSyncMode { .. } => "settings set-sync-mode",
        }
    }
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum SkillsCommand {
    Scan,
    State,
    List,
    Doc { target: String },
    Enable { skill_id: String },
    Disable { skill_id: String },
}

impl SkillsCommand {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Scan => "skills scan",
            Self::State => "skills state",
            Self::List => "skills list",
            Self::Doc { .. } => "skills doc",
            Self::Enable { .. } => "skills enable",
            Self::Disable { .. } => "skills disable",
        }
    }
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum AgentsCommand {
    List,
    Enable {
        key: String,
    },
    Disable {
        key: String,
    },
    SetPath {
        key: String,
        path: String,
    },
    ClearPath {
        key: String,
    },
    Sync {
        #[arg(long)]
        sync_mode: Option<String>,
    },
}

impl AgentsCommand {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::List => "agents list",
            Self::Enable { .. } => "agents enable",
            Self::Disable { .. } => "agents disable",
            Self::SetPath { .. } => "agents set-path",
            Self::ClearPath { .. } => "agents clear-path",
            Self::Sync { .. } => "agents sync",
        }
    }
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum ScenesCommand {
    List,
    Create {
        id: String,
        name: String,
        #[arg(long, default_value = "")]
        description: String,
    },
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    Delete {
        id: String,
    },
    SetActive {
        id: String,
    },
    ClearActive,
    SetSkills {
        id: String,
        #[arg(value_name = "SKILL_ID", num_args = 0..)]
        skill_ids: Vec<String>,
    },
    SetAgents {
        id: String,
        #[arg(value_name = "AGENT_KEY", num_args = 0..)]
        enabled_agent_keys: Vec<String>,
    },
    SetSkillOrder {
        id: String,
        #[arg(value_name = "SKILL_ID", num_args = 0..)]
        skill_order: Vec<String>,
    },
    Apply {
        id: String,
    },
}

impl ScenesCommand {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::List => "scenes list",
            Self::Create { .. } => "scenes create",
            Self::Update { .. } => "scenes update",
            Self::Delete { .. } => "scenes delete",
            Self::SetActive { .. } => "scenes set-active",
            Self::ClearActive => "scenes clear-active",
            Self::SetSkills { .. } => "scenes set-skills",
            Self::SetAgents { .. } => "scenes set-agents",
            Self::SetSkillOrder { .. } => "scenes set-skill-order",
            Self::Apply { .. } => "scenes apply",
        }
    }
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum ProjectsCommand {
    List,
    Add {
        project_path: PathBuf,
        #[arg(long, default_value = "")]
        display_name: String,
        #[arg(long = "skill")]
        skill_ids: Vec<String>,
        #[arg(long = "agent")]
        agent_keys: Vec<String>,
    },
    Update {
        project_path: PathBuf,
        #[arg(long)]
        display_name: Option<String>,
        #[arg(long = "skill")]
        skill_ids: Vec<String>,
        #[arg(long = "agent")]
        agent_keys: Vec<String>,
    },
    Remove {
        project_path: PathBuf,
    },
    Apply,
}

impl ProjectsCommand {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::List => "projects list",
            Self::Add { .. } => "projects add",
            Self::Update { .. } => "projects update",
            Self::Remove { .. } => "projects remove",
            Self::Apply => "projects apply",
        }
    }
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum GitCommand {
    Status,
    Diff {
        #[arg(long)]
        staged: bool,
    },
    Log {
        #[arg(long, default_value_t = 20)]
        max_count: usize,
    },
    Fetch,
    Pull,
    Push,
    Commit {
        #[arg(short, long)]
        message: String,
    },
    SyncExternal,
}

impl GitCommand {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Status => "git status",
            Self::Diff { .. } => "git diff",
            Self::Log { .. } => "git log",
            Self::Fetch => "git fetch",
            Self::Pull => "git pull",
            Self::Push => "git push",
            Self::Commit { .. } => "git commit",
            Self::SyncExternal => "git sync-external",
        }
    }
}
