import type { ProjectAssignment, ProjectPathInspection } from "./projects";
import type { AgentInventoryItem, SkillSummary } from "./tauri";

export interface ProjectDraft {
  mode: "create" | "edit";
  sourceProjectPath: string | null;
  projectPath: string;
  displayName: string;
  selectedSkillIds: string[];
  selectedAgentKeys: string[];
  unsupportedAgentKeys: string[];
}

export function suggestProjectDisplayName(projectPath: string) {
  const trimmed = projectPath.trim();
  if (!trimmed) return "";
  const segments = trimmed.split(/[/\\]/).filter(Boolean);
  return segments.at(-1) ?? trimmed;
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

export function filterProjectSkills(skills: SkillSummary[], query: string) {
  const needle = query.trim().toLowerCase();
  if (!needle) return skills;
  return skills.filter((skill) =>
    `${skill.name} ${skill.description}`.toLowerCase().includes(needle),
  );
}

export function filterProjectAgents(agents: AgentInventoryItem[], query: string) {
  const needle = query.trim().toLowerCase();
  if (!needle) return agents;
  return agents.filter((agent) =>
    `${agent.displayName} ${agent.key}`.toLowerCase().includes(needle),
  );
}

export function buildProjectSummary(
  draft: ProjectDraft,
  inspection: ProjectPathInspection | null,
  agents: AgentInventoryItem[],
  disabledSkillIds: string[],
) {
  const selectedAgents = agents.filter((agent) =>
    draft.selectedAgentKeys.includes(agent.key),
  );

  return {
    selectedSkillCount: draft.selectedSkillIds.length,
    selectedAgentCount: draft.selectedAgentKeys.length,
    disabledSelectedSkillIds: draft.selectedSkillIds.filter((skillId) =>
      disabledSkillIds.includes(skillId),
    ),
    selectedAgents,
    inspectionAgents: inspection?.agents ?? [],
    unsupportedAgentKeys: draft.unsupportedAgentKeys,
    warnings: inspection?.warnings ?? [],
  };
}
