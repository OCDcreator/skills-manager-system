# Tauri Library Entrypoint

> **Source**: `src-tauri/src/lib.rs`
> **Status**: [REVIEW]

## Overview

Builds and runs the Tauri application, registers plugins, and exposes the Rust command handlers used by the frontend.

## Core Logic

The handler registration now includes both repo-path settings commands and the new `get_agent_sync_mode` / `set_agent_sync_mode` commands in addition to the existing skills, agents, git, scenes, and projects command surfaces.

## Interactions

Must stay aligned with every `#[tauri::command]` wrapper under `src-tauri/src/commands/`, especially when new settings fields are added and exposed to `src/lib/tauri.ts`.
