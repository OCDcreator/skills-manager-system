import { useState } from "react";
import { useTranslation } from "react-i18next";
import { ProjectTargetSkillList } from "./ProjectTargetSkillList";
import { useRememberedScrollPosition } from "../../lib/scroll-memory";
import type { ProjectAgentSummary } from "../../lib/project-summary";
import type { AgentTargetSkillEntry } from "../../lib/tauri";

interface ProjectAssignmentSummaryProps {
  title: string;
  selectedAgentCount: number;
  projectDirectSkillCount: number;
  projectSceneCount: number;
  agentSummaries: ProjectAgentSummary[];
  duplicatePath: boolean;
  unsupportedAgentKeys: string[];
  isInspecting: boolean;
  targetActionId: string | null;
  onDeleteTargetSkill: (agentKey: string, entry: AgentTargetSkillEntry) => void;
}

export function ProjectAssignmentSummary(props: ProjectAssignmentSummaryProps) {
  const { t } = useTranslation();
  const scrollRef = useRememberedScrollPosition(`projects:summary:${props.title}`);
  const [expandedTargetSkillGroups, setExpandedTargetSkillGroups] = useState<Record<string, boolean>>({});

  return (
    <aside className="flex min-h-0 max-h-[clamp(22rem,calc(100vh-13rem),34rem)] flex-col overflow-hidden rounded-2xl border border-slate-800 bg-slate-900/80 min-[1380px]:max-h-[calc(100vh-8rem)]">
      <div className="border-b border-slate-800 px-4 py-4">
        <h3 className="text-base font-semibold text-slate-100">{props.title}</h3>
        <p className="mt-2 text-sm text-slate-400">
          {t("projects.summary.layerCounts", {
            agents: props.selectedAgentCount,
            skills: props.projectDirectSkillCount,
            scenes: props.projectSceneCount,
          })}
        </p>
      </div>
      <div
        className="skill-markdown-scroll flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto px-4 py-4 text-sm"
        ref={scrollRef}
      >
        <section>
          <div className="text-xs font-medium uppercase tracking-wide text-slate-500">
            {t("projects.summary.selectedAgentsTitle")}
          </div>
          {props.agentSummaries.length ? (
            <div className="mt-2 space-y-2">
              {props.agentSummaries.map((summary) => (
                <AgentPreviewGroup
                  expandedTargetSkillGroups={expandedTargetSkillGroups}
                  key={summary.agentKey}
                  onDeleteTargetSkill={props.onDeleteTargetSkill}
                  onToggleTargetSkillGroup={() =>
                    setExpandedTargetSkillGroups((current) => ({
                      ...current,
                      [summary.agentKey]: !current[summary.agentKey],
                    }))
                  }
                  summary={summary}
                  targetActionId={props.targetActionId}
                />
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

function AgentPreviewGroup({
  expandedTargetSkillGroups,
  onDeleteTargetSkill,
  onToggleTargetSkillGroup,
  summary,
  targetActionId,
}: {
  expandedTargetSkillGroups: Record<string, boolean>;
  onDeleteTargetSkill: (agentKey: string, entry: AgentTargetSkillEntry) => void;
  onToggleTargetSkillGroup: () => void;
  summary: ProjectAgentSummary;
  targetActionId: string | null;
}) {
  const { t } = useTranslation();
  const chipClassName =
    "rounded-full border border-slate-700/80 bg-slate-950/70 px-2.5 py-1 text-[11px] text-slate-200";
  const isTargetSkillExpanded = Boolean(expandedTargetSkillGroups[summary.agentKey]);

  return (
    <div className="rounded-xl border border-slate-800 bg-slate-950/60 px-3 py-3">
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0">
          <div className="truncate font-medium text-slate-100">{summary.displayName}</div>
          <div className="mt-0.5 truncate text-xs text-slate-500">
            {summary.projectSkillsDirRule}
          </div>
        </div>
        <span className="rounded-full bg-slate-800 px-2 py-1 text-[10px] text-slate-300">
          {t("projects.summary.agentPreviewTitle")}
        </span>
      </div>

      <div className="mt-3 grid gap-2 text-xs text-slate-400">
        <Metric label={t("projects.summary.inheritedGlobal")} value={summary.inheritedGlobalSkillIds.length} />
        <Metric label={t("projects.summary.projectDirect")} value={summary.projectDirectSkillIds.length} />
        <Metric label={t("projects.summary.projectScenes")} value={summary.projectSceneIds.length} />
      </div>

      {summary.previewItems.length ? (
        <div className="mt-3 flex flex-wrap gap-2">
          {summary.previewItems.map((item) => (
            <span
              className={`${chipClassName} ${
                item.isGloballyDisabled
                    ? "border-amber-800 bg-amber-950/50 text-amber-200"
                    : ""
              }`}
              key={item.skill.id}
              title={item.skill.description}
            >
              {item.skill.name}
            </span>
          ))}
        </div>
      ) : (
        <p className="mt-3 text-xs text-slate-500">
          {t("projects.summary.noPreviewSkills")}
        </p>
      )}

      <div className="mt-3 break-all text-xs text-slate-500">
        {t("projects.summary.targetsTitle")}:{" "}
        {summary.targetDir ?? t("projects.summary.targetPending")}
      </div>
      {summary.markerExists !== null ? (
        <div className="mt-1 text-xs text-slate-500">
          {t("projects.summary.markerStatus")}:{" "}
          {summary.markerExists
            ? t("projects.summary.present")
            : t("projects.summary.missing")}{" "}
          · {t("projects.summary.targetStatus")}:{" "}
          {summary.targetExists
            ? t("projects.summary.present")
            : t("projects.summary.missing")}
        </div>
      ) : null}
      <ProjectTargetSkillList
        agentKey={summary.agentKey}
        entries={summary.targetSkillEntries}
        isExpanded={isTargetSkillExpanded}
        onDeleteTargetSkill={onDeleteTargetSkill}
        onToggle={onToggleTargetSkillGroup}
        scanError={summary.targetSkillScanError}
        targetActionId={targetActionId}
        targetDir={summary.targetDir}
      />
    </div>
  );
}

function Metric({ label, value }: { label: string; value: number }) {
  return (
    <div className="flex items-center justify-between gap-3 rounded-lg bg-slate-900/70 px-2 py-1.5">
      <span className="truncate">{label}</span>
      <span className="font-medium text-slate-100">{value}</span>
    </div>
  );
}
