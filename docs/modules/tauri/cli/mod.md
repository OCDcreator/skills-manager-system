# CLI Dispatch Module

> **Source**: `src-tauri/src/cli/mod.rs`
> **Status**: [REVIEW]

## Overview

Acts as the headless adapter boundary: it builds the runtime context from parsed flags, dispatches to the requested command group, and converts bootstrap failures into structured CLI errors.

## Import Relationships

```text
Upstream: src-tauri/src/lib.rs, src-tauri/src/cli/main.rs
Downstream: src-tauri/src/cli/args.rs, src-tauri/src/cli/commands/*, src-tauri/src/app_runtime/*
```

## Public Surface

| Export | Purpose |
|---|---|
| `CliArgs` | Re-exported root parser type used by the binary entrypoint. |
| `run` | Builds `AppRuntimeContext` and dispatches one CLI command tree leaf. |

## Core Logic

The module keeps command adapters thin by centralizing only two concerns: translating root flags into `AppRuntimeOptions` and routing parsed subcommands to the correct command module. It also wires focused CLI command tests into the same nested command namespace. If config-dir resolution fails before a runtime exists, it still emits a structured bootstrap error.

## Interactions

Must stay aligned with the `cli` Cargo feature and with the command labels exposed from `src-tauri/src/cli/args.rs`.
