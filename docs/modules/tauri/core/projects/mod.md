# Projects Core Module

> **Source**: `src-tauri/src/core/projects/mod.rs`
> **Status**: [DRAFT]

## Overview

Barrel module for the projects subsystem, exporting persistence, sync, shared path helpers, and read-only inspection.

## Public Surface

| Export | Purpose |
|---|---|
| `project_paths` | Shared project-path normalization helpers |
| `path_inspection` | Read-only draft-path inspection logic |
| `store` | Project assignment config persistence |
| `sync` | Project-local skill deployment logic |
