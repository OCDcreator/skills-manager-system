import { invoke } from "@tauri-apps/api/core";

export interface ProjectAssignment {
  projectPath: string;
  displayName: string;
  skillIds: string[];
  agentKeys: string[];
}

export interface ProjectConfigSnapshot {
  projects: Record<string, ProjectAssignment>;
}

export interface ProjectAgentResult {
  agentKey: string;
  status: string;
  writtenCount: number;
  message: string;
}

export interface ProjectApplyResult {
  projectPath: string;
  results: ProjectAgentResult[];
}

export const getProjectConfig = () =>
  invoke<ProjectConfigSnapshot>("get_project_config");

export const addProject = (
  projectPath: string,
  displayName: string,
  skillIds: string[],
  agentKeys: string[],
) =>
  invoke<ProjectConfigSnapshot>("add_project", {
    projectPath,
    displayName,
    skillIds,
    agentKeys,
  });

export const updateProject = (
  projectPath: string,
  displayName: string | null,
  skillIds: string[] | null,
  agentKeys: string[] | null,
) =>
  invoke<ProjectConfigSnapshot>("update_project", {
    projectPath,
    displayName,
    skillIds,
    agentKeys,
  });

export const removeProject = (projectPath: string) =>
  invoke<ProjectConfigSnapshot>("remove_project", { projectPath });

export const applyProjectAssignments = () =>
  invoke<ProjectApplyResult[]>("apply_project_assignments");
