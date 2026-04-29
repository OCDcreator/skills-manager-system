import { useState } from "react";
import { openUrl } from "@tauri-apps/plugin-opener";
import { ChevronDown, ChevronUp, ExternalLink } from "lucide-react";
import { useTranslation } from "react-i18next";
import { externalSourceName, shortCommit, sourceAgentLabels, statusClasses, warningSummary } from "../../lib/external-sources";
import type { ExternalSourceSnapshotItem } from "../../lib/tauri";
import { ExternalImportList } from "./ExternalImportList";

interface ExternalSourceCardProps {
  source: ExternalSourceSnapshotItem;
  repoPath: string | null;
  updatingExternalSourceId: string | null;
  updatingExternalImportId: string | null;
  onFetchSource: (sourceId: string) => Promise<void>;
  onImportVariant: (sourceId: string, agentKey: ExternalSourceSnapshotItem["variants"][number]["agentKey"], variantPath: string) => Promise<void>;
  onRemoveSource: (sourceId: string, removeImports: boolean) => Promise<void>;
  onUpdateImport: (importId: string) => Promise<void>;
  onRepairImport: (importId: string) => Promise<void>;
}

export function ExternalSourceCard({
  source,
  repoPath,
  updatingExternalSourceId,
  updatingExternalImportId,
  onFetchSource,
  onImportVariant,
  onRemoveSource,
  onUpdateImport,
  onRepairImport,
}: ExternalSourceCardProps) {
  const { t } = useTranslation();
  const [isExpanded, setIsExpanded] = useState(false);
  const { record, variants, imports } = source;
  const isBusy = updatingExternalSourceId === record.id;
  const importedKeys = new Set(imports.map((item) => `${item.agentKey}:${item.upstreamVariantPath}`));
  const unknownLabel = t("sources.meta.unknown");
  const agentLabels = sourceAgentLabels(variants);
  const primaryVariant = variants.find((variant) => variant.description) ?? variants[0] ?? null;
  const detailsId = `external-source-${record.id}`;
  const summaryLabel = primaryVariant?.description
    ?? (variants.length
      ? t("sources.summary.descriptionFallback", { agents: agentLabels.join(", ") })
      : t("sources.summary.noSupportedVariants"));

  async function handleOpenRepo(event: React.MouseEvent<HTMLAnchorElement>) {
    event.preventDefault();
    try {
      await openUrl(record.repoUrl);
    } catch {
      window.open(record.repoUrl, "_blank", "noopener,noreferrer");
    }
  }

  return (
    <article className="rounded-2xl border border-slate-800 bg-slate-900 p-6">
      <div className="flex flex-col gap-4 xl:flex-row xl:items-start xl:justify-between">
        <div className="space-y-2">
          <div className="flex flex-wrap items-center gap-3">
            <h3 className="text-lg font-semibold text-slate-100">{externalSourceName(record)}</h3>
            <span className={`rounded-full border px-3 py-1 text-xs font-medium ${statusClasses(record.status)}`}>
              {t(`sources.status.${record.status ?? "pending"}`)}
            </span>
          </div>
          <p className="text-sm text-slate-300">{summaryLabel}</p>
          <div className="flex flex-wrap items-center gap-2">
            {variants.length ? (
              <span className="rounded-full border border-sky-700/60 bg-sky-950/40 px-3 py-1 text-xs font-medium text-sky-200">
                {t("sources.tags.generatedBundle")}
              </span>
            ) : null}
            {agentLabels.map((label) => (
              <span
                key={`${record.id}-${label}`}
                className="rounded-full border border-slate-700 bg-slate-950/70 px-3 py-1 text-xs text-slate-300"
              >
                {label}
              </span>
            ))}
            {imports.length ? (
              <span className="rounded-full border border-emerald-700/60 bg-emerald-950/40 px-3 py-1 text-xs font-medium text-emerald-200">
                {t("sources.tags.importedCount", { count: imports.length })}
              </span>
            ) : null}
            {variants.length ? (
              <span className="rounded-full border border-slate-700 bg-slate-950/70 px-3 py-1 text-xs text-slate-300">
                {t("sources.summary.supportedVariants", { count: variants.length })}
              </span>
            ) : null}
          </div>
          <a
            className="inline-flex items-center gap-2 text-sm text-sky-300 underline decoration-sky-500/40 underline-offset-4 transition hover:text-sky-200"
            href={record.repoUrl}
            onClick={(event) => void handleOpenRepo(event)}
            rel="noreferrer"
            target="_blank"
          >
            <span>{record.repoUrl}</span>
            <ExternalLink className="h-4 w-4" />
            <span className="text-xs text-slate-500">{t("sources.actions.openRepo")}</span>
          </a>
          <div className="flex flex-wrap gap-3 text-xs text-slate-500">
            <span>{t("sources.meta.branch", { branch: record.defaultBranch ?? unknownLabel })}</span>
            <span>{t("sources.meta.head", { commit: shortCommit(record.lastFetchedCommit, unknownLabel) })}</span>
            <span>{t("sources.meta.kind", { kind: record.detectedKind ?? t("sources.meta.unclassified") })}</span>
          </div>
        </div>

        <div className="flex flex-wrap gap-2">
          <button
            className="rounded-lg bg-slate-800 px-4 py-2 text-sm text-slate-100 transition hover:bg-slate-700 disabled:cursor-not-allowed disabled:bg-slate-900 disabled:text-slate-500"
            disabled={isBusy}
            onClick={() => void onFetchSource(record.id)}
            type="button"
          >
            {isBusy ? t("sources.actions.fetching") : t("sources.actions.fetch")}
          </button>
          <button
            className="rounded-lg border border-rose-700/60 bg-rose-950/50 px-4 py-2 text-sm text-rose-100 transition hover:bg-rose-900/60 disabled:cursor-not-allowed disabled:opacity-50"
            disabled={isBusy}
            onClick={() => {
              const removeImports = imports.length > 0;
              const confirmed = window.confirm(
                removeImports
                  ? t("sources.confirm.removeWithImports")
                  : t("sources.confirm.removeSourceOnly"),
              );
              if (confirmed) {
                void onRemoveSource(record.id, removeImports);
              }
            }}
            type="button"
          >
            {t("sources.actions.remove")}
          </button>
          <button
            aria-controls={detailsId}
            aria-expanded={isExpanded}
            className="inline-flex items-center gap-2 rounded-lg border border-slate-700 bg-slate-950/70 px-4 py-2 text-sm text-slate-200 transition hover:border-slate-600 hover:bg-slate-800"
            onClick={() => setIsExpanded((value) => !value)}
            type="button"
          >
            {isExpanded ? <ChevronUp className="h-4 w-4" /> : <ChevronDown className="h-4 w-4" />}
            <span>{isExpanded ? t("sources.actions.collapse") : t("sources.actions.expand")}</span>
          </button>
        </div>
      </div>

      {isExpanded ? (
        <div className="mt-4 grid gap-4 xl:grid-cols-[1.15fr_0.85fr]" id={detailsId}>
          <section className="space-y-4 rounded-xl border border-slate-800 bg-slate-950/60 p-4">
            <div className="flex items-center justify-between">
              <div>
                <h4 className="text-sm font-semibold text-slate-100">{t("sources.variants.title")}</h4>
                <p className="text-xs text-slate-400">
                  {variants.length
                    ? t("sources.variants.count", { count: variants.length })
                    : t("sources.variants.emptyHint")}
                </p>
              </div>
            </div>

            {record.warnings.length ? (
              <div className="rounded-xl border border-amber-700/50 bg-amber-950/40 p-3 text-sm text-amber-100">
                <p className="font-medium">
                  {warningSummary(
                    record.warnings,
                    t("sources.warnings.count", { count: record.warnings.length }),
                  )}
                </p>
                <ul className="mt-2 space-y-1 text-xs text-amber-200">
                  {record.warnings.map((warning) => (
                    <li key={`${record.id}-${warning.code}`}>{warning.message}</li>
                  ))}
                </ul>
              </div>
            ) : null}

            {variants.length ? (
              <div className="space-y-3">
                {variants.map((variant) => {
                  const variantKey = `${variant.agentKey}:${variant.variantPath}`;
                  const isImported = importedKeys.has(variantKey);
                  return (
                    <div
                      key={variantKey}
                      className="flex flex-col gap-3 rounded-xl border border-slate-800 bg-slate-950 p-4 lg:flex-row lg:items-start lg:justify-between"
                    >
                      <div className="space-y-1">
                        <div className="flex flex-wrap items-center gap-2">
                          <span className="rounded-full border border-slate-700 px-2 py-1 text-xs text-slate-300">
                            {variant.agentKey}
                          </span>
                          <span className="text-sm font-medium text-slate-100">{variant.variantPath}</span>
                        </div>
                        {variant.sourceOfTruthPath ? (
                          <p className="text-xs text-slate-500">
                            {t("sources.variants.sourceOfTruth", {
                              path: variant.sourceOfTruthPath,
                            })}
                          </p>
                        ) : null}
                      </div>
                      <button
                        className="rounded-lg bg-sky-400 px-3 py-2 text-xs font-medium text-slate-950 transition hover:bg-sky-300 disabled:cursor-not-allowed disabled:bg-slate-800 disabled:text-slate-400"
                        disabled={isBusy || isImported || !repoPath}
                        onClick={() => void onImportVariant(record.id, variant.agentKey, variant.variantPath)}
                        type="button"
                      >
                        {!repoPath
                          ? t("sources.variants.repoRequired")
                          : isImported
                            ? t("sources.variants.imported")
                            : isBusy
                              ? t("sources.variants.importing")
                              : t("sources.variants.import")}
                      </button>
                    </div>
                  );
                })}
              </div>
            ) : (
              <p className="text-sm text-slate-400">{t("sources.variants.noneDetected")}</p>
            )}
          </section>

          <section className="space-y-4 rounded-xl border border-slate-800 bg-slate-950/60 p-4">
            <div>
              <h4 className="text-sm font-semibold text-slate-100">{t("sources.imports.title")}</h4>
              <p className="text-xs text-slate-400">
                {t("sources.imports.description")}
              </p>
            </div>
            <ExternalImportList
              imports={imports}
              onRepairImport={onRepairImport}
              onUpdateImport={onUpdateImport}
              updatingImportId={updatingExternalImportId}
            />
          </section>
        </div>
      ) : null}
    </article>
  );
}
