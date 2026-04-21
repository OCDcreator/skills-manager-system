import { useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { Plus } from "lucide-react";
import { ProjectCard } from "../components/projects/ProjectCard";
import { useAppContext } from "../context/AppContext";
import * as projectsApi from "../lib/projects";
import type { ProjectConfigSnapshot } from "../lib/projects";

export function ProjectsView() {
  const { t } = useTranslation();
  const { repoPath, scanResult, agentInventory } = useAppContext();
  const [config, setConfig] = useState<ProjectConfigSnapshot | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [newPath, setNewPath] = useState("");
  const [newName, setNewName] = useState("");
  const [selectedSkills, setSelectedSkills] = useState<string[]>([]);
  const [selectedAgents, setSelectedAgents] = useState<string[]>([]);
  const [applying, setApplying] = useState<string | null>(null);
  const [lastResult, setLastResult] = useState<string | null>(null);

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
    void refresh();
  }, [refresh]);

  const handleAdd = async () => {
    if (!newPath.trim()) return;
    try {
      const snapshot = await projectsApi.addProject(
        newPath.trim(),
        newName.trim() || newPath.trim().split(/[/\\]/).pop() || "Project",
        selectedSkills,
        selectedAgents,
      );
      setConfig(snapshot);
      setNewPath("");
      setNewName("");
      setSelectedSkills([]);
      setSelectedAgents([]);
    } catch (error) {
      setLastResult(error instanceof Error ? error.message : String(error));
    }
  };

  const handleDelete = async (projectPath: string) => {
    try {
      const snapshot = await projectsApi.removeProject(projectPath);
      setConfig(snapshot);
    } catch (error) {
      setLastResult(error instanceof Error ? error.message : String(error));
    }
  };

  const handleApply = async (projectPath: string) => {
    setApplying(projectPath);
    try {
      const response = await projectsApi.applyProjectAssignments();
      const match = response.results.find((result) => result.projectPath === projectPath);
      if (match) {
        const written = match.results.reduce((sum, result) => sum + result.writtenCount, 0);
        setLastResult(
          `Applied to "${match.displayName}": ${written} skill(s) across ${match.results.length} agent(s)`,
        );
      } else {
        setLastResult(`Applied project assignments for ${response.projectCount} project(s).`);
      }
      await refresh();
    } catch (error) {
      setLastResult(error instanceof Error ? error.message : String(error));
    } finally {
      setApplying(null);
    }
  };

  const toggleSkill = (id: string) => {
    setSelectedSkills((prev) =>
      prev.includes(id) ? prev.filter((skillId) => skillId !== id) : [...prev, id],
    );
  };

  const toggleAgent = (key: string) => {
    setSelectedAgents((prev) =>
      prev.includes(key) ? prev.filter((agentKey) => agentKey !== key) : [...prev, key],
    );
  };

  if (!repoPath) {
    return (
      <section className="rounded-2xl border border-dashed border-slate-700 bg-slate-900 p-8 text-center">
        <h2 className="text-xl font-semibold text-slate-100">
          {t("projects.unconfigured")}
        </h2>
        <p className="mt-3 text-sm text-slate-400">
          {t("projects.unconfiguredBody")}
        </p>
      </section>
    );
  }

  const projectList = config ? Object.values(config.projects) : [];
  const skills = scanResult.skills;
  const agents = agentInventory;

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-semibold text-slate-100">
          {t("projects.title")}
        </h2>
        <p className="mt-1 text-sm text-slate-500">
          {t("projects.description")}
        </p>
      </div>

      {lastResult ? (
        <div className="rounded-lg bg-sky-950/40 px-4 py-2 text-sm text-sky-300">
          {lastResult}
          <button
            className="ml-2 text-sky-500 hover:text-sky-300"
            onClick={() => setLastResult(null)}
            type="button"
          >
            ×
          </button>
        </div>
      ) : null}

      <div className="space-y-3 rounded-2xl border border-slate-800 bg-slate-900 p-4">
        <div className="flex items-end gap-3">
          <div className="flex-1">
            <label className="mb-1 block text-xs text-slate-400">
              {t("projects.add.pathLabel")}
            </label>
            <input
              className="w-full rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm text-slate-200 placeholder:text-slate-500 focus:border-sky-400 focus:outline-none"
              onChange={(event) => setNewPath(event.target.value)}
              placeholder="/path/to/project"
              value={newPath}
            />
          </div>
          <div className="flex-1">
            <label className="mb-1 block text-xs text-slate-400">
              {t("projects.add.nameLabel")}
            </label>
            <input
              className="w-full rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm text-slate-200 placeholder:text-slate-500 focus:border-sky-400 focus:outline-none"
              onChange={(event) => setNewName(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === "Enter") void handleAdd();
              }}
              placeholder="My Project"
              value={newName}
            />
          </div>
          <button
            className="flex items-center gap-2 rounded-lg bg-sky-600 px-4 py-2 text-sm text-white hover:bg-sky-500 disabled:opacity-50"
            disabled={!newPath.trim()}
            onClick={() => void handleAdd()}
            type="button"
          >
            <Plus className="h-4 w-4" />
            {t("projects.add.button")}
          </button>
        </div>

        <div className="grid grid-cols-2 gap-3">
          <div>
            <label className="mb-1 block text-xs text-slate-400">
              {t("projects.add.selectSkills")}
            </label>
            <div className="max-h-32 space-y-1 overflow-y-auto rounded-lg border border-slate-700 bg-slate-800 p-2 text-xs">
              {skills.map((skill) => (
                <label className="flex items-center gap-2 text-slate-300" key={skill.id}>
                  <input
                    checked={selectedSkills.includes(skill.id)}
                    className="rounded border-slate-600"
                    onChange={() => toggleSkill(skill.id)}
                    type="checkbox"
                  />
                  {skill.name}
                </label>
              ))}
              {skills.length === 0 ? (
                <span className="text-slate-500">{t("projects.add.noSkills")}</span>
              ) : null}
            </div>
          </div>
          <div>
            <label className="mb-1 block text-xs text-slate-400">
              {t("projects.add.selectAgents")}
            </label>
            <div className="max-h-32 space-y-1 overflow-y-auto rounded-lg border border-slate-700 bg-slate-800 p-2 text-xs">
              {agents.map((agent) => (
                <label className="flex items-center gap-2 text-slate-300" key={agent.key}>
                  <input
                    checked={selectedAgents.includes(agent.key)}
                    className="rounded border-slate-600"
                    onChange={() => toggleAgent(agent.key)}
                    type="checkbox"
                  />
                  {agent.displayName}
                </label>
              ))}
              {agents.length === 0 ? (
                <span className="text-slate-500">{t("projects.add.noAgents")}</span>
              ) : null}
            </div>
          </div>
        </div>
      </div>

      {isLoading ? (
        <div className="text-sm text-slate-400">{t("projects.loading")}</div>
      ) : projectList.length === 0 ? (
        <div className="text-sm text-slate-500">{t("projects.empty")}</div>
      ) : (
        <div className="space-y-4">
          {projectList.map((project) => (
            <ProjectCard
              isApplying={applying === project.projectPath}
              key={project.projectPath}
              onApply={() => void handleApply(project.projectPath)}
              onDelete={() => void handleDelete(project.projectPath)}
              project={project}
              t={t}
            />
          ))}
        </div>
      )}
    </div>
  );
}
