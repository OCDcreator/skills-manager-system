import { invoke } from "./invoke";

export type SceneSkillSelectionMode = "allExceptDisabled" | "onlySelected";

export interface SceneEntry {
  id: string;
  name: string;
  description: string;
  skillSelectionMode: SceneSkillSelectionMode;
  disabledSkillIds: string[];
  selectedSkillIds: string[];
  enabledAgentKeys: string[];
  skillOrder: string[];
}

export interface SceneConfigSnapshot {
  scenes: Record<string, SceneEntry>;
  activeSceneId: string | null;
}

export const getSceneConfig = () =>
  invoke<SceneConfigSnapshot>("get_scene_config");

export const createScene = (id: string, name: string, description: string) =>
  invoke<SceneConfigSnapshot>("create_scene", { id, name, description });

export const updateScene = (
  id: string,
  name?: string,
  description?: string,
) => invoke<SceneConfigSnapshot>("update_scene", { id, name: name ?? null, description: description ?? null });

export const deleteScene = (id: string) =>
  invoke<SceneConfigSnapshot>("delete_scene", { id });

export const setActiveScene = (id: string | null) =>
  invoke<SceneConfigSnapshot>("set_active_scene", { id });

export const setSceneSkills = (id: string, skillIds: string[]) =>
  invoke<SceneConfigSnapshot>("set_scene_skills", { id, skillIds });

export const setSceneAgents = (id: string, enabledAgentKeys: string[]) =>
  invoke<SceneConfigSnapshot>("set_scene_agents", { id, enabledAgentKeys });

export const setSceneSkillOrder = (id: string, skillOrder: string[]) =>
  invoke<SceneConfigSnapshot>("set_scene_skill_order", { id, skillOrder });
