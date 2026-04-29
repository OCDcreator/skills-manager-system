import { useState } from "react";
import { X } from "lucide-react";
import { useTranslation } from "react-i18next";
import { AssistantLauncherMenu } from "./AssistantLauncherMenu";
import type { CliKey } from "../../lib/terminal";

interface ProjectAssistantPanelProps {
  onClose: () => void;
}

export function ProjectAssistantPanel({ onClose }: ProjectAssistantPanelProps) {
  const { t } = useTranslation();
  const [selectedCliKey, setSelectedCliKey] = useState<CliKey | null>(null);
  const [workingDirectory, setWorkingDirectory] = useState("");
  const [launchError, setLaunchError] = useState<string | null>(null);

  return (
    <section className="fixed bottom-24 right-6 z-50 flex h-[min(520px,calc(100vh-8rem))] w-[min(760px,calc(100vw-3rem))] flex-col overflow-hidden rounded-[28px] border border-sky-400/30 bg-slate-950/95 shadow-2xl shadow-slate-950/60 backdrop-blur">
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
      <AssistantLauncherMenu
        launchError={launchError}
        onCliSelect={setSelectedCliKey}
        onLaunch={() => setLaunchError("Terminal session view arrives in the next task")}
        onWorkingDirectoryChange={setWorkingDirectory}
        selectedCliKey={selectedCliKey}
        workingDirectory={workingDirectory}
      />
    </section>
  );
}
