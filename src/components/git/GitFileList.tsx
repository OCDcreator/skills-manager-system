import clsx from "clsx";
import { FileText } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { GitStatusEntry } from "../../lib/git";

interface GitFileListProps {
  staged: GitStatusEntry[];
  unstaged: GitStatusEntry[];
  untracked: GitStatusEntry[];
  selectedPath: string | null;
  onSelect: (entry: GitStatusEntry) => void;
}

export function GitFileList({ staged, unstaged, untracked, selectedPath, onSelect }: GitFileListProps) {
  const { t } = useTranslation();

  return (
    <div className="space-y-4">
      <Section
        entries={staged}
        selectedPath={selectedPath}
        onSelect={onSelect}
        title={t("git.files.staged")}
        emptyText={t("git.files.empty")}
        statusColor="text-emerald-400"
        tooltipPrefix={t("tooltip.git.file.staged")}
      />
      <Section
        entries={unstaged}
        selectedPath={selectedPath}
        onSelect={onSelect}
        title={t("git.files.unstaged")}
        emptyText={t("git.files.empty")}
        statusColor="text-amber-400"
        tooltipPrefix={t("tooltip.git.file.unstaged")}
      />
      <Section
        entries={untracked}
        selectedPath={selectedPath}
        onSelect={onSelect}
        title={t("git.files.untracked")}
        emptyText={t("git.files.empty")}
        statusColor="text-slate-400"
        tooltipPrefix={t("tooltip.git.file.untracked")}
      />
    </div>
  );
}

function Section({
  entries,
  selectedPath,
  onSelect,
  title,
  emptyText,
  statusColor,
  tooltipPrefix,
}: {
  entries: GitStatusEntry[];
  selectedPath: string | null;
  onSelect: (entry: GitStatusEntry) => void;
  title: string;
  emptyText: string;
  statusColor: string;
  tooltipPrefix: string;
}) {
  return (
    <div>
      <h3 className="mb-2 text-xs font-semibold uppercase tracking-wider text-slate-500">{title}</h3>
      {entries.length === 0 ? (
        <p className="text-xs text-slate-600">{emptyText}</p>
      ) : (
        <ul className="space-y-1">
          {entries.map((entry) => (
            <li key={entry.path}>
              <button
                className={clsx(
                  "flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm hover:bg-slate-800",
                  selectedPath === entry.path && "bg-slate-800",
                )}
                onClick={() => onSelect(entry)}
                title={`${tooltipPrefix}: ${entry.path}`}
              >
                <FileText className="h-4 w-4 shrink-0 text-slate-500" />
                <span className={`font-mono text-xs ${statusColor}`}>
                  {entry.x}{entry.y}
                </span>
                <span className="truncate text-slate-200">{entry.path}</span>
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
