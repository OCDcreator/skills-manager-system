import { isSceneSkillEnabled } from "./scene-skill-order";
import type { ProjectPathInspection } from "./projects";
import type { ProjectDraft } from "./project-draft";
import {
  ensureProjectAgentDraft,
  projectDraftAgentKeys,
} from "./project-draft";
import type { SceneEntry } from "./scenes";
import type { AgentInventoryItem, AgentTargetSkillEntry, SkillSummary } from "./tauri";

export interface ProjectPreviewItem {
  skill: SkillSummary;
  inheritedGlobalSkillIds: string[];
  projectDirectSkillIds: string[];
  projectSceneNames: string[];
  isExcludedByProject: boolean;
  isGloballyDisabled: boolean;
}

export interface ProjectAgentSummary {
  agentKey: string;
  displayName: string;
  projectSkillsDirRule: string;
  inheritedGlobalSkillIds: string[];
  projectDirectSkillIds: string[];
  projectSceneIds: string[];
  projectSceneNames: string[];
  excludedSkillIds: string[];
  disabledSkillIds: string[];
  previewItems: ProjectPreviewItem[];
  exclusionCandidateSkills: SkillSummary[];
  targetDir: string | null;
  markerExists: boolean | null;
  targetExists: boolean | null;
  targetSkillEntries: AgentTargetSkillEntry[];
  targetSkillScanError: string | null;
}

export function buildProjectSummary(
  draft: ProjectDraft,
  inspection: ProjectPathInspection | null,
  skills: SkillSummary[],
  agents: AgentInventoryItem[],
  disabledSkillIds: string[],
  scenes: Record<string, SceneEntry> = {},
) {
  const inspectionByAgent = new Map(
    (inspection?.agents ?? []).map((target) => [target.agentKey, target]),
  );
  const disabled = new Set(disabledSkillIds);
  const agentSummaries = projectDraftAgentKeys(draft)
    .map((agentKey) => {
      const agent = agents.find((candidate) => candidate.key === agentKey);
      if (!agent) return null;

      const agentDraft = ensureProjectAgentDraft(draft, agentKey).agents[agentKey];
      const inheritedGlobalSkillIds = resolveInheritedGlobalSkillIds(
        agent,
        skills,
        scenes,
      );
      const projectDirectSkillIds = new Set(agentDraft.selectedSkillIds);
      const projectSceneSkillIds = new Set(
        resolveSceneSkillIds(agentDraft.selectedSceneIds, skills, scenes),
      );
      const excluded = new Set(agentDraft.excludedSkillIds);
      const contributionIds = new Set([
        ...inheritedGlobalSkillIds,
        ...projectDirectSkillIds,
        ...projectSceneSkillIds,
      ].filter((skillId) => !excluded.has(skillId)));
      const projectSceneNames = agentDraft.selectedSceneIds.map(
        (sceneId) => scenes[sceneId]?.name || sceneId,
      );
      const inspectionTarget = inspectionByAgent.get(agentKey);
      const previewItems = skills
        .filter((skill) => contributionIds.has(skill.id))
        .map((skill) => ({
          skill,
          inheritedGlobalSkillIds: inheritedGlobalSkillIds.has(skill.id)
            ? [skill.id]
            : [],
          projectDirectSkillIds: projectDirectSkillIds.has(skill.id)
            ? [skill.id]
            : [],
          projectSceneNames: projectSceneSkillIds.has(skill.id)
            ? projectSceneNames
            : [],
          isExcludedByProject: excluded.has(skill.id),
          isGloballyDisabled: disabled.has(skill.id),
        }));

      return {
        agentKey,
        displayName: agent.displayName,
        projectSkillsDirRule: agent.projectSkillsDirRule,
        inheritedGlobalSkillIds: [...inheritedGlobalSkillIds].sort(),
        projectDirectSkillIds: [...projectDirectSkillIds].sort(),
        projectSceneIds: normalizeIds(agentDraft.selectedSceneIds),
        projectSceneNames,
        excludedSkillIds: normalizeIds(agentDraft.excludedSkillIds),
        disabledSkillIds: previewItems
          .filter((item) => item.isGloballyDisabled)
          .map((item) => item.skill.id),
        previewItems,
        exclusionCandidateSkills: skills.filter((skill) =>
          contributionIds.has(skill.id),
        ),
        targetDir: inspectionTarget?.targetDir ?? null,
        markerExists: inspectionTarget?.markerExists ?? null,
        targetExists: inspectionTarget?.targetExists ?? null,
        targetSkillEntries: inspectionTarget?.targetSkillEntries ?? [],
        targetSkillScanError: inspectionTarget?.targetSkillScanError ?? null,
      };
    })
    .filter((summary): summary is ProjectAgentSummary => Boolean(summary));

  return {
    selectedAgentCount: agentSummaries.length,
    projectDirectSkillCount: countAgentIds(
      agentSummaries,
      "projectDirectSkillIds",
    ),
    projectSceneCount: countAgentIds(agentSummaries, "projectSceneIds"),
    projectExclusionCount: countAgentIds(agentSummaries, "excludedSkillIds"),
    disabledSelectedSkillIds: agentSummaries.flatMap(
      (summary) => summary.disabledSkillIds,
    ),
    agentSummaries,
    inspectionAgents: inspection?.agents ?? [],
    unsupportedAgentKeys: draft.unsupportedAgentKeys,
    warnings: inspection?.warnings ?? [],
  };
}

function countAgentIds(
  agentSummaries: ProjectAgentSummary[],
  field: keyof Pick<
    ProjectAgentSummary,
    "projectDirectSkillIds" | "projectSceneIds" | "excludedSkillIds"
  >,
) {
  return agentSummaries.reduce(
    (count, summary) => count + summary[field].length,
    0,
  );
}

function resolveSceneSkillIds(
  sceneIds: string[],
  skills: SkillSummary[],
  scenes: Record<string, SceneEntry>,
) {
  return sceneIds.flatMap((sceneId) => {
    const scene = scenes[sceneId];
    if (!scene) return [];
    return skills
      .filter((skill) => isSceneSkillEnabled(scene, skill.id))
      .map((skill) => skill.id);
  });
}

function resolveInheritedGlobalSkillIds(
  agent: AgentInventoryItem,
  skills: SkillSummary[],
  scenes: Record<string, SceneEntry>,
) {
  const excluded = new Set(agent.excludedSkillIds);
  return new Set(
    [...agent.selectedSkillIds, ...resolveSceneSkillIds(agent.selectedSceneIds, skills, scenes)]
      .filter((skillId) => !excluded.has(skillId)),
  );
}

function normalizeIds(ids: string[]) {
  return [...new Set(ids.map((id) => id.trim()).filter(Boolean))].sort();
}
