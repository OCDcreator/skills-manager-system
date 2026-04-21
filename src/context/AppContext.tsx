import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";
import type { PropsWithChildren } from "react";
import * as api from "../lib/tauri";
import type {
  AgentInventoryItem,
  AgentSyncMode,
  ApplyAgentSyncResponse,
  ScanSkillsResponse,
  SkillDocument,
  SkillSummary,
} from "../lib/tauri";

export type AppView = "skills" | "agents" | "git" | "scenes" | "projects" | "settings";

interface AppContextValue {
  activeView: AppView;
  repoPath: string | null;
  scanResult: ScanSkillsResponse;
  selectedSkill: SkillSummary | null;
  selectedDocument: SkillDocument | null;
  disabledSkillIds: string[];
  agentInventory: AgentInventoryItem[];
  lastAgentApplyResult: ApplyAgentSyncResponse | null;
  isLoading: boolean;
  isLoadingAgents: boolean;
  isSavingPath: boolean;
  isApplyingAgentSync: boolean;
  updatingSkillId: string | null;
  updatingAgentKey: string | null;
  errorMessage: string | null;
  setActiveView: (view: AppView) => void;
  refreshSkills: () => Promise<void>;
  refreshAgents: () => Promise<void>;
  saveRepoPath: (nextPath: string) => Promise<void>;
  selectSkill: (skill: SkillSummary | null) => Promise<void>;
  setSkillEnabled: (skillId: string, enabled: boolean) => Promise<void>;
  setAgentEnabled: (key: string, enabled: boolean) => Promise<void>;
  setAgentPathOverride: (key: string, path: string) => Promise<void>;
  clearAgentPathOverride: (key: string) => Promise<void>;
  applyAgentSync: (syncMode?: AgentSyncMode) => Promise<void>;
}

const AppContext = createContext<AppContextValue | null>(null);
function errorMessageFrom(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}

export function AppProvider({ children }: PropsWithChildren) {
  const [activeView, setActiveView] = useState<AppView>("skills");
  const [repoPath, setRepoPath] = useState<string | null>(null);
  const [scanResult, setScanResult] = useState<ScanSkillsResponse>({
    skills: [],
    warnings: [],
  });
  const [selectedSkill, setSelectedSkill] = useState<SkillSummary | null>(null);
  const [selectedDocument, setSelectedDocument] = useState<SkillDocument | null>(null);
  const [disabledSkillIds, setDisabledSkillIds] = useState<string[]>([]);
  const [agentInventory, setAgentInventory] = useState<AgentInventoryItem[]>([]);
  const [lastAgentApplyResult, setLastAgentApplyResult] =
    useState<ApplyAgentSyncResponse | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isLoadingAgents, setIsLoadingAgents] = useState(true);
  const [isSavingPath, setIsSavingPath] = useState(false);
  const [isApplyingAgentSync, setIsApplyingAgentSync] = useState(false);
  const [updatingSkillId, setUpdatingSkillId] = useState<string | null>(null);
  const [updatingAgentKey, setUpdatingAgentKey] = useState<string | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const refreshSkills = useCallback(async () => {
    if (!repoPath) {
      setScanResult({ skills: [], warnings: [] });
      setSelectedSkill(null);
      setSelectedDocument(null);
      setDisabledSkillIds([]);
      setErrorMessage(null);
      return;
    }

    setIsLoading(true);
    try {
      const response = await api.scanSkills();
      setScanResult(response);

      let nextError: string | null = null;

      try {
        const state = await api.getSkillState();
        setDisabledSkillIds(state.disabledSkillIds);
      } catch (error) {
        setDisabledSkillIds([]);
        nextError = error instanceof Error ? error.message : String(error);
      }

      setSelectedSkill((currentSkill) => {
        if (!currentSkill) {
          return null;
        }

        const refreshedSkill =
          response.skills.find((skill) => skill.id === currentSkill.id) ?? null;
        if (!refreshedSkill) {
          setSelectedDocument(null);
        }
        return refreshedSkill;
      });

      setErrorMessage(nextError);
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : String(error));
    } finally {
      setIsLoading(false);
    }
  }, [repoPath]);

  const refreshAgents = useCallback(async () => {
    setIsLoadingAgents(true);
    try {
      const snapshot = await api.getAgentInventory();
      setAgentInventory(snapshot.agents);
      setErrorMessage(null);
    } catch (error) {
      setErrorMessage(errorMessageFrom(error));
    } finally {
      setIsLoadingAgents(false);
    }
  }, []);

  const saveRepoPath = useCallback(async (nextPath: string) => {
    setIsSavingPath(true);
    try {
      const savedPath = await api.setRepoPath(nextPath);
      setRepoPath(savedPath);
      setSelectedSkill(null);
      setSelectedDocument(null);
      setDisabledSkillIds([]);
      setLastAgentApplyResult(null);
      setActiveView("skills");
      setErrorMessage(null);
    } catch (error) {
      const message = errorMessageFrom(error);
      setErrorMessage(message);
      throw error instanceof Error ? error : new Error(message);
    } finally {
      setIsSavingPath(false);
    }
  }, []);

  const setSkillEnabled = useCallback(async (skillId: string, enabled: boolean) => {
    setUpdatingSkillId(skillId);
    try {
      const snapshot = await api.setSkillEnabled(skillId, enabled);
      setDisabledSkillIds(snapshot.disabledSkillIds);
      setErrorMessage(null);
    } catch (error) {
      const message = errorMessageFrom(error);
      setErrorMessage(message);
      throw error instanceof Error ? error : new Error(message);
    } finally {
      setUpdatingSkillId(null);
    }
  }, []);

  const updateAgentInventory = useCallback(
    async (key: string, action: () => Promise<api.AgentInventorySnapshot>) => {
      setUpdatingAgentKey(key);
      try {
        const snapshot = await action();
        setAgentInventory(snapshot.agents);
        setLastAgentApplyResult(null);
        setErrorMessage(null);
      } catch (error) {
        const message = errorMessageFrom(error);
        setErrorMessage(message);
        throw error instanceof Error ? error : new Error(message);
      } finally {
        setUpdatingAgentKey(null);
      }
    },
    [],
  );

  const setAgentEnabled = useCallback(async (key: string, enabled: boolean) => {
    await updateAgentInventory(key, () => api.setAgentEnabled(key, enabled));
  }, [updateAgentInventory]);

  const setAgentPathOverride = useCallback(async (key: string, path: string) => {
    await updateAgentInventory(key, () => api.setAgentPathOverride(key, path));
  }, [updateAgentInventory]);

  const clearAgentPathOverride = useCallback(async (key: string) => {
    await updateAgentInventory(key, () => api.clearAgentPathOverride(key));
  }, [updateAgentInventory]);

  const applyAgentSync = useCallback(async (syncMode?: AgentSyncMode) => {
    setIsApplyingAgentSync(true);
    try {
      const result = await api.applyAgentSync(syncMode);
      setLastAgentApplyResult(result);
      setErrorMessage(null);
    } catch (error) {
      const message = errorMessageFrom(error);
      setErrorMessage(message);
      throw error instanceof Error ? error : new Error(message);
    } finally {
      setIsApplyingAgentSync(false);
    }
  }, []);
  const selectSkill = useCallback(async (skill: SkillSummary | null) => {
    setSelectedSkill(skill);
    if (!skill) {
      setSelectedDocument(null);
      return;
    }

    try {
      const document = await api.getSkillDocument(skill.relativePath);
      setSelectedDocument(document);
      setErrorMessage(null);
    } catch (error) {
      setSelectedDocument(null);
      setErrorMessage(errorMessageFrom(error));
    }
  }, []);

  useEffect(() => {
    void (async () => {
      try {
        const savedPath = await api.getRepoPath();
        setRepoPath(savedPath);
      } catch (error) {
        setErrorMessage(errorMessageFrom(error));
      } finally {
        setIsLoading(false);
      }
    })();
  }, []);

  useEffect(() => {
    queueMicrotask(() => void refreshAgents());
  }, [refreshAgents]);

  useEffect(() => {
    queueMicrotask(() => void refreshSkills());
  }, [refreshSkills, repoPath]);

  const value = useMemo<AppContextValue>(
    () => ({
      activeView,
      repoPath,
      scanResult,
      selectedSkill,
      selectedDocument,
      disabledSkillIds,
      agentInventory,
      lastAgentApplyResult,
      isLoading,
      isLoadingAgents,
      isSavingPath,
      isApplyingAgentSync,
      updatingSkillId,
      updatingAgentKey,
      errorMessage,
      setActiveView,
      refreshSkills,
      refreshAgents,
      saveRepoPath,
      selectSkill,
      setSkillEnabled,
      setAgentEnabled,
      setAgentPathOverride,
      clearAgentPathOverride,
      applyAgentSync,
    }),
    [
      activeView,
      repoPath,
      scanResult,
      selectedSkill,
      selectedDocument,
      disabledSkillIds,
      agentInventory,
      lastAgentApplyResult,
      isLoading,
      isLoadingAgents,
      isSavingPath,
      isApplyingAgentSync,
      updatingSkillId,
      updatingAgentKey,
      errorMessage,
      refreshSkills,
      refreshAgents,
      saveRepoPath,
      selectSkill,
      setSkillEnabled,
      setAgentEnabled,
      setAgentPathOverride,
      clearAgentPathOverride,
      applyAgentSync,
    ],
  );

  return <AppContext.Provider value={value}>{children}</AppContext.Provider>;
}

export function useAppContext() {
  const context = useContext(AppContext);
  if (!context) {
    throw new Error("useAppContext must be used inside AppProvider");
  }
  return context;
}
