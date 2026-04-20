# Skills Commands

> **Source**: `src-tauri/src/commands/skills.rs`
> **Status**: [REVIEW]

## Overview

Exposes Tauri commands for scanning configured skills, reading a selected skill's `SKILL.md`, and loading or mutating repo-scoped enable/disable state.

## Import Relationships

```text
Upstream: src-tauri/src/lib.rs, src/lib/tauri.ts
Downstream: src-tauri/src/core/settings.rs, src-tauri/src/core/skills/documents.rs, src-tauri/src/core/skills/scan.rs, src-tauri/src/core/skills/state.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `scan_skills` | Scans the configured repository for custom and external skills. |
| `get_skill_document` | Reads one skill document by repo-relative skill directory path. |
| `get_skill_state` | Loads the disabled skill ID snapshot for the configured repository. |
| `set_skill_enabled` | Persists a single enable/disable change and returns the updated snapshot. |

## Core Logic

Each command resolves the configured repo path through a shared helper, keeps validation thin, and delegates scanning, document loading, or state persistence to the skills core modules.

## Data Flow

Settings provide the repository path. Scan results, document payloads, and disabled-ID snapshots are serialized back to TypeScript through Tauri.

## Interactions

Depends on `SettingsStore`, `scan_repo_skills`, `read_skill_document`, `SkillStateStore`, and frontend wrappers in `src/lib/tauri.ts`.

## Configuration

Uses the persisted repo path from `settings.json`.

## Change Notes

Do not add scanning, parsing, repo-key normalization, or persistence rules here; keep those rules in `src-tauri/src/core/skills/`.
