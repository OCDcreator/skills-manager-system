import { open } from "@tauri-apps/plugin-dialog";
import { FolderSearch, TerminalSquare } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { CliKey } from "../../lib/terminal";

const CLI_OPTIONS: { key: CliKey; label: string }[] = [
  { key: "codex", label: "Codex" },
  { key: "opencode", label: "OpenCode" },
  { key: "claude_code", label: "Claude Code" },
  { key: "kimi", label: "Kimi Code" },
];

interface AssistantLauncherMenuProps {
  defaultWorkingDirectory: string;
  launchError: string | null;
  selectedCliKey: CliKey | null;
  workingDirectory: string;
  onCliSelect: (key: CliKey) => void;
  onDefaultPathSelect: () => void;
  onLaunch: () => void;
  onWorkingDirectoryChange: (value: string) => void;
}

export function AssistantLauncherMenu(props: AssistantLauncherMenuProps) {
  const { t } = useTranslation();

  async function handleBrowse() {
    const selected = await open({ directory: true, multiple: false });
    if (typeof selected === "string") {
      props.onWorkingDirectoryChange(selected.replace(/\\/g, "/"));
    }
  }

  return (
    <div className="grid gap-4 p-5">
      <div className="space-y-2">
        <h3 className="text-lg font-semibold text-slate-100">
          {t("assistant.launcherTitle")}
        </h3>
        <p className="text-sm text-slate-400">
          {t("assistant.launcherBody")}
        </p>
      </div>

      <div className="grid gap-3 md:grid-cols-2">
        {CLI_OPTIONS.map((option) => {
          const isSelected = props.selectedCliKey === option.key;
          return (
            <button
              key={option.key}
              className={`rounded-2xl border p-4 text-left transition ${
                isSelected
                  ? "border-sky-400/70 bg-sky-400/10 text-sky-100"
                  : "border-slate-700 bg-slate-900/70 text-slate-100 hover:border-sky-400/40"
              }`}
              onClick={() => props.onCliSelect(option.key)}
              type="button"
            >
              <div className="flex items-center gap-3">
                <TerminalSquare className="h-5 w-5 text-sky-300" />
                <span className="font-medium">{option.label}</span>
              </div>
            </button>
          );
        })}
      </div>

      <div className="flex items-center gap-3 rounded-2xl border border-slate-800 bg-slate-950/70 p-3">
        <input
          className="min-w-0 flex-1 bg-transparent text-sm text-slate-100 outline-none placeholder:text-slate-500"
          onChange={(event) => props.onWorkingDirectoryChange(event.target.value)}
          placeholder={t("assistant.cwdPlaceholder")}
          value={props.workingDirectory}
        />
        <button
          className="rounded-xl border border-slate-700 px-3 py-2 text-sm text-slate-100 hover:border-sky-400/50 disabled:cursor-not-allowed disabled:text-slate-500"
          disabled={!props.defaultWorkingDirectory}
          onClick={props.onDefaultPathSelect}
          title={props.defaultWorkingDirectory || t("assistant.defaultPath")}
          type="button"
        >
          {t("assistant.defaultPath")}
        </button>
        <button
          className="rounded-xl border border-slate-700 px-3 py-2 text-sm text-slate-100 hover:border-sky-400/50"
          onClick={() => void handleBrowse()}
          type="button"
        >
          <FolderSearch className="h-4 w-4" />
        </button>
      </div>

      {props.launchError ? (
        <p className="text-sm text-rose-300">{props.launchError}</p>
      ) : null}

      <button
        className="rounded-full bg-sky-400 px-5 py-3 text-sm font-medium text-slate-950 disabled:cursor-not-allowed disabled:bg-slate-700 disabled:text-slate-400"
        disabled={!props.selectedCliKey || !props.workingDirectory.trim()}
        onClick={props.onLaunch}
        type="button"
      >
        {t("assistant.launchCta")}
      </button>
    </div>
  );
}
