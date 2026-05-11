import type { ProjectAssignment } from "./projects";
import type { AgentInventoryItem } from "./tauri";

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
  const ensured = ensureProjectAgentDraft(draft, agentKey);
  const agentDraft = ensured.agents[agentKey];
  const wasSelected = agentDraft.selectedSkillIds.includes(skillId);

  return {
    ...ensured,
    agents: {
      ...ensured.agents,
      [agentKey]: {
        ...agentDraft,
        selectedSkillIds: toggleId(agentDraft.selectedSkillIds, skillId),
        excludedSkillIds: wasSelected
          ? agentDraft.excludedSkillIds
          : removeId(agentDraft.excludedSkillIds, skillId),
      },
    },
  };
}

export function toggleProjectAgentScene(draft: ProjectDraft, agentKey: string, sceneId: string) {
  return toggleProjectAgentList(draft, agentKey, "selectedSceneIds", sceneId);
}

export function toggleProjectAgentSkillForAgents(
  draft: ProjectDraft,
  agentKeys: string[],
  skillId: string,
) {
  const shouldSelect = !agentKeys.every((key) => {
    const agent = draft.agents[key];
    return agent && agent.selectedSkillIds.includes(skillId);
  });

  let next = draft;
  for (const agentKey of agentKeys) {
    if (shouldSelect) {
      const ensured = ensureProjectAgentDraft(next, agentKey);
      const agentDraft = ensured.agents[agentKey];
      if (!agentDraft.selectedSkillIds.includes(skillId)) {
        next = {
          ...ensured,
          agents: {
            ...ensured.agents,
            [agentKey]: {
              ...agentDraft,
              selectedSkillIds: normalizeIds([...agentDraft.selectedSkillIds, skillId]),
              excludedSkillIds: removeId(agentDraft.excludedSkillIds, skillId),
            },
          },
        };
      }
    } else {
      const agent = next.agents[agentKey];
      if (agent && agent.selectedSkillIds.includes(skillId)) {
        next = {
          ...next,
          agents: {
            ...next.agents,
            [agentKey]: {
              ...agent,
              selectedSkillIds: removeId(agent.selectedSkillIds, skillId),
            },
          },
        };
      }
    }
  }
  return next;
}

export function toggleProjectAgentSceneForAgents(
  draft: ProjectDraft,
  agentKeys: string[],
  sceneId: string,
) {
  const shouldSelect = !agentKeys.every((key) => {
    const agent = draft.agents[key];
    return agent && agent.selectedSceneIds.includes(sceneId);
  });

  let next = draft;
  for (const agentKey of agentKeys) {
    if (shouldSelect) {
      const ensured = ensureProjectAgentDraft(next, agentKey);
      const agentDraft = ensured.agents[agentKey];
      if (!agentDraft.selectedSceneIds.includes(sceneId)) {
        next = {
          ...ensured,
          agents: {
            ...ensured.agents,
            [agentKey]: {
              ...agentDraft,
              selectedSceneIds: normalizeIds([...agentDraft.selectedSceneIds, sceneId]),
            },
          },
        };
      }
    } else {
      const agent = next.agents[agentKey];
      if (agent && agent.selectedSceneIds.includes(sceneId)) {
        next = {
          ...next,
          agents: {
            ...next.agents,
            [agentKey]: {
              ...agent,
              selectedSceneIds: removeId(agent.selectedSceneIds, sceneId),
            },
          },
        };
      }
    }
  }
  return next;
}

export function toggleProjectAgentExclusion(
  draft: ProjectDraft,
  agentKey: string,
  skillId: string,
) {
  return toggleProjectAgentList(draft, agentKey, "excludedSkillIds", skillId);
}

export function removeProjectManagedTargetSkill(
  draft: ProjectDraft,
  agentKey: string,
  skillId: string,
) {
  const ensured = ensureProjectAgentDraft(draft, agentKey);
  const agentDraft = ensured.agents[agentKey];

  return {
    ...ensured,
    agents: {
      ...ensured.agents,
      [agentKey]: {
        ...agentDraft,
        selectedSkillIds: removeId(agentDraft.selectedSkillIds, skillId),
        excludedSkillIds: normalizeIds([...agentDraft.excludedSkillIds, skillId]),
      },
    },
  };
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

function removeId(ids: string[], id: string) {
  return ids.filter((candidate) => candidate !== id);
}

function normalizeIds(ids: string[]) {
  return [...new Set(ids.map((id) => id.trim()).filter(Boolean))].sort();
}
