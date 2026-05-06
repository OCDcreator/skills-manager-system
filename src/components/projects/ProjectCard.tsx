import type { ProjectAssignment } from "../../lib/projects";

interface ProjectCardProps {
  project: ProjectAssignment;
  sessionApplyMessage: string | null;
  onDelete: () => void;
  onEdit: () => void;
  t: (key: string, options?: Record<string, unknown>) => string;
}

export function ProjectCard({
  project,
  sessionApplyMessage,
  onDelete,
  onEdit,
  t,
}: ProjectCardProps) {
  const agentEntries = Object.entries(project.agents ?? {});
  const legacyAgentKeys = project.agentKeys ?? [];
  const legacySkillIds = project.skillIds ?? [];
  const agentCount = agentEntries.length || legacyAgentKeys.length;
  const directSkillCount = agentEntries.length
    ? agentEntries.reduce(
        (count, [, assignment]) => count + assignment.selectedSkillIds.length,
        0,
      )
    : legacySkillIds.length;
  const sceneCount = agentEntries.reduce(
    (count, [, assignment]) => count + assignment.selectedSceneIds.length,
    0,
  );
  const statusEntries = [
    ...new Set([
      ...agentEntries.map(([agentKey]) => agentKey),
      ...legacyAgentKeys,
      ...(project.unsupportedAgentKeys ?? []),
    ]),
  ].sort();

  return (
    <div className="rounded-2xl border border-slate-800 bg-slate-900 p-5">
      <div className="flex items-start justify-between gap-4">
        <div>
          <h3 className="font-semibold text-slate-100">
            {project.displayName}
          </h3>
          <p className="mt-0.5 text-xs text-slate-500 font-mono truncate max-w-md">
            {project.projectPath}
          </p>
        </div>
        <div className="flex items-center gap-1">
          <button
            className="rounded p-1.5 text-slate-400 hover:bg-rose-900/40 hover:text-rose-300"
            title={t("tooltip.projects.delete")}
            onClick={onDelete}
          >
            <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg>
          </button>
        </div>
      </div>

      <div className="mt-3 flex items-center gap-4 text-xs text-slate-400">
        <span>
          {t("projects.card.agents", { count: agentCount })}
        </span>
        <span>
          {t("projects.card.directSkills", { count: directSkillCount })}
        </span>
        <span>
          {t("projects.card.scenes", { count: sceneCount })}
        </span>
      </div>

      {statusEntries.length ? (
        <div className="mt-3 flex flex-wrap gap-2 text-xs">
          {statusEntries.map((agentKey) => {
            const status =
              project.applyStatuses?.[agentKey]?.applyStatus ??
              (project.unsupportedAgentKeys?.includes(agentKey)
                ? "unsupported"
                : "neverApplied");
            return (
              <span
                className={`rounded-full border px-2.5 py-1 ${statusClassName(status)}`}
                key={agentKey}
              >
                {agentKey}: {t(`projects.card.applyStatus.${status}`)}
              </span>
            );
          })}
        </div>
      ) : null}

      <div className="mt-4 flex items-center gap-2">
        <button
          className="rounded-lg bg-slate-800 px-4 py-2 text-sm text-slate-200 hover:bg-slate-700"
          onClick={onEdit}
          type="button"
        >
          {t("projects.saved.edit")}
        </button>
        <button
          className="rounded-lg border border-rose-800 px-4 py-2 text-sm text-rose-200 hover:bg-rose-950/50"
          onClick={onDelete}
          type="button"
        >
          {t("projects.saved.delete")}
        </button>
      </div>
      {sessionApplyMessage ? (
        <div className="mt-3 rounded-xl border border-sky-900 bg-sky-950/40 px-3 py-2 text-xs text-sky-200">
          {sessionApplyMessage}
        </div>
      ) : null}
    </div>
  );
}

function statusClassName(status: string) {
  if (status === "current") {
    return "border-emerald-800 bg-emerald-950/50 text-emerald-200";
  }
  if (status === "stale") {
    return "border-amber-800 bg-amber-950/50 text-amber-200";
  }
  if (status === "unsupported") {
    return "border-rose-800 bg-rose-950/50 text-rose-200";
  }
  return "border-slate-700 bg-slate-950/70 text-slate-300";
}
