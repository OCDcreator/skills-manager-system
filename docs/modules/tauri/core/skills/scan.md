# Skill Scanner

> **Source**: `src-tauri/src/core/skills/scan.rs`
> **Status**: [REVIEW]

## Overview

Scans the configured `my-skills` repository and builds sorted skill summaries, including managed-source enrichment for imported GitHub mirrors.

## Public Surface

| Export | Purpose |
|---|---|
| `ManagedSourceInfo` | Serializable managed-mirror metadata attached to externally imported skills. |
| `SkillSummary` | Serializable summary shown in skill lists. |
| `ScanSkillsResponse` | Skill list plus non-fatal warnings. |
| `scan_repo_skills` | Base repo scan over `custom/` and `external/`. |
| `scan_repo_skills_with_external_sources` | Base scan plus managed-mirror enrichment from `external-sources.json`. |

## Core Logic

The base scan still reads only first-level custom skills, recursively walks `external/`, ignores noise directories, canonicalizes relative paths, and sorts by skill id. The new enrichment pass looks for `.skills-manager-source.json` inside external skill directories, parses that manifest, matches it against persisted import records, and attaches `managed_source` metadata when the mirror is healthy. If manifest and import records drift, the skill still scans as external but carries `integrity: mismatch`. Desktop startup now normally routes through `cache.rs`, which preserves these same traversal and DTO rules while avoiding repeated metadata parsing for unchanged entries.

## Interactions

This module is the bridge that lets `SkillList` and `SkillDetailPanel` treat managed GitHub mirrors as normal external skills with additive metadata. Keep it aligned with `core/external_sources/models.rs`, `documents.rs`, and the TypeScript DTOs in `src/lib/tauri.ts`.
