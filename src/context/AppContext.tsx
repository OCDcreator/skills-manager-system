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
import type { ScanSkillsResponse, SkillDocument, SkillSummary } from "../lib/tauri";

export type AppView = "skills" | "settings";

interface AppContextValue {
  activeView: AppView;
  repoPath: string | null;
  scanResult: ScanSkillsResponse;
  selectedSkill: SkillSummary | null;
  selectedDocument: SkillDocument | null;
  isLoading: boolean;
  isSavingPath: boolean;
  errorMessage: string | null;
  setActiveView: (view: AppView) => void;
  refreshSkills: () => Promise<void>;
  saveRepoPath: (nextPath: string) => Promise<void>;
  selectSkill: (skill: SkillSummary | null) => Promise<void>;
}

const AppContext = createContext<AppContextValue | null>(null);

export function AppProvider({ children }: PropsWithChildren) {
  const [activeView, setActiveView] = useState<AppView>("skills");
  const [repoPath, setRepoPath] = useState<string | null>(null);
  const [scanResult, setScanResult] = useState<ScanSkillsResponse>({
    skills: [],
    warnings: [],
  });
  const [selectedSkill, setSelectedSkill] = useState<SkillSummary | null>(null);
  const [selectedDocument, setSelectedDocument] = useState<SkillDocument | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isSavingPath, setIsSavingPath] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const refreshSkills = useCallback(async () => {
    if (!repoPath) {
      setScanResult({ skills: [], warnings: [] });
      setSelectedSkill(null);
      setSelectedDocument(null);
      return;
    }

    setIsLoading(true);
    try {
      const response = await api.scanSkills();
      setScanResult(response);
      setErrorMessage(null);
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
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : String(error));
    } finally {
      setIsLoading(false);
    }
  }, [repoPath]);

  const saveRepoPath = useCallback(async (nextPath: string) => {
    setIsSavingPath(true);
    try {
      const savedPath = await api.setRepoPath(nextPath);
      setRepoPath(savedPath);
      setSelectedSkill(null);
      setSelectedDocument(null);
      setActiveView("skills");
      setErrorMessage(null);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      setErrorMessage(message);
      throw error instanceof Error ? error : new Error(message);
    } finally {
      setIsSavingPath(false);
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
      setErrorMessage(error instanceof Error ? error.message : String(error));
    }
  }, []);

  useEffect(() => {
    void (async () => {
      try {
        const savedPath = await api.getRepoPath();
        setRepoPath(savedPath);
      } catch (error) {
        setErrorMessage(error instanceof Error ? error.message : String(error));
      } finally {
        setIsLoading(false);
      }
    })();
  }, []);

  useEffect(() => {
    void refreshSkills();
  }, [refreshSkills, repoPath]);

  const value = useMemo<AppContextValue>(
    () => ({
      activeView,
      repoPath,
      scanResult,
      selectedSkill,
      selectedDocument,
      isLoading,
      isSavingPath,
      errorMessage,
      setActiveView,
      refreshSkills,
      saveRepoPath,
      selectSkill,
    }),
    [
      activeView,
      repoPath,
      scanResult,
      selectedSkill,
      selectedDocument,
      isLoading,
      isSavingPath,
      errorMessage,
      refreshSkills,
      saveRepoPath,
      selectSkill,
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
