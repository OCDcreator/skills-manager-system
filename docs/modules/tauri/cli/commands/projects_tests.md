# CLI Project Command Tests

> **Source**: `src-tauri/src/cli/commands/projects_tests.rs`
> **Status**: [REVIEW]

## Overview

Validates project config command behavior and no-argument full-snapshot apply.

## Import Relationships

```text
Upstream: cargo test --features cli
Downstream: src-tauri/src/cli/commands/projects.rs, src-tauri/src/core/projects/*
```

## Public Surface

| Export | Purpose |
|---|---|
| tests | Cover add/update/list/remove and full-snapshot apply. |

## Core Logic

Tests create two independent project assignments and assert one `projects apply` invocation writes both target directories, guarding against reintroducing a per-path apply argument.

## Interactions

Keep aligned with `ProjectsCommand::Apply` parse tests and `apply_project_assignments`.

