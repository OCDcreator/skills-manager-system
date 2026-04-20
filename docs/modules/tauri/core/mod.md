# Core Module Boundary

> **Source**: `src-tauri/src/core/mod.rs`
> **Status**: [REVIEW]

## Overview

Declares backend core domains used by Tauri command modules.

## Import Relationships

```text
Upstream: src-tauri/src/commands/*
Downstream: src-tauri/src/core/settings.rs, src-tauri/src/core/skills/mod.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `settings` | Settings persistence domain. |
| `skills` | Skill scanning and document-reading domain. |

## Core Logic

This is an aggregation module only; it exposes core domain modules to the command layer.

## Data Flow

Not applicable.

## Interactions

Must include new core domains before command modules can import them.

## Configuration

None.

## Change Notes

Avoid turning this file into a business-logic host; create domain modules under `core/` instead.
