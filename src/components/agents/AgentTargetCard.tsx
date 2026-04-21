import { useState } from "react";
import { useTranslation } from "react-i18next";
import type { AgentInventoryItem } from "../../lib/tauri";

interface AgentTargetCardProps {
  agent: AgentInventoryItem;
  isUpdating: boolean;
  onToggleEnabled: (key: string, enabled: boolean) => Promise<void>;
  onSavePathOverride: (key: string, path: string) => Promise<void>;
  onClearPathOverride: (key: string) => Promise<void>;
}

export function AgentTargetCard(props: AgentTargetCardProps) {
  const { agent, isUpdating, onClearPathOverride, onSavePathOverride, onToggleEnabled } =
    props;
  const { t } = useTranslation();
  const [overrideDraft, setOverrideDraft] = useState<string | null>(null);
  const overrideValue = overrideDraft ?? agent.pathOverride ?? "";
  const trimmedOverride = overrideValue.trim();
  const saveDisabled =
    isUpdating || !trimmedOverride || trimmedOverride === (agent.pathOverride ?? "");
  const resetDisabled = isUpdating || !agent.pathOverride;

  const handleSaveOverride = async () => {
    await onSavePathOverride(agent.key, trimmedOverride);
    setOverrideDraft(null);
  };

  const handleResetOverride = async () => {
    await onClearPathOverride(agent.key);
    setOverrideDraft(null);
  };

  return (
    <article className="space-y-4 rounded-2xl border border-slate-800 bg-slate-900 p-5">
      <header className="flex items-start justify-between gap-3">
        <div>
          <h3 className="text-lg font-semibold text-slate-100">{agent.displayName}</h3>
          <span
            className={`mt-2 inline-flex rounded-full px-2.5 py-1 text-xs ${
              agent.pathMode === "missing"
                ? "bg-amber-500/15 text-amber-200"
                : agent.pathMode === "override"
                  ? "bg-violet-500/15 text-violet-200"
                  : "bg-emerald-500/15 text-emerald-200"
            }`}
          >
            {t(`agents.pathMode.${agent.pathMode}`)}
          </span>
        </div>

        <button
          className={`rounded-lg px-3 py-2 text-xs font-semibold disabled:opacity-60 ${
            agent.enabled
              ? "bg-sky-400 text-slate-950"
              : "border border-slate-700 bg-slate-950 text-slate-200"
          }`}
          disabled={isUpdating}
          onClick={() => void onToggleEnabled(agent.key, !agent.enabled)}
          title={agent.enabled ? t("tooltip.agents.disableTarget") : t("tooltip.agents.enableTarget")}
          type="button"
        >
          {agent.enabled ? t("agents.card.disableTarget") : t("agents.card.enableTarget")}
        </button>
      </header>

      <dl className="space-y-3 text-sm">
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
          onChange={(event) => setOverrideDraft(event.target.value)}
          placeholder={agent.defaultSkillsDir}
          value={overrideValue}
        />
        <div className="flex flex-wrap gap-2">
          <button
            className="rounded-lg bg-slate-100 px-3 py-2 text-xs font-semibold text-slate-950 disabled:opacity-60"
            disabled={saveDisabled}
            onClick={() => void handleSaveOverride()}
            title={t("tooltip.agents.saveOverride")}
            type="button"
          >
            {t("agents.card.saveOverride")}
          </button>
          <button
            className="rounded-lg border border-slate-700 px-3 py-2 text-xs text-slate-200 disabled:opacity-60"
            disabled={resetDisabled}
            onClick={() => void handleResetOverride()}
            title={t("tooltip.agents.resetOverride")}
            type="button"
          >
            {t("agents.card.resetOverride")}
          </button>
        </div>
      </div>
    </article>
  );
}
