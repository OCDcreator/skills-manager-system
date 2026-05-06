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

The provider loads the saved repo path, refreshes agents, skills, agent order, and external sources, and centralizes all async mutation flags and error handling. It keeps the shared skill-loading flag active across saved-path initialization until the first skill scan finishes, so the skill browser does not briefly render the initial empty scan snapshot as a real empty result. When a repo path exists, `refreshSkills()` first asks Tauri for the previous cached skill snapshot and hydrates the list immediately when one is available, then runs the incremental scan to refresh changed entries in the background. For the external-sources slice it exposes source-list refresh, source add/fetch/remove mutations, and variant import plus import update/repair mutations. Adding a source now accepts the structured URL/branch/subpath input and leaves validation to the Rust command, preserving the older default-branch behavior when optional fields are empty. Import-changing operations deliberately refresh both external-source snapshots and skill summaries so managed-source badges and detail metadata stay current. When the same skill remains selected after a refresh, `refreshSkills()` now also reloads the selected document so the Skills detail pane does not keep stale markdown beside fresh managed-source metadata.

## Interactions

Must stay aligned with `src/lib/tauri.ts`, `src/context/app-context-types.ts`, and `src/context/navigation-guard.ts`. `refreshSkills()` intentionally uses the managed-source-aware scanner so GitHub mirror metadata becomes visible after import/update/repair operations.
