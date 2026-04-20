import { GitBranch, Globe, RefreshCw, ArrowDownToLine, ArrowUpFromLine } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { GitStatusResponse } from "../../lib/git";

interface GitStatusBarProps {
  status: GitStatusResponse | null;
  isLoading: boolean;
  onRefresh: () => void;
  onFetch: () => void;
  isFetching: boolean;
}

export function GitStatusBar({ status, isLoading, onRefresh, onFetch, isFetching }: GitStatusBarProps) {
  const { t } = useTranslation();

  return (
    <section className="flex items-center gap-4 rounded-2xl border border-slate-800 bg-slate-900 px-5 py-4">
      <div className="flex items-center gap-2 min-w-0">
        <GitBranch className="h-4 w-4 shrink-0 text-sky-400" />
        <span className="truncate text-sm font-medium text-slate-100">
          {status?.branch ?? t("git.status.noBranch")}
        </span>
      </div>

      {status?.remoteUrl ? (
        <div className="flex items-center gap-2 min-w-0">
          <Globe className="h-4 w-4 shrink-0 text-slate-500" />
          <span className="truncate text-xs text-slate-400">{status.remoteUrl}</span>
        </div>
      ) : (
        <span className="text-xs text-slate-600">{t("git.status.noRemote")}</span>
      )}

      {status?.aheadBehind ? (
        <div className="flex items-center gap-3 text-xs">
          {status.aheadBehind[0] > 0 && (
            <span className="flex items-center gap-1 text-emerald-400">
              <ArrowUpFromLine className="h-3 w-3" />
              {t("git.status.ahead", { count: status.aheadBehind[0] })}
            </span>
          )}
          {status.aheadBehind[1] > 0 && (
            <span className="flex items-center gap-1 text-amber-400">
              <ArrowDownToLine className="h-3 w-3" />
              {t("git.status.behind", { count: status.aheadBehind[1] })}
            </span>
          )}
        </div>
      ) : null}

      <span
        className={`rounded-full px-2 py-0.5 text-xs font-medium ${
          status?.isClean
            ? "bg-emerald-900/40 text-emerald-300"
            : "bg-amber-900/40 text-amber-300"
        }`}
      >
        {status?.isClean ? t("git.status.clean") : t("git.status.dirty")}
      </span>

      <div className="ml-auto flex items-center gap-2">
        <button
          className="rounded-lg bg-slate-800 px-3 py-1.5 text-xs text-slate-300 hover:bg-slate-700 disabled:opacity-50"
          disabled={isFetching}
          onClick={onFetch}
        >
          {isFetching ? t("git.operation.running") : t("git.actions.fetch")}
        </button>
        <button
          className="rounded-lg bg-slate-800 p-1.5 text-slate-300 hover:bg-slate-700 disabled:opacity-50"
          disabled={isLoading}
          onClick={onRefresh}
        >
          <RefreshCw className={`h-4 w-4 ${isLoading ? "animate-spin" : ""}`} />
        </button>
      </div>
    </section>
  );
}
