# Tauri API Wrapper

> **Source**: `src/lib/tauri.ts`
> **Status**: [REVIEW]

## Overview

Defines TypeScript shapes and thin wrappers for the Tauri commands used by the frontend.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentKey` | Union of all supported global/project agent keys. |
| `AgentSyncMode` | Frontend union for `copy` vs `symlink`. |
| `AgentConfigurationInput` | Save payload for one persisted agent configuration. |
| `getAgentSyncMode` / `setAgentSyncMode` | Persist the preferred global sync mode. |
| `setAgentConfiguration` | Persists a full agent draft payload and returns refreshed inventory. |
| `applyAgentSync` | Runs manual agent sync, optionally scoped to one agent and/or an explicit mode override. |
| other exports | Existing repo/skill/agent DTOs and invoke wrappers. |

## Core Logic

Each function delegates directly to `invoke` with the command name and payload, keeping frontend command wiring thin while exposing enough type information for draft editing, scoped sync, and page-level orchestration.

## Interactions

Must stay aligned with `src-tauri/src/commands/settings.rs`, `src-tauri/src/commands/agents.rs`, and the `tauri::generate_handler!` registration in `src-tauri/src/lib.rs`.
