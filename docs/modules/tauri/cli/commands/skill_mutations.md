# CLI Skill Mutation Adapter

> **Source**: `src-tauri/src/cli/commands/skill_mutations.rs`
> **Status**: [REVIEW]

## Overview

Implements `skills enable` and `skills disable` as locked state mutations.

## Import Relationships

```text
Upstream: src-tauri/src/cli/commands/skills.rs, cli command tests
Downstream: src-tauri/src/app_runtime/*, src-tauri/src/core/skills/scan.rs, src-tauri/src/core/skills/state.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `set_enabled` | Enables or disables one scanned skill id for the effective repo. |

## Core Logic

The adapter resolves the repo path, verifies the skill exists in the current scan snapshot, acquires the config lock, and writes through `SkillStateStore` without changing the persisted file format.

## Interactions

Missing repo paths emit `repo_path_not_configured`; unknown skill ids emit `skill_not_found`; write and lock failures use the shared CLI error schema.

