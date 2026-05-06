# Skills Core Module Boundary

> **Source**: `src-tauri/src/core/skills/mod.rs`
> **Status**: [REVIEW]

## Overview

Declares the skills domain modules used by commands and sibling backend domains.

## Public Surface

| Export | Purpose |
|---|---|
| `cache` | Cached and incremental scan domain. |
| `cache_tests` | Test-only regression coverage for cached scan behavior. |
| `documents` | Skill document reading domain. |
| `identity` | Canonical skill path and id helpers. |
| `managed_scan` | Internal managed-mirror enrichment helper used by the scanner. |
| `metadata` | `SKILL.md` metadata parser. |
| `scan` | Repository scanning plus managed-source enrichment. |
| `state` | Repo-scoped enable/disable persistence. |

## Core Logic

This module remains declarative. The main Task 7 changes are making `identity` explicit so both the skills domain and the external-source domain share the same path/id rules, and splitting managed-mirror enrichment into `managed_scan` so `scan.rs` stays focused on traversal and summary assembly. The cache module is now the owner for persisted scan snapshots and incremental rebuild behavior, keeping startup performance logic out of the command layer.
