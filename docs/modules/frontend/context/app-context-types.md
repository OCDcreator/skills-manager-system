# App Context Types

> **Source**: `src/context/app-context-types.ts`
> **Status**: [REVIEW]

## Overview

Defines the consumer-facing `AppContext` contract separately from the provider implementation.

## Public Surface

| Export | Purpose |
|---|---|
| `AppView` / `NavigationGuard` / `PendingNavigation` | Re-exported top-level navigation types. |
| `AppContextValue` | Full shared frontend state and action surface exposed by `useAppContext()`. |

## Core Logic

`AppContextValue` now includes `externalSources`, loading and mutation flags for source/import actions, plus CRUD-style methods for add/fetch/import/update/remove/repair flows. The add-source action uses the shared `AddExternalSourceInput` DTO so branch and repository subpath stay typed from the form down to the Tauri command, while import actions accept `ExternalVariantKey` so neutral skill-repository variants can be imported without pretending to be an installed agent. The interface remains the source of truth for cross-view state shared by `SkillsView`, `AgentsView`, `ExternalSourcesView`, and the shell.

## Interactions

Must stay aligned with `src/context/AppContext.tsx`, the Tauri DTOs in `src/lib/tauri.ts`, and any top-level page that consumes the new external-source actions.
