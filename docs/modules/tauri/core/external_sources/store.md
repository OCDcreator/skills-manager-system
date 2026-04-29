# External Source Store

> **Source**: `src-tauri/src/core/external_sources/store.rs`
> **Status**: [REVIEW]

## Overview

Persists `external-sources.json` under the app config directory.

## Public Surface

| Export | Purpose |
|---|---|
| `ExternalSourcesStore` | Load/save wrapper for the external-source snapshot file. |

## Core Logic

`load()` returns an empty schema when the file is missing. `save()` requires a caller-owned `ConfigLockGuard`, serializes the snapshot with pretty JSON, and writes it through `write_text_atomic()` so persistence uses lock plus atomic write-rename discipline instead of plain overwrite.

## Interactions

Used by the external-source service, import lifecycle, and scanner enrichment path. Lock ownership intentionally stays with callers so multi-step operations can hold one config lock across validation and persistence.
