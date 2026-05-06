# CLI Project Command Tests

> **Source**: `src-tauri/src/cli/commands/projects_tests.rs`
> **Status**: [REVIEW]

## Overview

Validates project config command behavior, per-agent project-layer flags, and no-argument full-snapshot apply.

## Import Relationships

```text
Upstream: cargo test --features cli
Downstream: src-tauri/src/cli/commands/projects.rs, src-tauri/src/core/projects/*
```

## Public Surface

| Export | Purpose |
|---|---|
| tests | Cover add/update/list/remove, per-agent layer parsing at the adapter boundary, malformed layer pairs, and full-snapshot apply. |

## Core Logic

Tests create two independent project assignments and assert one `projects apply` invocation writes both target directories with folder-name skill entries, guarding against reintroducing a per-path apply argument. Per-agent layer tests confirm that scene IDs, direct skills, and exclusions remain assigned to the addressed project agent rather than flattening into one shared project list.

## Interactions

Keep aligned with `ProjectsCommand::Apply` parse tests and `apply_project_assignments`.
