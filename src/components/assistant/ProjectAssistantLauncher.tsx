import { Bot, MessageSquarePlus, X } from "lucide-react";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import {
  getAssistantContextStatus,
  type AssistantContextStatus,
} from "../../lib/assistant";
import { ProjectAssistantPanel } from "./ProjectAssistantPanel";

export function ProjectAssistantLauncher() {
  const { t } = useTranslation();
  const [isOpen, setIsOpen] = useState(false);
  const [status, setStatus] = useState<AssistantContextStatus | null>(null);
  const [statusError, setStatusError] = useState<string | null>(null);
  const [isLoadingStatus, setIsLoadingStatus] = useState(false);

  useEffect(() => {
    if (!isOpen || status || isLoadingStatus) {
      return;
    }

    setIsLoadingStatus(true);
    void getAssistantContextStatus()
      .then((nextStatus) => {
        setStatus(nextStatus);
        setStatusError(null);
      })
      .catch((error: unknown) => {
        setStatusError(error instanceof Error ? error.message : String(error));
      })
      .finally(() => setIsLoadingStatus(false));
  }, [isLoadingStatus, isOpen, status]);

  return (
    <>
      <div className="fixed bottom-6 right-6 z-40 group" data-project-assistant-launcher>
        {!isOpen ? (
          <div className="pointer-events-none absolute bottom-full right-0 mb-3 flex translate-y-1 items-center rounded-full bg-slate-900/95 px-3 py-2 text-xs text-slate-300 opacity-0 shadow-lg shadow-slate-950/40 transition duration-200 group-hover:translate-y-0 group-hover:opacity-100 group-focus-within:translate-y-0 group-focus-within:opacity-100">
            <MessageSquarePlus className="mr-2 inline h-4 w-4" />
            {t("assistant.launcherHint")}
          </div>
        ) : null}

        <button
          className="relative flex h-14 w-14 items-center justify-center rounded-full border border-sky-400/40 bg-slate-900 text-sky-300 shadow-2xl shadow-sky-950/40 transition hover:border-sky-300 hover:text-sky-100"
          onClick={() => setIsOpen((current) => !current)}
          title={t("assistant.launcherLabel")}
          type="button"
        >
          {isOpen ? <X className="h-6 w-6" /> : <Bot className="h-6 w-6" />}
        </button>
      </div>

      {isOpen ? (
        <ProjectAssistantPanel
          isLoadingStatus={isLoadingStatus}
          onClose={() => setIsOpen(false)}
          status={status}
          statusError={statusError}
        />
      ) : null}
    </>
  );
}
