# External Source Import Paths

> **Source**: `src-tauri/src/core/external_sources/import_paths.rs`
> **Status**: [REVIEW]

## Overview

Owns deterministic path and id derivation for managed external mirrors.

## Public Surface

| Export | Purpose |
|---|---|
| `determine_mirror_relative_path` | Chooses a collision-safe mirror path under `external/managed/github/...`. |
| `stable_import_id` | Builds the persistent import id from source, agent, and mirror path. |
| `resolve_cached_repo_path` | Resolves and validates the cached repo directory stored in the source record. |

## Core Logic

This helper layer converts normalized GitHub repo URLs into stable repo slugs, sanitizes path segments for cross-platform safety, rejects overlong or colliding managed paths, and keeps existing imports pinned to their current mirror path. It also guards against bad cached-repo pointers before import work reaches git export or filesystem swap logic.

## Interactions

Used by `imports.rs`. Keep its slug and path rules aligned with `skills::identity`, `hash.rs`, and the managed mirror layout expected by scanner enrichment and manifests.
