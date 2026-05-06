import { useTranslation } from "react-i18next";
import { ProjectAssignmentEditor } from "./ProjectAssignmentEditor";
import { ProjectAssignmentSummary } from "./ProjectAssignmentSummary";
import { ProjectIdentityPanel } from "./ProjectIdentityPanel";
import type {
  ProjectAgentDraft,
  ProjectAgentStatusFilter,
  ProjectDraft,
  ProjectSkillSelectionFilter,
} from "../../lib/project-draft";
import type { ProjectAgentSummary } from "../../lib/project-summary";
import type { SceneEntry } from "../../lib/scenes";
import type { SkillPathSummary, SkillPathFilter } from "../../lib/skills/filters";
import type { AgentInventoryItem, SkillSummary } from "../../lib/tauri";

interface ProjectLayerWorkbenchProps {
  draft: ProjectDraft;
  duplicatePath: boolean;
  inferredName: string;
  inspectionError: string | null;
  isInspecting: boolean;
  isSaving: boolean;
  activeAgentDraft: ProjectAgentDraft | null;
  agentQuery: string;
  agentStatusFilter: ProjectAgentStatusFilter;
  agents: AgentInventoryItem[];
  scenes: SceneEntry[];
  selectedAgentKey: string | null;
  selectedAgentKeys: string[];
  skillPathFilter: SkillPathFilter;
  skillSelectionFilter: ProjectSkillSelectionFilter;
  skillPathSummaries: SkillPathSummary[];
  skillQuery: string;
  skills: SkillSummary[];
  summary: {
    selectedAgentCount: number;
    projectDirectSkillCount: number;
    projectSceneCount: number;
    agentSummaries: ProjectAgentSummary[];
    unsupportedAgentKeys: string[];
  };
  onAgentQueryChange: (value: string) => void;
  onAgentStatusFilterChange: (value: ProjectAgentStatusFilter) => void;
  onBrowseProjectPath: () => void;
  onCancelEdit: () => void;
  onDisplayNameChange: (value: string) => void;
  onProjectPathChange: (value: string) => void;
  onSave: () => void;
  onSelectAgent: (agentKey: string | null) => void;
  onSkillPathFilterChange: (value: SkillPathFilter) => void;
  onSkillSelectionFilterChange: (value: ProjectSkillSelectionFilter) => void;
  onSkillQueryChange: (value: string) => void;
  onToggleAgent: (agentKey: string) => void;
  onToggleProjectScene: (sceneId: string) => void;
  onToggleProjectSkill: (skillId: string) => void;
}

export function ProjectLayerWorkbench(props: ProjectLayerWorkbenchProps) {
  const { t } = useTranslation();

  return (
    <div className="grid gap-6 min-[1380px]:items-stretch min-[1380px]:grid-cols-[minmax(0,1fr)_clamp(22rem,28vw,34rem)]">
      <div className="space-y-4">
        <ProjectIdentityPanel
          canBrowseProjectPath={props.draft.mode === "create"}
          canSave={!props.duplicatePath && props.draft.projectPath.trim().length > 0}
          displayName={props.draft.displayName}
          duplicatePath={props.duplicatePath}
          inferredName={props.inferredName}
          inspectionError={props.inspectionError}
          isInspecting={props.isInspecting}
          isSaving={props.isSaving}
          mode={props.draft.mode}
          onBrowseProjectPath={props.onBrowseProjectPath}
          onCancelEdit={props.onCancelEdit}
          onDisplayNameChange={props.onDisplayNameChange}
          onProjectPathChange={props.onProjectPathChange}
          onSave={props.onSave}
          projectPath={props.draft.projectPath}
          saveLabel={
            props.draft.mode === "edit"
              ? t("projects.summary.saveEdit")
              : t("projects.summary.saveCreate")
          }
        />
        <ProjectAssignmentEditor
          activeAgentDraft={props.activeAgentDraft}
          agentQuery={props.agentQuery}
          agentStatusFilter={props.agentStatusFilter}
          agents={props.agents}
          onAgentQueryChange={props.onAgentQueryChange}
          onAgentStatusFilterChange={props.onAgentStatusFilterChange}
          onSelectAgent={props.onSelectAgent}
          onSkillPathFilterChange={props.onSkillPathFilterChange}
          onSkillSelectionFilterChange={props.onSkillSelectionFilterChange}
          onSkillQueryChange={props.onSkillQueryChange}
          onToggleAgent={props.onToggleAgent}
          onToggleProjectScene={props.onToggleProjectScene}
          onToggleProjectSkill={props.onToggleProjectSkill}
          scenes={props.scenes}
          selectedAgentKey={props.selectedAgentKey}
          selectedAgentKeys={props.selectedAgentKeys}
          skillPathFilter={props.skillPathFilter}
          skillSelectionFilter={props.skillSelectionFilter}
          skillPathSummaries={props.skillPathSummaries}
          skillQuery={props.skillQuery}
          skills={props.skills}
        />
      </div>
      <div className="min-h-0 min-[1380px]:sticky min-[1380px]:top-8 min-[1380px]:relative min-[1380px]:self-stretch">
        <ProjectAssignmentSummary
          agentSummaries={props.summary.agentSummaries}
          duplicatePath={props.duplicatePath}
          isInspecting={props.isInspecting}
          projectDirectSkillCount={props.summary.projectDirectSkillCount}
          projectSceneCount={props.summary.projectSceneCount}
          selectedAgentCount={props.summary.selectedAgentCount}
          title={
            props.draft.mode === "edit"
              ? t("projects.summary.editTitle")
              : t("projects.summary.createTitle")
          }
          unsupportedAgentKeys={props.summary.unsupportedAgentKeys}
        />
      </div>
    </div>
  );
}
