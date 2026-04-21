# CLI Output Schema

> **Source**: `src-tauri/src/app_runtime/output.rs`
> **Status**: [REVIEW]

## Overview

Defines the stable machine-first CLI response schema, warning payloads, pretty rendering, and final result builders.

## Import Relationships

```text
Upstream: src-tauri/src/cli/mod.rs, src-tauri/src/cli/commands/*
Downstream: serde_json, time, src-tauri/src/app_runtime/context.rs, src-tauri/src/app_runtime/output_errors.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `CliStatus` | Success / partial / error classifier serialized to JSON. |
| `CliMeta` | Shared metadata block with config dir, repo path, timestamp, and version. |
| `CliWarning` | Structured warning object for partial success cases. |
| `CliRunResult` | Final response + exit status returned by the CLI dispatch layer, including explicit partial builders. |

## Core Logic

Success and partial responses share one schema, while errors move details into the `error` object and keep `warnings` present for automation consumers. Explicit partial responses carry exit code `8` for conflict-bearing sync operations. Default rendering is compact JSON; `--pretty` renders from the same response object rather than from a second bespoke formatter.

## Interactions

Any CLI command that changes its payload shape must keep this schema stable. Exit-code values live in `output_errors.rs` and must stay aligned with the approved CLI design doc under `docs/superpowers/specs/2026-04-21-cli-foundation-design.md`.
