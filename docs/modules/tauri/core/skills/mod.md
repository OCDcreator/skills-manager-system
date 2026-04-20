# Skills Core Module Boundary

> **Source**: `src-tauri/src/core/skills/mod.rs`
> **Status**: [REVIEW]

## Overview

Declares the skills domain modules used by command handlers, including repo-scoped local skill-state persistence.

## Import Relationships

```text
Upstream: src-tauri/src/core/mod.rs, src-tauri/src/commands/skills.rs
Downstream: src-tauri/src/core/skills/documents.rs, src-tauri/src/core/skills/metadata.rs, src-tauri/src/core/skills/scan.rs, src-tauri/src/core/skills/state.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `documents` | Skill document reading domain. |
| `metadata` | `SKILL.md` metadata parser. |
| `scan` | Repository scanning domain. |
| `state` | Repo-scoped local skill enable/disable persistence. |

## Core Logic

This is a domain aggregation module only.

## Data Flow

Not applicable.

## Interactions

Must expose any new skills-domain module before commands or sibling modules can import it.

## Configuration

None.

## Change Notes

Keep this file declarative; add behavior to focused modules such as scanners, parsers, persistence stores, or synchronizers.
