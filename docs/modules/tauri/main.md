# Tauri Main

> **Source**: `src-tauri/src/main.rs`
> **Status**: [REVIEW]

## Overview

Provides the native binary entrypoint for the Tauri application.

## Import Relationships

```text
Upstream: Cargo binary entrypoint
Downstream: src-tauri/src/lib.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `main` | Starts the desktop Tauri app when the `desktop` feature is enabled. |

## Core Logic

The desktop binary is now an explicit Cargo target gated by `required-features = ["desktop"]`. This entrypoint still stays tiny: hide the Windows console for non-debug desktop builds and delegate startup to `app_lib::run()`.

## Data Flow

No app data is transformed here.

## Interactions

Depends on the library crate exposing `run` behind the `desktop` feature and on `src-tauri/Cargo.toml` keeping the desktop binary target explicit.

## Configuration

Uses `#![cfg_attr(all(feature = "desktop", not(debug_assertions)), windows_subsystem = "windows")]`.

## Change Notes

Keep startup behavior in `lib.rs`; this file should stay a minimal binary entrypoint.
