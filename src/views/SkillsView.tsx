import { Suspense, lazy, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { SkillFilters } from "../components/skills/SkillFilters";
import { SkillList } from "../components/skills/SkillList";
import { useAppContext } from "../context/AppContext";
import type { SkillSummary } from "../lib/tauri";
import {
  buildSourceSummaries,
  buildStatusSummaries,
  filterSkills,
  groupSkills,
  type SkillStatusFilter,
  type SourceFilter,
  resolveVisibleSources,
} from "../lib/skills/filters";

const SkillDetailPanel = lazy(() =>
  import("../components/skills/SkillDetailPanel").then((module) => ({
    default: module.SkillDetailPanel,
  })),
);

function SkillDetailPlaceholder({ message }: { message: string }) {
  return (
    <aside className="min-w-0 rounded-2xl border border-slate-800 bg-slate-900 p-6 text-sm text-slate-400 min-[1280px]:sticky min-[1280px]:top-8 min-[1280px]:max-h-[calc(100vh-7rem)]">
      {message}
    </aside>
  );
}

export function SkillsView() {
  const { t } = useTranslation();
  const {
    disabledSkillIds,
    errorMessage,
    isLoading,
    refreshSkills,
    repoPath,
    scanResult,
    selectSkill,
    selectedDocument,
    selectedSkill,
    setSkillEnabled,
    updatingSkillId,
  } = useAppContext();
  const [search, setSearch] = useState("");
  const [sourceFilter, setSourceFilter] = useState<SourceFilter>("all");
  const [statusFilter, setStatusFilter] = useState<SkillStatusFilter>("all");
  const [isCompactDetailOpen, setIsCompactDetailOpen] = useState(false);

  const disabledSkillIdSet = useMemo(
    () => new Set(disabledSkillIds),
    [disabledSkillIds],
  );

  const filteredSkills = useMemo(
    () =>
      filterSkills(
        scanResult.skills,
        search,
        sourceFilter,
        statusFilter,
        disabledSkillIdSet,
      ),
    [scanResult.skills, search, sourceFilter, statusFilter, disabledSkillIdSet],
  );
  const grouped = useMemo(() => groupSkills(filteredSkills), [filteredSkills]);
  const visibleSources = useMemo(
    () => resolveVisibleSources(sourceFilter),
    [sourceFilter],
  );
  const summaries = useMemo(
    () => buildSourceSummaries(scanResult.skills),
    [scanResult.skills],
  );
  const statusSummaries = useMemo(
    () => buildStatusSummaries(scanResult.skills, disabledSkillIdSet),
    [scanResult.skills, disabledSkillIdSet],
  );
  const selectedSkillEnabled = selectedSkill
    ? !disabledSkillIdSet.has(selectedSkill.id)
    : true;
  const isInitialSkillLoad = isLoading && scanResult.skills.length === 0;

  async function handleSetManySkillsEnabled(skillIds: string[], enabled: boolean) {
    for (const skillId of skillIds) {
      await setSkillEnabled(skillId, enabled);
    }
  }

  async function handleSelectSkill(skill: SkillSummary) {
    await selectSkill(skill);
    setIsCompactDetailOpen(true);
  }

  const detailPanel = selectedSkill ? (
    <Suspense fallback={<SkillDetailPlaceholder message={t("skills.detail.loadingDocument")} />}>
      <SkillDetailPanel
        document={selectedDocument}
        isEnabled={selectedSkillEnabled}
        skill={selectedSkill}
      />
    </Suspense>
  ) : (
    <SkillDetailPlaceholder
      message={isInitialSkillLoad ? t("skills.detail.loadingInitial") : t("skills.selectPrompt")}
    />
  );

  return (
    <>
      {!repoPath ? (
        <section
          className="mx-auto max-w-[52rem] rounded-2xl border border-dashed border-slate-700 bg-slate-900 p-6 text-center shadow-[0_18px_60px_rgba(2,6,23,0.18)] sm:p-8"
          data-skills-unconfigured
        >
          <h2 className="text-xl font-semibold text-slate-100 [text-wrap:balance]">
            {t("skills.unconfigured")}
          </h2>
          <p className="mx-auto mt-3 max-w-[38rem] break-keep text-sm leading-6 text-slate-400">
            {t("skills.unconfiguredBody")}
          </p>
        </section>
      ) : (
        <div className="grid gap-6 min-[1280px]:grid-cols-[minmax(0,1fr)_clamp(24rem,30vw,42rem)]">
          <div className="min-w-0 space-y-6">
            <SkillFilters
              isRefreshing={isLoading}
              onRefresh={refreshSkills}
              onSearchChange={setSearch}
              onSourceFilterChange={setSourceFilter}
              onStatusFilterChange={setStatusFilter}
              search={search}
              sourceFilter={sourceFilter}
              statusFilter={statusFilter}
              summaries={summaries}
              statusSummaries={statusSummaries}
              isInitialLoading={isInitialSkillLoad}
            />

            {scanResult.warnings.length > 0 ? (
              <div className="rounded-2xl border border-amber-700/50 bg-amber-950/40 p-4 text-sm text-amber-100">
                {scanResult.warnings.join(" ")}
              </div>
            ) : null}

            {errorMessage ? (
              <div className="rounded-2xl border border-rose-700/50 bg-rose-950/40 p-4 text-sm text-rose-100">
                {errorMessage}
              </div>
            ) : null}

            <div
              className={`grid gap-6 ${
                visibleSources.length === 2 ? "min-[1536px]:grid-cols-2" : ""
              }`}
            >
              {visibleSources.map((source) => (
                <SkillList
                  key={source}
                  disabledSkillIds={disabledSkillIdSet}
                  onSelect={(skill) => void handleSelectSkill(skill)}
                  onSetManyEnabled={handleSetManySkillsEnabled}
                  onToggleEnabled={setSkillEnabled}
                  isLoading={isInitialSkillLoad}
                  selectedSkillId={selectedSkill?.id ?? null}
                  skills={grouped[source]}
                  title={
                    source === "custom"
                      ? t("skills.section.custom")
                      : t("skills.section.external")
                  }
                  updatingSkillId={updatingSkillId}
                />
              ))}
            </div>
          </div>

          <aside
            className="hidden min-[1280px]:block"
            data-skills-detail-wide
          >
            {detailPanel}
          </aside>
        </div>
      )}

      {selectedSkill ? (
        <div
          className={`hidden min-[900px]:max-[1279px]:block ${isCompactDetailOpen ? "" : "pointer-events-none opacity-0"}`}
          data-skills-detail-drawer
        >
          <div className="fixed inset-y-0 right-0 z-40 w-[min(30rem,calc(100vw-2rem))] border-l border-slate-800 bg-slate-950/98 p-4 shadow-2xl">
            <div className="mb-3 flex items-center justify-between gap-3">
              <h2 className="min-w-0 truncate text-sm font-semibold text-slate-100">
                {selectedSkill.name}
              </h2>
              <button
                className="rounded-lg border border-slate-700 px-3 py-2 text-xs text-slate-200"
                onClick={() => setIsCompactDetailOpen(false)}
                type="button"
              >
                {t("skills.detail.close")}
              </button>
            </div>
            {detailPanel}
          </div>
        </div>
      ) : null}

      {selectedSkill ? (
        <section
          className="min-[900px]:hidden"
          data-skills-detail-stacked
        >
          {detailPanel}
        </section>
      ) : null}
    </>
  );
}
