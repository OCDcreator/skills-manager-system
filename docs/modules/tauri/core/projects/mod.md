# Projects Core Module

> **Source**: `src-tauri/src/core/projects/mod.rs`
> **Status**: [DRAFT]

## Overview

Barrel module for the projects subsystem, exporting persistence, sync, shared path helpers, and read-only inspection.

## Public Surface

| Export | Purpose |
|---|---|
| `apply_status` | Project apply freshness calculation from ledger hashes |
| `path_inspection` | Read-only draft-path inspection logic |
| `project_paths` | Shared project-path normalization helpers |
| `store` | Project assignment config persistence |
| `sync` | Project-local skill deployment logic |
| `sync_ledger` | Project sync ledger persistence and cleanup helpers |
