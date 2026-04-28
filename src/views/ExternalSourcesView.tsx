import { AddExternalSourceForm } from "../components/external-sources/AddExternalSourceForm";
import { ExternalSourceList } from "../components/external-sources/ExternalSourceList";
import { useAppContext } from "../context/AppContext";
import { useTranslation } from "react-i18next";

export function ExternalSourcesView() {
  const { t } = useTranslation();
  const {
    addExternalSource,
    externalSources,
    fetchExternalSource,
    importExternalVariant,
    isAddingExternalSource,
    isLoadingExternalSources,
    refreshExternalSources,
    removeExternalSource,
    repoPath,
    repairExternalImport,
    updateExternalImport,
    updatingExternalImportId,
    updatingExternalSourceId,
  } = useAppContext();

  return (
    <div className="space-y-6">
      <section className="rounded-2xl border border-slate-800 bg-slate-900 p-6">
        <div className="flex flex-col gap-4 xl:flex-row xl:items-start xl:justify-between">
          <div>
            <h2 className="text-2xl font-semibold text-slate-100">{t("sources.title")}</h2>
            <p className="mt-2 max-w-3xl text-sm text-slate-400">
              {t("sources.description")}
            </p>
          </div>
          <button
            className="rounded-xl bg-slate-800 px-4 py-2 text-sm text-slate-100 transition hover:bg-slate-700 disabled:cursor-not-allowed disabled:bg-slate-900 disabled:text-slate-500"
            disabled={isLoadingExternalSources}
            onClick={() => void refreshExternalSources()}
            type="button"
          >
            {isLoadingExternalSources ? t("sources.refreshing") : t("sources.refresh")}
          </button>
        </div>

        {!repoPath ? (
          <div className="mt-4 rounded-xl border border-amber-700/50 bg-amber-950/40 p-4 text-sm text-amber-100">
            {t("sources.repoMissing")}
          </div>
        ) : null}
      </section>

      <AddExternalSourceForm isSubmitting={isAddingExternalSource} onSubmit={addExternalSource} />

      {isLoadingExternalSources && !externalSources.length ? (
        <section className="rounded-2xl border border-slate-800 bg-slate-900 p-8 text-sm text-slate-400">
          {t("sources.loading")}
        </section>
      ) : (
        <ExternalSourceList
          onFetchSource={fetchExternalSource}
          onImportVariant={importExternalVariant}
          onRemoveSource={removeExternalSource}
          onRepairImport={repairExternalImport}
          onUpdateImport={updateExternalImport}
          repoPath={repoPath}
          sources={externalSources}
          updatingExternalImportId={updatingExternalImportId}
          updatingExternalSourceId={updatingExternalSourceId}
        />
      )}
    </div>
  );
}
