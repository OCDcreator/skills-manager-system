import { useState } from "react";
import { openUrl } from "@tauri-apps/plugin-opener";
import { ChevronDown, ChevronUp, ExternalLink } from "lucide-react";
import { useTranslation } from "react-i18next";
import {
  agentLabel,
  EXTERNAL_IMPORT_TARGETS,
  externalSourceName,
  shortCommit,
  sourceAgentLabels,
  statusClasses,
  warningSummary,
} from "../../lib/external-sources";
import type { AgentKey, ExternalSourceSnapshotItem } from "../../lib/tauri";
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
  const [selectedTargets, setSelectedTargets] = useState<Record<string, AgentKey | "">>({});
  const { record, variants, imports } = source;
  const isBusy = updatingExternalSourceId === record.id;
  const unknownLabel = t("sources.meta.unknown");
  const agentLabels = sourceAgentLabels(variants);
  const primaryVariant = variants.find((variant) => variant.description) ?? variants[0] ?? null;
  const kindTag = record.detectedKind === "skill_repository"
    ? t("sources.tags.skillRepository")
    : t("sources.tags.generatedBundle");
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
      <div className="grid gap-4 xl:grid-cols-[minmax(0,1fr)_max-content] xl:items-start">
        <div className="min-w-0 space-y-2">
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
                {kindTag}
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
            className="inline-flex max-w-full min-w-0 items-center gap-2 text-sm text-sky-300 underline decoration-sky-500/40 underline-offset-4 transition hover:text-sky-200"
            href={record.repoUrl}
            onClick={(event) => void handleOpenRepo(event)}
            rel="noreferrer"
            target="_blank"
          >
            <span className="min-w-0 break-all">{record.repoUrl}</span>
            <ExternalLink className="h-4 w-4 shrink-0" />
            <span className="shrink-0 text-xs text-slate-500">{t("sources.actions.openRepo")}</span>
          </a>
          <div className="flex flex-wrap gap-3 text-xs text-slate-500">
            <span>{t("sources.meta.branch", { branch: record.branch ?? record.defaultBranch ?? unknownLabel })}</span>
            {record.branch && record.defaultBranch ? (
              <span>{t("sources.meta.defaultBranch", { branch: record.defaultBranch })}</span>
            ) : null}
            <span>{t("sources.meta.subpath", { path: record.subpath ?? t("sources.meta.rootSubpath") })}</span>
            <span>{t("sources.meta.head", { commit: shortCommit(record.lastFetchedCommit, unknownLabel) })}</span>
            <span>{t("sources.meta.kind", { kind: record.detectedKind ?? t("sources.meta.unclassified") })}</span>
          </div>
        </div>

        <div className="grid grid-cols-[repeat(auto-fit,minmax(8rem,1fr))] gap-2 xl:min-w-max xl:grid-cols-1">
          <button
            className="w-full whitespace-nowrap rounded-lg bg-slate-800 px-4 py-2 text-sm text-slate-100 transition hover:bg-slate-700 disabled:cursor-not-allowed disabled:bg-slate-900 disabled:text-slate-500"
            disabled={isBusy}
            onClick={() => void onFetchSource(record.id)}
            type="button"
          >
            {isBusy ? t("sources.actions.fetching") : t("sources.actions.fetch")}
          </button>
          <button
            className="w-full whitespace-nowrap rounded-lg border border-rose-700/60 bg-rose-950/50 px-4 py-2 text-sm text-rose-100 transition hover:bg-rose-900/60 disabled:cursor-not-allowed disabled:opacity-50"
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
            className="inline-flex w-full items-center justify-center gap-2 whitespace-nowrap rounded-lg border border-slate-700 bg-slate-950/70 px-4 py-2 text-sm text-slate-200 transition hover:border-slate-600 hover:bg-slate-800"
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
              <div className="skill-markdown-scroll max-h-[32rem] overflow-y-auto space-y-3 pr-1">
                {variants.map((variant) => {
                  const variantKey = `${variant.agentKey}:${variant.variantPath}`;
                  const defaultTarget =
                    selectedTargets[variantKey]
                    ?? variant.suggestedTargetAgents[0]
                    ?? variant.detectedAgentHint
                    ?? "";
                  const isImported = defaultTarget
                    ? imports.some(
                        (item) =>
                          item.agentKey === defaultTarget
                          && item.upstreamVariantPath === variant.variantPath,
                      )
                    : false;
                  const canImport = Boolean(defaultTarget) && !isImported && !isBusy && Boolean(repoPath);
                  return (
                    <div
                      key={variantKey}
                      className="flex flex-col gap-3 rounded-xl border border-slate-800 bg-slate-950 p-4 lg:flex-row lg:items-start lg:justify-between"
                    >
                      <div className="space-y-1">
                        <div className="flex flex-wrap items-center gap-2">
                          <span className="rounded-full border border-slate-700 px-2 py-1 text-xs text-slate-300">
                            {t(`sources.variants.detection.${variant.detectionClass}`)}
                          </span>
                          {variant.detectedAgentHint ? (
                            <span className="rounded-full border border-sky-800/80 bg-sky-950/40 px-2 py-1 text-xs text-sky-200">
                              {t("sources.variants.hint", {
                                agent: agentLabel(variant.detectedAgentHint),
                              })}
                            </span>
                          ) : null}
                          <span className="text-sm font-medium text-slate-100">{variant.variantPath}</span>
                        </div>
                        {variant.sourceOfTruthPath ? (
                          <p className="text-xs text-slate-500">
                            {t("sources.variants.sourceOfTruth", {
                              path: variant.sourceOfTruthPath,
                            })}
                          </p>
                        ) : null}
                        <div className="space-y-2 pt-1 text-xs text-slate-500">
                          <div className="flex flex-wrap items-center gap-2">
                            <span className="font-medium text-slate-400">
                              {t("sources.variants.childDirectories")}
                            </span>
                            {variant.childDirectories.length ? (
                              <>
                                {variant.childDirectories.slice(0, 5).map((directory) => (
                                  <span
                                    key={`${variantKey}:dir:${directory}`}
                                    className="rounded-full border border-slate-800 bg-slate-900/80 px-2 py-1 text-[11px] text-slate-300"
                                  >
                                    {directory}
                                  </span>
                                ))}
                                {variant.childDirectories.length > 5 ? (
                                  <span className="text-[11px] text-slate-500">
                                    +{variant.childDirectories.length - 5}
                                  </span>
                                ) : null}
                              </>
                            ) : (
                              <span className="text-[11px] text-slate-500">
                                {t("sources.variants.none")}
                              </span>
                            )}
                          </div>
                          <div className="flex flex-wrap items-center gap-2">
                            <span className="font-medium text-slate-400">
                              {t("sources.variants.childFiles")}
                            </span>
                            {variant.childFiles.length ? (
                              <>
                                {variant.childFiles.slice(0, 5).map((file) => (
                                  <span
                                    key={`${variantKey}:file:${file}`}
                                    className="rounded-full border border-slate-800 bg-slate-950/70 px-2 py-1 text-[11px] text-slate-400"
                                  >
                                    {file}
                                  </span>
                                ))}
                                {variant.childFiles.length > 5 ? (
                                  <span className="text-[11px] text-slate-500">
                                    +{variant.childFiles.length - 5}
                                  </span>
                                ) : null}
                              </>
                            ) : variant.childDirectories.length ? (
                              <span className="text-[11px] text-slate-500">
                                {t("sources.variants.none")}
                              </span>
                            ) : (
                              <span className="text-[11px] text-slate-500">
                                {t("sources.variants.rootOnly")}
                              </span>
                            )}
                          </div>
                        </div>
                      </div>
                      <div className="flex min-w-[15rem] flex-col gap-2">
                        <label className="space-y-1">
                          <span className="text-[11px] font-medium uppercase tracking-wide text-slate-500">
                            {t("sources.variants.targetLabel")}
                          </span>
                          <select
                            className="w-full rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-xs text-slate-100 outline-none transition focus:border-sky-400"
                            onChange={(event) =>
                              setSelectedTargets((current) => ({
                                ...current,
                                [variantKey]: event.target.value as AgentKey | "",
                              }))
                            }
                            value={defaultTarget}
                          >
                            <option value="">{t("sources.variants.selectTarget")}</option>
                            {EXTERNAL_IMPORT_TARGETS.map((target) => (
                              <option key={`${variantKey}:${target}`} value={target}>
                                {agentLabel(target)}
                              </option>
                            ))}
                          </select>
                        </label>
                        <button
                          className="rounded-lg bg-sky-400 px-3 py-2 text-xs font-medium text-slate-950 transition hover:bg-sky-300 disabled:cursor-not-allowed disabled:bg-slate-800 disabled:text-slate-400"
                          disabled={!canImport}
                          onClick={() =>
                            defaultTarget
                              ? void onImportVariant(record.id, defaultTarget, variant.variantPath)
                              : undefined
                          }
                          type="button"
                        >
                          {!repoPath
                            ? t("sources.variants.repoRequired")
                            : !defaultTarget
                              ? t("sources.variants.targetRequired")
                              : isImported
                                ? t("sources.variants.imported")
                                : isBusy
                                  ? t("sources.variants.importing")
                                  : t("sources.variants.importAs", {
                                      agent: agentLabel(defaultTarget),
                                    })}
                        </button>
                      </div>
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
