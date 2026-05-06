import { useTranslation } from "react-i18next";
import type { ReactNode } from "react";
import { useRememberedScrollPosition } from "../../lib/scroll-memory";
import type {
  ProjectAgentDraft,
  ProjectAgentStatusFilter,
  ProjectSkillSelectionFilter,
} from "../../lib/project-draft";
import type { SceneEntry } from "../../lib/scenes";
import type { SkillPathSummary, SkillPathFilter } from "../../lib/skills/filters";
import type { AgentInventoryItem, SkillSummary } from "../../lib/tauri";

interface ProjectAssignmentEditorProps {
  skills: SkillSummary[];
  exclusionSkills: SkillSummary[];
  agents: AgentInventoryItem[];
  scenes: SceneEntry[];
  skillPathSummaries: SkillPathSummary[];
  activeAgentDraft: ProjectAgentDraft | null;
  selectedAgentKey: string | null;
  selectedAgentKeys: string[];
  skillQuery: string;
  agentQuery: string;
  skillPathFilter: SkillPathFilter;
  skillSelectionFilter: ProjectSkillSelectionFilter;
  agentStatusFilter: ProjectAgentStatusFilter;
  onSkillQueryChange: (value: string) => void;
  onAgentQueryChange: (value: string) => void;
  onSkillPathFilterChange: (value: SkillPathFilter) => void;
  onSkillSelectionFilterChange: (value: ProjectSkillSelectionFilter) => void;
  onAgentStatusFilterChange: (value: ProjectAgentStatusFilter) => void;
  onSelectAgent(agentKey: string): void;
  onToggleAgent: (agentKey: string) => void;
  onToggleProjectSkill: (skillId: string) => void;
  onToggleProjectScene: (sceneId: string) => void;
  onToggleProjectExclusion: (skillId: string) => void;
}

export function ProjectAssignmentEditor(props: ProjectAssignmentEditorProps) {
  const { t } = useTranslation();
  const skillScrollRef = useRememberedScrollPosition("projects:editor:skills");
  const agentScrollRef = useRememberedScrollPosition("projects:editor:agents");
  const sceneScrollRef = useRememberedScrollPosition("projects:editor:scenes");
  const exclusionScrollRef = useRememberedScrollPosition("projects:editor:exclusions");
  const panelClassName =
    "flex max-h-[clamp(22rem,calc(100vh-20rem),34rem)] min-h-0 flex-col overflow-hidden rounded-2xl border border-slate-800 bg-slate-950/60";
  const activeAgentDraft = props.activeAgentDraft;
  const filterPillClass = (active: boolean) =>
    `rounded-full border px-2 py-1 transition ${
      active
        ? "border-sky-500/60 bg-sky-500/15 text-sky-100"
        : "border-slate-700 bg-slate-900 text-slate-300 hover:bg-slate-800"
    }`;

  return (
    <section className="grid min-h-0 gap-4 rounded-2xl border border-slate-800 bg-slate-900/60 p-4">
      <div className="grid min-h-0 gap-4 min-[1380px]:grid-cols-[minmax(14rem,4fr)_minmax(0,8fr)]">
        <div className={panelClassName}>
          <div className="border-b border-slate-800 px-4 py-3">
            <div className="text-sm font-semibold text-slate-100">
              {t("projects.editor.agentsTitle")}
            </div>
            <input
              className="mt-3 w-full rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-200 outline-none focus:border-sky-400"
              onChange={(event) => props.onAgentQueryChange(event.target.value)}
              value={props.agentQuery}
            />
            <div className="mt-3 flex flex-wrap gap-2 text-[11px] text-slate-400">
              {(["all", "enabled", "disabled"] as const).map((status) => (
                <button
                  className={filterPillClass(props.agentStatusFilter === status)}
                  key={status}
                  onClick={() => props.onAgentStatusFilterChange(status)}
                  type="button"
                >
                  {status === "all"
                    ? t("projects.editor.allAgents")
                    : status === "enabled"
                      ? t("projects.editor.enabledAgents")
                      : t("projects.editor.disabledAgents")}
                </button>
              ))}
            </div>
          </div>
          <div
            className="skill-markdown-scroll min-h-0 flex-1 space-y-1 overflow-y-auto px-4 py-3"
            ref={agentScrollRef}
          >
            {props.agents.map((agent) => {
              const selected = props.selectedAgentKeys.includes(agent.key);
              const active = props.selectedAgentKey === agent.key;
              return (
                <div
                  className={`rounded-xl border px-2 py-2 text-sm ${
                    active
                      ? "border-sky-500/50 bg-sky-500/10"
                      : "border-transparent hover:border-slate-800 hover:bg-slate-900/70"
                  }`}
                  key={agent.key}
                >
                  <div className="flex items-center gap-3">
                    <input
                      checked={selected}
                      onChange={() => props.onToggleAgent(agent.key)}
                      type="checkbox"
                    />
                    <button
                      className="min-w-0 flex-1 text-left"
                      onClick={() => props.onSelectAgent(agent.key)}
                      type="button"
                    >
                      <span className="block truncate font-medium text-slate-100">
                        {agent.displayName}
                      </span>
                      <span className="block truncate text-xs text-slate-500">
                        {agent.projectSkillsDirRule}
                      </span>
                    </button>
                  </div>
                </div>
              );
            })}
          </div>
        </div>

        <div className="min-h-0 space-y-3">
          <div className="flex min-h-8 items-center justify-between gap-3">
            <div className="text-sm font-semibold text-slate-100">
              {t("projects.editor.agentLayerTitle")}
            </div>
            <div className="truncate text-xs text-slate-500">
              {props.selectedAgentKey ?? t("projects.editor.noAgentSelected")}
            </div>
          </div>
          <div className="grid min-h-0 gap-4 min-[1040px]:grid-cols-2">
            <ProjectChecklistPanel title={t("projects.editor.skillsTitle")}>
              <input
                className="mb-3 w-full rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-200 outline-none focus:border-sky-400"
                onChange={(event) => props.onSkillQueryChange(event.target.value)}
                value={props.skillQuery}
              />
              <div className="mb-3 flex flex-wrap items-center gap-2 text-[11px] text-slate-400">
                {props.skillPathSummaries.map((summary) => (
                  <button
                    className={filterPillClass(props.skillPathFilter === summary.key)}
                    key={summary.key}
                    onClick={() => props.onSkillPathFilterChange(summary.key)}
                    type="button"
                  >
                    {summary.key === "all"
                      ? t("projects.editor.allPaths", { count: summary.count })
                      : `${summary.key} · ${summary.count}`}
                  </button>
                ))}
                {(["all", "selected", "unselected"] as const).map((status) => (
                  <button
                    className={filterPillClass(props.skillSelectionFilter === status)}
                    key={status}
                    onClick={() => props.onSkillSelectionFilterChange(status)}
                    type="button"
                  >
                    {status === "all"
                      ? t("projects.editor.allSkills")
                      : status === "selected"
                        ? t("projects.editor.selectedSkills")
                        : t("projects.editor.unselectedSkills")}
                  </button>
                ))}
              </div>
              <ProjectSkillList
                checkedIds={activeAgentDraft?.selectedSkillIds ?? []}
                disabled={!activeAgentDraft}
                onToggle={props.onToggleProjectSkill}
                scrollRef={skillScrollRef}
                skills={props.skills}
              />
            </ProjectChecklistPanel>

            <ProjectChecklistPanel title={t("projects.editor.projectScenesTitle")}>
              <div
                className="skill-markdown-scroll min-h-0 flex-1 space-y-1 overflow-y-auto px-4 py-3"
                ref={sceneScrollRef}
              >
                {props.scenes.map((scene) => (
                  <label
                    className="flex items-start gap-3 rounded-xl border border-transparent px-2 py-2 text-sm text-slate-300 hover:border-slate-800 hover:bg-slate-900/70"
                    key={scene.id}
                  >
                    <input
                      checked={activeAgentDraft?.selectedSceneIds.includes(scene.id) ?? false}
                      disabled={!activeAgentDraft}
                      onChange={() => props.onToggleProjectScene(scene.id)}
                      type="checkbox"
                    />
                    <span className="min-w-0 flex-1">
                      <span className="block truncate font-medium text-slate-100">
                        {scene.name}
                      </span>
                      <span className="block truncate text-xs text-slate-500">
                        {scene.description || scene.id}
                      </span>
                    </span>
                  </label>
                ))}
              </div>
            </ProjectChecklistPanel>

            <ProjectChecklistPanel title={t("projects.editor.exclusionsTitle")}>
              <ProjectSkillList
                checkedIds={activeAgentDraft?.excludedSkillIds ?? []}
                disabled={!activeAgentDraft}
                onToggle={props.onToggleProjectExclusion}
                scrollRef={exclusionScrollRef}
                skills={props.exclusionSkills}
              />
            </ProjectChecklistPanel>
          </div>
        </div>
      </div>
    </section>
  );
}

function ProjectChecklistPanel(props: {
  title: string;
  children: ReactNode;
}) {
  return (
    <div className="flex max-h-[clamp(18rem,calc(100vh-23rem),27rem)] min-h-0 flex-col overflow-hidden rounded-2xl border border-slate-800 bg-slate-950/60">
      <div className="border-b border-slate-800 px-4 py-3 text-sm font-semibold text-slate-100">
        {props.title}
      </div>
      {props.children}
    </div>
  );
}

function ProjectSkillList(props: {
  skills: SkillSummary[];
  checkedIds: string[];
  disabled: boolean;
  scrollRef: (node: HTMLElement | null) => void;
  onToggle: (skillId: string) => void;
}) {
  return (
    <div
      className="skill-markdown-scroll min-h-0 flex-1 space-y-1 overflow-y-auto px-4 py-3"
      ref={props.scrollRef}
    >
      {props.skills.map((skill) => (
        <label
          className="flex items-start gap-3 rounded-xl border border-transparent px-2 py-2 text-sm text-slate-300 hover:border-slate-800 hover:bg-slate-900/70"
          key={skill.id}
        >
          <input
            checked={props.checkedIds.includes(skill.id)}
            disabled={props.disabled}
            onChange={() => props.onToggle(skill.id)}
            type="checkbox"
          />
          <span className="min-w-0 flex-1">
            <span className="block font-medium text-slate-100">{skill.name}</span>
            <span className="block overflow-hidden text-xs text-slate-500 [display:-webkit-box] [-webkit-box-orient:vertical] [-webkit-line-clamp:2]">
              {skill.description}
            </span>
            <span className="mt-1 block truncate text-[11px] text-slate-600">
              {skill.relativePath}
            </span>
          </span>
        </label>
      ))}
    </div>
  );
}
