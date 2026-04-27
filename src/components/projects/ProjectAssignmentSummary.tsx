import { useTranslation } from "react-i18next";

interface ProjectAssignmentSummaryProps {
  title: string;
  selectedSkillCount: number;
  selectedAgentCount: number;
  duplicatePath: boolean;
  unsupportedAgentKeys: string[];
  disabledSelectedSkillIds: string[];
  inspectionTargets: Array<{
    agentKey: string;
    targetDir: string;
    markerExists: boolean;
    targetExists: boolean;
  }>;
  isInspecting: boolean;
  canSave: boolean;
  isSaving: boolean;
  saveLabel: string;
  onSave: () => void;
}

export function ProjectAssignmentSummary(props: ProjectAssignmentSummaryProps) {
  const { t } = useTranslation();

  return (
    <aside className="flex min-h-0 flex-col overflow-hidden rounded-2xl border border-slate-800 bg-slate-900/80">
      <div className="border-b border-slate-800 px-4 py-4">
        <h3 className="text-base font-semibold text-slate-100">{props.title}</h3>
        <p className="mt-2 text-sm text-slate-400">
          {props.selectedSkillCount} skills · {props.selectedAgentCount} agents
        </p>
      </div>
      <div className="skill-markdown-scroll min-h-0 flex-1 space-y-4 overflow-y-auto px-4 py-4 text-sm">
        {props.unsupportedAgentKeys.length ? (
          <section>
            <div className="text-xs font-medium uppercase tracking-wide text-amber-400">
              {t("projects.summary.unsupportedAgents")}
            </div>
            <div className="mt-2 flex flex-wrap gap-2">
              {props.unsupportedAgentKeys.map((key) => (
                <span
                  className="rounded-full border border-amber-800 bg-amber-950/60 px-3 py-1 text-xs text-amber-200"
                  key={key}
                >
                  {key}
                </span>
              ))}
            </div>
          </section>
        ) : null}
        <section>
          <div className="text-xs font-medium uppercase tracking-wide text-slate-500">
            {t("projects.summary.targetsTitle")}
          </div>
          <div className="mt-2 space-y-2">
            {props.inspectionTargets.map((target) => (
              <div
                className="rounded-xl border border-slate-800 bg-slate-950/60 px-3 py-3"
                key={target.agentKey}
              >
                <div className="font-medium text-slate-100">{target.agentKey}</div>
                <div className="mt-1 break-all text-xs text-slate-400">{target.targetDir}</div>
                <div className="mt-2 text-xs text-slate-500">
                  {t("projects.summary.markerStatus")}:{" "}
                  {target.markerExists
                    ? t("projects.summary.present")
                    : t("projects.summary.missing")}{" "}
                  · {t("projects.summary.targetStatus")}:{" "}
                  {target.targetExists
                    ? t("projects.summary.present")
                    : t("projects.summary.missing")}
                </div>
              </div>
            ))}
          </div>
        </section>
        {props.disabledSelectedSkillIds.length ? (
          <section className="rounded-xl border border-amber-800 bg-amber-950/50 px-3 py-3 text-xs text-amber-200">
            {t("projects.summary.disabledSkills")}
          </section>
        ) : null}
        {props.duplicatePath ? (
          <section className="rounded-xl border border-rose-800 bg-rose-950/50 px-3 py-3 text-xs text-rose-200">
            {t("projects.identity.duplicate")}
          </section>
        ) : null}
        {props.isInspecting ? (
          <div className="text-xs text-sky-300">{t("projects.summary.inspecting")}</div>
        ) : null}
      </div>
      <div className="border-t border-slate-800 px-4 py-4">
        <button
          className="w-full rounded-xl bg-sky-500 px-4 py-3 text-sm font-semibold text-slate-950 disabled:opacity-60"
          disabled={!props.canSave || props.isSaving}
          onClick={props.onSave}
          type="button"
        >
          {props.isSaving ? t("projects.summary.saving") : props.saveLabel}
        </button>
      </div>
    </aside>
  );
}
