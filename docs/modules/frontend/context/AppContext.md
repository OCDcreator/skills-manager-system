# AppContext

> **Source**: `src/context/AppContext.tsx`
> **Status**: [REVIEW]

## Overview

Owns the frontend global state for active view, configured repository path, skill scan results, selected skill document, repo-scoped disabled skill IDs, agent inventory, manual apply results, loading flags, shared error messages, and guarded-navigation actions.

## Public Surface

| Export | Purpose |
|---|---|
| `AppProvider` | Context provider that initializes settings and shared state. |
| `useAppContext` | Consumer hook that enforces provider presence. |

## Core Logic

The provider loads repo path on mount, defers agent/skill refresh calls through microtasks, persists skill toggles and saved agent draft payloads, exposes scoped or global `applyAgentSync(syncMode?, agentKey?)`, and delegates unsaved-navigation state to `navigation-guard.ts`.

## Interactions

Must stay aligned with command names and TypeScript response types in `src/lib/tauri.ts`, plus consumers in `src/views/SkillsView.tsx`, `src/views/AgentsView.tsx`, `src/views/ScenesView.tsx`, and `src/components/AppShell.tsx`.
