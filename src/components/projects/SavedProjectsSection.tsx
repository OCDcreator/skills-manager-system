import { useTranslation } from "react-i18next";
import { ProjectCard } from "./ProjectCard";
import type { ProjectAssignment } from "../../lib/projects";

interface SavedProjectsSectionProps {
  projects: ProjectAssignment[];
  applyingKey: string | null;
  sessionApplyFeedback: Record<string, string>;
  onApplyAll: () => void;
  onDelete: (projectPath: string) => void;
  onEdit: (project: ProjectAssignment) => void;
}

export function SavedProjectsSection(props: SavedProjectsSectionProps) {
  const { t } = useTranslation();

  if (props.projects.length === 0) {
    return <div className="text-sm text-slate-500">{t("projects.empty")}</div>;
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-base font-semibold text-slate-100">
            {t("projects.saved.title")}
          </h3>
          <p className="mt-1 text-sm text-slate-500">
            {t("projects.saved.description")}
          </p>
        </div>
        <button
          className="rounded-xl bg-slate-100 px-4 py-2 text-sm font-medium text-slate-950 disabled:opacity-60"
          disabled={props.projects.length === 0 || props.applyingKey === "__all__"}
          onClick={props.onApplyAll}
          type="button"
        >
          {t("projects.saved.applyAll")}
        </button>
      </div>
      <div className="space-y-4">
        {props.projects.map((project) => (
          <ProjectCard
            key={project.projectPath}
            onDelete={() => props.onDelete(project.projectPath)}
            onEdit={() => props.onEdit(project)}
            project={project}
            sessionApplyMessage={
              props.sessionApplyFeedback[project.projectPath] ?? null
            }
            t={t}
          />
        ))}
      </div>
    </div>
  );
}
