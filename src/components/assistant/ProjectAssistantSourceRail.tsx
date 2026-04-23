import { AlertTriangle } from "lucide-react";
import { useTranslation } from "react-i18next";
import type {
  AssistantContextStatus,
  AssistantSource,
} from "../../lib/assistant";

interface ProjectAssistantSourceRailProps {
  status: AssistantContextStatus | null;
  statusError: string | null;
  isLoadingStatus: boolean;
  sources: AssistantSource[];
}

function formatTimestamp(iso: string): string {
  try {
    const date = new Date(iso);
    if (Number.isNaN(date.getTime())) {
      return iso;
    }
    return date.toLocaleString(undefined, {
      dateStyle: "medium",
      timeStyle: "short",
    });
  } catch {
    return iso;
  }
}

function ScopeChips({ label }: { label: string }) {
  const parts = label.split(" + ").map((s) => s.trim()).filter(Boolean);
  return (
    <div className="flex flex-wrap gap-1.5">
      {parts.map((part) => (
        <span
          key={part}
          className="inline-block break-all rounded-md border border-slate-700 bg-slate-800/80 px-2 py-0.5 text-xs text-slate-300"
        >
          {part}
        </span>
      ))}
    </div>
  );
}

export function ProjectAssistantSourceRail({
  status,
  statusError,
  isLoadingStatus,
  sources,
}: ProjectAssistantSourceRailProps) {
  const { t } = useTranslation();
  const hasWarnings = status != null && status.warnings.length > 0;

  return (
    <aside className="self-start rounded-2xl border border-slate-800 bg-slate-900/70 p-4 space-y-5">
      <div>
        <p className="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400">
          {t("assistant.contextTitle")}
        </p>
        <div className="mt-3 space-y-2 text-sm text-slate-300">
          {isLoadingStatus ? (
            <p>{t("assistant.statusLoading")}</p>
          ) : status ? (
            <ScopeChips label={status.scopeLabel} />
          ) : (
            <p>{t("assistant.statusUnavailable")}</p>
          )}
          {status ? (
            <p>
              {t("assistant.documentCount", {
                count: status.indexedDocumentCount,
              })}
            </p>
          ) : null}
          {status ? (
            <p>
              {t("assistant.chunkCount", {
                count: status.indexedChunkCount,
              })}
            </p>
          ) : null}
          <p className="text-xs text-slate-500">
            {statusError
              ? statusError
              : status
                ? t("assistant.indexedAt", {
                    time: formatTimestamp(status.lastIndexedAt),
                  })
                : null}
          </p>
        </div>
      </div>

      {hasWarnings ? (
        <div>
          <p className="text-xs font-semibold uppercase tracking-[0.24em] text-amber-400">
            <AlertTriangle className="mr-1 inline h-3 w-3" />
            {t("assistant.warningsTitle")}
          </p>
          <ul className="mt-2 space-y-1 text-xs text-amber-200/80">
            {status.warnings.map((warning, index) => (
              <li key={index} className="break-words">
                {warning}
              </li>
            ))}
          </ul>
        </div>
      ) : null}

      <div>
        <p className="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400">
          {t("assistant.sourcesTitle")}
        </p>
        <ul className="mt-3 space-y-2 text-sm text-slate-300">
          {sources.length === 0 ? (
            <li>{t("assistant.sourcesEmpty")}</li>
          ) : null}
          {sources.map((source) => (
            <li
              key={source.path}
              className="rounded-xl border border-slate-800 bg-slate-950/60 px-3 py-2"
            >
              <p className="break-words font-medium text-slate-100">
                {source.title}
              </p>
              <p className="mt-1 break-all text-xs text-slate-400">
                {source.path}
              </p>
            </li>
          ))}
        </ul>
      </div>
    </aside>
  );
}
