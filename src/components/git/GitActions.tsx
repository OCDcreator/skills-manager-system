import { useState } from "react";
import { ArrowDownToLine, ArrowUpFromLine, GitCommitHorizontal, RefreshCw, Upload } from "lucide-react";
import { useTranslation } from "react-i18next";
import * as gitApi from "../../lib/git";
import type { OperationLogEntry } from "./GitOperationLog";

interface GitActionsProps {
  isRunning: boolean;
  onOperationComplete: () => void;
  onLogEntry: (entry: OperationLogEntry) => void;
}

export function GitActions({ isRunning, onOperationComplete, onLogEntry }: GitActionsProps) {
  const { t } = useTranslation();
  const [commitMessage, setCommitMessage] = useState("");
  const [committing, setCommitting] = useState(false);
  const [pulling, setPulling] = useState(false);
  const [pushing, setPushing] = useState(false);
  const [syncing, setSyncing] = useState(false);

  const runOp = async (label: string, op: () => Promise<gitApi.GitOperationResult>) => {
    let result: gitApi.GitOperationResult;
    try {
      result = await op();
    } catch (error) {
      result = {
        success: false,
        message: error instanceof Error ? error.message : String(error),
      };
    }
    onLogEntry({ timestamp: new Date(), label, success: result.success, message: result.message });
    onOperationComplete();
  };

  const handleCommit = async () => {
    if (!commitMessage.trim()) return;
    setCommitting(true);
    await runOp(t("git.actions.commit"), () => gitApi.gitCommit(commitMessage.trim()));
    setCommitting(false);
    setCommitMessage("");
  };

  const handlePull = async () => {
    setPulling(true);
    await runOp(t("git.actions.pull"), gitApi.gitPull);
    setPulling(false);
  };

  const handlePush = async () => {
    setPushing(true);
    await runOp(t("git.actions.push"), gitApi.gitPush);
    setPushing(false);
  };

  const handleSync = async () => {
    setSyncing(true);
    await runOp(t("git.actions.sync"), gitApi.runSyncScript);
    setSyncing(false);
  };

  const busy = isRunning || committing || pulling || pushing || syncing;

  return (
    <div className="space-y-3">
      <div className="flex flex-wrap items-center gap-3">
        <button
          className="flex items-center gap-2 rounded-lg bg-slate-800 px-4 py-2 text-sm text-slate-200 hover:bg-slate-700 disabled:opacity-50"
          disabled={busy}
          onClick={handlePull}
          title={t("tooltip.git.pull")}
        >
          <ArrowDownToLine className="h-4 w-4" />
          {pulling ? t("git.operation.running") : t("git.actions.pull")}
        </button>
        <button
          className="flex items-center gap-2 rounded-lg bg-slate-800 px-4 py-2 text-sm text-slate-200 hover:bg-slate-700 disabled:opacity-50"
          disabled={busy}
          onClick={handlePush}
          title={t("tooltip.git.push")}
        >
          <ArrowUpFromLine className="h-4 w-4" />
          {pushing ? t("git.operation.running") : t("git.actions.push")}
        </button>
        <button
          className="flex items-center gap-2 rounded-lg bg-slate-800 px-4 py-2 text-sm text-slate-200 hover:bg-slate-700 disabled:opacity-50"
          disabled={busy}
          onClick={handleSync}
          title={t("tooltip.git.sync")}
        >
          <RefreshCw className={`h-4 w-4 ${syncing ? "animate-spin" : ""}`} />
          {syncing ? t("git.operation.running") : t("git.actions.sync")}
        </button>
      </div>

      <div className="flex items-center gap-2">
        <input
          className="flex-1 rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm text-slate-200 placeholder:text-slate-500 focus:border-sky-400 focus:outline-none"
          disabled={busy}
          placeholder={t("git.actions.commitPlaceholder")}
          value={commitMessage}
          onChange={(e) => setCommitMessage(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter" && commitMessage.trim()) void handleCommit();
          }}
        />
        <button
          className="flex items-center gap-2 rounded-lg bg-sky-600 px-4 py-2 text-sm text-white hover:bg-sky-500 disabled:opacity-50"
          disabled={busy || !commitMessage.trim()}
          onClick={() => void handleCommit()}
          title={t("tooltip.git.commit")}
        >
          <GitCommitHorizontal className="h-4 w-4" />
          <Upload className="h-3 w-3" />
          {committing ? t("git.operation.running") : t("git.actions.commit")}
        </button>
      </div>
    </div>
  );
}
