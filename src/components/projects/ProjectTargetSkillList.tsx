import { ChevronDown, Trash2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { AgentTargetSkillEntry } from "../../lib/tauri";

interface ProjectTargetSkillListProps {
  agentKey: string;
  entries: AgentTargetSkillEntry[];
  isExpanded: boolean;
  onDeleteTargetSkill: (agentKey: string, entry: AgentTargetSkillEntry) => void;
  onToggle: () => void;
  scanError: string | null;
  targetActionId: string | null;
  targetDir: string | null;
}

export function ProjectTargetSkillList(props: ProjectTargetSkillListProps) {
  const { t } = useTranslation();

  if (!props.targetDir) {
    return null;
  }

  return (
    <div className="mt-3 rounded-lg border border-slate-800 bg-slate-900/50">
      <button
        aria-expanded={props.isExpanded}
        className="flex w-full items-center justify-between gap-3 px-2.5 py-2 text-left text-xs"
        onClick={props.onToggle}
        type="button"
      >
        <span className="min-w-0">
          <span className="block font-medium text-slate-300">
            {t("projects.summary.existingTargetSkills")}
          </span>
          <span className="mt-0.5 block truncate text-[11px] text-slate-500">
            {props.entries.length
              ? t("projects.summary.existingTargetSkillsCount", {
                  count: props.entries.length,
                })
              : t("projects.summary.noExistingTargetSkills")}
          </span>
        </span>
        <span className="flex shrink-0 items-center gap-2">
          <span className="rounded-full bg-slate-800 px-2 py-0.5 text-[10px] text-slate-400">
            {props.entries.length}
          </span>
          <ChevronDown
            className={`size-3.5 text-slate-500 transition ${
              props.isExpanded ? "rotate-180" : ""
            }`}
          />
        </span>
      </button>

      {props.isExpanded ? (
        <div className="border-t border-slate-800 px-2.5 py-2">
          {props.scanError ? (
            <p className="text-xs text-amber-300">{props.scanError}</p>
          ) : props.entries.length ? (
            <div className="skill-markdown-scroll max-h-64 space-y-1 overflow-y-auto pr-1">
              {props.entries.map((entry) => (
                <ProjectTargetSkillRow
                  actionId={buildTargetActionId(props.agentKey, entry.entryName)}
                  agentKey={props.agentKey}
                  entry={entry}
                  key={entry.entryName}
                  onDeleteTargetSkill={props.onDeleteTargetSkill}
                  targetActionId={props.targetActionId}
                />
              ))}
            </div>
          ) : (
            <p className="text-xs text-slate-500">
              {t("projects.summary.noExistingTargetSkills")}
            </p>
          )}
        </div>
      ) : null}
    </div>
  );
}

function ProjectTargetSkillRow({
  actionId,
  agentKey,
  entry,
  onDeleteTargetSkill,
  targetActionId,
}: {
  actionId: string;
  agentKey: string;
  entry: AgentTargetSkillEntry;
  onDeleteTargetSkill: (agentKey: string, entry: AgentTargetSkillEntry) => void;
  targetActionId: string | null;
}) {
  const { t } = useTranslation();
  const statusLabel = entry.managed
    ? t("projects.summary.managed")
    : t("projects.summary.unmanaged");
  const detailLabel = entry.relativePath ?? entry.entryName;
  const actionLabel = entry.managed
    ? t("projects.summary.cancelSelection")
    : t("projects.summary.deleteTargetSkill");
  const disabled = targetActionId === actionId || (entry.managed && !entry.skillId);

  return (
    <div className="rounded-md bg-slate-950/60 px-2 py-1.5 text-xs">
      <div className="flex items-center justify-between gap-2">
        <span className="min-w-0 truncate font-medium text-slate-200">
          {entry.displayName}
        </span>
        <span
          className={`shrink-0 rounded-full px-2 py-0.5 text-[10px] ${
            entry.managed
              ? "bg-emerald-500/10 text-emerald-200"
              : "bg-amber-500/10 text-amber-200"
          }`}
        >
          {statusLabel}
        </span>
      </div>
      <div className="mt-1 flex items-end justify-between gap-2">
        <div className="flex min-w-0 flex-wrap items-center gap-1.5 text-[11px] text-slate-500">
          <span className="min-w-0 truncate">{detailLabel}</span>
          {!entry.hasSkillDocument ? (
            <span className="rounded-full bg-slate-800 px-1.5 py-0.5 text-[10px] text-slate-400">
              {t("projects.summary.noSkillDocument")}
            </span>
          ) : null}
        </div>
        <button
          className="inline-flex shrink-0 items-center gap-1 rounded-md border border-rose-900/70 bg-rose-950/30 px-2 py-1 text-[11px] text-rose-200 transition hover:bg-rose-950/60 disabled:cursor-not-allowed disabled:opacity-50"
          disabled={disabled}
          onClick={() => onDeleteTargetSkill(agentKey, entry)}
          title={actionLabel}
          type="button"
        >
          <Trash2 className="size-3" />
          <span>{actionLabel}</span>
        </button>
      </div>
    </div>
  );
}

function buildTargetActionId(agentKey: string, entryName: string) {
  return `${agentKey}:${entryName}:delete`;
}
