# Skill Scan Cache

> **Source**: `src-tauri/src/core/skills/cache.rs`
> **Status**: [REVIEW]

## Overview

Persists and reuses repository skill-scan summaries so startup can show cached results immediately and full scans only rebuild changed skill entries.

## Public Surface

| Export | Purpose |
|---|---|
| `load_cached_repo_skills` | Loads the last cached base scan for a repo without touching the repo tree. |
| `load_cached_repo_skills_with_external_sources` | Loads cached summaries and re-applies managed-source enrichment from current external-source state. |
| `scan_repo_skills_cached` | Collects current `SKILL.md` fingerprints, reuses unchanged cached summaries, rebuilds changed/new entries, and writes the next cache. |
| `scan_repo_skills_cached_with_external_sources` | Cached scan plus current managed-source enrichment for desktop payloads. |

## Core Logic

The cache file lives under the app config directory as `skill-scan-cache.json`, keyed per configured repo using the same normalized repo-key helper as skill state. Each cached entry stores the skill directory relative path, `SKILL.md` modified timestamp, file size, and the serialized `SkillSummary`. A scan still walks `custom/` first-level directories and recursive `external/` paths to find current documents, but only reparses metadata for fingerprints that are new or changed. Entries missing from the current fingerprint set are dropped from the next cache, so deleted skills disappear after the background refresh.

## Interactions

Keep this aligned with `scan.rs` traversal semantics, `state.rs` repo-key normalization, `managed_scan.rs` enrichment, and the `load_cached_skills` / `scan_skills` Tauri commands. Cached summaries deliberately clear `managed_source` before enrichment so GitHub mirror status stays current even when the base skill metadata is reused.
