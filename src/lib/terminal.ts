import { invoke } from "@tauri-apps/api/core";

export type CliKey = "codex" | "opencode" | "claude_code" | "kimi";
export type TerminalSessionStatus =
  | "idle"
  | "starting"
  | "running"
  | "stopped"
  | "failed";

export interface TerminalLaunchInput {
  cliKey: CliKey;
  workingDirectory: string;
  cols: number;
  rows: number;
}

export interface TerminalSessionSnapshot {
  cliKey: CliKey;
  workingDirectory: string;
  cols: number;
  rows: number;
  status: TerminalSessionStatus;
  message: string | null;
}

export interface TerminalDrainResponse {
  output: string;
  session: TerminalSessionSnapshot | null;
}

export interface TerminalLauncherPreferences {
  defaultWorkingDirectory: string;
  workingDirectory: string;
}

export const getTerminalLauncherPreferences = () =>
  invoke<TerminalLauncherPreferences>("get_terminal_launcher_preferences");

export const setTerminalWorkingDirectoryPreference = (path: string) =>
  invoke<string | null>("set_terminal_working_directory_preference", { path });

export const getTerminalSession = () =>
  invoke<TerminalSessionSnapshot | null>("get_terminal_session");

export const startTerminalSession = (input: TerminalLaunchInput) =>
  invoke<TerminalSessionSnapshot>("start_terminal_session", { input });

export const drainTerminalOutput = () =>
  invoke<TerminalDrainResponse>("drain_terminal_output");

export const writeTerminalInput = (input: string) =>
  invoke<void>("write_terminal_input", { input });

export const resizeTerminalSession = (cols: number, rows: number) =>
  invoke<TerminalSessionSnapshot | null>("resize_terminal_session", {
    cols,
    rows,
  });

export const stopTerminalSession = () =>
  invoke<TerminalSessionSnapshot | null>("stop_terminal_session");
