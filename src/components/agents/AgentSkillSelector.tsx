import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { toggleId, type AgentConfigDraft } from "../../lib/agent-selection";
import { useRememberedScrollPosition } from "../../lib/scroll-memory";
import {
  buildSkillPathSummaries,
  matchesSkillPathFilter,
  type SkillPathFilter,
} from "../../lib/skills/filters";
import type { SkillSummary } from "../../lib/tauri";

interface AgentSkillSelectorProps {
  disabledSkillIds: string[];
  draft: AgentConfigDraft;
  skills: SkillSummary[];
  onDraftChange: (draft: AgentConfigDraft) => void;
}

export function AgentSkillSelector({
  disabledSkillIds,
  draft,
  skills,
  onDraftChange,
}: AgentSkillSelectorProps) {
  const { t } = useTranslation();
  const [search, setSearch] = useState("");
  const [showSelectedOnly, setShowSelectedOnly] = useState(false);
  const [pathFilter, setPathFilter] = useState<SkillPathFilter>("all");
  const disabled = useMemo(() => new Set(disabledSkillIds), [disabledSkillIds]);
  const selected = useMemo(() => new Set(draft.selectedSkillIds), [draft.selectedSkillIds]);
  const pathSummaries = useMemo(() => buildSkillPathSummaries(skills), [skills]);
  const filteredSkills = useMemo(() => {
    const lowered = search.trim().toLowerCase();
    return skills.filter((skill) => {
      if (showSelectedOnly && !selected.has(skill.id)) {
        return false;
      }
      if (!matchesSkillPathFilter(skill, pathFilter)) {
        return false;
      }
      if (!lowered) {
        return true;
      }
      return [skill.name, skill.description, skill.relativePath]
        .join(" ")
        .toLowerCase()
        .includes(lowered);
    });
  }, [pathFilter, search, selected, showSelectedOnly, skills]);
  const scrollRef = useRememberedScrollPosition(`agents:skill-selector:${draft.key}`);
  const pathButtonClass = (key: SkillPathFilter) =>
    `rounded-full border px-2 py-1 transition ${
      pathFilter === key
        ? "border-sky-500/60 bg-sky-500/15 text-sky-100"
        : "border-slate-700 bg-slate-900 text-slate-300 hover:bg-slate-800"
    }`;

  const toggleSkill = (skillId: string) => {
    const nextSelected = toggleId(draft.selectedSkillIds, skillId);
    const nextExcluded = nextSelected.includes(skillId)
      ? draft.excludedSkillIds.filter((id) => id !== skillId)
      : draft.excludedSkillIds;
    onDraftChange({
      ...draft,
      excludedSkillIds: nextExcluded,
      selectedSkillIds: nextSelected,
    });
  };

  return (
    <div className="space-y-2 rounded-xl border border-slate-800 bg-slate-950/50 p-3">
      <div className="flex items-center justify-between gap-3">
        <label className="text-xs font-semibold uppercase tracking-wide text-slate-500">
          {t("agents.card.directSkills")}
        </label>
        <span className="text-[11px] text-slate-500">
          {t("agents.card.selectedCount", { count: draft.selectedSkillIds.length })}
        </span>
      </div>
      <div className="flex items-center gap-2">
        <input
          className="min-w-0 flex-1 rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-xs text-slate-100 outline-none focus:border-sky-400"
          onChange={(event) => setSearch(event.target.value)}
          placeholder={t("agents.card.searchSkills")}
          value={search}
        />
        <button
          className={`shrink-0 rounded-lg border px-3 py-2 text-xs transition ${
            showSelectedOnly
              ? "border-sky-500/70 bg-sky-500/15 text-sky-100"
              : "border-slate-700 bg-slate-900 text-slate-300 hover:bg-slate-800"
          }`}
          onClick={() => setShowSelectedOnly((current) => !current)}
          type="button"
        >
          {t("agents.card.selectedOnly")}
        </button>
      </div>
      <div className="flex flex-wrap gap-2 text-[11px] text-slate-400">
        {pathSummaries.map((summary) => (
          <button
            className={pathButtonClass(summary.key)}
            key={summary.key}
            onClick={() => setPathFilter(summary.key)}
            type="button"
          >
            {summary.key === "all"
              ? t("agents.card.allPaths", { count: summary.count })
              : `${summary.key} · ${summary.count}`}
          </button>
        ))}
      </div>
      <div
        className="skill-markdown-scroll max-h-56 space-y-1 overflow-y-auto pr-1"
        ref={scrollRef}
      >
        {filteredSkills.length === 0 ? (
          <div className="px-2 py-1.5 text-xs text-slate-500">
            {t("agents.card.noMatchingDirectSkills")}
          </div>
        ) : (
          filteredSkills.map((skill) => {
            const isSelected = selected.has(skill.id);
            const isGloballyDisabled = disabled.has(skill.id);
            return (
              <label
                className={`flex items-start gap-2 rounded-md px-2 py-1.5 text-xs ${
                  isGloballyDisabled ? "text-slate-500" : "text-slate-300"
                }`}
                key={skill.id}
              >
                <input
                  checked={isSelected}
                  className="mt-0.5 rounded border-slate-600"
                  disabled={isGloballyDisabled && !isSelected}
                  onChange={() => toggleSkill(skill.id)}
                  type="checkbox"
                />
                <span className="min-w-0 flex-1">
                  <span className="block truncate">{skill.name}</span>
                  <span className="block truncate text-[11px] text-slate-500">
                    {skill.relativePath}
                  </span>
                </span>
                {isGloballyDisabled ? (
                  <span className="rounded bg-amber-500/10 px-1.5 py-0.5 text-[10px] text-amber-200">
                    {t("agents.card.globalDisabled")}
                  </span>
                ) : null}
              </label>
            );
          })
        )}
      </div>
    </div>
  );
}
