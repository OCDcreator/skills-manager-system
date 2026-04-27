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
  displayName: string;
  enabledSkillCount: number;
  results: ProjectAgentResult[];
}

export interface ApplyProjectAssignmentsResponse {
  projectCount: number;
  results: ProjectApplyResult[];
}

export interface ProjectPathInspectionAgentResult {
  agentKey: string;
  displayName: string;
  markerDir: string;
  targetDir: string;
  markerExists: boolean;
  targetExists: boolean;
}

export interface ProjectPathInspection {
  normalizedPath: string;
  agents: ProjectPathInspectionAgentResult[];
  unsupportedAgentKeys: string[];
  warnings: string[];
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

export const inspectProjectAssignmentPath = (
  projectPath: string,
  agentKeys: string[],
) =>
  invoke<ProjectPathInspection>("inspect_project_assignment_path", {
    projectPath,
    agentKeys,
  });

export const applyProjectAssignments = () =>
  invoke<ApplyProjectAssignmentsResponse>("apply_project_assignments");
