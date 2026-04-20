# Tauri Library Entrypoint

> **Source**: `src-tauri/src/lib.rs`
> **Status**: [REVIEW]

## Overview

Builds and runs the Tauri application, registers plugins, and exposes command handlers.

## Import Relationships

```text
Upstream: src-tauri/src/main.rs
Downstream: src-tauri/src/commands/mod.rs, src-tauri/src/core/mod.rs, tauri_plugin_dialog
```

## Public Surface

| Export | Purpose |
|---|---|
| `run` | Starts the Tauri builder and application runtime. |

## Core Logic

The function initializes the dialog plugin, registers settings commands plus skill scan/document/state commands and phase-three agent inventory/config/apply commands with `tauri::generate_handler!`, runs the generated Tauri context, and panics with a fixed message if runtime startup fails.

## Data Flow

Command handlers bridge frontend invocations into Rust command modules.

## Interactions

Must stay aligned with every `#[tauri::command]` wrapper added under `src-tauri/src/commands/`, including the phase-two skill-state commands and the phase-three agent-sync commands.

## Configuration

Uses `tauri::generate_context!()` and the mobile entrypoint attribute when building for mobile.

## Change Notes

When adding commands, register them here after implementing the Rust command and TypeScript wrapper.
