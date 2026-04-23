import { useTranslation } from "react-i18next";
import { toggleId, type AgentConfigDraft, type AgentSelectionPreview } from "../../lib/agent-selection";

interface AgentSelectionSummaryProps {
  draft: AgentConfigDraft;
  preview: AgentSelectionPreview;
  onDraftChange: (draft: AgentConfigDraft) => void;
}

export function AgentSelectionSummary({
  draft,
  preview,
  onDraftChange,
}: AgentSelectionSummaryProps) {
  const { t } = useTranslation();

  return (
    <div className="space-y-2 rounded-xl border border-slate-800 bg-slate-950/50 p-3">
      <div className="flex flex-wrap gap-2 text-[11px] text-slate-400">
        <span className="rounded-full bg-emerald-500/10 px-2 py-1 text-emerald-200">
          {t("agents.card.syncCount", { count: preview.syncCount })}
        </span>
        <span className="rounded-full bg-amber-500/10 px-2 py-1 text-amber-200">
          {t("agents.card.excludedCount", { count: preview.excludedCount })}
        </span>
        <span className="rounded-full bg-slate-800 px-2 py-1 text-slate-300">
          {t("agents.card.globalIgnoredCount", {
            count: preview.globallyDisabledCount,
          })}
        </span>
      </div>

      <div className="skill-markdown-scroll max-h-48 space-y-1 overflow-y-auto pr-1">
        {preview.items.length === 0 ? (
          <div className="text-xs text-slate-500">{t("agents.card.noEffectiveSkills")}</div>
        ) : (
          preview.items.map((item) => {
            const sourceText = [
              item.isDirect ? t("agents.card.sourceDirect") : null,
              ...item.sceneNames,
            ]
              .filter(Boolean)
              .join(" + ");
            return (
              <div
                className="flex items-center gap-2 rounded-md px-2 py-1.5 text-xs text-slate-300"
                key={item.skill.id}
              >
                <span className="min-w-0 flex-1">
                  <span className="block truncate">{item.skill.name}</span>
                  <span className="block truncate text-[11px] text-slate-500">
                    {sourceText}
                  </span>
                </span>
                <span
                  className={`rounded px-1.5 py-0.5 text-[10px] ${
                    item.willSync
                      ? "bg-emerald-500/10 text-emerald-200"
                      : "bg-slate-800 text-slate-400"
                  }`}
                >
                  {item.willSync
                    ? t("agents.card.willSync")
                    : item.isGloballyDisabled
                      ? t("agents.card.globalDisabled")
                      : t("agents.card.excluded")}
                </span>
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
              </div>
            );
          })
        )}
      </div>
    </div>
  );
}
