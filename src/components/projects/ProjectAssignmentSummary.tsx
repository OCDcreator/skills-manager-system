import { useTranslation } from "react-i18next";
import { useRememberedScrollPosition } from "../../lib/scroll-memory";

interface ProjectAssignmentSummaryProps {
  title: string;
  selectedSkillCount: number;
  selectedAgentCount: number;
  selectedSkills: Array<{
    id: string;
    name: string;
    description: string;
  }>;
  selectedAgents: Array<{
    key: string;
    displayName: string;
    skillsDirRule: string;
    projectSkillsDirRule: string;
  }>;
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
}

export function ProjectAssignmentSummary(props: ProjectAssignmentSummaryProps) {
  const { t } = useTranslation();
  const scrollRef = useRememberedScrollPosition(`projects:summary:${props.title}`);
  const chipClassName =
    "rounded-full border border-slate-700/80 bg-slate-950/70 px-3 py-1 text-xs text-slate-200";

  return (
    <aside className="flex h-full min-h-0 max-h-[clamp(22rem,calc(100vh-13rem),34rem)] flex-col overflow-hidden rounded-2xl border border-slate-800 bg-slate-900/80 min-[1380px]:absolute min-[1380px]:inset-0 min-[1380px]:max-h-none">
      <div className="border-b border-slate-800 px-4 py-4">
        <h3 className="text-base font-semibold text-slate-100">{props.title}</h3>
        <p className="mt-2 text-sm text-slate-400">
          {props.selectedSkillCount} skills · {props.selectedAgentCount} agents
        </p>
      </div>
      <div
        className="skill-markdown-scroll flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto px-4 py-4 text-sm"
        ref={scrollRef}
      >
        <section>
          <div className="text-xs font-medium uppercase tracking-wide text-slate-500">
            {t("projects.summary.selectedSkillsTitle")}
          </div>
          {props.selectedSkills.length ? (
            <div className="mt-2 rounded-xl border border-slate-800 bg-slate-950/50 p-2">
              <div className="skill-markdown-scroll max-h-28 overflow-y-auto pr-1">
                <div className="flex flex-wrap gap-2">
                  {props.selectedSkills.map((skill) => (
                    <span className={chipClassName} key={skill.id} title={skill.description}>
                      {skill.name}
                    </span>
                  ))}
                </div>
              </div>
            </div>
          ) : (
            <p className="mt-2 text-xs text-slate-500">
              {t("projects.summary.noSelectedSkills")}
            </p>
          )}
        </section>

        <section>
          <div className="text-xs font-medium uppercase tracking-wide text-slate-500">
            {t("projects.summary.selectedAgentsTitle")}
          </div>
          {props.selectedAgents.length ? (
            <div className="mt-2 space-y-2">
              {props.selectedAgents.map((agent) => (
                <div
                  className="rounded-xl border border-slate-800 bg-slate-950/60 px-3 py-2"
                  key={agent.key}
                >
                  <div className="font-medium text-slate-100">{agent.displayName}</div>
                  <div className="mt-0.5 truncate text-xs text-slate-500">
                    {agent.projectSkillsDirRule}
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <p className="mt-2 text-xs text-slate-500">
              {t("projects.summary.noSelectedAgents")}
            </p>
          )}
        </section>

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
        <section className="flex min-h-0 flex-1 flex-col">
          <div className="text-xs font-medium uppercase tracking-wide text-slate-500">
            {t("projects.summary.targetsTitle")}
          </div>
          <div className="mt-2 flex min-h-0 flex-1 flex-col rounded-xl border border-slate-800 bg-slate-950/50 p-2">
            <div className="skill-markdown-scroll min-h-0 flex-1 space-y-2 overflow-y-auto pr-1">
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
    </aside>
  );
}
