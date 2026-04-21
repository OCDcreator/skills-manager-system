# AppContext

> **Source**: `src/context/AppContext.tsx`
> **Status**: [REVIEW]

## Overview

Owns the frontend global state for active view, configured repository path, skill scan results, selected skill document, repo-scoped disabled skill IDs, agent inventory, manual apply results, loading flags, and shared error messages.

## Public Surface

| Export | Purpose |
|---|---|
| `AppView` | Union of supported top-level views. |
| `AppProvider` | Context provider that initializes settings and shared state. |
| `useAppContext` | Consumer hook that enforces provider presence. |

## Core Logic

The provider loads repo path and agent inventory on mount, refreshes skills when the repo changes, persists skill toggles and agent target updates, and exposes `applyAgentSync(syncMode?)` so callers can trigger manual sync with an explicit `copy`/`symlink` mode while keeping the command wiring centralized.

## Interactions

Must stay aligned with command names and TypeScript response types in `src/lib/tauri.ts`, plus consumers in `src/views/SkillsView.tsx`, `src/views/AgentsView.tsx`, and `src/views/ScenesView.tsx`.
