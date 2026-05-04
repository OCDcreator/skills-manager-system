import { useTranslation } from "react-i18next";
import type { ExternalSourceSnapshotItem, ExternalVariantKey } from "../../lib/tauri";
import { ExternalSourceCard } from "./ExternalSourceCard";

interface ExternalSourceListProps {
  sources: ExternalSourceSnapshotItem[];
  repoPath: string | null;
  updatingExternalSourceId: string | null;
  updatingExternalImportId: string | null;
  onFetchSource: (sourceId: string) => Promise<void>;
  onImportVariant: (sourceId: string, agentKey: ExternalVariantKey, variantPath: string) => Promise<void>;
  onRemoveSource: (sourceId: string, removeImports: boolean) => Promise<void>;
  onUpdateImport: (importId: string) => Promise<void>;
  onRepairImport: (importId: string) => Promise<void>;
}

export function ExternalSourceList(props: ExternalSourceListProps) {
  const { t } = useTranslation();

  if (!props.sources.length) {
    return (
      <section className="rounded-2xl border border-dashed border-slate-700 bg-slate-900/60 p-10 text-center">
        <h3 className="text-lg font-semibold text-slate-100">{t("sources.emptyTitle")}</h3>
        <p className="mt-2 text-sm text-slate-400">
          {t("sources.emptyBody")}
        </p>
      </section>
    );
  }

  return (
    <div className="space-y-5">
      {props.sources.map((source) => (
        <ExternalSourceCard
          key={source.record.id}
          onFetchSource={props.onFetchSource}
          onImportVariant={props.onImportVariant}
          onRemoveSource={props.onRemoveSource}
          onRepairImport={props.onRepairImport}
          onUpdateImport={props.onUpdateImport}
          repoPath={props.repoPath}
          source={source}
          updatingExternalImportId={props.updatingExternalImportId}
          updatingExternalSourceId={props.updatingExternalSourceId}
        />
      ))}
    </div>
  );
}
