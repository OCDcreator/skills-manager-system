import type { ProjectAssignment, ProjectPathInspection } from "./projects";
import {
  matchesSkillPathFilter,
  type SkillPathFilter,
} from "./skills/filters";
import type { AgentInventoryItem, SkillSummary } from "./tauri";

export type ProjectAgentStatusFilter = "all" | "enabled" | "disabled";
export type ProjectSkillSelectionFilter = "all" | "selected" | "unselected";

export interface ProjectDraft {
  mode: "create" | "edit";
  sourceProjectPath: string | null;
  projectPath: string;
  displayName: string;
  displayNameManuallyEdited: boolean;
  selectedSkillIds: string[];
  selectedAgentKeys: string[];
  unsupportedAgentKeys: string[];
}

export function suggestProjectDisplayName(projectPath: string) {
  const trimmed = projectPath.trim();
  if (!trimmed) return "";
  const segments = trimmed.split(/[/\\]/).filter(Boolean);
  return segments[segments.length - 1] ?? trimmed;
}

export function isProjectDraftDirty(
  draft: ProjectDraft,
  original: ProjectAssignment | null,
) {
  if (!original) {
    return Boolean(
      draft.projectPath.trim() ||
        draft.displayName.trim() ||
        draft.selectedSkillIds.length ||
        draft.selectedAgentKeys.length ||
        draft.unsupportedAgentKeys.length,
    );
  }

  return (
    draft.displayName !== original.displayName ||
    draft.projectPath !== original.projectPath ||
    draft.selectedSkillIds.join("\n") !== original.skillIds.join("\n") ||
    [...draft.selectedAgentKeys, ...draft.unsupportedAgentKeys].join("\n") !==
      original.agentKeys.join("\n")
  );
}

export function filterProjectSkills(
  skills: SkillSummary[],
  query: string,
  pathFilter: SkillPathFilter = "all",
  selectionFilter: ProjectSkillSelectionFilter = "all",
  selectedSkillIds: string[] = [],
) {
  const needle = query.trim().toLowerCase();
  const selectedSkillIdSet = new Set(selectedSkillIds);
  return skills.filter((skill) => {
    if (!matchesSkillPathFilter(skill, pathFilter)) {
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

export function applyProjectPathToDraft(draft: ProjectDraft, projectPath: string) {
  return {
    ...draft,
    projectPath,
    displayName: draft.displayNameManuallyEdited
      ? draft.displayName
      : suggestProjectDisplayName(projectPath),
  };
}

export function applyProjectDisplayNameToDraft(
  draft: ProjectDraft,
  displayName: string,
) {
  return {
    ...draft,
    displayName,
    displayNameManuallyEdited: true,
  };
}

export function buildProjectSummary(
  draft: ProjectDraft,
  inspection: ProjectPathInspection | null,
  skills: SkillSummary[],
  agents: AgentInventoryItem[],
  disabledSkillIds: string[],
) {
  const selectedSkillIdSet = new Set(draft.selectedSkillIds);
  const selectedAgentKeySet = new Set(draft.selectedAgentKeys);
  const selectedSkills = skills.filter((skill) => selectedSkillIdSet.has(skill.id));
  const selectedAgents = agents.filter((agent) =>
    selectedAgentKeySet.has(agent.key),
  );

  return {
    selectedSkillCount: draft.selectedSkillIds.length,
    selectedAgentCount: draft.selectedAgentKeys.length,
    selectedSkills,
    selectedAgents,
    disabledSelectedSkillIds: draft.selectedSkillIds.filter((skillId) =>
      disabledSkillIds.includes(skillId),
    ),
    inspectionAgents: inspection?.agents ?? [],
    unsupportedAgentKeys: draft.unsupportedAgentKeys,
    warnings: inspection?.warnings ?? [],
  };
}
