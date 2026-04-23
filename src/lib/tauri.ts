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

export type AgentKey =
  | "codex"
  | "claude_code"
  | "opencode"
  | "cursor"
  | "amp"
  | "kilo_code"
  | "roo_code"
  | "goose"
  | "gemini_cli"
  | "github_copilot"
  | "windsurf";

export type AgentPathMode = "override" | "detected" | "missing";
export type AgentSyncMode = "copy" | "symlink";
export type AgentTargetSkillEntryKind = "directory" | "symlink" | "file" | "other";

export interface AgentTargetSkillEntry {
  entryName: string;
  displayName: string;
  absolutePath: string;
  managed: boolean;
  skillId: string | null;
  relativePath: string | null;
  hasSkillDocument: boolean;
  entryKind: AgentTargetSkillEntryKind;
}

export interface AgentInventoryItem {
  key: AgentKey;
  displayName: string;
  enabled: boolean;
  selectedSkillIds: string[];
  selectedSceneIds: string[];
  excludedSkillIds: string[];
  defaultSkillsDir: string;
  detectedSkillsDir: string | null;
  effectiveSkillsDir: string | null;
  targetSkillEntries: AgentTargetSkillEntry[];
  targetSkillScanError: string | null;
  pathOverride: string | null;
  pathMode: AgentPathMode;
}

export interface AgentInventorySnapshot {
  agents: AgentInventoryItem[];
}

export interface AgentConfigurationInput {
  key: string;
  enabled: boolean;
  pathOverride: string | null;
  selectedSkillIds: string[];
  selectedSceneIds: string[];
  excludedSkillIds: string[];
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

export const getAgentSyncMode = () =>
  invoke<AgentSyncMode>("get_agent_sync_mode");

export const setAgentSyncMode = (syncMode: AgentSyncMode) =>
  invoke<AgentSyncMode>("set_agent_sync_mode", { syncMode });

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

export const setAgentConfiguration = (config: AgentConfigurationInput) =>
  invoke<AgentInventorySnapshot>("set_agent_configuration", {
    key: config.key,
    enabled: config.enabled,
    pathOverride: config.pathOverride,
    selectedSkillIds: config.selectedSkillIds,
    selectedSceneIds: config.selectedSceneIds,
    excludedSkillIds: config.excludedSkillIds,
  });

export const applyAgentSync = (syncMode?: AgentSyncMode, agentKey?: string) =>
  invoke<ApplyAgentSyncResponse>("apply_agent_sync", {
    syncMode: syncMode ?? null,
    agentKey: agentKey ?? null,
  });
