# Agent Target Commands

> **Source**: `src-tauri/src/commands/agent_targets.rs`
> **Status**: [REVIEW]

## Overview

Provides thin Tauri commands for explicit per-entry management inside an agent's global skills directory.

## Public Surface

| Export | Purpose |
|---|---|
| `take_over_agent_target_skill` | Marks one unmanaged target entry as preserved app-managed state. |
| `delete_agent_target_skill` | Deletes exactly one named target entry. |
| `import_agent_target_skill` | Imports one unmanaged target entry into the configured my-skills repository and optionally deletes the original. |

## Core Logic

The module validates the agent key, resolves that agent's effective target directory from the latest inventory snapshot, loads the repo path only for import operations, and then delegates business rules into `core::agents::target_management`. The command layer does not inspect target contents itself.

## Interactions

Must stay aligned with `src/lib/tauri.ts`, the command registration in `src-tauri/src/lib.rs`, and the target-management core module under `src-tauri/src/core/agents/`.

## Current Note

The current branch change for this module is formatting-only; runtime behavior is unchanged.
