import type {
  AgentKey,
  ExternalSourceRecord,
  ExternalSourceWarning,
  ExternalVariantSnapshot,
  ImportedExternalSkillRecord,
} from "./tauri";

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

const AGENT_LABELS: Record<string, string> = {
  codex: "Codex",
  claude_code: "Claude Code",
  opencode: "OpenCode",
  cursor: "Cursor",
  amp: "Amp",
  kilo_code: "Kilo Code",
  kimi: "Kimi Code",
  roo_code: "Roo Code",
  goose: "Goose",
  gemini_cli: "Gemini CLI",
  github_copilot: "GitHub Copilot",
  windsurf: "Windsurf",
  skill_repository: "Skill Repository",
};

export const EXTERNAL_IMPORT_TARGETS: AgentKey[] = [
  "codex",
  "claude_code",
  "opencode",
  "cursor",
  "amp",
  "kilo_code",
  "kimi",
  "roo_code",
  "goose",
  "gemini_cli",
  "github_copilot",
  "windsurf",
];

export function agentLabel(agentKey: string) {
  return AGENT_LABELS[agentKey] ?? agentKey;
}

export function sourceAgentLabels(variants: ExternalVariantSnapshot[]) {
  const labels = new Set<string>();
  for (const variant of variants) {
    if (variant.suggestedTargetAgents.length) {
      for (const target of variant.suggestedTargetAgents) {
        labels.add(agentLabel(target));
      }
      continue;
    }
    if (variant.detectedAgentHint) {
      labels.add(agentLabel(variant.detectedAgentHint));
      continue;
    }
    labels.add(agentLabel(variant.agentKey));
  }
  return [...labels];
}

export function variantsWithSameContent(
  variant: ExternalVariantSnapshot,
  variants: ExternalVariantSnapshot[],
) {
  if (!variant.contentFingerprint) return [];
  return variants.filter(
    (item) =>
      item.variantPath !== variant.variantPath
      && item.contentFingerprint === variant.contentFingerprint,
  );
}

export function equivalentContentGroupCount(variants: ExternalVariantSnapshot[]) {
  const counts = new Map<string, number>();
  for (const variant of variants) {
    if (!variant.contentFingerprint) continue;
    counts.set(
      variant.contentFingerprint,
      (counts.get(variant.contentFingerprint) ?? 0) + 1,
    );
  }
  return [...counts.values()].filter((count) => count > 1).length;
}

export type ExternalVariantAgentGroupKey = AgentKey | "manual";

export interface ExternalVariantAgentGroup {
  agentKey: ExternalVariantAgentGroupKey;
  variants: ExternalVariantSnapshot[];
  imports: ImportedExternalSkillRecord[];
}

function isKnownImportTarget(agentKey: string): agentKey is AgentKey {
  return EXTERNAL_IMPORT_TARGETS.includes(agentKey as AgentKey);
}

function variantTargetAgents(variant: ExternalVariantSnapshot): ExternalVariantAgentGroupKey[] {
  if (variant.suggestedTargetAgents.length) {
    return variant.suggestedTargetAgents;
  }
  if (variant.detectedAgentHint) {
    return [variant.detectedAgentHint];
  }
  if (isKnownImportTarget(variant.agentKey)) {
    return [variant.agentKey];
  }
  return ["manual"];
}

export function groupExternalVariantsByAgent(
  variants: ExternalVariantSnapshot[],
  imports: ImportedExternalSkillRecord[],
): ExternalVariantAgentGroup[] {
  const groups = new Map<ExternalVariantAgentGroupKey, ExternalVariantAgentGroup>();

  function ensureGroup(agentKey: ExternalVariantAgentGroupKey) {
    const existing = groups.get(agentKey);
    if (existing) return existing;
    const group = { agentKey, variants: [], imports: [] };
    groups.set(agentKey, group);
    return group;
  }

  for (const variant of variants) {
    for (const agentKey of variantTargetAgents(variant)) {
      ensureGroup(agentKey).variants.push(variant);
    }
  }

  for (const item of imports) {
    const agentKey = isKnownImportTarget(item.agentKey) ? item.agentKey : "manual";
    ensureGroup(agentKey).imports.push(item);
  }

  const orderedKeys: ExternalVariantAgentGroupKey[] = [
    ...EXTERNAL_IMPORT_TARGETS.filter((agentKey) => groups.has(agentKey)),
  ];
  for (const key of groups.keys()) {
    if (key !== "manual" && !orderedKeys.includes(key)) {
      orderedKeys.push(key);
    }
  }
  if (groups.has("manual")) {
    orderedKeys.push("manual");
  }

  return orderedKeys.map((agentKey) => groups.get(agentKey)!);
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
