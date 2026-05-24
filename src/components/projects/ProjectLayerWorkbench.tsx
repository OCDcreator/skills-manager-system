import { useTranslation } from "react-i18next";
import { ProjectAssignmentEditor } from "./ProjectAssignmentEditor";
import { ProjectAssignmentSummary } from "./ProjectAssignmentSummary";
import { ProjectIdentityPanel } from "./ProjectIdentityPanel";
import type { ProjectDraft } from "../../lib/project-draft";
import type {
  ProjectAgentStatusFilter,
  ProjectSkillSelectionFilter,
} from "../../lib/project-filters";
import type { ProjectAgentSummary } from "../../lib/project-summary";
import type {
  ExternalGroupFilter,
  ExternalGroupSummary,
} from "../../lib/scene-skill-filters";
import type { SceneEntry } from "../../lib/scenes";
import type { SkillPathSummary, SkillPathFilter } from "../../lib/skills/filters";
import type { AgentInventoryItem, AgentTargetSkillEntry, SkillSummary } from "../../lib/tauri";

interface ProjectLayerWorkbenchProps {
  draft: ProjectDraft;
  duplicatePath: boolean;
  inferredName: string;
  inspectionError: string | null;
  isInspecting: boolean;
  isSaving: boolean;
  agentQuery: string;
  agentStatusFilter: ProjectAgentStatusFilter;
  agents: AgentInventoryItem[];
  scenes: SceneEntry[];
  selectedAgentKeys: string[];
  skillPathFilter: SkillPathFilter;
  externalGroupFilter: ExternalGroupFilter;
  skillSelectionFilter: ProjectSkillSelectionFilter;
  externalGroupSummaries: ExternalGroupSummary[];
  skillPathSummaries: SkillPathSummary[];
  skillQuery: string;
  skills: SkillSummary[];
  targetActionId: string | null;
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
  onDeleteTargetSkill: (agentKey: string, entry: AgentTargetSkillEntry) => void;
  onDisplayNameChange: (value: string) => void;
  onProjectPathChange: (value: string) => void;
  onSave: () => void;
  onExternalGroupFilterChange: (value: ExternalGroupFilter) => void;
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
    <div className="grid gap-6 max-[1379px]:space-y-6 min-[1380px]:items-start min-[1380px]:grid-cols-[minmax(0,1fr)_clamp(22rem,28vw,34rem)]">
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
          agentDrafts={props.draft.agents}
          agentQuery={props.agentQuery}
          agentStatusFilter={props.agentStatusFilter}
          agents={props.agents}
          externalGroupFilter={props.externalGroupFilter}
          externalGroupSummaries={props.externalGroupSummaries}
          onAgentQueryChange={props.onAgentQueryChange}
          onAgentStatusFilterChange={props.onAgentStatusFilterChange}
          onExternalGroupFilterChange={props.onExternalGroupFilterChange}
          onSkillPathFilterChange={props.onSkillPathFilterChange}
          onSkillSelectionFilterChange={props.onSkillSelectionFilterChange}
          onSkillQueryChange={props.onSkillQueryChange}
          onToggleAgent={props.onToggleAgent}
          onToggleProjectScene={props.onToggleProjectScene}
          onToggleProjectSkill={props.onToggleProjectSkill}
          scenes={props.scenes}
          selectedAgentKeys={props.selectedAgentKeys}
          skillPathFilter={props.skillPathFilter}
          skillSelectionFilter={props.skillSelectionFilter}
          skillPathSummaries={props.skillPathSummaries}
          skillQuery={props.skillQuery}
          skills={props.skills}
        />
      </div>
      <div className="min-h-0 min-[1380px]:sticky min-[1380px]:top-8">
        <ProjectAssignmentSummary
          agentSummaries={props.summary.agentSummaries}
          duplicatePath={props.duplicatePath}
          isInspecting={props.isInspecting}
          onDeleteTargetSkill={props.onDeleteTargetSkill}
          projectDirectSkillCount={props.summary.projectDirectSkillCount}
          projectSceneCount={props.summary.projectSceneCount}
          selectedAgentCount={props.summary.selectedAgentCount}
          targetActionId={props.targetActionId}
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
