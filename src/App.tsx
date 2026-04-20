import { AppShell } from "./components/AppShell";
import { AppProvider, useAppContext } from "./context/AppContext";
import { AgentsView } from "./views/AgentsView";
import { GitView } from "./views/GitView";
import { ProjectsView } from "./views/ProjectsView";
import { ScenesView } from "./views/ScenesView";
import { SettingsView } from "./views/SettingsView";
import { SkillsView } from "./views/SkillsView";

function AppBody() {
  const { activeView } = useAppContext();

  const view =
    activeView === "skills" ? (
      <SkillsView />
    ) : activeView === "agents" ? (
      <AgentsView />
    ) : activeView === "git" ? (
      <GitView />
    ) : activeView === "scenes" ? (
      <ScenesView />
    ) : activeView === "projects" ? (
      <ProjectsView />
    ) : (
      <SettingsView />
    );

  return <AppShell>{view}</AppShell>;
}

export default function App() {
  return (
    <AppProvider>
      <AppBody />
    </AppProvider>
  );
}
