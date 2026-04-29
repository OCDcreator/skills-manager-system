import { Bot, MessageSquarePlus, X } from "lucide-react";
import { Suspense, lazy, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import {
  getAssistantContextStatus,
  type AssistantContextStatus,
} from "../../lib/assistant";

const ProjectAssistantPanel = lazy(() =>
  import("./ProjectAssistantPanel").then((module) => ({
    default: module.ProjectAssistantPanel,
  })),
);

function AssistantPanelFallback({
  onClose,
  title,
  subtitle,
}: {
  onClose: () => void;
  subtitle: string;
  title: string;
}) {
  return (
    <section className="fixed bottom-24 right-6 z-50 flex h-[min(760px,calc(100vh-8rem))] w-[min(860px,calc(100vw-3rem))] flex-col overflow-hidden rounded-[28px] border border-sky-400/30 bg-slate-950/95 shadow-2xl shadow-slate-950/60 backdrop-blur">
      <header className="flex items-center justify-between border-b border-slate-800 px-5 py-4">
        <div>
          <h2 className="text-base font-semibold text-slate-100">{title}</h2>
          <p className="text-sm text-slate-400">{subtitle}</p>
        </div>
        <button
          aria-label={title}
          className="rounded-full p-2 text-slate-400 hover:bg-slate-900 hover:text-slate-100"
          onClick={onClose}
          type="button"
        >
          <X className="h-5 w-5" />
        </button>
      </header>
      <div className="grid flex-1 place-items-center px-5 py-6 text-sm text-slate-400">
        Loading assistant...
      </div>
    </section>
  );
}

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
        <Suspense
          fallback={(
            <AssistantPanelFallback
              onClose={() => setIsOpen(false)}
              subtitle={t("assistant.subtitle")}
              title={t("assistant.title")}
            />
          )}
        >
          <ProjectAssistantPanel
            isLoadingStatus={isLoadingStatus}
            onClose={() => setIsOpen(false)}
            status={status}
            statusError={statusError}
          />
        </Suspense>
      ) : null}
    </>
  );
}
