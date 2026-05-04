import type { PropsWithChildren } from "react";
import { useTranslation } from "react-i18next";
import {
  Bot,
  Boxes,
  FolderKanban,
  GitBranch,
  Library,
  Settings,
  Sparkles,
  type LucideIcon,
} from "lucide-react";
import { useAppContext, type AppView } from "../context/AppContext";
import { ProjectAssistantLauncher } from "./assistant/ProjectAssistantLauncher";
import { UnsavedChangesDialog } from "./UnsavedChangesDialog";

interface NavItem {
  icon: LucideIcon;
  view: AppView;
}

const NAV_ITEMS: NavItem[] = [
  { view: "skills", icon: Library },
  { view: "agents", icon: Bot },
  { view: "git", icon: GitBranch },
  { view: "scenes", icon: Sparkles },
  { view: "projects", icon: FolderKanban },
  { view: "sources", icon: Boxes },
  { view: "settings", icon: Settings },
];
const BRAND_MARK_TEXT = "SM";

interface AppShellProps extends PropsWithChildren {
  contentWidthClassName?: string;
}

export function AppShell({ children, contentWidthClassName }: AppShellProps) {
  const { t } = useTranslation();
  const {
    activeView,
    cancelNavigation,
    confirmNavigationDiscard,
    confirmNavigationSave,
    errorMessage,
    pendingNavigation,
    setActiveView,
  } = useAppContext();
  const widthClassName = contentWidthClassName ?? "max-w-7xl";
  const appTitle = t("app.title");
  const desktopNavLabel = `${appTitle} desktop navigation`;
  const mobileNavLabel = `${appTitle} mobile navigation`;

  const renderNavItem = (item: NavItem) => {
    const Icon = item.icon;
    const isActive = activeView === item.view;

    return (
      <button
        key={item.view}
        aria-current={isActive ? "page" : undefined}
        className="app-shell__nav-item"
        data-active={isActive ? "true" : "false"}
        onClick={() => setActiveView(item.view)}
        title={t(`tooltip.nav.${item.view}`)}
      >
        <Icon className="app-shell__nav-icon" aria-hidden="true" />
        <span>{t(`nav.${item.view}`)}</span>
      </button>
    );
  };

  return (
    <div className="app-shell">
      <aside className="app-shell__sidebar" data-app-shell-sidebar>
        <div className="app-shell__brand">
          <div className="app-shell__brand-mark">{BRAND_MARK_TEXT}</div>
          <div>
            <h1 className="app-shell__title">{appTitle}</h1>
            <p className="app-shell__subtitle">{t("app.subtitle")}</p>
          </div>
        </div>
        <nav className="app-shell__nav" aria-label={desktopNavLabel}>
          {NAV_ITEMS.map(renderNavItem)}
        </nav>
      </aside>

      <div className="app-shell__workspace">
        <header className="app-shell__mobile-header" data-app-shell-mobile-header>
          <div className="app-shell__brand app-shell__brand--mobile">
            <div className="app-shell__brand-mark">{BRAND_MARK_TEXT}</div>
            <div>
              <h1 className="app-shell__title">{appTitle}</h1>
              <p className="app-shell__subtitle">{t("app.subtitle")}</p>
            </div>
          </div>
          <nav className="app-shell__mobile-nav" aria-label={mobileNavLabel}>
            {NAV_ITEMS.map(renderNavItem)}
          </nav>
        </header>

        {errorMessage ? (
          <div className="app-shell__error" role="alert">
            {errorMessage}
          </div>
        ) : null}

        <main
          className={`app-shell__content mx-auto ${widthClassName}`}
          data-app-shell-main
        >
          {children}
        </main>
      </div>

      <ProjectAssistantLauncher />

      {pendingNavigation ? (
        <UnsavedChangesDialog
          onCancel={cancelNavigation}
          onDiscard={confirmNavigationDiscard}
          onSave={confirmNavigationSave}
          pendingNavigation={pendingNavigation}
        />
      ) : null}
    </div>
  );
}
