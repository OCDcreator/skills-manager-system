import { useCallback, useEffect, useMemo, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { useTranslation } from "react-i18next";
import { ProjectLayerWorkbench } from "../components/projects/ProjectLayerWorkbench";
import { SavedProjectsSection } from "../components/projects/SavedProjectsSection";
import { useProjectDraftInspection } from "../components/projects/useProjectDraftInspection";
import { useAppContext } from "../context/AppContext";
import {
  applyProjectDisplayNameToDraft,
  applyProjectPathToDraft,
  ensureProjectAgentDraft,
  filterProjectAgents,
  filterProjectSkills,
  projectDraftAgentKeys,
  projectDraftFromAssignment,
  projectDraftToAgentAssignments,
  removeProjectAgentDraft,
  sortProjectAgentsForEditor,
  suggestProjectDisplayName,
  toggleProjectAgentScene,
  toggleProjectAgentSkill,
  type ProjectAgentStatusFilter,
  type ProjectSkillSelectionFilter,
  type ProjectDraft,
} from "../lib/project-draft";
import { buildProjectSummary } from "../lib/project-summary";
import * as projectsApi from "../lib/projects";
import type { ProjectConfigSnapshot } from "../lib/projects";
import * as scenesApi from "../lib/scenes";
import type { SceneConfigSnapshot } from "../lib/scenes";
import { buildExternalGroupSummaries, type ExternalGroupFilter } from "../lib/scene-skill-filters";
import { buildSkillPathSummaries, type SkillPathFilter } from "../lib/skills/filters";

export function ProjectsView() {
  const { t } = useTranslation();
  const { repoPath, scanResult, sortedAgentInventory, disabledSkillIds } = useAppContext();
  const [config, setConfig] = useState<ProjectConfigSnapshot | null>(null);
  const [sceneConfig, setSceneConfig] = useState<SceneConfigSnapshot | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [lastResult, setLastResult] = useState<string | null>(null);
  const [applying, setApplying] = useState<string | null>(null);
  const [isSavingDraft, setIsSavingDraft] = useState(false);
  const [skillQuery, setSkillQuery] = useState("");
  const [agentQuery, setAgentQuery] = useState("");
  const [skillPathFilter, setSkillPathFilter] = useState<SkillPathFilter>("all");
  const [externalGroupFilter, setExternalGroupFilter] = useState<ExternalGroupFilter>("all");
  const [skillSelectionFilter, setSkillSelectionFilter] = useState<ProjectSkillSelectionFilter>("all");
  const [agentStatusFilter, setAgentStatusFilter] = useState<ProjectAgentStatusFilter>("all");
  const [selectedAgentKey, setSelectedAgentKey] = useState<string | null>(null);
  const [sessionProjectApplyFeedback, setSessionProjectApplyFeedback] = useState<
    Record<string, string>
  >({});

  const emptyDraft = useMemo<ProjectDraft>(
    () => ({
      mode: "create",
      sourceProjectPath: null,
      projectPath: "",
      displayName: "",
      displayNameManuallyEdited: false,
      agents: {},
      unsupportedAgentKeys: [],
    }),
    [],
  );
  const [draft, setDraft] = useState<ProjectDraft>(emptyDraft);

  const refresh = useCallback(async () => {
    try {
      const [snapshot, scenes] = await Promise.all([
        projectsApi.getProjectConfig(),
        scenesApi.getSceneConfig(),
      ]);
      setConfig(snapshot);
      setSceneConfig(scenes);
    } catch {
      setConfig(null);
      setSceneConfig(null);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    queueMicrotask(() => {
      void refresh();
    });
  }, [refresh]);

  const projectList = config ? Object.values(config.projects) : [];
  const sceneList = useMemo(() => Object.values(sceneConfig?.scenes ?? {}), [sceneConfig]);
  const selectedAgentKeys = projectDraftAgentKeys(draft);
  const activeAgentKey = selectedAgentKey && draft.agents[selectedAgentKey] ? selectedAgentKey : selectedAgentKeys[0] ?? null;
  const activeAgentDraft = activeAgentKey ? draft.agents[activeAgentKey] : null;
  const inspectableAgentKeys = useMemo(
    () => sortedAgentInventory.map((agent) => agent.key),
    [sortedAgentInventory],
  );
  const { inspection, isInspecting, inspectionError } = useProjectDraftInspection(
    draft.projectPath,
    inspectableAgentKeys,
  );

  const normalizedPath = inspection?.normalizedPath ?? draft.projectPath.trim();
  const duplicatePath =
    normalizedPath.length > 0 &&
    projectList.some(
      (project) =>
        project.projectPath === normalizedPath &&
        project.projectPath !== draft.sourceProjectPath,
    );

  const skillPathSummaries = useMemo(
    () => buildSkillPathSummaries(scanResult.skills),
    [scanResult.skills],
  );
  const externalGroupSummaries = useMemo(
    () => buildExternalGroupSummaries(scanResult.skills, t("projects.editor.externalGroups.all")),
    [scanResult.skills, t],
  );
  const visibleSkills = filterProjectSkills(
    scanResult.skills,
    skillQuery,
    skillPathFilter,
    externalGroupFilter,
    skillSelectionFilter,
    activeAgentDraft?.selectedSkillIds ?? [],
  );
  const visibleAgents = sortProjectAgentsForEditor(
    filterProjectAgents(sortedAgentInventory, agentQuery, agentStatusFilter),
    selectedAgentKeys,
    draft.mode === "edit",
  );
  const summary = buildProjectSummary(
    draft,
    inspection,
    scanResult.skills,
    sortedAgentInventory,
    disabledSkillIds,
    sceneConfig?.scenes ?? {},
  );
  const toggleSkill = (skillId: string) => {
    if (!activeAgentKey) return;
    setDraft((current) => toggleProjectAgentSkill(current, activeAgentKey, skillId));
  };

  const toggleAgent = (agentKey: string) => {
    setDraft((current) => {
      const next = current.agents[agentKey]
        ? removeProjectAgentDraft(current, agentKey)
        : ensureProjectAgentDraft(current, agentKey);
      if (!current.agents[agentKey]) {
        setSelectedAgentKey(agentKey);
      } else if (selectedAgentKey === agentKey) {
        setSelectedAgentKey(projectDraftAgentKeys(next)[0] ?? null);
      }
      return next;
    });
  };

  const resetEditorFilters = () => {
    setSkillQuery("");
    setAgentQuery("");
    setSkillPathFilter("all");
    setExternalGroupFilter("all");
    setSkillSelectionFilter("all");
    setAgentStatusFilter("all");
  };

  const handleBrowseProjectPath = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: draft.projectPath.trim() || undefined,
    });

    if (typeof selected === "string") {
      setDraft((current) => applyProjectPathToDraft(current, selected));
    }
  };

  const handleDelete = async (projectPath: string) => {
    try {
      const snapshot = await projectsApi.removeProject(projectPath);
      setConfig(snapshot);
      setSessionProjectApplyFeedback((current) => {
        const next = { ...current };
        delete next[projectPath];
        return next;
      });
      setLastResult(t("projects.saved.deleteSuccess"));
    } catch (error) {
      setLastResult(error instanceof Error ? error.message : String(error));
    }
  };

  const handleSaveDraft = async () => {
    setIsSavingDraft(true);
    try {
      const displayName =
        draft.displayName.trim() || suggestProjectDisplayName(draft.projectPath);
      if (draft.mode === "edit" && draft.sourceProjectPath) {
        const snapshot = await projectsApi.updateProjectWithAgents(
          draft.sourceProjectPath,
          displayName,
          projectDraftToAgentAssignments(draft),
          draft.unsupportedAgentKeys,
        );
        setConfig(snapshot);
      } else {
        const snapshot = await projectsApi.addProjectWithAgents(
          draft.projectPath.trim(),
          displayName,
          projectDraftToAgentAssignments(draft),
        );
        setConfig(snapshot);
        setDraft(emptyDraft);
        setSelectedAgentKey(null);
        resetEditorFilters();
      }
      setLastResult(t("projects.saved.saveSuccess"));
    } catch (error) {
      setLastResult(error instanceof Error ? error.message : String(error));
    } finally {
      setIsSavingDraft(false);
    }
  };

  const handleApplyAll = async () => {
    setApplying("__all__");
    try {
      const response = await projectsApi.applyProjectAssignments();
      setSessionProjectApplyFeedback(
        Object.fromEntries(
          response.results.map((result) => [
            result.projectPath,
            `Applied ${result.enabledSkillCount} selected skill(s) across ${result.results.length} agent(s).`,
          ]),
        ),
      );
      setLastResult(`Applied project assignments for ${response.projectCount} project(s).`);
      await refresh();
    } catch (error) {
      setLastResult(error instanceof Error ? error.message : String(error));
    } finally {
      setApplying(null);
    }
  };

  const handleEdit = (project: projectsApi.ProjectAssignment) => {
    resetEditorFilters();
    const nextDraft = projectDraftFromAssignment(project, sortedAgentInventory);
    setDraft(nextDraft);
    setSelectedAgentKey(projectDraftAgentKeys(nextDraft)[0] ?? null);
  };
  const handleCancelEdit = () => {
    setDraft(emptyDraft);
    setSelectedAgentKey(null);
    resetEditorFilters();
  };
  if (!repoPath) {
    return (
      <section className="rounded-2xl border border-dashed border-slate-700 bg-slate-900 p-8 text-center">
        <h2 className="text-xl font-semibold text-slate-100">{t("projects.unconfigured")}</h2>
        <p className="mt-3 text-sm text-slate-400">{t("projects.unconfiguredBody")}</p>
      </section>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-semibold text-slate-100">{t("projects.title")}</h2>
        <p className="mt-1 text-sm text-slate-500">{t("projects.description")}</p>
      </div>

      {lastResult ? (
        <div className="rounded-lg bg-sky-950/40 px-4 py-2 text-sm text-sky-300">
          {lastResult}
        </div>
      ) : null}

      <ProjectLayerWorkbench
        activeAgentDraft={activeAgentDraft}
        agentQuery={agentQuery}
        agentStatusFilter={agentStatusFilter}
        agents={visibleAgents}
        draft={draft}
        duplicatePath={duplicatePath}
        externalGroupFilter={externalGroupFilter}
        externalGroupSummaries={externalGroupSummaries}
        inferredName={suggestProjectDisplayName(draft.projectPath)}
        inspectionError={inspectionError}
        isInspecting={isInspecting}
        isSaving={isSavingDraft}
        onAgentQueryChange={setAgentQuery}
        onAgentStatusFilterChange={setAgentStatusFilter}
        onBrowseProjectPath={() => void handleBrowseProjectPath()}
        onCancelEdit={handleCancelEdit}
        onDisplayNameChange={(value) =>
          setDraft((current) => applyProjectDisplayNameToDraft(current, value))
        }
        onProjectPathChange={(value) =>
          setDraft((current) => applyProjectPathToDraft(current, value))
        }
        onSave={() => void handleSaveDraft()}
        onSelectAgent={setSelectedAgentKey}
        onExternalGroupFilterChange={setExternalGroupFilter}
        onSkillPathFilterChange={setSkillPathFilter}
        onSkillSelectionFilterChange={setSkillSelectionFilter}
        onSkillQueryChange={setSkillQuery}
        onToggleAgent={toggleAgent}
        onToggleProjectScene={(sceneId) => {
          if (!activeAgentKey) return;
          setDraft((current) => toggleProjectAgentScene(current, activeAgentKey, sceneId));
        }}
        onToggleProjectSkill={toggleSkill}
        scenes={sceneList}
        selectedAgentKey={activeAgentKey}
        selectedAgentKeys={selectedAgentKeys}
        skillPathFilter={skillPathFilter}
        skillPathSummaries={skillPathSummaries}
        skillQuery={skillQuery}
        skillSelectionFilter={skillSelectionFilter}
        skills={visibleSkills}
        summary={summary}
      />

      {isLoading ? (
        <div className="text-sm text-slate-400">{t("projects.loading")}</div>
      ) : (
        <SavedProjectsSection
          applyingKey={applying}
          onApplyAll={() => void handleApplyAll()}
          onDelete={(projectPath) => void handleDelete(projectPath)}
          onEdit={handleEdit}
          projects={projectList}
          sessionApplyFeedback={sessionProjectApplyFeedback}
        />
      )}
    </div>
  );
}
