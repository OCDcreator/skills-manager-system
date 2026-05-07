import { mergeAgentOrderWithInventory, resolveSortedAgentInventory } from "./agent-order";
import type { AgentInventoryItem, AgentKey } from "./tauri";

const AGENT_SIDE_NAV_ENABLED_ONLY_STORAGE_KEY =
  "skills-manager-system.agents.side-nav.enabled-only";

export function readAgentSideNavEnabledOnlyPreference() {
  if (typeof window === "undefined") {
    return false;
  }

  try {
    return window.localStorage.getItem(AGENT_SIDE_NAV_ENABLED_ONLY_STORAGE_KEY) === "true";
  } catch {
    return false;
  }
}

export function writeAgentSideNavEnabledOnlyPreference(enabled: boolean) {
  try {
    window.localStorage.setItem(AGENT_SIDE_NAV_ENABLED_ONLY_STORAGE_KEY, String(enabled));
  } catch {
    // Best-effort preference persistence only.
  }
}

export function applyAgentEnabledOverrides(
  agents: AgentInventoryItem[],
  enabledOverrides: Partial<Record<AgentKey, boolean>>,
) {
  return agents.map((agent) => ({
    ...agent,
    enabled: enabledOverrides[agent.key] ?? agent.enabled,
  }));
}

export function resolveAgentSideNavInventory(
  agents: AgentInventoryItem[],
  agentOrder: AgentKey[],
  showEnabledOnly: boolean,
) {
  const nextOrder = mergeAgentOrderWithInventory(agents, agentOrder);
  const visibleAgents = showEnabledOnly ? agents.filter((agent) => agent.enabled) : agents;
  return resolveSortedAgentInventory(visibleAgents, nextOrder);
}
