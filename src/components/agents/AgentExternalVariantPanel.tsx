import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import type {
  AgentInventoryItem,
  ExternalVariantKey,
  ExternalSourceSnapshotItem,
} from "../../lib/tauri";
import { AgentBrandIcon } from "./AgentBrandIcon";
import { AgentExternalSourceSection } from "./AgentExternalSourceSection";

interface AgentExternalVariantPanelProps {
  agent: AgentInventoryItem;
  repoPath: string | null;
  sources: ExternalSourceSnapshotItem[];
  updatingExternalImportId: string | null;
  updatingExternalSourceId: string | null;
  onImportVariant: (
    sourceId: string,
    agentKey: ExternalVariantKey,
    variantPath: string,
  ) => Promise<void>;
  onRepairImport: (importId: string) => Promise<void>;
  onUpdateImport: (importId: string) => Promise<void>;
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
  const relevantSources = useMemo(
    () =>
      sources
        .map((source) => ({
          imports: source.imports.filter((item) => item.agentKey === agent.key),
          record: source.record,
          variants: source.variants.filter(
            (variant) =>
              variant.agentKey === agent.key || variant.suggestedTargetAgents.includes(agent.key),
          ),
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
          {relevantSources.map(({ imports, record, variants }) => (
            <AgentExternalSourceSection
              agent={agent}
              imports={imports}
              key={`${agent.key}:${record.id}`}
              onImportVariant={onImportVariant}
              onRepairImport={onRepairImport}
              onUpdateImport={onUpdateImport}
              record={record}
              repoPath={repoPath}
              updatingExternalImportId={updatingExternalImportId}
              updatingExternalSourceId={updatingExternalSourceId}
              variants={variants}
            />
          ))}
        </div>
      )}
    </article>
  );
}
