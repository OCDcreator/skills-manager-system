import { useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { useRememberedScrollPosition } from "../../lib/scroll-memory";
import type { SkillSummary } from "../../lib/tauri";
import { truncateDescription } from "../../lib/skills/filters";

interface SkillListProps {
  title: string;
  skills: SkillSummary[];
  selectedSkillId: string | null;
  disabledSkillIds: ReadonlySet<string>;
  updatingSkillId: string | null;
  onSelect: (skill: SkillSummary) => void;
  onSetManyEnabled: (skillIds: string[], enabled: boolean) => Promise<void>;
  onToggleEnabled: (skillId: string, enabled: boolean) => Promise<void>;
}

export function SkillList(props: SkillListProps) {
  const {
    disabledSkillIds,
    onSelect,
    onSetManyEnabled,
    onToggleEnabled,
    selectedSkillId,
    skills,
    title,
    updatingSkillId,
  } = props;
  const { t } = useTranslation();
  const [isSelectionMode, setIsSelectionMode] = useState(false);
  const [selectedSkillIds, setSelectedSkillIds] = useState<string[]>([]);
  const visibleSkillIds = useMemo(() => skills.map((skill) => skill.id), [skills]);
  const selectedSkillIdSet = useMemo(() => new Set(selectedSkillIds), [selectedSkillIds]);
  const targetSkillIds = isSelectionMode ? selectedSkillIds : visibleSkillIds;
  const canEnableTargets = targetSkillIds.some((skillId) => disabledSkillIds.has(skillId));
  const canDisableTargets = targetSkillIds.some((skillId) => !disabledSkillIds.has(skillId));
  const isBusy = updatingSkillId !== null;
  const scrollRef = useRememberedScrollPosition(`skills:list:${title}`);

  useEffect(() => {
    const visibleSkillIdSet = new Set(visibleSkillIds);
    setSelectedSkillIds((currentIds) =>
      currentIds.filter((skillId) => visibleSkillIdSet.has(skillId)),
    );
  }, [visibleSkillIds]);

  function toggleSelectionMode() {
    setIsSelectionMode((currentMode) => !currentMode);
    setSelectedSkillIds([]);
  }

  function toggleSelectedSkill(skillId: string) {
    setSelectedSkillIds((currentIds) =>
      currentIds.includes(skillId)
        ? currentIds.filter((currentId) => currentId !== skillId)
        : [...currentIds, skillId],
    );
  }

  async function handleBulkSetEnabled(enabled: boolean) {
    const skillIds = targetSkillIds.filter((skillId) =>
      enabled ? disabledSkillIds.has(skillId) : !disabledSkillIds.has(skillId),
    );
    if (skillIds.length === 0) {
      return;
    }

    await onSetManyEnabled(skillIds, enabled);
    if (isSelectionMode) {
      setSelectedSkillIds([]);
    }
  }

  return (
    <section className="flex max-h-[clamp(34rem,calc(100vh-6rem),64rem)] flex-col overflow-hidden rounded-2xl border border-slate-800 bg-slate-900 p-4">
      <header className="flex flex-wrap items-start justify-between gap-3">
        <div className="min-w-0">
          <div className="flex flex-wrap items-center gap-2">
            <h3 className="text-sm font-semibold uppercase tracking-wide text-slate-300">{title}</h3>
            <span className="text-xs text-slate-500">{skills.length}</span>
            {isSelectionMode ? (
              <span className="text-xs text-slate-500">
                {t("skills.bulk.selectedCount", { count: selectedSkillIds.length })}
              </span>
            ) : null}
          </div>
        </div>
        {skills.length > 0 ? (
          <div className="flex flex-wrap justify-end gap-2">
            <button
              className="rounded-lg border border-slate-700 bg-slate-950 px-3 py-2 text-xs text-slate-100 disabled:opacity-50"
              disabled={isBusy || !canEnableTargets}
              onClick={() => void handleBulkSetEnabled(true)}
              type="button"
            >
              {isSelectionMode ? t("skills.bulk.enableSelected") : t("skills.bulk.enableAll")}
            </button>
            <button
              className="rounded-lg border border-slate-700 bg-slate-950 px-3 py-2 text-xs text-slate-100 disabled:opacity-50"
              disabled={isBusy || !canDisableTargets}
              onClick={() => void handleBulkSetEnabled(false)}
              type="button"
            >
              {isSelectionMode ? t("skills.bulk.disableSelected") : t("skills.bulk.disableAll")}
            </button>
            <button
              className={`rounded-lg border px-3 py-2 text-xs transition ${
                isSelectionMode
                  ? "border-cyan-400/60 bg-cyan-400/10 text-cyan-100"
                  : "border-slate-700 bg-slate-950 text-slate-100"
              }`}
              onClick={toggleSelectionMode}
              type="button"
            >
              {isSelectionMode ? t("skills.bulk.cancelSelect") : t("skills.bulk.select")}
            </button>
          </div>
        ) : null}
      </header>
      {skills.length === 0 ? (
        <div className="mt-3 rounded-xl border border-dashed border-slate-700 px-4 py-6 text-sm text-slate-500">
          {t("skills.empty")}
        </div>
      ) : null}
      {skills.length > 0 ? (
        <div
          className="skill-markdown-scroll -mr-3 mt-3 min-h-0 overflow-y-auto pr-3"
          ref={scrollRef}
        >
          <div className="grid auto-rows-[13.5rem] gap-3 [grid-template-columns:repeat(auto-fit,minmax(18rem,1fr))]">
            {skills.map((skill) => {
              const isDisabled = disabledSkillIds.has(skill.id);
              const isUpdating = updatingSkillId === skill.id;
              const isChecked = selectedSkillIdSet.has(skill.id);
              const externalBadge = skill.sourceType === "external"
                ? skill.managedSource
                  ? {
                      className: "bg-sky-500/15 text-sky-100",
                      label: t("skills.badges.managedGithubMirror"),
                    }
                  : {
                      className: "bg-amber-500/15 text-amber-200",
                      label: t("skills.badges.manualExternal"),
                    }
                : null;

              return (
                <article
                  key={skill.id}
                  className={`relative flex h-full min-h-0 flex-col overflow-hidden rounded-xl border p-4 transition ${
                    isChecked
                      ? "border-cyan-400 bg-cyan-400/10 ring-1 ring-cyan-400/50"
                      : selectedSkillId === skill.id
                      ? "border-sky-400 bg-sky-400/10"
                      : "border-slate-800 bg-slate-950"
                    } ${isDisabled ? "opacity-70" : ""}`}
                >
                  {isSelectionMode ? (
                    <input
                      aria-label={t("skills.bulk.selectCard")}
                      checked={isChecked}
                      className="absolute right-4 top-4 z-10 h-4 w-4 rounded border-slate-600 bg-slate-950 text-cyan-400 accent-cyan-400"
                      onChange={() => toggleSelectedSkill(skill.id)}
                      type="checkbox"
                    />
                  ) : null}
                  <button
                    className="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden text-left"
                    onClick={() => {
                      if (isSelectionMode) {
                        toggleSelectedSkill(skill.id);
                        return;
                      }
                      onSelect(skill);
                    }}
                    title={isSelectionMode ? t("tooltip.skills.selectForBulk") : t("tooltip.skills.select")}
                    type="button"
                  >
                    <div className="min-w-0 pr-8">
                      <strong className="block truncate text-sm font-semibold text-slate-100">{skill.name}</strong>
                    </div>
                    <div className="mt-2 flex max-h-12 min-h-[1.625rem] flex-wrap items-start gap-2 overflow-hidden pr-8">
                      <span className="rounded-full bg-slate-800 px-2 py-1 text-xs text-slate-300">
                        {skill.sourceType === "custom"
                          ? t("skills.source.custom")
                          : t("skills.source.external")}
                      </span>
                      <span
                        className={`rounded-full px-2 py-1 text-xs ${
                          isDisabled
                            ? "bg-amber-500/15 text-amber-200"
                            : "bg-emerald-500/15 text-emerald-200"
                        }`}
                      >
                        {isDisabled
                          ? t("skills.status.disabled")
                          : t("skills.status.enabled")}
                      </span>
                      {externalBadge ? (
                        <span className={`rounded-full px-2 py-1 text-xs ${externalBadge.className}`}>
                          {externalBadge.label}
                        </span>
                      ) : null}
                    </div>
                    <p className="mt-2 overflow-hidden text-sm text-slate-400 [display:-webkit-box] [-webkit-box-orient:vertical] [-webkit-line-clamp:2]">
                      {skill.description
                        ? truncateDescription(skill.description)
                        : t("skills.noDescription")}
                    </p>
                    <p className="mt-3 truncate text-xs text-slate-500">{skill.relativePath}</p>
                  </button>

                  {!isSelectionMode ? (
                    <div className="mt-4 flex justify-end">
                      <button
                        className="rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-xs text-slate-100 disabled:opacity-60"
                        disabled={isUpdating}
                        onClick={() => void onToggleEnabled(skill.id, isDisabled)}
                        title={isDisabled ? t("tooltip.skills.toggle.enable") : t("tooltip.skills.toggle.disable")}
                        type="button"
                      >
                        {isDisabled ? t("skills.toggle.enable") : t("skills.toggle.disable")}
                      </button>
                    </div>
                  ) : null}
                </article>
              );
            })}
          </div>
        </div>
      ) : null}
    </section>
  );
}
