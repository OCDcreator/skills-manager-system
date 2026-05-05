import { useTranslation } from "react-i18next";
import { useRememberedScrollPosition } from "../../lib/scroll-memory";
import type {
  ProjectAgentStatusFilter,
} from "../../lib/project-draft";
import type { SkillPathSummary, SkillPathFilter } from "../../lib/skills/filters";
import type { AgentInventoryItem, SkillSummary } from "../../lib/tauri";

interface ProjectAssignmentEditorProps {
  skills: SkillSummary[];
  agents: AgentInventoryItem[];
  skillPathSummaries: SkillPathSummary[];
  selectedSkillIds: string[];
  selectedAgentKeys: string[];
  skillQuery: string;
  agentQuery: string;
  skillPathFilter: SkillPathFilter;
  agentStatusFilter: ProjectAgentStatusFilter;
  onSkillQueryChange: (value: string) => void;
  onAgentQueryChange: (value: string) => void;
  onSkillPathFilterChange: (value: SkillPathFilter) => void;
  onAgentStatusFilterChange: (value: ProjectAgentStatusFilter) => void;
  onToggleSkill: (skillId: string) => void;
  onToggleAgent: (agentKey: string) => void;
}

export function ProjectAssignmentEditor(props: ProjectAssignmentEditorProps) {
  const { t } = useTranslation();
  const skillScrollRef = useRememberedScrollPosition("projects:editor:skills");
  const agentScrollRef = useRememberedScrollPosition("projects:editor:agents");
  const panelClassName =
    "flex max-h-[clamp(22rem,calc(100vh-20rem),34rem)] min-h-0 flex-col overflow-hidden rounded-2xl border border-slate-800 bg-slate-950/60";
  const filterPillClass = (active: boolean) =>
    `rounded-full border px-2 py-1 transition ${
      active
        ? "border-sky-500/60 bg-sky-500/15 text-sky-100"
        : "border-slate-700 bg-slate-900 text-slate-300 hover:bg-slate-800"
    }`;

  return (
    <section className="grid min-h-0 gap-4 rounded-2xl border border-slate-800 bg-slate-900/60 p-4">
      <div className="grid min-h-0 gap-4 min-[1380px]:grid-cols-[minmax(0,7fr)_minmax(18rem,5fr)]">
        <div className={panelClassName}>
          <div className="border-b border-slate-800 px-4 py-3">
            <div className="text-sm font-semibold text-slate-100">
              {t("projects.editor.skillsTitle")}
            </div>
            <input
              className="mt-3 w-full rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-200 outline-none focus:border-sky-400"
              onChange={(event) => props.onSkillQueryChange(event.target.value)}
              value={props.skillQuery}
            />
            <div className="mt-3 flex flex-wrap gap-2 text-[11px] text-slate-400">
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
            </div>
          </div>
          <div
            className="skill-markdown-scroll min-h-0 flex-1 space-y-1 overflow-y-auto px-4 py-3"
            ref={skillScrollRef}
          >
            {props.skills.map((skill) => (
              <label
                className="flex items-start gap-3 rounded-xl border border-transparent px-2 py-2 text-sm text-slate-300 hover:border-slate-800 hover:bg-slate-900/70"
                key={skill.id}
              >
                <input
                  checked={props.selectedSkillIds.includes(skill.id)}
                  onChange={() => props.onToggleSkill(skill.id)}
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
        </div>
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
            {props.agents.map((agent) => (
              <label
                className="flex items-center gap-3 rounded-xl border border-transparent px-2 py-2 text-sm text-slate-300 hover:border-slate-800 hover:bg-slate-900/70"
                key={agent.key}
              >
                <input
                  checked={props.selectedAgentKeys.includes(agent.key)}
                  onChange={() => props.onToggleAgent(agent.key)}
                  type="checkbox"
                />
                <span className="grid min-w-0 flex-1 grid-cols-[minmax(0,1fr)] gap-0.5">
                  <span className="truncate font-medium text-slate-100">{agent.displayName}</span>
                  <span className="truncate text-xs text-slate-500">
                    {agent.projectSkillsDirRule}
                  </span>
                </span>
                <span
                  className={`shrink-0 rounded-full px-2 py-1 text-[10px] ${
                    agent.enabled
                      ? "bg-emerald-500/10 text-emerald-200"
                      : "bg-slate-800 text-slate-400"
                  }`}
                >
                  {agent.enabled
                    ? t("projects.editor.enabledAgents")
                    : t("projects.editor.disabledAgents")}
                </span>
              </label>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
}
