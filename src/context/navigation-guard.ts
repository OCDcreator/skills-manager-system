import { useCallback, useMemo, useState } from "react";

export type AppView = "skills" | "agents" | "git" | "scenes" | "projects" | "sources" | "settings";

export interface NavigationGuard {
  view: AppView;
  isDirty: () => boolean;
  save: () => Promise<void>;
  discard: () => void;
}

export interface PendingNavigation {
  sourceView: AppView;
  targetView: AppView;
}

export function useNavigationGuardState(
  activeView: AppView,
  setActiveViewState: (view: AppView) => void,
) {
  const [guard, setGuard] = useState<NavigationGuard | null>(null);
  const [pendingTargetView, setPendingTargetView] = useState<AppView | null>(null);

  const registerNavigationGuard = useCallback((nextGuard: NavigationGuard) => {
    setGuard(nextGuard);
    return () => {
      setGuard((current) => (current === nextGuard ? null : current));
    };
  }, []);

  const setActiveView = useCallback(
    (nextView: AppView) => {
      if (nextView === activeView) {
        return;
      }

      if (guard?.view === activeView && guard.isDirty()) {
        setPendingTargetView(nextView);
        return;
      }

      setActiveViewState(nextView);
    },
    [activeView, guard, setActiveViewState],
  );

  const confirmNavigationSave = useCallback(async () => {
    if (!pendingTargetView || !guard) {
      return;
    }

    const nextView = pendingTargetView;
    await guard.save();
    setPendingTargetView(null);
    setActiveViewState(nextView);
  }, [guard, pendingTargetView, setActiveViewState]);

  const confirmNavigationDiscard = useCallback(() => {
    if (!pendingTargetView || !guard) {
      return;
    }

    const nextView = pendingTargetView;
    guard.discard();
    setPendingTargetView(null);
    setActiveViewState(nextView);
  }, [guard, pendingTargetView, setActiveViewState]);

  const cancelNavigation = useCallback(() => {
    setPendingTargetView(null);
  }, []);

  const pendingNavigation = useMemo<PendingNavigation | null>(
    () =>
      pendingTargetView
        ? {
            sourceView: activeView,
            targetView: pendingTargetView,
          }
        : null,
    [activeView, pendingTargetView],
  );

  return {
    cancelNavigation,
    confirmNavigationDiscard,
    confirmNavigationSave,
    pendingNavigation,
    registerNavigationGuard,
    setActiveView,
  };
}
