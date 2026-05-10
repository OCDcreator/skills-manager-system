import { Suspense, lazy } from "react";
import { useTranslation } from "react-i18next";
import { AppShell } from "./components/AppShell";
import { AppProvider, useAppContext } from "./context/AppContext";
import { SkillsView } from "./views/SkillsView";

const AgentsView = lazy(() =>
  import("./views/AgentsView").then((module) => ({ default: module.AgentsView })),
);
const ExternalSourcesView = lazy(() =>
  import("./views/ExternalSourcesView").then((module) => ({
    default: module.ExternalSourcesView,
  })),
);
const GitView = lazy(() =>
  import("./views/GitView").then((module) => ({ default: module.GitView })),
);
const ProjectsView = lazy(() =>
  import("./views/ProjectsView").then((module) => ({ default: module.ProjectsView })),
);
const ScenesView = lazy(() =>
  import("./views/ScenesView").then((module) => ({ default: module.ScenesView })),
);
const SettingsView = lazy(() =>
  import("./views/SettingsView").then((module) => ({ default: module.SettingsView })),
);

function ViewLoadingFallback() {
  const { t } = useTranslation();

  return (
    <section className="rounded-2xl border border-slate-800 bg-slate-900 p-8 text-sm text-slate-400">
      {t("app.loadingView")}
    </section>
  );
}

function AppBody() {
  const { activeView } = useAppContext();
  const useWideWorkbenchWidth =
    activeView === "skills" ||
    activeView === "agents" ||
    activeView === "projects" ||
    activeView === "sources";
  const contentWidthClassName = useWideWorkbenchWidth
    ? "max-w-[min(96vw,1800px)]"
    : "max-w-7xl";

  const view = activeView === "skills" ? (
    <SkillsView />
  ) : (
    <Suspense fallback={<ViewLoadingFallback />}>
      {activeView === "agents" ? (
        <AgentsView />
      ) : activeView === "git" ? (
        <GitView />
      ) : activeView === "scenes" ? (
        <ScenesView />
      ) : activeView === "projects" ? (
        <ProjectsView />
      ) : activeView === "sources" ? (
        <ExternalSourcesView />
      ) : (
        <SettingsView />
      )}
    </Suspense>
  );

  return <AppShell contentWidthClassName={contentWidthClassName}>{view}</AppShell>;
}

export default function App() {
  return (
    <AppProvider>
      <AppBody />
    </AppProvider>
  );
}
