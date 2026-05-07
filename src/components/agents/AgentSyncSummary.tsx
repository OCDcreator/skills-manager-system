import { useTranslation } from "react-i18next";
import type { AgentSyncMode } from "../../lib/tauri";

interface AgentSyncSummaryProps {
  repoPath: string | null;
  enabledSkillCount: number;
  enabledAgentCount: number;
  dirtyAgentCount: number;
  isApplying: boolean;
  isSavingMode: boolean;
  isSavingDrafts: boolean;
  canApply: boolean;
  syncMode: AgentSyncMode;
  onApply: () => Promise<void>;
  onDiscardChanges: () => void;
  onSaveChanges: () => Promise<void>;
  onSyncModeChange: (syncMode: AgentSyncMode) => Promise<void>;
}

export function AgentSyncSummary(props: AgentSyncSummaryProps) {
  const {
    canApply,
    dirtyAgentCount,
    enabledAgentCount,
    enabledSkillCount,
    isApplying,
    isSavingDrafts,
    isSavingMode,
    onApply,
    onDiscardChanges,
    onSaveChanges,
    onSyncModeChange,
    repoPath,
    syncMode,
  } = props;
  const { t } = useTranslation();
  const hasDirtyDrafts = dirtyAgentCount > 0;
  const disabledReason = !repoPath
    ? t("agents.summary.noRepo")
    : enabledAgentCount === 0
      ? t("agents.summary.noTargets")
      : null;

  return (
    <section className="rounded-2xl border border-slate-800 bg-slate-900 p-5">
      <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
        <div className="space-y-3">
          <div>
            <p className="text-xs font-semibold uppercase tracking-wide text-sky-300">
              {t("agents.phaseLabel")}
            </p>
            <h2 className="mt-1 text-2xl font-semibold text-slate-100">
              {t("agents.title")}
            </h2>
          </div>
          <p className="max-w-3xl text-sm leading-6 text-slate-400">
            {t("agents.description")}
          </p>
          <div className="grid gap-3 text-sm text-slate-300 md:grid-cols-3">
            <div className="rounded-xl bg-slate-950 p-3">
              <p className="text-xs text-slate-500">{t("agents.summary.repoPath")}</p>
              <p className="mt-1 break-all">{repoPath || t("agents.summary.repoMissing")}</p>
            </div>
            <div className="rounded-xl bg-slate-950 p-3">
              <p className="text-xs text-slate-500">{t("agents.summary.enabledSkills")}</p>
              <p className="mt-1 text-lg font-semibold text-slate-100">{enabledSkillCount}</p>
            </div>
            <div className="rounded-xl bg-slate-950 p-3">
              <p className="text-xs text-slate-500">{t("agents.summary.enabledTargets")}</p>
              <p className="mt-1 text-lg font-semibold text-slate-100">{enabledAgentCount}</p>
            </div>
          </div>
        </div>

        <div className="min-w-56 space-y-3 rounded-xl border border-slate-800 bg-slate-950 p-3">
          <div>
            <label className="text-xs font-semibold uppercase tracking-wide text-slate-500">
              {t("agents.syncMode.label")}
            </label>
            <select
              className="mt-1 w-full rounded-xl border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-100 outline-none focus:border-sky-400"
              disabled={isApplying || isSavingMode}
              onChange={(event) =>
                void onSyncModeChange(event.target.value as AgentSyncMode)
              }
              title={t("tooltip.agents.syncModeSelect")}
              value={syncMode}
            >
              <option value="copy">{t("agents.syncMode.copy")}</option>
              <option value="symlink">{t("agents.syncMode.symlink")}</option>
            </select>
          </div>
          <button
            className="w-full rounded-xl bg-emerald-400 px-4 py-3 text-sm font-semibold text-slate-950 disabled:cursor-not-allowed disabled:opacity-60"
            disabled={!canApply || isApplying || isSavingMode}
            onClick={() => void onApply()}
            title={t("tooltip.agents.applySync")}
            type="button"
          >
            {isApplying ? t("agents.apply.running") : t("agents.apply.button")}
          </button>
          <div
            className={`min-h-[5.75rem] rounded-xl border px-3 py-3 ${
              hasDirtyDrafts
                ? "border-sky-900/60 bg-sky-950/40"
                : "border-slate-800 bg-slate-950/70"
            }`}
          >
            {hasDirtyDrafts ? (
              <>
                <p className="text-xs leading-5 text-sky-200">
                  {t("agents.unsavedBanner", { count: dirtyAgentCount })}
                </p>
                <div className="mt-3 flex flex-wrap gap-2">
                  <button
                    className="rounded-lg border border-slate-700 px-3 py-2 text-xs text-slate-200 disabled:opacity-60"
                    disabled={isApplying || isSavingMode}
                    onClick={onDiscardChanges}
                    type="button"
                  >
                    {t("agents.discardAll")}
                  </button>
                  <button
                    className="rounded-lg bg-sky-400 px-3 py-2 text-xs font-semibold text-slate-950 disabled:opacity-60"
                    disabled={isApplying || isSavingMode}
                    onClick={() => void onSaveChanges()}
                    type="button"
                  >
                    {isSavingDrafts ? t("agents.card.saving") : t("agents.saveAll")}
                  </button>
                </div>
              </>
            ) : (
              <p className="text-xs leading-5 text-slate-500">
                {disabledReason ||
                  t("agents.apply.modeHint", {
                    mode: t(`agents.syncMode.${syncMode}`),
                  })}
              </p>
            )}
          </div>
        </div>
      </div>
    </section>
  );
}
