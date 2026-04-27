import type { ChangeEvent } from "react";
import { useTranslation } from "react-i18next";

interface ProjectIdentityPanelProps {
  mode: "create" | "edit";
  projectPath: string;
  displayName: string;
  inferredName: string;
  duplicatePath: boolean;
  inspectionError: string | null;
  isInspecting: boolean;
  onProjectPathChange: (value: string) => void;
  onDisplayNameChange: (value: string) => void;
}

export function ProjectIdentityPanel(props: ProjectIdentityPanelProps) {
  const { t } = useTranslation();
  const readOnlyPath = props.mode === "edit";

  return (
    <section className="rounded-2xl border border-slate-800 bg-slate-900/80 p-4">
      <div className="flex flex-col gap-4 xl:flex-row">
        <label className="flex-1">
          <span className="mb-2 block text-xs font-medium uppercase tracking-wide text-slate-400">
            {t("projects.identity.pathLabel")}
          </span>
          <input
            className="w-full rounded-xl border border-slate-700 bg-slate-950 px-4 py-3 text-sm text-slate-100 outline-none focus:border-sky-400"
            onChange={(event: ChangeEvent<HTMLInputElement>) =>
              props.onProjectPathChange(event.target.value)
            }
            readOnly={readOnlyPath}
            value={props.projectPath}
          />
        </label>
        <label className="flex-1">
          <span className="mb-2 block text-xs font-medium uppercase tracking-wide text-slate-400">
            {t("projects.identity.nameLabel")}
          </span>
          <input
            className="w-full rounded-xl border border-slate-700 bg-slate-950 px-4 py-3 text-sm text-slate-100 outline-none focus:border-sky-400"
            onChange={(event: ChangeEvent<HTMLInputElement>) =>
              props.onDisplayNameChange(event.target.value)
            }
            value={props.displayName}
          />
        </label>
      </div>
      <div className="mt-3 flex flex-wrap gap-2 text-xs text-slate-400">
        <span className="rounded-full border border-slate-700 px-3 py-1">
          {t("projects.identity.suggested")}: {props.inferredName || "—"}
        </span>
        {readOnlyPath ? (
          <span className="rounded-full border border-slate-700 px-3 py-1">
            {t("projects.identity.readOnlyPath")}
          </span>
        ) : null}
        {props.isInspecting ? (
          <span className="rounded-full border border-sky-800 bg-sky-950/60 px-3 py-1 text-sky-300">
            {t("projects.summary.inspecting")}
          </span>
        ) : null}
        {props.duplicatePath ? (
          <span className="rounded-full border border-amber-800 bg-amber-950/60 px-3 py-1 text-amber-300">
            {t("projects.identity.duplicate")}
          </span>
        ) : null}
        {props.inspectionError ? (
          <span className="rounded-full border border-rose-800 bg-rose-950/60 px-3 py-1 text-rose-300">
            {props.inspectionError}
          </span>
        ) : null}
      </div>
    </section>
  );
}
