# Agent Target Inventory

> **Source**: `src-tauri/src/core/agents/target_inventory.rs`
> **Status**: [REVIEW]

## Overview

Scans an agent's effective global skills directory and reports both app-managed and existing unmanaged entries without mutating the target directory.

## Import Relationships

```text
Upstream: src-tauri/src/core/agents/discovery.rs
Downstream: std::fs, serde, src-tauri/src/core/agents/target_manifest.rs, src-tauri/src/core/agents/target_sync.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `AgentTargetSkillEntryKind` | Serialized entry type for directory, symlink, file, or other target entries. |
| `AgentTargetSkillEntry` | Frontend-facing target skill row with managed/unmanaged metadata, preserved take-over state, and optional symlink-target metadata. |
| `scan_target_skill_entries` | Lists target-directory skill entries and classifies app-managed entries via the target manifest. |

## Core Logic

Loads managed snapshots through `target_sync`, uses `target_manifest` to hide the app manifest file, walks the target directory, and includes managed entries, entries with `SKILL.md`, and visible directory/symlink entries. Managed snapshots distinguish synced entries from preserved take-over entries so the frontend can show that explicit ownership state without mutating files here. When an entry is itself a symlink, the scanner also resolves the link target into a normalized display path so the UI can show where that entry points without taking ownership of it.

## Interactions

Keep aligned with `.skills-manager-system-manifest.json` semantics in `target_manifest.rs`, the reconciliation behavior in `target_sync.rs`, the explicit actions in `target_management.rs`, and the `AgentInventoryItem.targetSkillEntries` fields consumed by the Agent Sync UI.

## Current Note

The current branch change for this module is formatting-only; runtime behavior is unchanged.
