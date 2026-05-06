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
        skills_dir_rule: ".cursor/skills",
        detect_dir_rule: ".cursor",
    },
    AgentCatalogEntry {
        key: "amp",
        display_name: "Amp",
        skills_dir_rule: ".config/agents/skills",
        detect_dir_rule: ".config/agents",
    },
    AgentCatalogEntry {
        key: "kilo_code",
        display_name: "Kilo Code",
        skills_dir_rule: ".kilocode/skills",
        detect_dir_rule: ".kilocode",
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
        skills_dir_rule: ".roo/skills",
        detect_dir_rule: ".roo",
    },
    AgentCatalogEntry {
        key: "goose",
        display_name: "Goose",
        skills_dir_rule: ".config/goose/skills",
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
        skills_dir_rule: ".copilot/skills",
        detect_dir_rule: ".copilot",
    },
    AgentCatalogEntry {
        key: "windsurf",
        display_name: "Windsurf",
        skills_dir_rule: ".codeium/windsurf/skills",
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

    #[test]
    fn global_rules_match_reference_project_except_kimi() {
        let expected = [
            ("cursor", ".cursor/skills", ".cursor"),
            ("amp", ".config/agents/skills", ".config/agents"),
            ("kilo_code", ".kilocode/skills", ".kilocode"),
            ("roo_code", ".roo/skills", ".roo"),
            ("goose", ".config/goose/skills", ".config/goose"),
            ("github_copilot", ".copilot/skills", ".copilot"),
            ("windsurf", ".codeium/windsurf/skills", ".codeium/windsurf"),
        ];

        for (key, skills_dir_rule, detect_dir_rule) in expected {
            let agent = find_agent(key).expect("agent should exist");
            assert_eq!(agent.skills_dir_rule, skills_dir_rule, "{key} skills dir");
            assert_eq!(agent.detect_dir_rule, detect_dir_rule, "{key} detect dir");
        }

        let kimi = find_agent("kimi").expect("kimi should exist");
        assert_eq!(kimi.skills_dir_rule, ".kimi/skills");
        assert_eq!(kimi.detect_dir_rule, ".kimi");
    }
}
