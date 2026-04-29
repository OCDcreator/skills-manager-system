# Tauri Library Entrypoint

> **Source**: `src-tauri/src/lib.rs`
> **Status**: [REVIEW]

## Overview

Defines the shared Rust crate surface for both desktop and CLI builds and registers all desktop Tauri commands when the `desktop` feature is enabled.

## Public Surface

| Export | Purpose |
|---|---|
| `app_runtime` | Shared runtime helpers for CLI and desktop code. |
| `cli` | CLI surface behind the `cli` feature. |
| `core` | Shared backend business domains. |
| `run` | Desktop Tauri startup entrypoint behind the `desktop` feature. |

## Core Logic

The desktop `run()` function now registers the full external-source command set alongside existing agent, scene, project, git, settings, and skills handlers, and it also wires both the dialog and opener plugins into the desktop runtime. That keeps the dedicated source-management surface fully first-class while allowing the frontend to jump directly from a source card to the upstream GitHub page.

## Interactions

Must stay aligned with `src-tauri/src/commands/mod.rs`, the feature declarations in `src-tauri/src/Cargo.toml`, the desktop capability permissions, and any new command or plugin added to `tauri::generate_handler!` / `Builder`.
