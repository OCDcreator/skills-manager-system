import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { SkillDetailPanel } from "../components/skills/SkillDetailPanel";
import { SkillFilters } from "../components/skills/SkillFilters";
import { SkillList } from "../components/skills/SkillList";
import { useAppContext } from "../context/AppContext";
import {
  buildSourceSummaries,
  buildStatusSummaries,
  filterSkills,
  groupSkills,
  type SkillStatusFilter,
  type SourceFilter,
  resolveVisibleSources,
} from "../lib/skills/filters";

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

  return (
    <>
      {!repoPath ? (
        <section className="rounded-2xl border border-dashed border-slate-700 bg-slate-900 p-8 text-center">
          <h2 className="text-xl font-semibold text-slate-100">
            {t("skills.unconfigured")}
          </h2>
          <p className="mt-3 text-sm text-slate-400">{t("skills.unconfiguredBody")}</p>
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
                visibleSources.length === 2 ? "min-[1400px]:grid-cols-2" : ""
              }`}
            >
              {visibleSources.map((source) => (
                <SkillList
                  key={source}
                  disabledSkillIds={disabledSkillIdSet}
                  onSelect={(skill) => void selectSkill(skill)}
                  onToggleEnabled={setSkillEnabled}
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

          <SkillDetailPanel
            document={selectedDocument}
            isEnabled={selectedSkillEnabled}
            skill={selectedSkill}
          />
        </div>
      )}
    </>
  );
}
