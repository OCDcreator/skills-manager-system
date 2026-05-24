import { invoke } from "./invoke";

export interface GitStatusEntry {
  path: string;
  x: string;
  y: string;
  isUntracked: boolean;
}

export interface GitStatusResponse {
  branch: string | null;
  remoteUrl: string | null;
  aheadBehind: [number, number] | null;
  staged: GitStatusEntry[];
  unstaged: GitStatusEntry[];
  untracked: GitStatusEntry[];
  isClean: boolean;
}

export interface GitDiffResponse {
  diff: string;
  stat: string;
}

export interface GitLogEntry {
  hash: string;
  shortHash: string;
  author: string;
  date: string;
  message: string;
}

export interface GitLogResponse {
  entries: GitLogEntry[];
}

export interface GitOperationResult {
  success: boolean;
  message: string;
}

export const gitStatus = () => invoke<GitStatusResponse>("git_status");

export const gitDiff = (staged: boolean) =>
  invoke<GitDiffResponse>("git_diff", { staged });

export const gitLog = (maxCount?: number) =>
  invoke<GitLogResponse>("git_log", { maxCount: maxCount ?? null });

export const gitPull = () => invoke<GitOperationResult>("git_pull");

export const gitPush = () => invoke<GitOperationResult>("git_push");

export const gitCommit = (message: string) =>
  invoke<GitOperationResult>("git_commit", { message });

export const gitFetch = () => invoke<GitOperationResult>("git_fetch");

export const runSyncScript = () =>
  invoke<GitOperationResult>("run_sync_script");
