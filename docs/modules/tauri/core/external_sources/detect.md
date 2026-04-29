# External Source Detection

> **Source**: `src-tauri/src/core/external_sources/detect.rs`
> **Status**: [REVIEW]

## Overview

Classifies cached repositories and enumerates generated agent-specific skill variants from a rule table.

## Public Surface

| Export | Purpose |
|---|---|
| `DetectedExternalVariant` | One detected generated skill variant plus optional source-of-truth metadata. |
| `DetectionResult` | Detection kind, variants, and warnings bundle. |
| `detect_external_source_variants` | Scans a cached repository for supported generated layouts. |

## Core Logic

Detection currently recognizes generated bundles under `dist/agents/.agents/skills`, `dist/agents/.claude/skills`, and `dist/agents/.opencode/skills`. For supported roots it emits one variant per direct child skill directory that contains `SKILL.md`, canonicalizes repo-relative paths through `core/skills/identity.rs`, and tries to map each generated variant back to `source/skills/<name>` as a source-of-truth hint. It also walks the wider `dist/agents` tree and emits `unsupported_agent_variant` warnings for generated layouts that do not match the supported rule table.

## Interactions

Used by `service.rs` after fetch and by `list_external_sources()` when cached repos already exist. This module only detects and classifies; it does not fetch git refs or write mirrors.
