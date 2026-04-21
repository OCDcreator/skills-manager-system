# CLI Project Commands

> **Source**: `src-tauri/src/cli/commands/projects.rs`
> **Status**: [REVIEW]

## Overview

Implements headless project assignment listing, mutation, removal, and full-snapshot apply.

## Import Relationships

```text
Upstream: src-tauri/src/cli/mod.rs
Downstream: src-tauri/src/app_runtime/*, src-tauri/src/core/projects/*
```

## Public Surface

| Export | Purpose |
|---|---|
| `run` | Dispatches one parsed project subcommand. |
| `apply_with_system_dirs` | Testable apply adapter with injected host dirs. |

## Core Logic

Project config mutations acquire the config lock and delegate to `ProjectConfigStore`. `projects apply` intentionally has no path argument and applies the complete stored assignment snapshot.

## Interactions

Apply results with unmanaged conflicts become `partial` responses; invalid paths and missing assignments map to stable CLI errors.

