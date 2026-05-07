# CLI Skills Commands

> **Source**: `src-tauri/src/cli/commands/skills.rs`
> **Status**: [REVIEW]

## Overview

Owns skill scan, state, joined list, document lookup, and mutation dispatch.

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

Every command first resolves the effective repo path from the shared runtime context. `skills scan`, `skills list`, and `skills doc` now route through the managed-source-aware scanner so CLI output stays aligned with the desktop surface for GitHub mirrors. `skills list` joins the scan snapshot with disabled IDs from `SkillStateStore` to emit `enabled` booleans while preserving optional `managedSource` metadata. `skills doc` accepts either a stable skill id or a repo-relative path by resolving ids through a fresh managed-source-aware scan, then copies the matched `managedSource` payload onto the returned document. Enable/disable dispatches to `skill_mutations.rs`, which verifies the skill id before acquiring the config lock and writing state. Scan warnings are promoted into structured CLI warnings so missing `custom/` or `external/` folders become partial-success output instead of free-form stderr.

## Interactions

Must stay aligned with `scan_repo_skills_with_external_sources`, `read_skill_document`, `SkillStateStore`, `skill_mutations.rs`, and the CLI JSON schema in `src-tauri/src/app_runtime/output.rs`.

## Current Note

The current branch change for this module is formatting-only; runtime behavior is unchanged.
