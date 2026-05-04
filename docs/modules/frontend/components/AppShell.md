# AppShell

> **Source**: `src/components/AppShell.tsx`
> **Status**: [REVIEW]

## Overview

Provides the shared desktop workbench chrome: left navigation rail, compact mobile navigation header, global error banner, main content frame, project-assistant launcher, and pending-navigation dialog host.

## Public Surface

| Export | Purpose |
|---|---|
| `AppShell` | Layout wrapper that accepts page children plus an optional width override. |

## Core Logic

The shell reads `activeView`, guarded navigation actions, and `pendingNavigation` from `AppContext`. It owns one icon-backed nav configuration for all seven top-level views, renders it as a desktop left rail and as a horizontal mobile nav through CSS-controlled layout classes, gives those two nav landmarks distinct accessible names, forwards all route changes through guarded `setActiveView`, and mounts `UnsavedChangesDialog` whenever the current view registered dirty-state protection.

`contentWidthClassName` is still applied to the main workbench content region so wide surfaces can opt out of the default `max-w-7xl` frame without changing navigation chrome.

## Interactions

Must stay aligned with `AppView` in `src/context/navigation-guard.ts`, the active-view switch in `src/App.tsx`, and i18n keys under `nav.*` and `tooltip.nav.*`. Shell layout and palette primitives are intentionally centralized in `src/styles/foundation.css` so the component does not grow large Tailwind class strings for routine chrome.
