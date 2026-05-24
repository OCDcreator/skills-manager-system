import { execFile } from "node:child_process";
import path from "node:path";
import { promisify } from "node:util";
import { defineConfig, type Plugin } from "vite";
import react from "@vitejs/plugin-react";

const execFileAsync = promisify(execFile);

interface DevInvokeBody {
  command?: string;
  args?: Record<string, unknown>;
}

interface CliPayload {
  ok: boolean;
  status: string;
  data?: Record<string, unknown>;
  error?: { message?: string };
}

type DirectCommandResult = { direct: unknown };
type CommandMapper = (args: Record<string, unknown>) => string[] | DirectCommandResult;

const repoRoot = process.cwd();
const cliBinary = path.join(
  repoRoot,
  "src-tauri",
  "target",
  "debug",
  process.platform === "win32" ? "skills-manager.exe" : "skills-manager",
);

const commandMap: Record<string, CommandMapper> = {
  get_repo_path: () => ["settings", "get-repo-path"],
  set_repo_path: (args) => ["settings", "set-repo-path", String(args.path ?? "")],
  get_agent_sync_mode: () => ["settings", "get-sync-mode"],
  set_agent_sync_mode: (args) => ["settings", "set-sync-mode", String(args.syncMode ?? "")],
  scan_skills: () => ["skills", "scan"],
  load_cached_skills: () => ["skills", "scan"],
  get_skill_state: () => ["skills", "state"],
  get_skill_document: (args) => ["skills", "doc", String(args.relativePath ?? "")],
  set_skill_enabled: (args) => [
    "skills",
    args.enabled ? "enable" : "disable",
    String(args.skillId ?? ""),
  ],
  get_agent_inventory: () => ["agents", "list"],
  set_agent_enabled: (args) => [
    "agents",
    args.enabled ? "enable" : "disable",
    String(args.key ?? ""),
  ],
  set_agent_path_override: (args) => [
    "agents",
    "set-path",
    String(args.key ?? ""),
    String(args.path ?? ""),
  ],
  clear_agent_path_override: (args) => ["agents", "clear-path", String(args.key ?? "")],
  apply_agent_sync: (args) => [
    "agents",
    "sync",
    ...(args.syncMode ? ["--sync-mode", String(args.syncMode)] : []),
  ],
  get_scene_config: () => ["scenes", "list"],
  create_scene: (args) => [
    "scenes",
    "create",
    String(args.id ?? ""),
    String(args.name ?? ""),
    "--description",
    String(args.description ?? ""),
  ],
  update_scene: (args) => [
    "scenes",
    "update",
    String(args.id ?? ""),
    ...(args.name !== null && args.name !== undefined ? ["--name", String(args.name)] : []),
    ...(args.description !== null && args.description !== undefined
      ? ["--description", String(args.description)]
      : []),
  ],
  delete_scene: (args) => ["scenes", "delete", String(args.id ?? "")],
  set_active_scene: (args) =>
    args.id ? ["scenes", "set-active", String(args.id)] : ["scenes", "clear-active"],
  set_scene_skills: (args) => [
    "scenes",
    "set-skills",
    String(args.id ?? ""),
    ...stringList(args.skillIds),
  ],
  set_scene_agents: (args) => [
    "scenes",
    "set-agents",
    String(args.id ?? ""),
    ...stringList(args.enabledAgentKeys),
  ],
  set_scene_skill_order: (args) => [
    "scenes",
    "set-skill-order",
    String(args.id ?? ""),
    ...stringList(args.skillOrder),
  ],
  get_project_config: () => ["projects", "list"],
  remove_project: (args) => ["projects", "remove", String(args.projectPath ?? "")],
  apply_project_assignments: () => ["projects", "apply"],
  git_status: () => ["git", "status"],
  git_diff: (args) => ["git", "diff", ...(args.staged ? ["--staged"] : [])],
  git_log: (args) => ["git", "log", "--max-count", String(args.maxCount ?? 20)],
  git_fetch: () => ["git", "fetch"],
  git_pull: () => ["git", "pull"],
  git_push: () => ["git", "push"],
  git_commit: (args) => ["git", "commit", "--message", String(args.message ?? "")],
  run_sync_script: () => ["git", "sync-external"],
  list_external_sources: () => ({ direct: { sources: [] } }),
  get_agent_order: () => ["agents", "list"],
};

function stringList(value: unknown) {
  return Array.isArray(value) ? value.map(String) : [];
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function nestedRecord(value: unknown, key: string) {
  if (!isRecord(value)) return undefined;
  const nested = value[key];
  return isRecord(nested) ? nested : undefined;
}

function unwrapCommandData(command: string, data: Record<string, unknown> | undefined) {
  const settings = nestedRecord(data, "settings");
  const agentConfig = nestedRecord(data, "agentConfig");
  switch (command) {
    case "get_repo_path":
    case "set_repo_path":
      return data?.repoPath ?? settings?.repoPath ?? null;
    case "get_agent_sync_mode":
    case "set_agent_sync_mode":
      return data?.agentSyncMode ?? settings?.agentSyncMode ?? "symlink";
    case "get_agent_order":
      return Array.isArray(data?.agents)
        ? data.agents.map((agent) => (isRecord(agent) ? agent.key : null)).filter(Boolean)
        : [];
    case "scan_skills":
    case "load_cached_skills":
      return {
        skills: data?.skills ?? [],
        warnings: data?.warnings ?? [],
      };
    case "get_skill_state":
    case "set_skill_enabled":
      return data?.state ?? data ?? { disabledSkillIds: [] };
    case "get_skill_document":
      return data?.document ?? data;
    case "get_agent_inventory":
    case "set_agent_enabled":
    case "set_agent_path_override":
    case "clear_agent_path_override":
      return { agents: data?.agents ?? agentConfig?.agents ?? [] };
    case "apply_agent_sync":
      return data?.agentSync ?? data ?? { enabledSkillCount: 0, results: [] };
    case "get_scene_config":
    case "create_scene":
    case "update_scene":
    case "delete_scene":
    case "set_active_scene":
    case "set_scene_skills":
    case "set_scene_agents":
    case "set_scene_skill_order":
      return data?.sceneConfig ?? data ?? { scenes: {}, activeSceneId: null };
    case "get_project_config":
    case "remove_project":
      return normalizeProjectConfig(data?.projectConfig ?? data);
    case "apply_project_assignments":
      return data?.projects ?? data ?? { projectCount: 0, results: [] };
    case "git_status":
      return data?.status ?? data;
    case "git_diff":
      return data?.diff ?? data;
    case "git_log":
      return data?.log ?? data;
    case "git_fetch":
    case "git_pull":
    case "git_push":
    case "git_commit":
    case "run_sync_script":
      return data?.operation ?? data ?? { success: true, message: "OK" };
    default:
      return data ?? null;
  }
}

function normalizeProjectConfig(input: unknown) {
  const config = (input ?? { projects: {} }) as { projects?: Record<string, Record<string, unknown>> };
  for (const project of Object.values(config.projects ?? {})) {
    if (!project.agents) {
      const agentKeys = stringList(project.agentKeys);
      const skillIds = stringList(project.skillIds);
      project.agents = Object.fromEntries(
        agentKeys.map((agentKey) => [
          agentKey,
          { selectedSkillIds: skillIds, selectedSceneIds: [], excludedSkillIds: [] },
        ]),
      );
    }
    project.unsupportedAgentKeys ??= [];
    project.applyStatuses ??= {};
  }
  return config;
}

function skillsManagerDevInvokePlugin(): Plugin {
  return {
    configureServer(server) {
      server.middlewares.use("/__skills-manager-dev-invoke", (request, response) => {
        if (request.method !== "POST" || !isLoopbackAddress(request.socket.remoteAddress)) {
          response.statusCode = 404;
          response.end();
          return;
        }

        let rawBody = "";
        request.setEncoding("utf8");
        request.on("data", (chunk) => {
          rawBody += chunk;
        });
        request.on("end", () => {
          void (async () => {
            response.setHeader("content-type", "application/json; charset=utf-8");
            try {
              const body = JSON.parse(rawBody || "{}") as DevInvokeBody;
              const command = body.command ?? "";
              const args = body.args ?? {};
              const mapper = commandMap[command];
              if (!mapper) {
                throw new Error(`Unsupported dev invoke command: ${command}`);
              }
              const mapped = mapper(args);
              if ("direct" in mapped) {
                response.end(JSON.stringify({ ok: true, data: mapped.direct }));
                return;
              }
              const payload = await runCli(mapped);
              response.statusCode = payload.ok ? 200 : 500;
              response.end(JSON.stringify({
                ok: payload.ok,
                data: unwrapCommandData(command, payload.data),
                error: payload.error,
              }));
            } catch (error) {
              response.statusCode = 500;
              response.end(JSON.stringify({
                ok: false,
                error: { message: error instanceof Error ? error.message : String(error) },
              }));
            }
          })();
        });
      });
    },
    name: "skills-manager-dev-invoke",
  };
}

function isLoopbackAddress(address: string | undefined) {
  return (
    address === "127.0.0.1" ||
    address === "::1" ||
    address === "::ffff:127.0.0.1"
  );
}

async function runCli(args: string[]) {
  const { stdout } = await execFileAsync(cliBinary, ["--json", ...args], {
    cwd: repoRoot,
    maxBuffer: 80 * 1024 * 1024,
  });
  return JSON.parse(stdout) as CliPayload;
}

export default defineConfig({
  plugins: [react(), skillsManagerDevInvokePlugin()],
  build: {
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (id.includes("highlight.js") || id.includes("marked")) {
            return "vendor-markdown";
          }
          return undefined;
        },
      },
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
});
