# External Source Reference Check

> **Source**: `src-tauri/src/core/external_sources/reference_check.rs`
> **Status**: [REVIEW]

## Overview

Finds cross-domain references to a skill id before destructive managed-mirror removal.

## Public Surface

| Export | Purpose |
|---|---|
| `BlockingReferences` | Lists scene, agent, and project references that block deletion. |
| `find_skill_references` | Loads persisted config and collects all blocking references for one skill id. |

## Core Logic

The module reads scene, agent, and project config stores, then reports any references found in selected or excluded skill lists. `BlockingReferences.has_any()` is the simple gate used by import-removal preflight.

## Interactions

Used by `imports.rs` and transitively by `remove_external_source()`. This module is intentionally read-only and should remain the shared deletion-safety checker instead of duplicating reference scans in multiple domains.
