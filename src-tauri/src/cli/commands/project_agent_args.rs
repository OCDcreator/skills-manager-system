use anyhow::{anyhow, Result};
use std::collections::BTreeMap;

use crate::core::projects::store::ProjectAgentAssignment;

pub(super) fn build_layered_agent_map(
    agent_keys: &[String],
    skill_ids: &[String],
    agent_skill_ids: &[String],
    agent_scene_ids: &[String],
    agent_excluded_skill_ids: &[String],
) -> Result<Option<BTreeMap<String, ProjectAgentAssignment>>> {
    let has_layered_args = !agent_skill_ids.is_empty()
        || !agent_scene_ids.is_empty()
        || !agent_excluded_skill_ids.is_empty();
    if !has_layered_args {
        return Ok(None);
    }

    let mut agents = BTreeMap::<String, ProjectAgentAssignment>::new();
    for agent_key in agent_keys {
        let entry = agents.entry(normalize_agent_key(agent_key)?).or_default();
        entry.selected_skill_ids.extend(skill_ids.iter().cloned());
    }
    for pair in agent_skill_ids {
        let (agent_key, value) = parse_agent_pair("--agent-skill", pair)?;
        agents
            .entry(agent_key)
            .or_default()
            .selected_skill_ids
            .push(value);
    }
    for pair in agent_scene_ids {
        let (agent_key, value) = parse_agent_pair("--agent-scene", pair)?;
        agents
            .entry(agent_key)
            .or_default()
            .selected_scene_ids
            .push(value);
    }
    for pair in agent_excluded_skill_ids {
        let (agent_key, value) = parse_agent_pair("--agent-exclude", pair)?;
        agents
            .entry(agent_key)
            .or_default()
            .excluded_skill_ids
            .push(value);
    }

    Ok(Some(agents))
}

fn parse_agent_pair(flag: &str, pair: &str) -> Result<(String, String)> {
    let Some((agent_key, value)) = pair.split_once('=') else {
        return Err(anyhow!("{flag} must use AGENT=VALUE syntax"));
    };
    let agent_key = normalize_agent_key(agent_key)?;
    let value = value.trim();
    if value.is_empty() {
        return Err(anyhow!("{flag} value cannot be empty"));
    }

    Ok((agent_key, value.to_string()))
}

fn normalize_agent_key(agent_key: &str) -> Result<String> {
    let agent_key = agent_key.trim();
    if agent_key.is_empty() {
        return Err(anyhow!("agent key cannot be empty"));
    }

    Ok(agent_key.to_string())
}
