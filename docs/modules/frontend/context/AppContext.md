# AppContext

> **Source**: `src/context/AppContext.tsx`
> **Status**: [REVIEW]

## Overview

Owns the frontend global state for top-level view selection, repo path, skills, agents, guarded navigation, and the external GitHub source workflow.

## Public Surface

| Export | Purpose |
|---|---|
| `AppProvider` | Context provider that initializes and mutates shared app state. |
| `useAppContext` | Consumer hook that enforces provider presence. |

## Core Logic

The provider loads the saved repo path, refreshes agents, skills, agent order, and external sources, and centralizes all async mutation flags and error handling. For the external-sources slice it exposes source-list refresh, source add/fetch/remove mutations, and variant import plus import update/repair mutations. Import-changing operations deliberately refresh both external-source snapshots and skill summaries so managed-source badges and detail metadata stay current.

## Interactions

Must stay aligned with `src/lib/tauri.ts`, `src/context/app-context-types.ts`, and `src/context/navigation-guard.ts`. `refreshSkills()` intentionally uses the managed-source-aware scanner so GitHub mirror metadata becomes visible after import/update/repair operations.
