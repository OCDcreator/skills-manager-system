import { useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import {
  removeId,
  toggleId,
  type AgentConfigDraft,
  type AgentSelectionPreview,
} from "../../lib/agent-selection";
import { useRememberedScrollPosition } from "../../lib/scroll-memory";

interface AgentSelectionSummaryProps {
  draft: AgentConfigDraft;
  preview: AgentSelectionPreview;
  onDraftChange: (draft: AgentConfigDraft) => void;
}

type PreviewFilter = "all" | "will-sync" | "synced" | "excluded" | "global-ignored";

export function AgentSelectionSummary({
  draft,
  preview,
  onDraftChange,
}: AgentSelectionSummaryProps) {
  const { t } = useTranslation();
  const [activeFilter, setActiveFilter] = useState<PreviewFilter>("all");
  const [selectedPreviewSkillIds, setSelectedPreviewSkillIds] = useState<string[]>([]);
  const selectedPreviewSkillSet = useMemo(
    () => new Set(selectedPreviewSkillIds),
    [selectedPreviewSkillIds],
  );
  const filteredItems = useMemo(
    () =>
      preview.items.filter((item) => {
        if (activeFilter === "will-sync") return item.needsSync;
        if (activeFilter === "synced") return item.isSynced;
        if (activeFilter === "excluded") return item.isExcluded;
        if (activeFilter === "global-ignored") return item.isGloballyDisabled;
        return true;
      }),
    [activeFilter, preview.items],
  );
  const selectedItems = preview.items.filter((item) => selectedPreviewSkillSet.has(item.skill.id));
  const selectedDirectItems = selectedItems.filter((item) => item.isDirect);
  const hasSelectedItems = selectedItems.length > 0;
  const hasSelectedDirectItems = selectedDirectItems.length > 0;
  const scrollRef = useRememberedScrollPosition(`agents:selection-summary:${draft.key}`);
  const filterButtonClass = (filter: PreviewFilter, tone: string) =>
    `rounded-full border px-2 py-1 transition ${
      activeFilter === filter
        ? tone
        : "border-slate-700 bg-slate-900 text-slate-300 hover:bg-slate-800"
    }`;

  const togglePreviewSkillSelection = (skillId: string) => {
    setSelectedPreviewSkillIds((current) =>
      current.includes(skillId)
        ? current.filter((candidate) => candidate !== skillId)
        : [...current, skillId],
    );
  };

  const toggleFilter = (filter: PreviewFilter) => {
    setActiveFilter((current) => (current === filter ? "all" : filter));
  };

  useEffect(() => {
    const availableIds = new Set(preview.items.map((item) => item.skill.id));
    setSelectedPreviewSkillIds((current) => {
      const next = current.filter((skillId) => availableIds.has(skillId));
      return next.length === current.length ? current : next;
    });
  }, [preview.items]);

  const deselectDirectSkill = (item: AgentSelectionPreview["items"][number]) => {
    const hasSceneSource = item.sceneNames.length > 0;
    const nextExcludedSkillIds = hasSceneSource
      ? draft.excludedSkillIds.includes(item.skill.id)
        ? draft.excludedSkillIds
        : [...draft.excludedSkillIds, item.skill.id]
      : removeId(draft.excludedSkillIds, item.skill.id);

    onDraftChange({
      ...draft,
      excludedSkillIds: nextExcludedSkillIds,
      selectedSkillIds: removeId(draft.selectedSkillIds, item.skill.id),
    });
  };

  const batchExcludeSelected = () => {
    if (!selectedItems.length) return;

    const excluded = new Set(draft.excludedSkillIds);
    for (const item of selectedItems) {
      if (!item.isGloballyDisabled) {
        excluded.add(item.skill.id);
      }
    }

    onDraftChange({
      ...draft,
      excludedSkillIds: [...excluded],
    });
    setSelectedPreviewSkillIds([]);
  };

  const batchDeselectDirectSelected = () => {
    if (!selectedDirectItems.length) return;

    const directIds = new Set(selectedDirectItems.map((item) => item.skill.id));
    const excluded = new Set(draft.excludedSkillIds);
    for (const item of selectedDirectItems) {
      if (item.sceneNames.length > 0) {
        excluded.add(item.skill.id);
      } else {
        excluded.delete(item.skill.id);
      }
    }

    onDraftChange({
      ...draft,
      excludedSkillIds: [...excluded],
      selectedSkillIds: draft.selectedSkillIds.filter((id) => !directIds.has(id)),
    });
    setSelectedPreviewSkillIds([]);
  };

  return (
    <div className="flex h-full min-h-0 flex-col rounded-xl border border-slate-800 bg-slate-950/50 p-3">
      <div className="flex flex-wrap items-center gap-2 text-[11px] text-slate-400">
        <button
          className={filterButtonClass(
            "will-sync",
            "border-emerald-500/50 bg-emerald-500/15 text-emerald-100",
          )}
          onClick={() => toggleFilter("will-sync")}
          type="button"
        >
          {t("agents.card.syncCount", { count: preview.syncCount })}
        </button>
        <button
          className={filterButtonClass(
            "synced",
            "border-cyan-500/50 bg-cyan-500/15 text-cyan-100",
          )}
          onClick={() => toggleFilter("synced")}
          type="button"
        >
          {t("agents.card.syncedCount", { count: preview.syncedCount })}
        </button>
        <button
          className={filterButtonClass(
            "excluded",
            "border-amber-500/50 bg-amber-500/15 text-amber-100",
          )}
          onClick={() => toggleFilter("excluded")}
          type="button"
        >
          {t("agents.card.excludedCount", { count: preview.excludedCount })}
        </button>
        <button
          className={filterButtonClass(
            "global-ignored",
            "border-slate-600 bg-slate-800 text-slate-100",
          )}
          onClick={() => toggleFilter("global-ignored")}
          type="button"
        >
          {t("agents.card.globalIgnoredCount", {
            count: preview.globallyDisabledCount,
          })}
        </button>
        <span className="rounded-full bg-slate-800 px-2 py-1 text-slate-300">
          {t("agents.card.batchSelectedCount", { count: selectedPreviewSkillIds.length })}
        </span>
        <button
          className="rounded border border-slate-700 px-2 py-1 text-[11px] text-slate-200 hover:bg-slate-800 disabled:opacity-50"
          disabled={!hasSelectedItems}
          onClick={batchExcludeSelected}
          type="button"
        >
          {t("agents.card.batchExclude")}
        </button>
        <button
          className="rounded border border-slate-700 px-2 py-1 text-[11px] text-slate-200 hover:bg-slate-800 disabled:opacity-50"
          disabled={!hasSelectedDirectItems}
          onClick={batchDeselectDirectSelected}
          type="button"
        >
          {t("agents.card.batchDeselectDirect")}
        </button>
      </div>

      <div
        className="skill-markdown-scroll mt-2 max-h-64 min-h-0 flex-1 space-y-1 overflow-y-auto pr-1 xl:max-h-none"
        ref={scrollRef}
      >
        {preview.items.length === 0 ? (
          <div className="text-xs text-slate-500">{t("agents.card.noEffectiveSkills")}</div>
        ) : filteredItems.length === 0 ? (
          <div className="text-xs text-slate-500">{t("agents.card.noMatchingPreviewSkills")}</div>
        ) : (
          filteredItems.map((item) => {
            const sourceText = [item.isDirect ? t("agents.card.sourceDirect") : null, ...item.sceneNames]
              .filter(Boolean)
              .join(" + ");
            return (
              <div
                className="flex items-center gap-2 rounded-md px-2 py-1.5 text-xs text-slate-300"
                key={item.skill.id}
              >
                <input
                  aria-label={item.skill.name}
                  checked={selectedPreviewSkillSet.has(item.skill.id)}
                  className="h-3.5 w-3.5 rounded border-slate-700 bg-slate-950"
                  onChange={() => togglePreviewSkillSelection(item.skill.id)}
                  type="checkbox"
                />
                <span className="min-w-0 flex-1">
                  <span className="block truncate">{item.skill.name}</span>
                  <span className="block truncate text-[11px] text-slate-500">
                    {sourceText}
                  </span>
                </span>
                <span
                  className={`rounded px-1.5 py-0.5 text-[10px] ${
                    item.needsSync
                      ? "bg-emerald-500/10 text-emerald-200"
                      : item.isSynced
                        ? "bg-cyan-500/10 text-cyan-200"
                        : "bg-slate-800 text-slate-400"
                  }`}
                >
                  {item.needsSync
                    ? t("agents.card.willSync")
                    : item.isSynced
                      ? t("agents.card.synced")
                      : item.isGloballyDisabled
                        ? t("agents.card.globalDisabled")
                        : t("agents.card.excluded")}
                </span>
                <span className="flex shrink-0 flex-wrap justify-end gap-1">
                  {item.isDirect ? (
                    <button
                      className="rounded border border-slate-700 px-2 py-1 text-[11px] text-slate-200 hover:bg-slate-800"
                      onClick={() => deselectDirectSkill(item)}
                      type="button"
                    >
                      {t("agents.card.deselectDirect")}
                    </button>
                  ) : null}
                  {!item.isGloballyDisabled ? (
                    <button
                      className="rounded border border-slate-700 px-2 py-1 text-[11px] text-slate-200 hover:bg-slate-800"
                      onClick={() =>
                        onDraftChange({
                          ...draft,
                          excludedSkillIds: toggleId(draft.excludedSkillIds, item.skill.id),
                        })
                      }
                      type="button"
                    >
                      {item.isExcluded ? t("agents.card.restore") : t("agents.card.exclude")}
                    </button>
                  ) : null}
                </span>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
}
