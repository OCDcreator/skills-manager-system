import type {
  AgentConfigurationInput,
  AgentInventoryItem,
  AgentKey,
  AgentSyncMode,
  AddExternalSourceInput,
  ApplyAgentSyncResponse,
  ExternalVariantKey,
  ExternalSourceSnapshotItem,
  ScanSkillsResponse,
  SkillDocument,
  SkillStateSnapshot,
  SkillSummary,
} from "../lib/tauri";
import type {
  AppView,
  NavigationGuard,
  PendingNavigation,
} from "./navigation-guard";

export type { AppView, NavigationGuard, PendingNavigation };

export interface AppContextValue {
  activeView: AppView;
  repoPath: string | null;
  scanResult: ScanSkillsResponse;
  selectedSkill: SkillSummary | null;
  selectedDocument: SkillDocument | null;
  disabledSkillIds: string[];
  agentOrder: AgentKey[];
  agentInventory: AgentInventoryItem[];
  sortedAgentInventory: AgentInventoryItem[];
  lastAgentApplyResult: ApplyAgentSyncResponse | null;
  externalSources: ExternalSourceSnapshotItem[];
  isLoading: boolean;
  isLoadingAgents: boolean;
  isLoadingExternalSources: boolean;
  isSavingPath: boolean;
  isApplyingAgentSync: boolean;
  isAddingExternalSource: boolean;
  updatingSkillId: string | null;
  updatingAgentKey: string | null;
  updatingExternalSourceId: string | null;
  updatingExternalImportId: string | null;
  errorMessage: string | null;
  pendingNavigation: PendingNavigation | null;
  setActiveView: (view: AppView) => void;
  refreshSkills: () => Promise<void>;
  refreshAgents: () => Promise<void>;
  refreshExternalSources: () => Promise<void>;
  saveRepoPath: (path: string) => Promise<void>;
  selectSkill: (skill: SkillSummary | null) => Promise<void>;
  setSkillEnabled: (skillId: string, enabled: boolean) => Promise<void>;
  setAgentEnabled: (key: string, enabled: boolean) => Promise<void>;
  setAgentPathOverride: (key: string, path: string) => Promise<void>;
  clearAgentPathOverride: (key: string) => Promise<void>;
  saveAgentConfiguration: (config: AgentConfigurationInput) => Promise<void>;
  saveAgentOrder: (agentOrder: AgentKey[]) => Promise<void>;
  applyAgentSync: (syncMode?: AgentSyncMode, agentKey?: string) => Promise<void>;
  addExternalSource: (input: AddExternalSourceInput) => Promise<void>;
  fetchExternalSource: (sourceId: string) => Promise<void>;
  importExternalVariant: (sourceId: string, agentKey: ExternalVariantKey, variantPath: string) => Promise<void>;
  updateExternalImport: (importId: string) => Promise<void>;
  removeExternalSource: (sourceId: string, removeImports: boolean) => Promise<void>;
  repairExternalImport: (importId: string) => Promise<void>;
  registerNavigationGuard: (guard: NavigationGuard) => () => void;
  confirmNavigationSave: () => Promise<void>;
  confirmNavigationDiscard: () => void;
  cancelNavigation: () => void;
}
