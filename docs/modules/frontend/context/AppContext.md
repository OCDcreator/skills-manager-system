# AppContext

> **Source**: `src/context/AppContext.tsx`
> **Status**: [REVIEW]

## Overview

Owns the frontend global state for active view, configured repository path, skill scan results, selected skill document, repo-scoped disabled skill IDs, phase-three agent inventory, apply results, loading flags, and global error messages.

## Import Relationships

```text
Upstream: src/App.tsx, shared consumers through useAppContext()
Downstream: src/lib/tauri.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `AppView` | Union of supported top-level views. |
| `AppProvider` | Context provider that initializes settings and skill state. |
| `useAppContext` | Consumer hook that enforces provider presence. |

## Core Logic

The provider loads the saved repo path and agent inventory on mount, refreshes skills when the path changes, joins scan results with persisted disabled IDs, saves repository path changes through Tauri, persists row-level skill toggles, manages per-agent enabled/override updates, executes manual agent sync, and loads a skill document when selection changes. It clears stale selection, document, disabled-ID state, and latest apply result when repository changes invalidate the current browser state.

## Data Flow

Tauri API calls populate context state. Views and components consume that state and call context actions for refresh, path save, view changes, skill selection, skill enable/disable updates, agent target updates, and manual apply.

## Interactions

Must stay aligned with command names and TypeScript response types in `src/lib/tauri.ts`, plus view routing in `src/App.tsx` and consumers in both `src/views/SkillsView.tsx` and `src/views/AgentsView.tsx`.

## Configuration

Initial active view is `skills`. No environment configuration is read here.

## Change Notes

Keep heavy Git/filesystem/database logic out of this context; add it behind Tauri APIs or domain services and keep this file as orchestration only.
