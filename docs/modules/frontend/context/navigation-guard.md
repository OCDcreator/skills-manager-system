# Navigation Guard

> **Source**: `src/context/navigation-guard.ts`
> **Status**: [REVIEW]

## Overview

Owns guarded top-level navigation for views with unsaved local drafts.

## Public Surface

| Export | Purpose |
|---|---|
| `AppView` | Shared union of top-level desktop routes. |
| `NavigationGuard` | View-local dirty/save/discard contract. |
| `PendingNavigation` | Pending route transition captured for the shell dialog. |
| `useNavigationGuardState` | Hook that intercepts route changes and exposes confirm/discard/cancel actions. |

## Core Logic

`AppView` now includes the dedicated `sources` page. The hook only blocks navigation when the currently active view registered itself as guard owner and reports dirty state; otherwise it forwards the change immediately.

## Interactions

Used by `AppContext` and rendered by `AppShell`. Keep the route union synchronized with the view switch in `src/App.tsx`.
