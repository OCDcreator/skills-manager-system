# Agent Target Manifest

> **Source**: `src-tauri/src/core/agents/target_manifest.rs`
> **Status**: [REVIEW]

## Overview

Owns serialization and validation for the target-local `.skills-manager-system-manifest.json` file.

## Import Relationships

```text
Upstream: src-tauri/src/core/agents/target_sync.rs, src-tauri/src/core/agents/target_management.rs, src-tauri/src/core/agents/target_inventory.rs
Downstream: anyhow, serde_json, std::fs
```

## Public Surface

| Export | Purpose |
|---|---|
| `AgentTargetManifest` | In-memory manifest entries scoped to the already-validated agent key. |
| `AgentTargetManifestEntry` | Stored ownership metadata for a target entry, including preserved take-over state. |
| `load_manifest` / `save_manifest` / `save_or_remove_manifest` / `remove_manifest` | Safe target-manifest lifecycle helpers. |
| `is_manifest_file_name` | Shared predicate for hiding and protecting the app manifest file. |

## Core Logic

The module keeps `appId` and `agentKey` validation at the persistence boundary while exposing only the validated entry map to callers. Existing manifests without `preserveExisting` continue to deserialize as normal synced entries because the field defaults to `false`.

## Interactions

Keep aligned with `target_sync.rs` reconciliation rules, `target_management.rs` explicit actions, and `target_inventory.rs` scan output.
