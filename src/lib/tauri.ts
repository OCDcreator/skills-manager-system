import { invoke } from "@tauri-apps/api/core";

export type AgentKey =
  | "codex"
  | "claude_code"
  | "opencode"
  | "cursor"
  | "amp"
  | "kilo_code"
  | "kimi"
  | "roo_code"
  | "goose"
  | "gemini_cli"
  | "github_copilot"
  | "windsurf";

export interface ManagedSourceInfo {
  kind: "github_import";
  importId: string;
  repoUrl: string;
  pinnedCommit: string;
  agentKey: AgentKey;
  updateAvailable: boolean;
  integrity: "mismatch" | null;
}

export interface SkillSummary {
  id: string;
  name: string;
  description: string;
  sourceType: "custom" | "external";
  relativePath: string;
  directoryPath: string;
  skillDocumentPath: string;
  managedSource?: ManagedSourceInfo | null;
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
  managedSource?: ManagedSourceInfo | null;
}

export interface SkillStateSnapshot {
  disabledSkillIds: string[];
}

export type AgentPathMode = "override" | "detected" | "missing";
export type AgentSyncMode = "copy" | "symlink";
export type AgentTargetSkillEntryKind = "directory" | "symlink" | "file" | "other";

export interface AgentTargetSkillEntry {
  entryName: string;
  displayName: string;
  absolutePath: string;
  symlinkTargetPath: string | null;
  managed: boolean;
  preserveExisting: boolean;
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
  skillsDirRule: string;
  projectSkillsDirRule: string;
  detectDirRule: string;
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

export interface ImportAgentTargetSkillResult {
  relativePath: string;
  absolutePath: string;
  deletedSource: boolean;
}

export interface ExternalSourceWarning {
  code: string;
  severity: "warning" | "error" | string;
  message: string;
}

export interface ExternalSourceRecord {
  id: string;
  repoUrl: string;
  defaultBranch: string | null;
  cachedRepoPath: string | null;
  detectedKind: string | null;
  lastFetchedCommit: string | null;
  lastFetchedAt: string | null;
  status: "pending" | "ok" | "warning" | "error" | string | null;
  warnings: ExternalSourceWarning[];
}

export interface ExternalVariantSnapshot {
  agentKey: AgentKey;
  variantPath: string;
  sourceOfTruthPath: string | null;
  metadataPath: string | null;
  name: string | null;
  description: string | null;
}

export interface ImportedExternalSkillRecord {
  importId: string;
  externalSourceId: string;
  agentKey: AgentKey;
  upstreamVariantPath: string;
  pinnedCommit: string;
  pinnedVariantFingerprint: string | null;
  skillId: string;
  mirrorRelativePath: string;
  lastCheckedCommit: string | null;
  importedAt: string | null;
  warnings: ExternalSourceWarning[];
  updateAvailable: boolean;
}

export interface ExternalSourceSnapshotItem {
  record: ExternalSourceRecord;
  variants: ExternalVariantSnapshot[];
  imports: ImportedExternalSkillRecord[];
}

export interface ExternalSourcesListResponse {
  sources: ExternalSourceSnapshotItem[];
}

export interface ExternalImportResult {
  importId: string;
  skillId: string;
  mirrorRelativePath: string;
  warnings: string[];
}

export const getRepoPath = () => invoke<string | null>("get_repo_path");

export const setRepoPath = (path: string) =>
  invoke<string | null>("set_repo_path", { path });

export const getAgentSyncMode = () =>
  invoke<AgentSyncMode>("get_agent_sync_mode");

export const setAgentSyncMode = (syncMode: AgentSyncMode) =>
  invoke<AgentSyncMode>("set_agent_sync_mode", { syncMode });

export const getAgentOrder = () =>
  invoke<AgentKey[]>("get_agent_order");

export const setAgentOrder = (agentOrder: AgentKey[]) =>
  invoke<AgentKey[]>("set_agent_order", { agentOrder });

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

export const takeOverAgentTargetSkill = (agentKey: string, entryName: string) =>
  invoke<void>("take_over_agent_target_skill", { agentKey, entryName });

export const deleteAgentTargetSkill = (agentKey: string, entryName: string) =>
  invoke<void>("delete_agent_target_skill", { agentKey, entryName });

export const importAgentTargetSkill = (
  agentKey: string,
  entryName: string,
  deleteSourceAfterImport: boolean,
) =>
  invoke<ImportAgentTargetSkillResult>("import_agent_target_skill", {
    agentKey,
    entryName,
    deleteSourceAfterImport,
  });

export const listExternalSources = () =>
  invoke<ExternalSourcesListResponse>("list_external_sources");

export const addExternalSource = (repoUrl: string) =>
  invoke<ExternalSourcesListResponse>("add_external_source", { repoUrl });

export const fetchExternalSource = (sourceId: string) =>
  invoke<ExternalSourcesListResponse>("fetch_external_source", { sourceId });

export const importExternalVariant = (
  sourceId: string,
  agentKey: AgentKey,
  variantPath: string,
) =>
  invoke<ExternalImportResult>("import_external_variant", {
    sourceId,
    agentKey,
    variantPath,
  });

export const updateExternalImport = (importId: string) =>
  invoke<ExternalImportResult>("update_external_import", { importId });

export const removeExternalSource = (sourceId: string, removeImports: boolean) =>
  invoke<ExternalSourcesListResponse>("remove_external_source", {
    sourceId,
    removeImports,
  });

export const repairExternalImport = (importId: string) =>
  invoke<ExternalImportResult>("repair_external_import", { importId });
