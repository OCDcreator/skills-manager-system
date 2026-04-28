import { FolderSearch } from "lucide-react";
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
  canBrowseProjectPath: boolean;
  onBrowseProjectPath: () => void;
  onProjectPathChange: (value: string) => void;
  onDisplayNameChange: (value: string) => void;
}

export function ProjectIdentityPanel(props: ProjectIdentityPanelProps) {
  const { t } = useTranslation();
  const readOnlyPath = props.mode === "edit";

  return (
    <section className="rounded-[1.75rem] border border-slate-800/90 bg-gradient-to-br from-slate-900/95 via-slate-900/85 to-slate-950/95 p-5 shadow-[0_18px_60px_rgba(2,6,23,0.34)]">
      <div className="space-y-5">
        <label className="block space-y-3">
          <div className="flex items-center justify-between gap-3">
            <span className="block text-xs font-medium uppercase tracking-[0.22em] text-slate-400">
              {t("projects.identity.pathLabel")}
            </span>
            {props.canBrowseProjectPath ? (
              <span className="text-[11px] uppercase tracking-[0.2em] text-slate-500">
                {t("projects.identity.browse")}
              </span>
            ) : null}
          </div>
          <div className="group flex items-center gap-2 rounded-[1.35rem] border border-slate-700/80 bg-slate-950/90 p-2 shadow-inner shadow-slate-950/40 transition-colors focus-within:border-sky-400/80">
            <input
              className="min-w-0 flex-1 rounded-[1rem] border-0 bg-transparent px-4 py-3 text-sm text-slate-100 outline-none placeholder:text-slate-600"
              onChange={(event: ChangeEvent<HTMLInputElement>) =>
                props.onProjectPathChange(event.target.value)
              }
              readOnly={readOnlyPath}
              value={props.projectPath}
            />
            {props.canBrowseProjectPath ? (
              <button
                className="inline-flex shrink-0 items-center gap-2 whitespace-nowrap rounded-[1rem] border border-slate-600/80 bg-slate-800/80 px-4 py-3 text-sm font-medium text-slate-100 transition hover:border-sky-400/50 hover:bg-slate-800"
                onClick={props.onBrowseProjectPath}
                title={t("tooltip.projects.browsePath")}
                type="button"
              >
                <FolderSearch className="size-4" strokeWidth={1.9} />
                <span>{t("projects.identity.browse")}</span>
              </button>
            ) : null}
          </div>
        </label>

        <label className="mt-1 block max-w-xl space-y-3">
          <span className="block text-xs font-medium uppercase tracking-[0.22em] text-slate-400">
            {t("projects.identity.nameLabel")}
          </span>
          <div className="group rounded-[1.2rem] border border-slate-700/80 bg-slate-950/90 p-2 shadow-inner shadow-slate-950/35 transition-colors focus-within:border-sky-400/75">
            <input
              className="w-full rounded-[0.95rem] border-0 bg-transparent px-4 py-3 text-sm text-slate-100 outline-none placeholder:text-slate-600"
              onChange={(event: ChangeEvent<HTMLInputElement>) =>
                props.onDisplayNameChange(event.target.value)
              }
              value={props.displayName}
            />
          </div>
        </label>
      </div>
      <div className="mt-4 flex flex-wrap gap-2 text-xs text-slate-400">
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
