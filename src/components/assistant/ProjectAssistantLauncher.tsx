import { Bot, MessageSquarePlus, X } from "lucide-react";
import { Suspense, lazy, useState } from "react";
import { useTranslation } from "react-i18next";

const ProjectAssistantPanel = lazy(() =>
  import("./ProjectAssistantPanel").then((module) => ({
    default: module.ProjectAssistantPanel,
  })),
);

export function ProjectAssistantLauncher() {
  const { t } = useTranslation();
  const [isOpen, setIsOpen] = useState(false);

  return (
    <>
      <div
        className="fixed bottom-6 right-6 z-40 group max-[1279px]:bottom-4 max-[1279px]:right-4"
        data-project-assistant-launcher
      >
        {!isOpen ? (
          <div className="pointer-events-none absolute bottom-full right-0 mb-3 flex translate-y-1 items-center rounded-full bg-slate-900/95 px-3 py-2 text-xs text-slate-300 opacity-0 shadow-lg shadow-slate-950/40 transition duration-200 group-hover:translate-y-0 group-hover:opacity-100 group-focus-within:translate-y-0 group-focus-within:opacity-100">
            <MessageSquarePlus className="mr-2 inline h-4 w-4" />
            {t("assistant.launcherHint")}
          </div>
        ) : null}

        <button
          className="relative flex h-14 w-14 items-center justify-center rounded-full border border-sky-400/40 bg-slate-900 text-sky-300 shadow-2xl shadow-sky-950/40 transition hover:border-sky-300 hover:text-sky-100 max-[1279px]:h-12 max-[1279px]:w-12"
          onClick={() => setIsOpen((current) => !current)}
          title={t("assistant.launcherLabel")}
          type="button"
        >
          {isOpen ? <X className="h-6 w-6" /> : <Bot className="h-6 w-6" />}
        </button>
      </div>

      {isOpen ? (
        <Suspense fallback={null}>
          <ProjectAssistantPanel onClose={() => setIsOpen(false)} />
        </Suspense>
      ) : null}
    </>
  );
}
