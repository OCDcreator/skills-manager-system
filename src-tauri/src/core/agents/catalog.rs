#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentCatalogEntry {
    pub key: &'static str,
    pub display_name: &'static str,
    pub skills_dir_rule: &'static str,
    pub detect_dir_rule: &'static str,
}

const AGENT_CATALOG: [AgentCatalogEntry; 3] = [
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
];

pub fn agent_catalog() -> &'static [AgentCatalogEntry] {
    &AGENT_CATALOG
}

pub fn find_agent(key: &str) -> Option<&'static AgentCatalogEntry> {
    agent_catalog().iter().find(|agent| agent.key == key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_contains_only_phase_three_agents() {
        let keys: Vec<&str> = agent_catalog().iter().map(|agent| agent.key).collect();

        assert_eq!(keys, vec!["codex", "claude_code", "opencode"]);
    }
}
