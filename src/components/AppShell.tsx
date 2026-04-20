import type { PropsWithChildren } from "react";
import { useTranslation } from "react-i18next";
import { useAppContext, type AppView } from "../context/AppContext";

const NAV_ITEMS: AppView[] = ["skills", "agents", "settings"];

export function AppShell({ children }: PropsWithChildren) {
  const { t } = useTranslation();
  const { activeView, errorMessage, setActiveView } = useAppContext();

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100">
      <header className="border-b border-slate-800 bg-slate-900/90 backdrop-blur">
        <div className="mx-auto flex max-w-7xl items-center justify-between px-6 py-4">
          <div>
            <h1 className="text-2xl font-semibold">{t("app.title")}</h1>
            <p className="text-sm text-slate-400">{t("app.subtitle")}</p>
          </div>
          <nav className="flex gap-2">
            {NAV_ITEMS.map((view) => (
              <button
                key={view}
                className={`rounded-lg px-4 py-2 text-sm ${
                  activeView === view
                    ? "bg-sky-400 text-slate-950"
                    : "bg-slate-800 text-slate-200"
                }`}
                onClick={() => setActiveView(view)}
              >
                {t(`nav.${view}`)}
              </button>
            ))}
          </nav>
        </div>
        {errorMessage ? (
          <div className="border-t border-rose-900/50 bg-rose-950/60 px-6 py-3 text-sm text-rose-200">
            {errorMessage}
          </div>
        ) : null}
      </header>

      <main className="mx-auto max-w-7xl px-6 py-8">{children}</main>
    </div>
  );
}
