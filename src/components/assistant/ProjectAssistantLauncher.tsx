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
      <button
        className="fixed bottom-6 right-6 z-40 flex h-14 w-14 items-center justify-center rounded-full border border-sky-400/40 bg-slate-900 text-sky-300 shadow-2xl shadow-sky-950/40 transition hover:border-sky-300 hover:text-sky-100"
        onClick={() => setIsOpen((current) => !current)}
        title={t("assistant.launcherLabel")}
      >
        {isOpen ? <X className="h-6 w-6" /> : <Bot className="h-6 w-6" />}
      </button>

      {!isOpen ? (
        <div className="fixed bottom-24 right-6 z-30 rounded-full bg-slate-900/95 px-3 py-2 text-xs text-slate-300 shadow-lg shadow-slate-950/40">
          <MessageSquarePlus className="mr-2 inline h-4 w-4" />
          {t("assistant.launcherHint")}
        </div>
      ) : null}

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
