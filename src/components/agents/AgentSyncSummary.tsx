import { useTranslation } from "react-i18next";

interface AgentSyncSummaryProps {
  repoPath: string | null;
  enabledSkillCount: number;
  enabledAgentCount: number;
  isApplying: boolean;
  canApply: boolean;
  onApply: () => Promise<void>;
}

export function AgentSyncSummary(props: AgentSyncSummaryProps) {
  const {
    canApply,
    enabledAgentCount,
    enabledSkillCount,
    isApplying,
    onApply,
    repoPath,
  } = props;
  const { t } = useTranslation();
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

        <div className="min-w-56 space-y-2 rounded-xl border border-slate-800 bg-slate-950 p-3">
          <button
            className="w-full rounded-xl bg-emerald-400 px-4 py-3 text-sm font-semibold text-slate-950 disabled:cursor-not-allowed disabled:opacity-60"
            disabled={!canApply || isApplying}
            onClick={() => void onApply()}
            type="button"
          >
            {isApplying ? t("agents.apply.running") : t("agents.apply.button")}
          </button>
          <p className="text-xs leading-5 text-slate-500">
            {disabledReason || t("agents.apply.copyOnlyNote")}
          </p>
        </div>
      </div>
    </section>
  );
}
