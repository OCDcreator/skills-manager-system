import type { ProjectAssignment } from "../../lib/projects";

interface ProjectCardProps {
  project: ProjectAssignment;
  isApplying: boolean;
  onDelete: () => void;
  onApply: () => void;
  t: (key: string, options?: Record<string, unknown>) => string;
}

export function ProjectCard({
  project,
  isApplying,
  onDelete,
  onApply,
  t,
}: ProjectCardProps) {
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
            title={t("projects.card.delete")}
            onClick={onDelete}
          >
            <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg>
          </button>
        </div>
      </div>

      <div className="mt-3 flex items-center gap-4 text-xs text-slate-400">
        <span>
          {t("projects.card.skills", { count: project.skillIds.length })}
        </span>
        <span>
          {t("projects.card.agents", { count: project.agentKeys.length })}
        </span>
      </div>

      <div className="mt-3">
        <button
          className="flex items-center gap-2 rounded-lg bg-slate-800 px-4 py-2 text-sm text-slate-200 hover:bg-slate-700 disabled:opacity-50"
          disabled={isApplying}
          onClick={onApply}
        >
          {isApplying ? t("projects.card.applying") : t("projects.card.apply")}
        </button>
      </div>
    </div>
  );
}
