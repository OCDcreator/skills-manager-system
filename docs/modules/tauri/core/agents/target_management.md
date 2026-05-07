# Agent Target Management

> **Source**: `src-tauri/src/core/agents/target_management.rs`
> **Status**: [REVIEW]

## Overview

Owns explicit user-confirmed management actions for entries already present in an agent's global skills directory.

## Import Relationships

```text
Upstream: src-tauri/src/commands/agent_targets.rs
Downstream: std::fs, serde, src-tauri/src/core/agents/target_manifest.rs, src-tauri/src/core/agents/target_sync.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `ImportTargetSkillResult` | Returns the repo-relative and absolute import destination plus whether the original target entry was deleted. |
| `take_over_unmanaged_target_skill` | Marks an existing unmanaged target entry as a preserved app-managed entry without replacing its files. |
| `delete_target_skill_entry` | Deletes exactly one requested target entry and removes any matching manifest ownership. |
| `import_unmanaged_target_skill` | Copies an unmanaged target entry into `custom/<name>` in the configured my-skills repo and optionally deletes the original target entry afterward. |

## Core Logic

Validates that callers only operate on single target-directory entry names, reuses the shared manifest helpers from `target_manifest`, and keeps every action explicit. Take-over entries are stored as preserved manifest entries so later syncs refuse to overwrite them implicitly. Imports create a collision-safe folder under `custom/`, copy the directory recursively while skipping nested `.git`, and delete the original only when the caller asks for that follow-up step.

## Interactions

Keep aligned with `target_manifest.rs` manifest semantics, `target_sync.rs` overwrite protection, the Agent Sync UI confirmations in `src/views/AgentsView.tsx`, and the import destination rules expected by `src-tauri/src/core/skills/scan.rs`.

## Current Note

The current branch change for this module is formatting-only; runtime behavior is unchanged.
