import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import {
  externalSourceName,
  shortCommit,
  statusClasses,
} from "../../lib/external-sources";
import type {
  AgentInventoryItem,
  AgentKey,
  ExternalSourceSnapshotItem,
} from "../../lib/tauri";
import { AgentBrandIcon } from "./AgentBrandIcon";

interface AgentExternalVariantPanelProps {
  agent: AgentInventoryItem;
  repoPath: string | null;
  sources: ExternalSourceSnapshotItem[];
  updatingExternalImportId: string | null;
  updatingExternalSourceId: string | null;
  onImportVariant: (
    sourceId: string,
    agentKey: AgentKey,
    variantPath: string,
  ) => Promise<void>;
  onRepairImport: (importId: string) => Promise<void>;
  onUpdateImport: (importId: string) => Promise<void>;
}

function warningText(
  code: string,
  fallback: string,
  t: ReturnType<typeof useTranslation>["t"],
  agentKey: string,
  variantPath: string,
) {
  if (code === "variant_disappeared") {
    return t("skills.warnings.variantDisappeared", {
      agent: agentKey,
      variant: variantPath,
    });
  }

  if (code === "integrity_mismatch") {
    return t("skills.detail.managedSource.integrityMismatch");
  }

  return fallback;
}

export function AgentExternalVariantPanel({
  agent,
  repoPath,
  sources,
  updatingExternalImportId,
  updatingExternalSourceId,
  onImportVariant,
  onRepairImport,
  onUpdateImport,
}: AgentExternalVariantPanelProps) {
  const { t } = useTranslation();
  const [busyImportAction, setBusyImportAction] = useState<{
    action: "repair" | "update";
    importId: string;
  } | null>(null);
  const relevantSources = useMemo(
    () =>
      sources
        .map((source) => ({
          imports: source.imports.filter((item) => item.agentKey === agent.key),
          record: source.record,
          variants: source.variants.filter((variant) => variant.agentKey === agent.key),
        }))
        .filter((source) => source.imports.length > 0 || source.variants.length > 0),
    [agent.key, sources],
  );

  return (
    <article className="rounded-2xl border border-slate-800 bg-slate-900 p-5">
      <header className="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
        <div className="flex items-start gap-3">
          <div className="grid h-11 w-11 shrink-0 place-items-center rounded-2xl border border-slate-700 bg-slate-950">
            <AgentBrandIcon agentKey={agent.key} className="size-[70%]" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-slate-100">
              {agent.displayName} · {t("externalSources.agentPanel.title")}
            </h3>
            <p className="mt-1 text-sm text-slate-400">
              {t("externalSources.agentPanel.description", { agent: agent.displayName })}
            </p>
          </div>
        </div>
        {!repoPath ? (
          <span className="rounded-full border border-amber-700/60 bg-amber-950/40 px-3 py-1 text-xs text-amber-100">
            {t("sources.variants.repoRequired")}
          </span>
        ) : null}
      </header>

      {relevantSources.length === 0 ? (
        <p className="mt-4 text-sm text-slate-500">{t("externalSources.agentPanel.empty")}</p>
      ) : (
        <div className="mt-4 grid gap-4">
          {relevantSources.map(({ imports, record, variants }) => {
            const isBusySource = updatingExternalSourceId === record.id;

            return (
              <section
                className="rounded-2xl border border-slate-800 bg-slate-950/50 p-4"
                key={`${agent.key}:${record.id}`}
              >
                <div className="flex flex-col gap-3 xl:flex-row xl:items-start xl:justify-between">
                  <div>
                    <div className="flex flex-wrap items-center gap-2">
                      <h4 className="text-sm font-semibold text-slate-100">
                        {externalSourceName(record)}
                      </h4>
                      <span
                        className={`rounded-full border px-2.5 py-1 text-[11px] font-medium ${statusClasses(record.status)}`}
                      >
                        {t(`sources.status.${record.status ?? "pending"}`)}
                      </span>
                    </div>
                    <p className="mt-1 text-xs text-slate-400">{record.repoUrl}</p>
                    <p className="mt-1 text-xs text-slate-500">
                      {t("sources.meta.head", {
                        commit: shortCommit(record.lastFetchedCommit, t("sources.meta.unknown")),
                      })}
                    </p>
                  </div>
                  <div className="flex flex-wrap gap-2 text-[11px]">
                    <span className="rounded-full bg-slate-800 px-2 py-1 text-slate-300">
                      {t("sources.variants.count", { count: variants.length })}
                    </span>
                    <span className="rounded-full bg-slate-800 px-2 py-1 text-slate-300">
                      {t("externalSources.agentPanel.importCount", { count: imports.length })}
                    </span>
                  </div>
                </div>

                <div className="mt-4 grid gap-4 xl:grid-cols-2">
                  <div className="space-y-3 rounded-xl border border-slate-800 bg-slate-950/70 p-4">
                    <div>
                      <h5 className="text-xs font-semibold uppercase tracking-wide text-slate-400">
                        {t("sources.variants.title")}
                      </h5>
                      <p className="mt-1 text-xs text-slate-500">
                        {t("externalSources.agentPanel.variantDescription")}
                      </p>
                    </div>

                    {variants.length === 0 ? (
                      <p className="text-sm text-slate-500">{t("sources.variants.noneDetected")}</p>
                    ) : (
                      <div className="space-y-3">
                        {variants.map((variant) => {
                          const imported = imports.find(
                            (item) => item.upstreamVariantPath === variant.variantPath,
                          );

                          return (
                            <div
                              className="flex flex-col gap-3 rounded-xl border border-slate-800 bg-slate-950 p-3"
                              key={`${record.id}:${variant.variantPath}`}
                            >
                              <div className="space-y-1">
                                <div className="flex flex-wrap items-center gap-2">
                                  <span className="rounded-full border border-slate-700 px-2 py-1 text-[11px] text-slate-300">
                                    {variant.agentKey}
                                  </span>
                                  <span className="text-sm font-medium text-slate-100">
                                    {variant.variantPath}
                                  </span>
                                </div>
                                {variant.sourceOfTruthPath ? (
                                  <p className="text-xs text-slate-500">
                                    {t("sources.variants.sourceOfTruth", {
                                      path: variant.sourceOfTruthPath,
                                    })}
                                  </p>
                                ) : null}
                              </div>
                              <div className="flex justify-end">
                                <button
                                  className="rounded-lg bg-sky-400 px-3 py-2 text-xs font-medium text-slate-950 transition hover:bg-sky-300 disabled:cursor-not-allowed disabled:bg-slate-800 disabled:text-slate-400"
                                  disabled={isBusySource || Boolean(imported) || !repoPath}
                                  onClick={() =>
                                    void onImportVariant(record.id, variant.agentKey, variant.variantPath)
                                  }
                                  type="button"
                                >
                                  {!repoPath
                                    ? t("sources.variants.repoRequired")
                                    : imported
                                      ? t("externalSources.agentPanel.imported")
                                      : isBusySource
                                        ? t("externalSources.agentPanel.importing")
                                        : t("externalSources.agentPanel.import")}
                                </button>
                              </div>
                            </div>
                          );
                        })}
                      </div>
                    )}
                  </div>

                  <div className="space-y-3 rounded-xl border border-slate-800 bg-slate-950/70 p-4">
                    <div>
                      <h5 className="text-xs font-semibold uppercase tracking-wide text-slate-400">
                        {t("sources.imports.title")}
                      </h5>
                      <p className="mt-1 text-xs text-slate-500">
                        {t("externalSources.agentPanel.importDescription")}
                      </p>
                    </div>

                    {imports.length === 0 ? (
                      <p className="text-sm text-slate-500">{t("externalSources.agentPanel.noImports")}</p>
                    ) : (
                      <div className="space-y-3">
                        {imports.map((item) => {
                          const isBusyImport = updatingExternalImportId === item.importId;
                          const isUpdatingImport =
                            isBusyImport &&
                            (busyImportAction?.importId === item.importId
                              ? busyImportAction.action === "update"
                              : item.updateAvailable);
                          const isRepairingImport =
                            isBusyImport &&
                            (busyImportAction?.importId === item.importId
                              ? busyImportAction.action === "repair"
                              : !item.updateAvailable);
                          const warningMessages = item.warnings.map((warning) =>
                            warningText(
                              warning.code,
                              warning.message,
                              t,
                              item.agentKey,
                              item.upstreamVariantPath,
                            ),
                          );

                          return (
                            <div
                              className="rounded-xl border border-slate-800 bg-slate-950 p-3"
                              key={item.importId}
                            >
                              <div className="flex flex-wrap items-start justify-between gap-3">
                                <div className="space-y-1">
                                  <div className="flex flex-wrap items-center gap-2">
                                    <span className="rounded-full border border-slate-700 px-2 py-1 text-[11px] text-slate-300">
                                      {item.agentKey}
                                    </span>
                                    <span className="text-sm font-medium text-slate-100">
                                      {item.upstreamVariantPath}
                                    </span>
                                  </div>
                                  <p className="text-xs text-slate-400">{item.mirrorRelativePath}</p>
                                  <p className="text-xs text-slate-500">
                                    {t("sources.imports.pinned", {
                                      commit: shortCommit(item.pinnedCommit),
                                    })}
                                    {item.lastCheckedCommit
                                      ? t("sources.imports.checked", {
                                          commit: shortCommit(item.lastCheckedCommit),
                                        })
                                      : ""}
                                  </p>
                                </div>
                                <div className="flex flex-wrap gap-2">
                                      <button
                                        className="rounded-lg bg-slate-800 px-3 py-2 text-xs text-slate-100 transition hover:bg-slate-700 disabled:cursor-not-allowed disabled:bg-slate-900 disabled:text-slate-500"
                                        disabled={isBusyImport || !item.updateAvailable}
                                        onClick={() => {
                                          setBusyImportAction({ action: "update", importId: item.importId });
                                          void onUpdateImport(item.importId).finally(() => {
                                            setBusyImportAction((current) =>
                                              current?.importId === item.importId ? null : current,
                                            );
                                          });
                                        }}
                                        type="button"
                                      >
                                        {isUpdatingImport
                                          ? t("externalSources.agentPanel.updating")
                                          : item.updateAvailable
                                            ? t("externalSources.agentPanel.update")
                                        : t("externalSources.agentPanel.upToDate")}
                                  </button>
                                      <button
                                        className="rounded-lg border border-amber-700/60 bg-amber-950/50 px-3 py-2 text-xs text-amber-100 transition hover:bg-amber-900/60 disabled:cursor-not-allowed disabled:opacity-50"
                                        disabled={isBusyImport}
                                        onClick={() => {
                                          setBusyImportAction({ action: "repair", importId: item.importId });
                                          void onRepairImport(item.importId).finally(() => {
                                            setBusyImportAction((current) =>
                                              current?.importId === item.importId ? null : current,
                                            );
                                          });
                                        }}
                                        type="button"
                                      >
                                        {isRepairingImport
                                          ? t("sources.imports.repairing")
                                          : t("sources.imports.repair")}
                                      </button>
                                </div>
                              </div>

                              {warningMessages.length ? (
                                <ul className="mt-3 space-y-1 rounded-xl border border-amber-700/50 bg-amber-950/30 px-3 py-2 text-xs text-amber-100">
                                  {warningMessages.map((message, index) => (
                                    <li key={`${item.importId}:${index}`}>{message}</li>
                                  ))}
                                </ul>
                              ) : null}
                            </div>
                          );
                        })}
                      </div>
                    )}
                  </div>
                </div>
              </section>
            );
          })}
        </div>
      )}
    </article>
  );
}
