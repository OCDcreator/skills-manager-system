import { useState } from "react";
import { useTranslation } from "react-i18next";
import { shortCommit } from "../../lib/external-sources";
import type { ImportedExternalSkillRecord } from "../../lib/tauri";

interface ExternalImportListProps {
  imports: ImportedExternalSkillRecord[];
  updatingImportId: string | null;
  onUpdateImport: (importId: string) => Promise<void>;
  onRepairImport: (importId: string) => Promise<void>;
}

export function ExternalImportList({
  imports,
  updatingImportId,
  onUpdateImport,
  onRepairImport,
}: ExternalImportListProps) {
  const { t } = useTranslation();
  const [busyImportAction, setBusyImportAction] = useState<{
    action: "repair" | "update";
    importId: string;
  } | null>(null);

  if (!imports.length) {
    return <p className="text-sm text-slate-400">{t("sources.imports.empty")}</p>;
  }

  return (
    <div className="space-y-3">
      {imports.map((item) => {
        const isBusyImport = updatingImportId === item.importId;
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
        return (
          <article
            key={item.importId}
            className="rounded-xl border border-slate-800 bg-slate-950/70 p-4"
          >
            <div className="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
              <div className="space-y-1">
                <div className="flex flex-wrap items-center gap-2">
                  <span className="rounded-full border border-slate-700 px-2 py-1 text-xs text-slate-300">
                    {item.agentKey}
                  </span>
                  <span className="text-sm font-medium text-slate-100">{item.skillId}</span>
                </div>
                <p className="text-xs text-slate-400">{item.mirrorRelativePath}</p>
                <p className="text-xs text-slate-500">
                  {t("sources.imports.pinned", { commit: shortCommit(item.pinnedCommit) })}
                  {item.lastCheckedCommit
                    ? t("sources.imports.checked", {
                        commit: shortCommit(item.lastCheckedCommit),
                      })
                    : ""}
                </p>
                {item.warnings.length ? (
                  <ul className="space-y-1 pt-1 text-xs text-amber-200">
                    {item.warnings.map((warning) => (
                      <li key={`${item.importId}-${warning.code}`}>{warning.message}</li>
                    ))}
                  </ul>
                ) : null}
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
                    ? t("sources.imports.updating")
                    : item.updateAvailable
                      ? t("sources.imports.update")
                      : t("sources.imports.upToDate")}
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
                  {isRepairingImport ? t("sources.imports.repairing") : t("sources.imports.repair")}
                </button>
              </div>
            </div>
          </article>
        );
      })}
    </div>
  );
}
