import { useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { GitActions } from "../components/git/GitActions";
import { GitDiffViewer } from "../components/git/GitDiffViewer";
import { GitFileList } from "../components/git/GitFileList";
import { GitLogList } from "../components/git/GitLogList";
import { GitOperationLog, type OperationLogEntry } from "../components/git/GitOperationLog";
import { GitStatusBar } from "../components/git/GitStatusBar";
import { useAppContext } from "../context/AppContext";
import * as gitApi from "../lib/git";

export function GitView() {
  const { t } = useTranslation();
  const { repoPath } = useAppContext();

  const [status, setStatus] = useState<gitApi.GitStatusResponse | null>(null);
  const [log, setLog] = useState<gitApi.GitLogResponse | null>(null);
  const [diff, setDiff] = useState<gitApi.GitDiffResponse | null>(null);
  const [isLoadingStatus, setIsLoadingStatus] = useState(false);
  const [isLoadingLog, setIsLoadingLog] = useState(false);
  const [isLoadingDiff, setIsLoadingDiff] = useState(false);
  const [isFetching, setIsFetching] = useState(false);
  const [selectedPath, setSelectedPath] = useState<string | null>(null);
  const [diffMode, setDiffMode] = useState<"staged" | "unstaged">("unstaged");
  const [opLog, setOpLog] = useState<OperationLogEntry[]>([]);

  const refreshStatus = useCallback(async () => {
    setIsLoadingStatus(true);
    try {
      const response = await gitApi.gitStatus();
      setStatus(response);
    } catch {
      setStatus(null);
    } finally {
      setIsLoadingStatus(false);
    }
  }, []);

  const refreshLog = useCallback(async () => {
    setIsLoadingLog(true);
    try {
      const response = await gitApi.gitLog(20);
      setLog(response);
    } catch {
      setLog(null);
    } finally {
      setIsLoadingLog(false);
    }
  }, []);

  const refreshDiff = useCallback(
    async (mode: "staged" | "unstaged") => {
      setIsLoadingDiff(true);
      try {
        const response = await gitApi.gitDiff(mode === "staged");
        setDiff(response);
      } catch {
        setDiff(null);
      } finally {
        setIsLoadingDiff(false);
      }
    },
    [],
  );

  const handleFetch = useCallback(async () => {
    setIsFetching(true);
    try {
      await gitApi.gitFetch();
    } catch (error) {
      setOpLog((prev) => [
        {
          timestamp: new Date(),
          label: t("git.actions.fetch"),
          success: false,
          message: error instanceof Error ? error.message : String(error),
        },
        ...prev,
      ].slice(0, 50));
    } finally {
      setIsFetching(false);
      await refreshStatus();
    }
  }, [refreshStatus, t]);

  const handleSelectFile = useCallback(
    (entry: gitApi.GitStatusEntry) => {
      setSelectedPath(entry.path);
      const mode = entry.x !== "?" && entry.x !== " " && entry.x !== "" ? "staged" : "unstaged";
      setDiffMode(mode);
      void refreshDiff(mode);
    },
    [refreshDiff],
  );

  const handleModeChange = useCallback(
    (mode: "staged" | "unstaged") => {
      setDiffMode(mode);
      void refreshDiff(mode);
    },
    [refreshDiff],
  );

  const handleOperationComplete = useCallback(() => {
    void refreshStatus();
    void refreshLog();
  }, [refreshStatus, refreshLog]);

  const handleLogEntry = useCallback((entry: OperationLogEntry) => {
    setOpLog((prev) => [entry, ...prev].slice(0, 50));
  }, []);

  useEffect(() => {
    if (!repoPath) return;
    queueMicrotask(() => {
      void refreshStatus();
      void refreshLog();
    });
  }, [repoPath, refreshStatus, refreshLog]);

  if (!repoPath) {
    return (
      <section className="rounded-2xl border border-dashed border-slate-700 bg-slate-900 p-8 text-center">
        <h2 className="text-xl font-semibold text-slate-100">{t("git.unconfigured")}</h2>
        <p className="mt-3 text-sm text-slate-400">{t("git.unconfiguredBody")}</p>
      </section>
    );
  }

  return (
    <div className="space-y-6">
      <GitStatusBar
        isFetching={isFetching}
        isLoading={isLoadingStatus}
        onFetch={() => void handleFetch()}
        onRefresh={() => {
          void refreshStatus();
          void refreshLog();
        }}
        status={status}
      />

      <div className="grid gap-6 lg:grid-cols-[280px_1fr]">
        <GitFileList
          onSelect={handleSelectFile}
          selectedPath={selectedPath}
          staged={status?.staged ?? []}
          untracked={status?.untracked ?? []}
          unstaged={status?.unstaged ?? []}
        />
        <GitDiffViewer
          diff={diff}
          diffMode={diffMode}
          isLoading={isLoadingDiff}
          onModeChange={handleModeChange}
        />
      </div>

      <GitActions
        isRunning={false}
        onOperationComplete={handleOperationComplete}
        onLogEntry={handleLogEntry}
      />

      <GitOperationLog entries={opLog} />

      <GitLogList entries={log?.entries ?? []} isLoading={isLoadingLog} />
    </div>
  );
}
