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
| `scan_skills` | Runs the cached incremental repository scan and enriches managed mirrors when external-source state is available. |
| `load_cached_skills` | Loads the previous cached skill snapshot for immediate startup display when available. |
| `get_skill_document` | Reads one skill document by repo-relative skill directory path. |
| `get_skill_state` | Loads the disabled skill ID snapshot for the configured repository. |
| `set_skill_enabled` | Persists a single enable/disable change and returns the updated snapshot. |

## Core Logic

Each command resolves the configured repo path through a shared helper, keeps validation thin, and delegates scanning, document loading, or state persistence to the skills core modules. `load_cached_skills` resolves the app config directory and returns the last cached payload without walking the repo when a snapshot exists. `scan_skills` forwards both paths into `scan_repo_skills_cached_with_external_sources()` so desktop payloads reuse unchanged metadata while still including additive `managedSource` metadata for external GitHub mirrors.

## Data Flow

Settings provide the repository path. `load_cached_skills` and `scan_skills` pair that repo root with the config root so cached summaries can be read/written and live managed-mirror manifests can be joined to persisted import records before serializing results back to TypeScript.

## Interactions

Depends on `SettingsStore`, `load_cached_repo_skills_with_external_sources`, `scan_repo_skills_cached_with_external_sources`, `read_skill_document`, `SkillStateStore`, and frontend wrappers in `src/lib/tauri.ts`.

## Configuration

Uses the persisted repo path from `settings.json`.

## Change Notes

Do not add scanning, parsing, repo-key normalization, or persistence rules here; keep those rules in `src-tauri/src/core/skills/`. Git operations are handled by the separate `commands::git` module.
