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
  canSave: boolean;
  isSaving: boolean;
  saveLabel: string;
  onBrowseProjectPath: () => void;
  onProjectPathChange: (value: string) => void;
  onDisplayNameChange: (value: string) => void;
  onSave: () => void;
  onCancelEdit: () => void;
}

export function ProjectIdentityPanel(props: ProjectIdentityPanelProps) {
  const { t } = useTranslation();
  const readOnlyPath = props.mode === "edit";
  const showDisplayInlineHint = !props.displayName.trim();

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
          <div className="group flex h-12 items-center gap-2 rounded-[1.2rem] border border-slate-700/80 bg-slate-950/90 p-1 shadow-inner shadow-slate-950/40 transition-colors focus-within:border-sky-400/80">
            <input
              className="h-full min-w-0 flex-1 rounded-[0.95rem] border-0 bg-transparent px-3 text-sm text-slate-100 outline-none placeholder:text-slate-600"
              onChange={(event: ChangeEvent<HTMLInputElement>) =>
                props.onProjectPathChange(event.target.value)
              }
              readOnly={readOnlyPath}
              value={props.projectPath}
            />
            {props.canBrowseProjectPath ? (
              <button
                className="inline-flex h-10 shrink-0 items-center gap-2 whitespace-nowrap rounded-[0.95rem] border border-slate-600/80 bg-slate-800/80 px-3 text-sm font-medium text-slate-100 transition hover:border-sky-400/50 hover:bg-slate-800"
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

        <div className="mt-1 flex flex-col gap-3 min-[720px]:flex-row min-[720px]:items-end">
          <label className="block min-w-0 flex-1 space-y-3 min-[720px]:max-w-xl">
            <span className="block text-xs font-medium uppercase tracking-[0.22em] text-slate-400">
              {t("projects.identity.nameLabel")}
            </span>
            <div className="group relative h-12 rounded-[1.1rem] border border-slate-700/80 bg-slate-950/90 p-1 shadow-inner shadow-slate-950/35 transition-colors focus-within:border-sky-400/75">
              <input
                className="relative z-10 h-full w-full rounded-[0.9rem] border-0 bg-transparent px-3 text-sm text-slate-100 outline-none placeholder:text-slate-600"
                onChange={(event: ChangeEvent<HTMLInputElement>) =>
                  props.onDisplayNameChange(event.target.value)
                }
                value={props.displayName}
              />
              {showDisplayInlineHint ? (
                <div className="pointer-events-none absolute inset-y-1 left-4 right-3 flex items-center justify-start gap-2 overflow-hidden text-xs text-slate-500">
                  <span className="min-w-0 truncate">
                    {t("projects.identity.suggested")}: {props.inferredName || "—"}
                  </span>
                  {readOnlyPath ? (
                    <span className="hidden shrink-0 rounded-full border border-slate-700/80 px-2 py-0.5 text-[11px] text-slate-400 min-[920px]:inline">
                      {t("projects.identity.readOnlyPath")}
                    </span>
                  ) : null}
                </div>
              ) : null}
            </div>
          </label>
          <div className="flex shrink-0 flex-col gap-2 min-[520px]:flex-row">
            {props.mode === "edit" ? (
              <button
                className="inline-flex min-h-[3rem] items-center justify-center rounded-[1.05rem] border border-slate-700 bg-slate-950/70 px-4 py-3 text-sm font-semibold text-slate-200 transition hover:border-sky-500/60 hover:text-sky-100"
                onClick={props.onCancelEdit}
                title={t("tooltip.projects.cancelEdit")}
                type="button"
              >
                {t("projects.identity.cancelEdit")}
              </button>
            ) : null}
            <button
              className="inline-flex min-h-[3rem] items-center justify-center rounded-[1.05rem] bg-sky-500 px-5 py-3 text-sm font-semibold text-slate-950 shadow-[0_14px_34px_rgba(14,165,233,0.22)] transition hover:bg-sky-400 disabled:opacity-60"
              disabled={!props.canSave || props.isSaving}
              onClick={props.onSave}
              type="button"
            >
              {props.isSaving ? t("projects.summary.saving") : props.saveLabel}
            </button>
          </div>
        </div>
      </div>
      <div className="mt-4 flex flex-wrap gap-2 text-xs text-slate-400">
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
