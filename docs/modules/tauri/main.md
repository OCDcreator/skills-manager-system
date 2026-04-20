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
| `main` | Calls `app_lib::run()` to start the application. |

## Core Logic

The module disables the console subsystem for non-debug Windows builds and delegates startup to the library crate.

## Data Flow

No app data is transformed here.

## Interactions

Depends on the library crate exposing `run`.

## Configuration

Uses `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`.

## Change Notes

Keep startup behavior in `lib.rs`; this file should stay a minimal binary entrypoint.
