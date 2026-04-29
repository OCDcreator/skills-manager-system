# AppShell

> **Source**: `src/components/AppShell.tsx`
> **Status**: [REVIEW]

## Overview

Provides the shared desktop chrome: header, top-level navigation, global error banner, main content frame, project-assistant launcher, and pending-navigation dialog host.

## Public Surface

| Export | Purpose |
|---|---|
| `AppShell` | Layout wrapper that accepts page children plus an optional width override. |

## Core Logic

The shell reads `activeView`, guarded navigation actions, and `pendingNavigation` from `AppContext`. It renders seven nav buttons now that `sources` is a first-class view, forwards all route changes through guarded `setActiveView`, and mounts `UnsavedChangesDialog` whenever the current view registered dirty-state protection.

## Interactions

Must stay aligned with `AppView` in `src/context/navigation-guard.ts`, the active-view switch in `src/App.tsx`, and i18n keys under `nav.*` and `tooltip.nav.*`.
