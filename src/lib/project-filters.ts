import { getExternalGroupKey, type ExternalGroupFilter } from "./scene-skill-filters";
import { matchesSkillPathFilter, type SkillPathFilter } from "./skills/filters";
import type { AgentInventoryItem, SkillSummary } from "./tauri";

export type ProjectAgentStatusFilter = "all" | "enabled" | "disabled";
export type ProjectSkillSelectionFilter = "all" | "selected" | "unselected";

export function filterProjectSkills(
  skills: SkillSummary[],
  query: string,
  pathFilter: SkillPathFilter = "all",
  externalGroupFilter: ExternalGroupFilter = "all",
  selectionFilter: ProjectSkillSelectionFilter = "all",
  selectedSkillIds: string[] = [],
) {
  const needle = query.trim().toLowerCase();
  const selectedSkillIdSet = new Set(selectedSkillIds);
  return skills.filter((skill) => {
    if (!matchesSkillPathFilter(skill, pathFilter)) {
      return false;
    }
    if (
      pathFilter === "external" &&
      externalGroupFilter !== "all" &&
      getExternalGroupKey(skill) !== externalGroupFilter
    ) {
      return false;
    }

    const isSelected = selectedSkillIdSet.has(skill.id);
    if (selectionFilter === "selected" && !isSelected) {
      return false;
    }
    if (selectionFilter === "unselected" && isSelected) {
      return false;
    }

    if (!needle) {
      return true;
    }

    return `${skill.name} ${skill.description} ${skill.relativePath}`
      .toLowerCase()
      .includes(needle);
  });
}

export function filterProjectAgents(
  agents: AgentInventoryItem[],
  query: string,
  statusFilter: ProjectAgentStatusFilter = "all",
) {
  const needle = query.trim().toLowerCase();
  return agents.filter((agent) => {
    if (statusFilter === "enabled" && !agent.enabled) {
      return false;
    }

    if (statusFilter === "disabled" && agent.enabled) {
      return false;
    }

    if (!needle) {
      return true;
    }

    return `${agent.displayName} ${agent.key} ${agent.projectSkillsDirRule}`
      .toLowerCase()
      .includes(needle);
  });
}

export function sortProjectAgentsForEditor(
  agents: AgentInventoryItem[],
  selectedAgentKeys: string[],
  prioritizeSelected: boolean,
) {
  if (!prioritizeSelected || selectedAgentKeys.length === 0) {
    return agents;
  }

  const selected = new Set(selectedAgentKeys);
  return [...agents].sort((left, right) => {
    const leftSelected = selected.has(left.key);
    const rightSelected = selected.has(right.key);

    if (leftSelected !== rightSelected) {
      return leftSelected ? -1 : 1;
    }

    return 0;
  });
}
