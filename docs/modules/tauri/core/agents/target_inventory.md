# Agent Target Inventory

> **Source**: `src-tauri/src/core/agents/target_inventory.rs`
> **Status**: [REVIEW]

## Overview

Scans an agent's effective global skills directory and reports both app-managed and existing unmanaged entries without mutating the target directory.

## Import Relationships

```text
Upstream: src-tauri/src/core/agents/discovery.rs
Downstream: std::fs, serde, src-tauri/src/core/agents/target_sync.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `AgentTargetSkillEntryKind` | Serialized entry type for directory, symlink, file, or other target entries. |
| `AgentTargetSkillEntry` | Frontend-facing target skill row with managed/unmanaged metadata. |
| `scan_target_skill_entries` | Lists target-directory skill entries and classifies app-managed entries via the target manifest. |

## Core Logic

Loads the target manifest through `target_sync`, walks the target directory, skips the app manifest file, and includes managed entries, entries with `SKILL.md`, and visible directory/symlink entries. Unmanaged entries are treated as protected inventory: this module never deletes, imports, or rewrites them.

## Interactions

Keep aligned with `.skills-manager-system-manifest.json` semantics in `target_sync.rs` and the `AgentInventoryItem.targetSkillEntries` fields consumed by the Agent Sync UI.
