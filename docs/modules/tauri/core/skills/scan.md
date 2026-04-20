# Skill Scanner

> **Source**: `src-tauri/src/core/skills/scan.rs`
> **Status**: [REVIEW]

## Overview

Scans the configured `my-skills` repository and builds sorted summaries for custom and external skills.

## Import Relationships

```text
Upstream: src-tauri/src/commands/skills.rs, src-tauri/src/core/agents/sync.rs
Downstream: src-tauri/src/core/skills/metadata.rs, walkdir
```

## Public Surface

| Export | Purpose |
|---|---|
| `SkillSummary` | Serializable summary shown in skill lists. |
| `ScanSkillsResponse` | Serializable scan result with skills and warnings. |
| `scan_repo_skills` | Scans `custom/` and `external/` for valid `SKILL.md` directories. |
| `build_skill_id` | Produces stable ids as `{source_type}:{source_relative}`. |

## Core Logic

The scanner requires the configured repo root to exist. It reads only first-level directories under `custom/`, recursively walks `external/`, ignores known noise directories, parses metadata, normalizes repo-relative paths with forward slashes, and sorts summaries by id.

## Data Flow

Filesystem directories become `SkillSummary` values. Missing `custom/` or `external/` roots become warnings instead of hard failures.

## Interactions

Source type values must stay aligned with frontend filters, i18n source labels, and document-reader validation.

## Configuration

Ignored directory names are defined by `IGNORED_DIRS`: `.git`, `node_modules`, `dist`, `target`, and `.tmp-skills`.

## Change Notes

If custom skill layout becomes nested, update the custom scan behavior and tests that currently require first-level-only custom scanning.
