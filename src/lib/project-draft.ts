import type { ProjectAssignment } from "./projects";
import {
  matchesSkillPathFilter,
  type SkillPathFilter,
} from "./skills/filters";
import type { AgentInventoryItem, SkillSummary } from "./tauri";

export type ProjectAgentStatusFilter = "all" | "enabled" | "disabled";
export type ProjectSkillSelectionFilter = "all" | "selected" | "unselected";

export interface ProjectAgentDraft {
  selectedSkillIds: string[];
  selectedSceneIds: string[];
  excludedSkillIds: string[];
}

export interface ProjectDraft {
  mode: "create" | "edit";
  sourceProjectPath: string | null;
  projectPath: string;
  displayName: string;
  displayNameManuallyEdited: boolean;
  agents: Record<string, ProjectAgentDraft>;
  unsupportedAgentKeys: string[];
}

export function suggestProjectDisplayName(projectPath: string) {
  const trimmed = projectPath.trim();
  if (!trimmed) return "";
  const segments = trimmed.split(/[/\\]/).filter(Boolean);
  return segments[segments.length - 1] ?? trimmed;
}

export function projectDraftFromAssignment(
  project: ProjectAssignment,
  agents: AgentInventoryItem[],
) {
  const supportedAgentKeys = new Set<string>(agents.map((agent) => agent.key));
  const migratedAgents = assignmentAgentDrafts(project);
  const unsupportedAgentKeys = normalizeIds([
    ...(project.unsupportedAgentKeys ?? []),
    ...assignmentAgentKeys(project).filter((key) => !supportedAgentKeys.has(key)),
  ]);

  return {
    mode: "edit" as const,
    sourceProjectPath: project.projectPath,
    projectPath: project.projectPath,
    displayName: project.displayName,
    displayNameManuallyEdited: true,
    agents: Object.fromEntries(
      Object.entries(migratedAgents).filter(([key]) => supportedAgentKeys.has(key)),
    ),
    unsupportedAgentKeys,
  };
}

export function projectDraftAgentKeys(draft: ProjectDraft) {
  return Object.keys(draft.agents).sort();
}

export function projectDraftToAgentAssignments(draft: ProjectDraft) {
  return Object.fromEntries(
    Object.entries(draft.agents).map(([agentKey, agentDraft]) => [
      agentKey,
      {
        selectedSkillIds: normalizeIds(agentDraft.selectedSkillIds),
        selectedSceneIds: normalizeIds(agentDraft.selectedSceneIds),
        excludedSkillIds: normalizeIds(agentDraft.excludedSkillIds),
      },
    ]),
  );
}

export function isProjectDraftDirty(
  draft: ProjectDraft,
  original: ProjectAssignment | null,
) {
  if (!original) {
    return Boolean(
      draft.projectPath.trim() ||
        draft.displayName.trim() ||
        projectDraftAgentKeys(draft).length ||
        draft.unsupportedAgentKeys.length,
    );
  }

  return (
    draft.displayName !== original.displayName ||
    draft.projectPath !== original.projectPath ||
    serializeAgents(draft.agents, draft.unsupportedAgentKeys) !==
      serializeAgents(assignmentAgentDrafts(original), original.unsupportedAgentKeys ?? [])
  );
}

export function ensureProjectAgentDraft(draft: ProjectDraft, agentKey: string) {
  if (draft.agents[agentKey]) {
    return draft;
  }

  return {
    ...draft,
    agents: {
      ...draft.agents,
      [agentKey]: {
        selectedSkillIds: [],
        selectedSceneIds: [],
        excludedSkillIds: [],
      },
    },
  };
}

export function removeProjectAgentDraft(draft: ProjectDraft, agentKey: string) {
  if (!draft.agents[agentKey]) {
    return draft;
  }

  const agents = { ...draft.agents };
  delete agents[agentKey];
  return { ...draft, agents };
}

export function toggleProjectAgentSkill(draft: ProjectDraft, agentKey: string, skillId: string) {
  return toggleProjectAgentList(draft, agentKey, "selectedSkillIds", skillId);
}

export function toggleProjectAgentScene(draft: ProjectDraft, agentKey: string, sceneId: string) {
  return toggleProjectAgentList(draft, agentKey, "selectedSceneIds", sceneId);
}

export function toggleProjectAgentExclusion(
  draft: ProjectDraft,
  agentKey: string,
  skillId: string,
) {
  return toggleProjectAgentList(draft, agentKey, "excludedSkillIds", skillId);
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

export function applyProjectDisplayNameToDraft(draft: ProjectDraft, displayName: string) {
  return {
    ...draft,
    displayName,
    displayNameManuallyEdited: true,
  };
}

function toggleProjectAgentList(
  draft: ProjectDraft,
  agentKey: string,
  field: keyof ProjectAgentDraft,
  id: string,
) {
  const ensured = ensureProjectAgentDraft(draft, agentKey);
  const agentDraft = ensured.agents[agentKey];
  return {
    ...ensured,
    agents: {
      ...ensured.agents,
      [agentKey]: {
        ...agentDraft,
        [field]: toggleId(agentDraft[field], id),
      },
    },
  };
}

function assignmentAgentDrafts(project: ProjectAssignment) {
  const agents = project.agents ?? {};
  if (Object.keys(agents).length > 0) {
    return Object.fromEntries(
      Object.entries(agents).map(([agentKey, assignment]) => [
        agentKey,
        {
          selectedSkillIds: normalizeIds(assignment.selectedSkillIds),
          selectedSceneIds: normalizeIds(assignment.selectedSceneIds),
          excludedSkillIds: normalizeIds(assignment.excludedSkillIds),
        },
      ]),
    );
  }

  return Object.fromEntries(
    (project.agentKeys ?? []).map((agentKey) => [
      agentKey,
      {
        selectedSkillIds: normalizeIds(project.skillIds ?? []),
        selectedSceneIds: [],
        excludedSkillIds: [],
      },
    ]),
  );
}

function assignmentAgentKeys(project: ProjectAssignment) {
  return normalizeIds([
    ...Object.keys(project.agents ?? {}),
    ...(project.agentKeys ?? []),
    ...(project.unsupportedAgentKeys ?? []),
  ]);
}

function serializeAgents(
  agents: Record<string, ProjectAgentDraft>,
  unsupportedAgentKeys: string[],
) {
  return JSON.stringify({
    agents: projectDraftToAgentAssignments({
      mode: "create",
      sourceProjectPath: null,
      projectPath: "",
      displayName: "",
      displayNameManuallyEdited: false,
      agents,
      unsupportedAgentKeys: [],
    }),
    unsupportedAgentKeys: normalizeIds(unsupportedAgentKeys),
  });
}

function toggleId(ids: string[], id: string) {
  return ids.includes(id) ? ids.filter((candidate) => candidate !== id) : normalizeIds([...ids, id]);
}

function normalizeIds(ids: string[]) {
  return [...new Set(ids.map((id) => id.trim()).filter(Boolean))].sort();
}
