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

The desktop `run()` function registers the full command set for agents, scenes,
projects, git, settings, skills, external sources, and the assistant terminal
runtime. It also wires both the dialog and opener plugins into the desktop
runtime, bootstraps the assistant terminal config workspace on startup, and
manages a shared `core::terminal::session::TerminalState` so the floating
assistant can launch, drain, resize, and stop a single PTY-backed session.
The skills command registration includes both `load_cached_skills` for immediate
startup hydration and `scan_skills` for the subsequent incremental refresh.

## Interactions

Must stay aligned with `src-tauri/src/commands/mod.rs`, the feature declarations in `src-tauri/src/Cargo.toml`, the desktop capability permissions, and any new command or plugin added to `tauri::generate_handler!` / `Builder`.
