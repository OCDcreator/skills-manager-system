import { useEffect, useState } from "react";
import { X } from "lucide-react";
import { useTranslation } from "react-i18next";
import { AssistantLauncherMenu } from "./AssistantLauncherMenu";
import { AssistantTerminalSession } from "./AssistantTerminalSession";
import {
  getTerminalSession,
  startTerminalSession,
  type CliKey,
  type TerminalLaunchInput,
  type TerminalSessionSnapshot,
} from "../../lib/terminal";

interface ProjectAssistantPanelProps {
  onClose: () => void;
}

export function ProjectAssistantPanel({ onClose }: ProjectAssistantPanelProps) {
  const { t } = useTranslation();
  const [selectedCliKey, setSelectedCliKey] = useState<CliKey | null>(null);
  const [workingDirectory, setWorkingDirectory] = useState("");
  const [launchError, setLaunchError] = useState<string | null>(null);
  const [launchInput, setLaunchInput] = useState<TerminalLaunchInput | null>(null);
  const [session, setSession] = useState<TerminalSessionSnapshot | null>(null);

  useEffect(() => {
    void getTerminalSession().then((activeSession) => {
      if (!activeSession) {
        return;
      }
      setSelectedCliKey(activeSession.cliKey);
      setWorkingDirectory(activeSession.workingDirectory);
      setLaunchInput({
        cliKey: activeSession.cliKey,
        workingDirectory: activeSession.workingDirectory,
        cols: activeSession.cols,
        rows: activeSession.rows,
      });
      setSession(activeSession);
    });
  }, []);

  async function handleLaunch() {
    if (!selectedCliKey || !workingDirectory.trim()) {
      return;
    }

    const nextLaunchInput = {
      cliKey: selectedCliKey,
      workingDirectory,
      cols: 120,
      rows: 32,
    } satisfies TerminalLaunchInput;

    try {
      const nextSession = await startTerminalSession(nextLaunchInput);
      setLaunchInput(nextLaunchInput);
      setSession(nextSession);
      setLaunchError(null);
    } catch (error: unknown) {
      setLaunchError(error instanceof Error ? error.message : String(error));
    }
  }

  return (
    <section className="fixed bottom-24 right-6 z-50 flex h-[min(760px,calc(100vh-8rem))] w-[min(960px,calc(100vw-3rem))] flex-col overflow-hidden rounded-[28px] border border-sky-400/30 bg-slate-950/95 shadow-2xl shadow-slate-950/60 backdrop-blur">
      <header className="flex shrink-0 items-center justify-between border-b border-slate-800 px-5 py-4">
        <div>
          <h2 className="text-base font-semibold text-slate-100">{t("assistant.title")}</h2>
          <p className="text-sm text-slate-400">{t("assistant.subtitle")}</p>
        </div>
        <button
          aria-label={t("assistant.closePanel")}
          className="rounded-full p-2 text-slate-400 hover:bg-slate-900 hover:text-slate-100"
          onClick={onClose}
          title={t("assistant.closePanel")}
          type="button"
        >
          <X className="h-5 w-5" />
        </button>
      </header>
      {session && launchInput ? (
        <AssistantTerminalSession
          launchInput={launchInput}
          onRestart={() => void handleLaunch()}
          onSessionChange={setSession}
          session={session}
        />
      ) : (
        <AssistantLauncherMenu
          launchError={launchError}
          onCliSelect={setSelectedCliKey}
          onLaunch={() => void handleLaunch()}
          onWorkingDirectoryChange={setWorkingDirectory}
          selectedCliKey={selectedCliKey}
          workingDirectory={workingDirectory}
        />
      )}
    </section>
  );
}
