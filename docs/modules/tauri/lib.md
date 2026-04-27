# Tauri Library Entrypoint

> **Source**: `src-tauri/src/lib.rs`
> **Status**: [REVIEW]

## Overview

Defines the shared Rust crate surface for both desktop and CLI builds, while keeping the Tauri startup path gated behind the `desktop` Cargo feature.

## Import Relationships

```text
Upstream: src-tauri/src/main.rs, src-tauri/src/cli/main.rs
Downstream: src-tauri/src/app_runtime/*, src-tauri/src/cli/*, src-tauri/src/core/*, src-tauri/src/commands/*
```

## Core Logic

`app_runtime` is always exposed so both build targets can share config/output helpers. The CLI module is only compiled when the `cli` feature is enabled. The Tauri command layer and `run()` function are compiled only with the `desktop` feature, preserving a CLI-only build that no longer pulls Tauri runtime wiring into headless checks.

## Interactions

Must stay aligned with every `#[tauri::command]` wrapper under `src-tauri/src/commands/`, including the assistant context and ask commands, the dedicated agent-target management commands, the settings commands for repo path, sync mode, and global agent ordering, the project-path inspection command for the Projects workbench, the full agent-configuration save/apply flows, the feature declarations in `src-tauri/Cargo.toml`, and the CLI modules under `src-tauri/src/cli/`.
