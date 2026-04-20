# Skills Commands

> **Source**: `src-tauri/src/commands/skills.rs`
> **Status**: [REVIEW]

## Overview

Exposes Tauri commands for scanning configured skills and reading a selected skill's `SKILL.md` content.

## Import Relationships

```text
Upstream: src-tauri/src/lib.rs, src/lib/tauri.ts
Downstream: src-tauri/src/core/settings.rs, src-tauri/src/core/skills/documents.rs, src-tauri/src/core/skills/scan.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `scan_skills` | Scans the configured repository for custom and external skills. |
| `get_skill_document` | Reads one skill document by repo-relative skill directory path. |

## Core Logic

Each command loads settings from the app config directory, requires a configured repo path, and delegates domain work to the skills core modules.

## Data Flow

Settings provide the repository path. Scan results or document payloads are serialized back to TypeScript through Tauri.

## Interactions

Depends on `SettingsStore`, `scan_repo_skills`, `read_skill_document`, and frontend wrappers in `src/lib/tauri.ts`.

## Configuration

Uses the persisted repo path from `settings.json`.

## Change Notes

Do not add scanning, parsing, or path traversal logic here; keep those rules in `src-tauri/src/core/skills/`.
