import { invoke } from "@tauri-apps/api/core";

export interface SkillSummary {
  id: string;
  name: string;
  description: string;
  sourceType: "custom" | "external";
  relativePath: string;
  directoryPath: string;
  skillDocumentPath: string;
}

export interface ScanSkillsResponse {
  skills: SkillSummary[];
  warnings: string[];
}

export interface SkillDocument {
  id: string;
  name: string;
  description: string;
  sourceType: "custom" | "external";
  relativePath: string;
  content: string;
}

export interface SkillStateSnapshot {
  disabledSkillIds: string[];
}

export type AgentPathMode = "override" | "detected" | "missing";

export interface AgentInventoryItem {
  key: "codex" | "claude_code" | "opencode";
  displayName: string;
  enabled: boolean;
  defaultSkillsDir: string;
  detectedSkillsDir: string | null;
  effectiveSkillsDir: string | null;
  pathOverride: string | null;
  pathMode: AgentPathMode;
}

export interface AgentInventorySnapshot {
  agents: AgentInventoryItem[];
}

export type AgentApplyStatus = "success" | "partial" | "skipped" | "failed";

export interface AgentApplyResult {
  key: string;
  displayName: string;
  targetDir: string | null;
  status: AgentApplyStatus;
  writtenCount: number;
  removedCount: number;
  conflictCount: number;
  message: string;
}

export interface ApplyAgentSyncResponse {
  enabledSkillCount: number;
  results: AgentApplyResult[];
}

export const getRepoPath = () => invoke<string | null>("get_repo_path");

export const setRepoPath = (path: string) =>
  invoke<string | null>("set_repo_path", { path });

export const scanSkills = () => invoke<ScanSkillsResponse>("scan_skills");

export const getSkillDocument = (relativePath: string) =>
  invoke<SkillDocument>("get_skill_document", { relativePath });

export const getSkillState = () =>
  invoke<SkillStateSnapshot>("get_skill_state");

export const setSkillEnabled = (skillId: string, enabled: boolean) =>
  invoke<SkillStateSnapshot>("set_skill_enabled", { skillId, enabled });

export const getAgentInventory = () =>
  invoke<AgentInventorySnapshot>("get_agent_inventory");

export const setAgentEnabled = (key: string, enabled: boolean) =>
  invoke<AgentInventorySnapshot>("set_agent_enabled", { key, enabled });

export const setAgentPathOverride = (key: string, path: string) =>
  invoke<AgentInventorySnapshot>("set_agent_path_override", { key, path });

export const clearAgentPathOverride = (key: string) =>
  invoke<AgentInventorySnapshot>("clear_agent_path_override", { key });

export const applyAgentSync = () =>
  invoke<ApplyAgentSyncResponse>("apply_agent_sync");
