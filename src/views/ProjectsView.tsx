import { useCallback, useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { ProjectAssignmentEditor } from "../components/projects/ProjectAssignmentEditor";
import { ProjectAssignmentSummary } from "../components/projects/ProjectAssignmentSummary";
import { ProjectCard } from "../components/projects/ProjectCard";
import { ProjectIdentityPanel } from "../components/projects/ProjectIdentityPanel";
import { useProjectDraftInspection } from "../components/projects/useProjectDraftInspection";
import { useAppContext } from "../context/AppContext";
import {
  buildProjectSummary,
  filterProjectAgents,
  filterProjectSkills,
  suggestProjectDisplayName,
  type ProjectDraft,
} from "../lib/project-draft";
import * as projectsApi from "../lib/projects";
import type { ProjectConfigSnapshot } from "../lib/projects";

export function ProjectsView() {
  const { t } = useTranslation();
  const { repoPath, scanResult, sortedAgentInventory, disabledSkillIds } = useAppContext();
  const [config, setConfig] = useState<ProjectConfigSnapshot | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [lastResult, setLastResult] = useState<string | null>(null);
  const [applying, setApplying] = useState<string | null>(null);
  const [isSavingDraft, setIsSavingDraft] = useState(false);
  const [skillQuery, setSkillQuery] = useState("");
  const [agentQuery, setAgentQuery] = useState("");
  const [sessionProjectApplyFeedback, setSessionProjectApplyFeedback] = useState<
    Record<string, string>
  >({});

  const emptyDraft = useMemo<ProjectDraft>(
    () => ({
      mode: "create",
      sourceProjectPath: null,
      projectPath: "",
      displayName: "",
      selectedSkillIds: [],
      selectedAgentKeys: [],
      unsupportedAgentKeys: [],
    }),
    [],
  );
  const [draft, setDraft] = useState<ProjectDraft>(emptyDraft);

  const refresh = useCallback(async () => {
    try {
      const snapshot = await projectsApi.getProjectConfig();
      setConfig(snapshot);
    } catch {
      setConfig(null);
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

  const visibleSkills = filterProjectSkills(scanResult.skills, skillQuery);
  const visibleAgents = filterProjectAgents(sortedAgentInventory, agentQuery);
  const summary = buildProjectSummary(
    draft,
    inspection,
    sortedAgentInventory,
    disabledSkillIds,
  );

  const toggleSkill = (skillId: string) => {
    setDraft((current) => ({
      ...current,
      selectedSkillIds: current.selectedSkillIds.includes(skillId)
        ? current.selectedSkillIds.filter((id) => id !== skillId)
        : [...current.selectedSkillIds, skillId],
    }));
  };

  const toggleAgent = (agentKey: string) => {
    setDraft((current) => ({
      ...current,
      selectedAgentKeys: current.selectedAgentKeys.includes(agentKey)
        ? current.selectedAgentKeys.filter((key) => key !== agentKey)
        : [...current.selectedAgentKeys, agentKey],
    }));
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
        const snapshot = await projectsApi.updateProject(
          draft.sourceProjectPath,
          displayName,
          [...draft.selectedSkillIds],
          [...draft.selectedAgentKeys, ...draft.unsupportedAgentKeys],
        );
        setConfig(snapshot);
      } else {
        const snapshot = await projectsApi.addProject(
          draft.projectPath.trim(),
          displayName,
          [...draft.selectedSkillIds],
          [...draft.selectedAgentKeys],
        );
        setConfig(snapshot);
        setDraft(emptyDraft);
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
    const unsupportedAgentKeys = project.agentKeys.filter(
      (key) => !sortedAgentInventory.some((agent) => agent.key === key),
    );
    setDraft({
      mode: "edit",
      sourceProjectPath: project.projectPath,
      projectPath: project.projectPath,
      displayName: project.displayName,
      selectedSkillIds: [...project.skillIds],
      selectedAgentKeys: project.agentKeys.filter((key) =>
        sortedAgentInventory.some((agent) => agent.key === key),
      ),
      unsupportedAgentKeys,
    });
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

      <div className="grid gap-6 min-[1380px]:grid-cols-[minmax(0,1fr)_clamp(22rem,28vw,34rem)]">
        <div className="space-y-4">
          <ProjectIdentityPanel
            displayName={draft.displayName}
            duplicatePath={duplicatePath}
            inferredName={suggestProjectDisplayName(draft.projectPath)}
            inspectionError={inspectionError}
            isInspecting={isInspecting}
            mode={draft.mode}
            onDisplayNameChange={(value) =>
              setDraft((current) => ({ ...current, displayName: value }))
            }
            onProjectPathChange={(value) =>
              setDraft((current) => ({ ...current, projectPath: value }))
            }
            projectPath={draft.projectPath}
          />
          <ProjectAssignmentEditor
            agentQuery={agentQuery}
            agents={visibleAgents}
            onAgentQueryChange={setAgentQuery}
            onSkillQueryChange={setSkillQuery}
            onToggleAgent={toggleAgent}
            onToggleSkill={toggleSkill}
            selectedAgentKeys={draft.selectedAgentKeys}
            selectedSkillIds={draft.selectedSkillIds}
            skillQuery={skillQuery}
            skills={visibleSkills}
          />
        </div>
        <div className="min-h-0 min-[1380px]:relative min-[1380px]:overflow-hidden">
          <div className="min-[1380px]:absolute min-[1380px]:inset-0">
            <ProjectAssignmentSummary
              canSave={!duplicatePath && draft.projectPath.trim().length > 0}
              disabledSelectedSkillIds={summary.disabledSelectedSkillIds}
              duplicatePath={duplicatePath}
              inspectionTargets={summary.inspectionAgents}
              isInspecting={isInspecting}
              isSaving={isSavingDraft}
              onSave={() => void handleSaveDraft()}
              saveLabel={
                draft.mode === "edit"
                  ? t("projects.summary.saveEdit")
                  : t("projects.summary.saveCreate")
              }
              selectedAgentCount={summary.selectedAgentCount}
              selectedSkillCount={summary.selectedSkillCount}
              title={
                draft.mode === "edit"
                  ? t("projects.summary.editTitle")
                  : t("projects.summary.createTitle")
              }
              unsupportedAgentKeys={summary.unsupportedAgentKeys}
            />
          </div>
        </div>
      </div>

      {isLoading ? (
        <div className="text-sm text-slate-400">{t("projects.loading")}</div>
      ) : projectList.length === 0 ? (
        <div className="text-sm text-slate-500">{t("projects.empty")}</div>
      ) : (
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-base font-semibold text-slate-100">
                {t("projects.saved.title")}
              </h3>
              <p className="mt-1 text-sm text-slate-500">
                {t("projects.saved.description")}
              </p>
            </div>
            <button
              className="rounded-xl bg-slate-100 px-4 py-2 text-sm font-medium text-slate-950 disabled:opacity-60"
              disabled={projectList.length === 0 || applying === "__all__"}
              onClick={() => void handleApplyAll()}
              type="button"
            >
              {t("projects.saved.applyAll")}
            </button>
          </div>
          <div className="space-y-4">
            {projectList.map((project) => (
              <ProjectCard
                key={project.projectPath}
                onDelete={() => void handleDelete(project.projectPath)}
                onEdit={() => handleEdit(project)}
                project={project}
                sessionApplyMessage={sessionProjectApplyFeedback[project.projectPath] ?? null}
                t={t}
              />
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
