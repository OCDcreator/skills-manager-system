import { useState } from "react";
import { useTranslation } from "react-i18next";
import type { GitDiffResponse } from "../../lib/git";

interface GitDiffViewerProps {
  diff: GitDiffResponse | null;
  isLoading: boolean;
  diffMode: "staged" | "unstaged";
  onModeChange: (mode: "staged" | "unstaged") => void;
}

export function GitDiffViewer({ diff, isLoading, diffMode, onModeChange }: GitDiffViewerProps) {
  const { t } = useTranslation();
  const [copied, setCopied] = useState(false);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center rounded-2xl border border-slate-800 bg-slate-900 p-8 text-sm text-slate-400">
        {t("git.diff.title")}…
      </div>
    );
  }

  if (!diff) {
    return (
      <div className="flex items-center justify-center rounded-2xl border border-slate-800 bg-slate-900 p-8 text-sm text-slate-500">
        {t("git.diff.selectFile")}
      </div>
    );
  }

  const handleCopy = () => {
    void navigator.clipboard.writeText(diff.diff).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    });
  };

  return (
    <div className="flex flex-col rounded-2xl border border-slate-800 bg-slate-900">
      <div className="flex items-center gap-2 border-b border-slate-800 px-4 py-2">
        <button
          className={`rounded px-2 py-1 text-xs ${
            diffMode === "staged" ? "bg-sky-400 text-slate-950" : "text-slate-400 hover:text-slate-200"
          }`}
          onClick={() => onModeChange("staged")}
          title={t("tooltip.git.diff.staged")}
        >
          {t("git.diff.staged")}
        </button>
        <button
          className={`rounded px-2 py-1 text-xs ${
            diffMode === "unstaged" ? "bg-sky-400 text-slate-950" : "text-slate-400 hover:text-slate-200"
          }`}
          onClick={() => onModeChange("unstaged")}
          title={t("tooltip.git.diff.unstaged")}
        >
          {t("git.diff.unstaged")}
        </button>
        <button
          className="ml-auto rounded px-2 py-1 text-xs text-slate-500 hover:text-slate-300"
          onClick={handleCopy}
          title={t("tooltip.git.diff.copy")}
        >
          {copied ? "✓" : "Copy"}
        </button>
      </div>
      {diff.stat ? (
        <div className="border-b border-slate-800 px-4 py-2 text-xs text-slate-400 font-mono">
          {diff.stat}
        </div>
      ) : null}
      <pre className="flex-1 overflow-auto p-4 text-xs leading-relaxed">
        <DiffContent content={diff.diff} />
      </pre>
    </div>
  );
}

function DiffContent({ content }: { content: string }) {
  if (!content) {
    return <span className="text-slate-500">No changes.</span>;
  }

  return (
    <code>
      {content.split("\n").map((line, i) => {
        let className = "text-slate-300";
        if (line.startsWith("+++") || line.startsWith("---")) {
          className = "text-slate-500 font-semibold";
        } else if (line.startsWith("@@")) {
          className = "text-sky-400";
        } else if (line.startsWith("+")) {
          className = "text-emerald-400";
        } else if (line.startsWith("-")) {
          className = "text-rose-400";
        }
        return (
          <span key={i} className={className}>
            {line}
            {"\n"}
          </span>
        );
      })}
    </code>
  );
}
