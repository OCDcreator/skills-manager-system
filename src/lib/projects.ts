import { invoke } from "@tauri-apps/api/core";
import type { AgentTargetSkillEntry } from "./tauri";

export interface ProjectAssignment {
  projectPath: string;
  displayName: string;
  agents: Record<string, ProjectAgentAssignment>;
  unsupportedAgentKeys: string[];
  applyStatuses: Record<string, ProjectAgentApplyStatus>;
  skillIds?: string[];
  agentKeys?: string[];
}

export interface ProjectAgentAssignment {
  selectedSkillIds: string[];
  selectedSceneIds: string[];
  excludedSkillIds: string[];
}

export type ProjectApplyFreshness =
  | "current"
  | "stale"
  | "neverApplied"
  | "unsupported";

export interface ProjectAgentApplyStatus {
  applyStatus: ProjectApplyFreshness;
  lastAppliedAt: number | null;
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
  applyStatuses: Record<string, ProjectAgentApplyStatus>;
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
  targetSkillEntries: AgentTargetSkillEntry[];
  targetSkillScanError: string | null;
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

export const addProjectWithAgents = (
  projectPath: string,
  displayName: string,
  agents: Record<string, ProjectAgentAssignment>,
) =>
  invoke<ProjectConfigSnapshot>("add_project_with_agents", {
    projectPath,
    displayName,
    agents,
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

export const updateProjectWithAgents = (
  projectPath: string,
  displayName: string | null,
  agents: Record<string, ProjectAgentAssignment> | null,
  _unsupportedAgentKeys: string[] = [],
) =>
  invoke<ProjectConfigSnapshot>("update_project_with_agents", {
    projectPath,
    displayName,
    agents,
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

export const deleteProjectTargetSkill = (
  projectPath: string,
  agentKey: string,
  entryName: string,
) =>
  invoke<void>("delete_project_target_skill", {
    projectPath,
    agentKey,
    entryName,
  });

export const applyProjectAssignments = () =>
  invoke<ApplyProjectAssignmentsResponse>("apply_project_assignments");
