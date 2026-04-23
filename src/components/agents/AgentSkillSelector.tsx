import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { toggleId, type AgentConfigDraft } from "../../lib/agent-selection";
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
  const disabled = useMemo(() => new Set(disabledSkillIds), [disabledSkillIds]);
  const selected = useMemo(() => new Set(draft.selectedSkillIds), [draft.selectedSkillIds]);
  const filteredSkills = useMemo(() => {
    const lowered = search.trim().toLowerCase();
    if (!lowered) return skills;
    return skills.filter((skill) =>
      [skill.name, skill.description, skill.relativePath]
        .join(" ")
        .toLowerCase()
        .includes(lowered),
    );
  }, [search, skills]);

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
      <input
        className="w-full rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-xs text-slate-100 outline-none focus:border-sky-400"
        onChange={(event) => setSearch(event.target.value)}
        placeholder={t("agents.card.searchSkills")}
        value={search}
      />
      <div className="max-h-56 space-y-1 overflow-y-auto pr-1">
        {filteredSkills.map((skill) => {
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
        })}
      </div>
    </div>
  );
}
