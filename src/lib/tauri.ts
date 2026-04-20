import { invoke } from "@tauri-apps/api/core";

export interface SkillSummary {
  id: string;
  name: string;
  description: string;
  sourceType: "custom" | "external";
  relativePath: string;
  directoryPath: string;
  skillDocumentPath: string;
}

export interface ScanSkillsResponse {
  skills: SkillSummary[];
  warnings: string[];
}

export interface SkillDocument {
  id: string;
  name: string;
  description: string;
  sourceType: "custom" | "external";
  relativePath: string;
  content: string;
}

export const getRepoPath = () => invoke<string | null>("get_repo_path");

export const setRepoPath = (path: string) =>
  invoke<string | null>("set_repo_path", { path });

export const scanSkills = () => invoke<ScanSkillsResponse>("scan_skills");

export const getSkillDocument = (relativePath: string) =>
  invoke<SkillDocument>("get_skill_document", { relativePath });
