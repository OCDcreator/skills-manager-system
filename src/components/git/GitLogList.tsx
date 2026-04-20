import { GitCommit } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { GitLogEntry } from "../../lib/git";

interface GitLogListProps {
  entries: GitLogEntry[];
  isLoading: boolean;
}

export function GitLogList({ entries, isLoading }: GitLogListProps) {
  const { t } = useTranslation();

  return (
    <section className="rounded-2xl border border-slate-800 bg-slate-900">
      <div className="border-b border-slate-800 px-5 py-3">
        <h3 className="text-sm font-semibold text-slate-100">{t("git.log.title")}</h3>
      </div>
      {isLoading ? (
        <div className="p-5 text-sm text-slate-400">{t("git.operation.running")}</div>
      ) : entries.length === 0 ? (
        <div className="p-5 text-sm text-slate-500">{t("git.log.empty")}</div>
      ) : (
        <ul className="divide-y divide-slate-800">
          {entries.map((entry) => (
            <li key={entry.hash} className="flex items-start gap-3 px-5 py-3">
              <GitCommit className="mt-0.5 h-4 w-4 shrink-0 text-slate-600" />
              <div className="min-w-0 flex-1">
                <p className="truncate text-sm text-slate-200">{entry.message}</p>
                <p className="mt-0.5 text-xs text-slate-500">
                  <span className="font-mono text-slate-400">{entry.shortHash}</span>
                  {" · "}
                  {entry.author}
                  {" · "}
                  {formatDate(entry.date)}
                </p>
              </div>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}

function formatDate(dateStr: string): string {
  try {
    const date = new Date(dateStr);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

    if (diffDays === 0) return "today";
    if (diffDays === 1) return "yesterday";
    if (diffDays < 7) return `${diffDays}d ago`;
    return date.toLocaleDateString();
  } catch {
    return dateStr;
  }
}
