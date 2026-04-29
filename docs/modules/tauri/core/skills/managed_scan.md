# Managed Skill Scan Enrichment

> **Source**: `src-tauri/src/core/skills/managed_scan.rs`
> **Status**: [REVIEW]

## Overview

Adds managed-source metadata onto scanned external skills when they are backed by imported GitHub mirrors.

## Public Surface

| Export | Purpose |
|---|---|
| `enrich_managed_external_skills` | Reads managed manifests and attaches `ManagedSourceInfo` to scanned skills. |

## Core Logic

The module only touches `external` skills. For each candidate it looks for `.skills-manager-source.json`, parses `ManagedSkillMirrorManifest`, finds the matching import record by `skill_id`, and fills `managed_source` either as healthy GitHub import metadata or as an `integrity: mismatch` fallback when manifest and record data no longer agree.

## Interactions

Called from `scan_repo_skills_with_external_sources()`. Its manifest checks must stay aligned with `core/external_sources/models.rs`, `imports.rs`, and the frontend managed-source badges and warnings.
