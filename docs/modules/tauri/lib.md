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

The desktop `run()` function now registers the full external-source command set alongside existing agent, scene, project, git, settings, and skills handlers. This keeps the dedicated source-management surface fully first-class in the desktop build while preserving the CLI-only feature split.

## Interactions

Must stay aligned with `src-tauri/src/commands/mod.rs`, the feature declarations in `src-tauri/src/Cargo.toml`, and any new command module added to `tauri::generate_handler!`.
