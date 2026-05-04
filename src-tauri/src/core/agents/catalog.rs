#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentCatalogEntry {
    pub key: &'static str,
    pub display_name: &'static str,
    pub skills_dir_rule: &'static str,
    pub detect_dir_rule: &'static str,
}

const AGENT_CATALOG: [AgentCatalogEntry; 12] = [
    AgentCatalogEntry {
        key: "codex",
        display_name: "Codex",
        skills_dir_rule: ".codex/skills",
        detect_dir_rule: ".codex",
    },
    AgentCatalogEntry {
        key: "claude_code",
        display_name: "Claude Code",
        skills_dir_rule: ".claude/skills",
        detect_dir_rule: ".claude",
    },
    AgentCatalogEntry {
        key: "opencode",
        display_name: "OpenCode",
        skills_dir_rule: ".config/opencode/skills",
        detect_dir_rule: ".config/opencode",
    },
    AgentCatalogEntry {
        key: "cursor",
        display_name: "Cursor",
        skills_dir_rule: ".claude/skills",
        detect_dir_rule: ".cursor",
    },
    AgentCatalogEntry {
        key: "amp",
        display_name: "Amp",
        skills_dir_rule: ".config/amp/skills",
        detect_dir_rule: ".config/amp",
    },
    AgentCatalogEntry {
        key: "kilo_code",
        display_name: "Kilo Code",
        skills_dir_rule: ".kilo/skills",
        detect_dir_rule: ".kilo",
    },
    AgentCatalogEntry {
        key: "kimi",
        display_name: "Kimi Code CLI",
        skills_dir_rule: ".kimi/skills",
        detect_dir_rule: ".kimi",
    },
    AgentCatalogEntry {
        key: "roo_code",
        display_name: "Roo Code",
        skills_dir_rule: ".roo/rules",
        detect_dir_rule: ".roo",
    },
    AgentCatalogEntry {
        key: "goose",
        display_name: "Goose",
        skills_dir_rule: ".config/goose",
        detect_dir_rule: ".config/goose",
    },
    AgentCatalogEntry {
        key: "gemini_cli",
        display_name: "Gemini CLI",
        skills_dir_rule: ".gemini/skills",
        detect_dir_rule: ".gemini",
    },
    AgentCatalogEntry {
        key: "github_copilot",
        display_name: "GitHub Copilot",
        skills_dir_rule: ".copilot",
        detect_dir_rule: ".copilot",
    },
    AgentCatalogEntry {
        key: "windsurf",
        display_name: "Windsurf",
        skills_dir_rule: ".codeium/windsurf",
        detect_dir_rule: ".codeium/windsurf",
    },
];

pub fn agent_catalog() -> &'static [AgentCatalogEntry] {
    &AGENT_CATALOG
}

pub fn find_agent(key: &str) -> Option<&'static AgentCatalogEntry> {
    agent_catalog().iter().find(|agent| agent.key == key)
}

pub fn project_skills_dir_rule(agent: &AgentCatalogEntry) -> &'static str {
    match agent.key {
        "opencode" => ".opencode/skills",
        "cursor" => ".cursor/skills",
        _ => agent.skills_dir_rule,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_contains_all_supported_agents() {
        let keys: Vec<&str> = agent_catalog().iter().map(|agent| agent.key).collect();

        assert_eq!(
            keys,
            vec![
                "codex",
                "claude_code",
                "opencode",
                "cursor",
                "amp",
                "kilo_code",
                "kimi",
                "roo_code",
                "goose",
                "gemini_cli",
                "github_copilot",
                "windsurf",
            ]
        );
    }

    #[test]
    fn project_local_rules_can_differ_from_global_rules() {
        let opencode = find_agent("opencode").unwrap();
        let cursor = find_agent("cursor").unwrap();
        let claude = find_agent("claude_code").unwrap();

        assert_eq!(opencode.skills_dir_rule, ".config/opencode/skills");
        assert_eq!(project_skills_dir_rule(opencode), ".opencode/skills");
        assert_eq!(project_skills_dir_rule(cursor), ".cursor/skills");
        assert_eq!(project_skills_dir_rule(claude), ".claude/skills");
    }
}
