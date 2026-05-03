import { useTranslation } from "react-i18next";
import type { AgentInventoryItem, SkillSummary } from "../../lib/tauri";

interface ProjectAssignmentEditorProps {
  skills: SkillSummary[];
  agents: AgentInventoryItem[];
  selectedSkillIds: string[];
  selectedAgentKeys: string[];
  skillQuery: string;
  agentQuery: string;
  onSkillQueryChange: (value: string) => void;
  onAgentQueryChange: (value: string) => void;
  onToggleSkill: (skillId: string) => void;
  onToggleAgent: (agentKey: string) => void;
}

export function ProjectAssignmentEditor(props: ProjectAssignmentEditorProps) {
  const { t } = useTranslation();
  const panelClassName =
    "flex max-h-[clamp(22rem,calc(100vh-20rem),34rem)] min-h-0 flex-col overflow-hidden rounded-2xl border border-slate-800 bg-slate-950/60";

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
          </div>
          <div className="skill-markdown-scroll min-h-0 flex-1 space-y-1 overflow-y-auto px-4 py-3">
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
          </div>
          <div className="skill-markdown-scroll min-h-0 flex-1 space-y-1 overflow-y-auto px-4 py-3">
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
                    {agent.skillsDirRule}
                  </span>
                </span>
              </label>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
}
