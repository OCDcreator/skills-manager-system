# CLI Skills Commands

> **Source**: `src-tauri/src/cli/commands/skills.rs`
> **Status**: [REVIEW]

## Overview

Owns the Phase 1a read-only skill commands: scan, state, joined list, and document lookup.

## Import Relationships

```text
Upstream: src-tauri/src/cli/mod.rs
Downstream: src-tauri/src/app_runtime/*, src-tauri/src/core/skills/*
```

## Public Surface

| Export | Purpose |
|---|---|
| `run` | Dispatches one parsed skills subcommand. |

## Core Logic

Every command first resolves the effective repo path from the shared runtime context. `skills list` joins the scan snapshot with disabled IDs from `SkillStateStore` to emit `enabled` booleans. `skills doc` accepts either a stable skill id or a repo-relative path by resolving ids through a fresh scan. Scan warnings are promoted into structured CLI warnings so missing `custom/` or `external/` folders become partial-success output instead of free-form stderr.

## Interactions

Must stay aligned with `scan_repo_skills`, `read_skill_document`, `SkillStateStore`, and the CLI JSON schema in `src-tauri/src/app_runtime/output.rs`.
