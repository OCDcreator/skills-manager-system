import { useTranslation } from "react-i18next";
import type { AgentConfigDraft, AgentSelectionPreview } from "../../lib/agent-selection";
import type { SceneEntry } from "../../lib/scenes";
import type { AgentInventoryItem, SkillSummary } from "../../lib/tauri";
import { AgentBrandIcon } from "./AgentBrandIcon";
import { AgentSceneSelector } from "./AgentSceneSelector";
import { AgentSelectionSummary } from "./AgentSelectionSummary";
import { AgentSkillSelector } from "./AgentSkillSelector";

interface AgentTargetCardProps {
  agent: AgentInventoryItem;
  disabledSkillIds: string[];
  draft: AgentConfigDraft;
  isDirty: boolean;
  isUpdating: boolean;
  preview: AgentSelectionPreview;
  scenes: SceneEntry[];
  skills: SkillSummary[];
  onDraftChange: (draft: AgentConfigDraft) => void;
  onSave: () => Promise<void>;
}

export function AgentTargetCard({
  agent,
  disabledSkillIds,
  draft,
  isDirty,
  isUpdating,
  preview,
  scenes,
  skills,
  onDraftChange,
  onSave,
}: AgentTargetCardProps) {
  const { t } = useTranslation();
  const trimmedOverride = draft.pathOverride.trim();
  const saveDisabled = isUpdating || !isDirty;
  const resetDisabled = isUpdating || !trimmedOverride;

  return (
    <article
      className={`space-y-4 rounded-2xl border p-5 ${
        isDirty ? "border-sky-700 bg-sky-950/20" : "border-slate-800 bg-slate-900"
      }`}
    >
      <header className="flex items-start justify-between gap-3">
        <div className="flex items-start gap-3">
          <div className="grid h-11 w-11 shrink-0 place-items-center rounded-2xl border border-slate-700 bg-slate-950">
            <AgentBrandIcon agentKey={agent.key} className="size-[70%]" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-slate-100">{agent.displayName}</h3>
            <div className="mt-2 flex flex-wrap gap-2">
              <span
                className={`inline-flex rounded-full px-2.5 py-1 text-xs ${
                  agent.pathMode === "missing"
                    ? "bg-amber-500/15 text-amber-200"
                    : agent.pathMode === "override"
                      ? "bg-violet-500/15 text-violet-200"
                      : "bg-emerald-500/15 text-emerald-200"
                }`}
              >
                {t(`agents.pathMode.${agent.pathMode}`)}
              </span>
              {isDirty ? (
                <span className="inline-flex rounded-full bg-sky-500/15 px-2.5 py-1 text-xs text-sky-200">
                  {t("agents.card.unsaved")}
                </span>
              ) : null}
            </div>
          </div>
        </div>

        <button
          className={`rounded-lg px-3 py-2 text-xs font-semibold disabled:opacity-60 ${
            draft.enabled
              ? "bg-sky-400 text-slate-950"
              : "border border-slate-700 bg-slate-950 text-slate-200"
          }`}
          disabled={isUpdating}
          onClick={() => onDraftChange({ ...draft, enabled: !draft.enabled })}
          title={
            draft.enabled
              ? t("tooltip.agents.disableTarget")
              : t("tooltip.agents.enableTarget")
          }
          type="button"
        >
          {draft.enabled ? t("agents.card.disableTarget") : t("agents.card.enableTarget")}
        </button>
      </header>

      <dl className="grid gap-3 text-sm md:grid-cols-3">
        <div>
          <dt className="text-xs uppercase tracking-wide text-slate-500">
            {t("agents.card.defaultPath")}
          </dt>
          <dd className="mt-1 break-all text-slate-300">{agent.defaultSkillsDir}</dd>
        </div>
        <div>
          <dt className="text-xs uppercase tracking-wide text-slate-500">
            {t("agents.card.detectedPath")}
          </dt>
          <dd className="mt-1 break-all text-slate-300">
            {agent.detectedSkillsDir || t("agents.card.notDetected")}
          </dd>
        </div>
        <div>
          <dt className="text-xs uppercase tracking-wide text-slate-500">
            {t("agents.card.effectivePath")}
          </dt>
          <dd className="mt-1 break-all text-slate-300">
            {agent.effectiveSkillsDir || t("agents.card.needsOverride")}
          </dd>
        </div>
      </dl>

      <div className="space-y-2">
        <label className="text-xs font-semibold uppercase tracking-wide text-slate-500">
          {t("agents.card.overridePath")}
        </label>
        <input
          className="w-full rounded-xl border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100 outline-none focus:border-sky-400"
          onChange={(event) => onDraftChange({ ...draft, pathOverride: event.target.value })}
          placeholder={agent.defaultSkillsDir}
          value={draft.pathOverride}
        />
        <button
          className="rounded-lg border border-slate-700 px-3 py-2 text-xs text-slate-200 disabled:opacity-60"
          disabled={resetDisabled}
          onClick={() => onDraftChange({ ...draft, pathOverride: "" })}
          type="button"
        >
          {t("agents.card.resetOverride")}
        </button>
      </div>

      <div className="grid gap-4 xl:grid-cols-3">
        <AgentSkillSelector
          disabledSkillIds={disabledSkillIds}
          draft={draft}
          onDraftChange={onDraftChange}
          skills={skills}
        />
        <AgentSceneSelector draft={draft} onDraftChange={onDraftChange} scenes={scenes} />
        <AgentSelectionSummary
          draft={draft}
          onDraftChange={onDraftChange}
          preview={preview}
        />
      </div>

      <div className="flex justify-end">
        <button
          className="rounded-lg bg-slate-100 px-4 py-2 text-sm font-semibold text-slate-950 disabled:opacity-60"
          disabled={saveDisabled}
          onClick={() => void onSave()}
          type="button"
        >
          {isUpdating ? t("agents.card.saving") : t("agents.card.saveAgent")}
        </button>
      </div>
    </article>
  );
}
