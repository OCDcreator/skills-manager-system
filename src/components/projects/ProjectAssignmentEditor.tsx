import { Search } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { ReactNode } from "react";
import { ProjectSkillFilterToolbar } from "./ProjectSkillFilterToolbar";
import { useRememberedScrollPosition } from "../../lib/scroll-memory";
import type {
  ProjectAgentDraft,
  ProjectAgentStatusFilter,
  ProjectSkillSelectionFilter,
} from "../../lib/project-draft";
import type {
  ExternalGroupFilter,
  ExternalGroupSummary,
} from "../../lib/scene-skill-filters";
import type { SceneEntry } from "../../lib/scenes";
import type { SkillPathSummary, SkillPathFilter } from "../../lib/skills/filters";
import type { AgentInventoryItem, SkillSummary } from "../../lib/tauri";

interface ProjectAssignmentEditorProps {
  skills: SkillSummary[];
  agents: AgentInventoryItem[];
  scenes: SceneEntry[];
  skillPathSummaries: SkillPathSummary[];
  externalGroupSummaries: ExternalGroupSummary[];
  activeAgentDraft: ProjectAgentDraft | null;
  selectedAgentKey: string | null;
  selectedAgentKeys: string[];
  skillQuery: string;
  agentQuery: string;
  skillPathFilter: SkillPathFilter;
  externalGroupFilter: ExternalGroupFilter;
  skillSelectionFilter: ProjectSkillSelectionFilter;
  agentStatusFilter: ProjectAgentStatusFilter;
  onSkillQueryChange: (value: string) => void;
  onAgentQueryChange: (value: string) => void;
  onSkillPathFilterChange: (value: SkillPathFilter) => void;
  onExternalGroupFilterChange: (value: ExternalGroupFilter) => void;
  onSkillSelectionFilterChange: (value: ProjectSkillSelectionFilter) => void;
  onAgentStatusFilterChange: (value: ProjectAgentStatusFilter) => void;
  onSelectAgent(agentKey: string): void;
  onToggleAgent: (agentKey: string) => void;
  onToggleProjectSkill: (skillId: string) => void;
  onToggleProjectScene: (sceneId: string) => void;
}

export function ProjectAssignmentEditor(props: ProjectAssignmentEditorProps) {
  const { t } = useTranslation();
  const skillScrollRef = useRememberedScrollPosition("projects:editor:skills");
  const agentScrollRef = useRememberedScrollPosition("projects:editor:agents");
  const sceneScrollRef = useRememberedScrollPosition("projects:editor:scenes");
  const activeAgentDraft = props.activeAgentDraft;
  const filterPillClass = (active: boolean) =>
    `rounded-full border px-2 py-1 transition ${
      active
        ? "border-sky-500/60 bg-sky-500/15 text-sky-100"
        : "border-slate-700 bg-slate-900 text-slate-300 hover:bg-slate-800"
    }`;

  return (
    <section className="grid min-h-0 gap-4 rounded-2xl border border-slate-800 bg-slate-900/60 p-4">
      <div className="overflow-hidden rounded-xl border border-slate-800 bg-slate-950/60">
        <div className="grid gap-3 border-b border-slate-800 px-4 py-3 min-[980px]:grid-cols-[minmax(12rem,0.55fr)_minmax(0,1fr)] min-[980px]:items-center">
          <div className="min-w-0">
            <div className="text-sm font-semibold text-slate-100">
              {t("projects.editor.agentsTitle")}
            </div>
            <div className="mt-1 truncate text-xs text-slate-500">
              {props.selectedAgentKey ?? t("projects.editor.noAgentSelected")}
            </div>
          </div>
          <div className="flex min-w-0 flex-wrap items-center gap-2">
            <SearchField
              onChange={props.onAgentQueryChange}
              placeholder={t("projects.editor.searchAgents")}
              value={props.agentQuery}
            />
            <div className="flex flex-wrap gap-2 text-[11px] text-slate-400">
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
        </div>
        <div
          className="skill-markdown-scroll flex gap-2 overflow-x-auto px-3 py-3"
          ref={agentScrollRef}
        >
          {props.agents.map((agent) => (
            <AgentSelectorCard
              active={props.selectedAgentKey === agent.key}
              agent={agent}
              key={agent.key}
              onSelect={props.onSelectAgent}
              onToggle={props.onToggleAgent}
              selected={props.selectedAgentKeys.includes(agent.key)}
            />
          ))}
        </div>
      </div>

      <div className="flex min-h-7 items-center justify-between gap-3 px-1">
        <div className="text-sm font-semibold text-slate-100">
          {t("projects.editor.agentLayerTitle")}
        </div>
        <div className="truncate text-xs text-slate-500">
          {props.selectedAgentKey ?? t("projects.editor.noAgentSelected")}
        </div>
      </div>

      <div className="grid min-h-0 gap-4 min-[1120px]:grid-cols-[minmax(0,1.45fr)_minmax(18rem,0.75fr)]">
        <ProjectChecklistPanel title={t("projects.editor.skillsTitle")} variant="primary">
          <ProjectSkillFilterToolbar
            externalGroupFilter={props.externalGroupFilter}
            externalGroupSummaries={props.externalGroupSummaries}
            onExternalGroupFilterChange={props.onExternalGroupFilterChange}
            onSkillPathFilterChange={props.onSkillPathFilterChange}
            onSkillQueryChange={props.onSkillQueryChange}
            onSkillSelectionFilterChange={props.onSkillSelectionFilterChange}
            skillPathFilter={props.skillPathFilter}
            skillPathSummaries={props.skillPathSummaries}
            skillQuery={props.skillQuery}
            skillSelectionFilter={props.skillSelectionFilter}
          />
          <ProjectSkillList
            checkedIds={activeAgentDraft?.selectedSkillIds ?? []}
            disabled={!activeAgentDraft}
            onToggle={props.onToggleProjectSkill}
            scrollRef={skillScrollRef}
            skills={props.skills}
          />
        </ProjectChecklistPanel>

        <div className="grid min-h-0 gap-4">
          <ProjectChecklistPanel title={t("projects.editor.projectScenesTitle")} variant="secondary">
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
        </div>
      </div>
    </section>
  );
}

function SearchField(props: { value: string; placeholder: string; onChange: (value: string) => void }) {
  return (
    <label className="flex h-9 min-w-[13rem] flex-1 items-center gap-2 rounded-lg border border-slate-700 bg-slate-900 px-3">
      <Search className="h-3.5 w-3.5 shrink-0 text-slate-500" />
      <input
        className="min-w-0 flex-1 bg-transparent text-xs text-slate-200 outline-none placeholder:text-slate-600"
        onChange={(event) => props.onChange(event.target.value)}
        placeholder={props.placeholder}
        value={props.value}
      />
    </label>
  );
}

function AgentSelectorCard(props: {
  agent: AgentInventoryItem;
  active: boolean;
  selected: boolean;
  onSelect: (agentKey: string) => void;
  onToggle: (agentKey: string) => void;
}) {
  return (
    <div
      className={`min-w-[12rem] max-w-[14rem] rounded-xl border px-3 py-2 text-sm ${
        props.active
          ? "border-sky-500/50 bg-sky-500/10"
          : props.selected
            ? "border-slate-700 bg-slate-900/80"
            : "border-transparent bg-slate-950/40 hover:border-slate-800 hover:bg-slate-900/70"
      }`}
    >
      <div className="flex items-center gap-2">
        <input
          checked={props.selected}
          onChange={() => props.onToggle(props.agent.key)}
          type="checkbox"
        />
        <button
          className="min-w-0 flex-1 text-left"
          onClick={() => props.onSelect(props.agent.key)}
          type="button"
        >
          <span className="block truncate font-medium text-slate-100">
            {props.agent.displayName}
          </span>
          <span className="block truncate text-xs text-slate-500">
            {props.agent.projectSkillsDirRule}
          </span>
        </button>
      </div>
    </div>
  );
}

function ProjectChecklistPanel(props: { title: string; variant: "primary" | "secondary"; children: ReactNode }) {
  const heightClass =
    props.variant === "primary"
      ? "min-h-[26rem] max-h-[clamp(26rem,calc(100vh-20rem),40rem)]"
      : "min-h-[26rem] max-h-[clamp(26rem,calc(100vh-20rem),40rem)]";
  const overflowClass = props.variant === "primary" ? "overflow-visible" : "overflow-hidden";

  return (
    <div className={`flex min-h-0 flex-col ${overflowClass} rounded-xl border border-slate-800 bg-slate-950/60 ${heightClass}`}>
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
      className="skill-markdown-scroll grid min-h-0 flex-1 grid-cols-[repeat(auto-fill,minmax(15rem,1fr))] content-start gap-x-3 gap-y-1 overflow-y-auto px-3 py-2 pr-4"
      ref={props.scrollRef}
    >
      {props.skills.map((skill) => {
        const selected = props.checkedIds.includes(skill.id);

        return (
          <label
            className={`flex h-10 min-w-0 items-center gap-2 rounded-md border px-2 text-xs transition ${
              selected
                ? "border-sky-500/40 bg-sky-500/10 text-sky-100"
                : "border-transparent text-slate-300 hover:border-slate-800 hover:bg-slate-900/70"
            } ${props.disabled ? "opacity-60" : ""}`}
            key={skill.id}
            title={skill.description || skill.relativePath}
          >
            <input
              checked={selected}
              className="shrink-0"
              disabled={props.disabled}
              onChange={() => props.onToggle(skill.id)}
              type="checkbox"
            />
            <span className="min-w-0 flex-1">
              <span className="block truncate font-medium text-slate-100">{skill.name}</span>
              <span className="block truncate text-[11px] text-slate-600">
                {skill.relativePath}
              </span>
            </span>
          </label>
        );
      })}
    </div>
  );
}
