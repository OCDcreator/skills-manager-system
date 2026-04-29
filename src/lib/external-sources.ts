import type { ExternalSourceRecord, ExternalSourceWarning, ImportedExternalSkillRecord } from "./tauri";

export function externalSourceName(record: ExternalSourceRecord) {
  const trimmed = record.repoUrl.trim().replace(/\/+$/, "");
  const segments = trimmed.split(/[/:]/).filter(Boolean);
  return segments.slice(-2).join("/") || trimmed;
}

export function shortCommit(
  commit: string | null | undefined,
  unknownLabel = "unknown",
) {
  return commit ? commit.slice(0, 8) : unknownLabel;
}

export function statusClasses(status: ExternalSourceRecord["status"]) {
  if (status === "error") return "border-rose-700/60 bg-rose-950/50 text-rose-200";
  if (status === "warning") return "border-amber-700/60 bg-amber-950/50 text-amber-200";
  if (status === "ok") return "border-emerald-700/60 bg-emerald-950/50 text-emerald-200";
  return "border-slate-700 bg-slate-900 text-slate-200";
}

export function warningSummary(
  warnings: ExternalSourceWarning[],
  warningCountLabel: string,
  noWarningsLabel = "",
) {
  if (!warnings.length) return noWarningsLabel;
  if (warnings.length === 1) return warnings[0].message;
  return warningCountLabel;
}

export interface BusyImportAction {
  action: "repair" | "update";
  importId: string;
}

export function resolveImportBusyState(
  item: ImportedExternalSkillRecord,
  updatingImportId: string | null,
  busyImportAction: BusyImportAction | null,
) {
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

  return {
    isBusyImport,
    isUpdatingImport,
    isRepairingImport,
  };
}
