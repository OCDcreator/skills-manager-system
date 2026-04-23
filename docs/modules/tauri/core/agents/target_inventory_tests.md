# Agent Target Inventory Tests

> **Source**: `src-tauri/src/core/agents/target_inventory_tests.rs`
> **Status**: [REVIEW]

## Overview

Verifies target inventory scanning for app-managed and unmanaged agent global skills.

## Test Coverage

| Test | Purpose |
|---|---|
| `target_inventory_classifies_managed_and_unmanaged_entries` | Confirms manifest-owned entries are marked managed and existing target skills remain unmanaged. |
| `target_inventory_ignores_manifest_and_keeps_unmanaged_after_cleanup` | Confirms the manifest file is hidden from inventory and unmanaged directories survive managed cleanup. |

## Interactions

Keep aligned with `target_sync_tests.rs` safety behavior and the serialized `AgentTargetSkillEntry` contract used by the frontend.
