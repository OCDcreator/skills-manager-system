import { AppShell } from "./components/AppShell";
import { AppProvider, useAppContext } from "./context/AppContext";
import { SettingsView } from "./views/SettingsView";
import { SkillsView } from "./views/SkillsView";

function AppBody() {
  const { activeView } = useAppContext();

  return <AppShell>{activeView === "skills" ? <SkillsView /> : <SettingsView />}</AppShell>;
}

export default function App() {
  return (
    <AppProvider>
      <AppBody />
    </AppProvider>
  );
}
