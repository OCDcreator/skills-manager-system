# CLI Output Errors

> **Source**: `src-tauri/src/app_runtime/output_errors.rs`
> **Status**: [REVIEW]

## Overview

Contains the stable CLI exit-code enum and structured error payload helpers used by the output layer.

## Import Relationships

```text
Upstream: src-tauri/src/app_runtime/output.rs, src-tauri/src/cli/commands/*
Downstream: serde_json
```

## Public Surface

| Export | Purpose |
|---|---|
| `CliExitStatus` | Maps success, partial success, and known failure classes to process exit codes. |
| `CliErrorBody` | Serializable `error` object embedded in failed CLI responses. |
| `CliCommandError` | Adapter-friendly error builder carrying code, message, details, and exit status. |

## Core Logic

The exit-code mapping follows the approved CLI design: `0` for success, `8` for partial success, and stable non-zero values for argument, configuration, target, filesystem, external command, and conflict failures.

## Interactions

Any command-specific error mapping should create `CliCommandError` values here rather than inventing ad hoc JSON in command modules.
