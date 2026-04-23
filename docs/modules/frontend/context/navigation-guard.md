# Navigation Guard

> **Source**: `src/context/navigation-guard.ts`
> **Status**: [REVIEW]

## Overview

Owns guarded top-level navigation for unsaved view-local draft state.

## Public Surface

| Export | Purpose |
|---|---|
| `AppView` | Shared union for top-level navigation destinations. |
| `NavigationGuard` | View-local contract for dirty-check, save, and discard hooks. |
| `PendingNavigation` | Pending source/target view pair for the shell dialog. |
| `useNavigationGuardState` | Hook that intercepts view changes and exposes confirm/discard/cancel actions. |

## Core Logic

Stores the active guard registration, blocks `setActiveView` when the current guarded view is dirty, records a pending target view, and only changes views after save or discard succeeds.
