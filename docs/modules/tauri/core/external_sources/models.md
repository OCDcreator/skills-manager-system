# External Source Models

> **Source**: `src-tauri/src/core/external_sources/models.rs`
> **Status**: [REVIEW]

## Overview

Defines the persisted schema for external-source records, managed import records, manifests, and warnings.

## Public Surface

| Export | Purpose |
|---|---|
| `ExternalSourcesSnapshot` | Whole persisted snapshot for `external-sources.json`. |
| `ExternalSourceRecord` | One tracked source repository record. |
| `ImportedExternalSkillRecord` | One managed mirror import record. |
| `ManagedSkillMirrorManifest` | Sidecar manifest written into each managed mirror directory. |
| `ExternalSourceWarning` | Serializable warning/error entry used across source and import flows. |

## Core Logic

The snapshot schema currently keeps two top-level collections: `sources` and `imports`. The mirror manifest intentionally repeats enough metadata to validate a live mirror without needing the source record to still exist.

## Interactions

Used by `store.rs`, `service.rs`, `imports.rs`, and the skill scanner. Field-name drift here must be reflected in TypeScript DTOs and managed-mirror integrity checks.
