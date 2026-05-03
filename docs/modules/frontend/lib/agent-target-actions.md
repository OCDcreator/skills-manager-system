# Agent Target Actions

> **Source**: `src/lib/agent-target-actions.ts`
> **Status**: [REVIEW]

## Overview

Provides the Agent Sync view hook for explicit per-entry target-directory actions.

## Public Surface

| Export | Purpose |
|---|---|
| `useAgentTargetActions` | Owns confirmation dialogs, in-flight action state, success notices, and Tauri calls for single-entry and batch take-over / delete / import target-entry actions. |

## Core Logic

The hook centralizes user-confirmed target management. It never performs implicit mutation: each handler opens a warning confirmation, calls the matching Tauri command only after acceptance, refreshes agent inventory, refreshes skills after imports, and reports action-specific success or error state back to the caller. Batch handlers confirm once, run selected entries sequentially, and refresh inventory once at the end so large unmanaged target lists do not trigger repeated reloads.

## Interactions

Keep aligned with `src/lib/tauri.ts`, `src/views/AgentsView.tsx`, and the `agents.globalSkills.*` / `tooltip.agents.*` i18n keys.
