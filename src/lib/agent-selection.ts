import { isSceneSkillEnabled } from "./scene-skill-order";
import type { SceneEntry } from "./scenes";
import type {
  AgentConfigurationInput,
  AgentInventoryItem,
  AgentTargetSkillEntry,
  SkillSummary,
} from "./tauri";

export interface AgentConfigDraft {
  key: string;
  enabled: boolean;
  pathOverride: string;
  selectedSkillIds: string[];
  selectedSceneIds: string[];
  excludedSkillIds: string[];
}

export interface AgentSkillPreviewItem {
  skill: SkillSummary;
  isDirect: boolean;
  sceneNames: string[];
  isExcluded: boolean;
  isGloballyDisabled: boolean;
  isSynced: boolean;
  needsSync: boolean;
}

export interface AgentSelectionPreview {
  items: AgentSkillPreviewItem[];
  syncCount: number;
  syncedCount: number;
  excludedCount: number;
  globallyDisabledCount: number;
}

export function draftFromAgent(agent: AgentInventoryItem): AgentConfigDraft {
  return {
    key: agent.key,
    enabled: agent.enabled,
    pathOverride: agent.pathOverride ?? "",
    selectedSkillIds: normalizeIds(agent.selectedSkillIds),
    selectedSceneIds: normalizeIds(agent.selectedSceneIds),
    excludedSkillIds: normalizeIds(agent.excludedSkillIds),
  };
}

export function draftToConfig(draft: AgentConfigDraft): AgentConfigurationInput {
  const trimmedPath = draft.pathOverride.trim();
  return {
    key: draft.key,
    enabled: draft.enabled,
    pathOverride: trimmedPath ? trimmedPath : null,
    selectedSkillIds: normalizeIds(draft.selectedSkillIds),
    selectedSceneIds: normalizeIds(draft.selectedSceneIds),
    excludedSkillIds: normalizeIds(draft.excludedSkillIds),
  };
}

export function isAgentDraftDirty(
  agent: AgentInventoryItem,
  draft: AgentConfigDraft | undefined,
) {
  if (!draft) return false;
  return serializeDraft(draftFromAgent(agent)) !== serializeDraft(draft);
}

export function toggleId(ids: string[], id: string) {
  return ids.includes(id)
    ? removeId(ids, id)
    : normalizeIds([...ids, id]);
}

export function removeId(ids: string[], id: string) {
  return ids.filter((candidate) => candidate !== id);
}

export function resolveAgentSelectionPreview(
  draft: AgentConfigDraft,
  skills: SkillSummary[],
  globallyDisabledSkillIds: string[],
  scenes: Record<string, SceneEntry>,
  targetSkillEntries: AgentTargetSkillEntry[],
): AgentSelectionPreview {
  const disabled = new Set(globallyDisabledSkillIds);
  const excluded = new Set(draft.excludedSkillIds);
  const syncedSkillIds = new Set(
    targetSkillEntries
      .filter((entry) => entry.managed && entry.skillId)
      .map((entry) => entry.skillId as string),
  );
  const contributions = new Map<string, { direct: boolean; sceneNames: Set<string> }>();

  for (const skillId of draft.selectedSkillIds) {
    const entry = contributions.get(skillId) ?? { direct: false, sceneNames: new Set() };
    entry.direct = true;
    contributions.set(skillId, entry);
  }

  for (const sceneId of draft.selectedSceneIds) {
    const scene = scenes[sceneId];
    if (!scene) continue;
    for (const skill of skills) {
      if (!isSceneSkillEnabled(scene, skill.id)) continue;
      const entry = contributions.get(skill.id) ?? { direct: false, sceneNames: new Set() };
      entry.sceneNames.add(scene.name || scene.id);
      contributions.set(skill.id, entry);
    }
  }

  const items = skills
    .filter((skill) => contributions.has(skill.id))
    .map((skill) => {
      const entry = contributions.get(skill.id)!;
      const isExcluded = excluded.has(skill.id);
      const isGloballyDisabled = disabled.has(skill.id);
      return {
        skill,
        isDirect: entry.direct,
        sceneNames: [...entry.sceneNames],
        isExcluded,
        isGloballyDisabled,
        isSynced: !isExcluded && !isGloballyDisabled && syncedSkillIds.has(skill.id),
        needsSync: !isExcluded && !isGloballyDisabled && !syncedSkillIds.has(skill.id),
      };
    });

  return {
    items,
    syncCount: items.filter((item) => item.needsSync).length,
    syncedCount: items.filter((item) => item.isSynced).length,
    excludedCount: items.filter((item) => item.isExcluded).length,
    globallyDisabledCount: items.filter((item) => item.isGloballyDisabled).length,
  };
}

function serializeDraft(draft: AgentConfigDraft) {
  return JSON.stringify({
    enabled: draft.enabled,
    pathOverride: draft.pathOverride.trim(),
    selectedSkillIds: normalizeIds(draft.selectedSkillIds),
    selectedSceneIds: normalizeIds(draft.selectedSceneIds),
    excludedSkillIds: normalizeIds(draft.excludedSkillIds),
  });
}

function normalizeIds(ids: string[]) {
  return [...new Set(ids.map((id) => id.trim()).filter(Boolean))].sort();
}
