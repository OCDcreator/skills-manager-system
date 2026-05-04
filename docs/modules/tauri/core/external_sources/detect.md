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
| `detect_external_source_variants` | Scans a cached repository for supported generated layouts, preferring fetched git refs over the worktree. |
| `detect_external_source_variants_at_ref` | Scans a known commit/ref without consulting remotes. |
| `detect_external_source_variants_from_worktree` | Scans local files as the fallback for temp repos and caches without a persisted commit. |

## Core Logic

Detection is now rule-table-driven. The broad `detect_external_source_variants` helper still resolves fetched upstream `HEAD` for fetch-time flows, then reads variant directories from git object data via `git_tree.rs` so cached repos do not need a checked-out worktree. Startup snapshot assembly calls the explicit `*_at_ref` helper with the persisted `lastFetchedCommit`, avoiding remote `HEAD` resolution on read-only list operations. If no fetched ref can be resolved, callers can fall back to the local filesystem for temp-repo tests and other non-fetched directories.

Supported roots now include both the legacy generated bundle layout under `dist/agents/...` and root-level hidden agent folders such as `.agents/skills`, `.claude/skills`, and `.opencode/skills`. The aligned rule table also recognizes additional upstream layouts that map cleanly onto the app's existing agent catalog:

- `.cursor/skills` and `dist/cursor/.cursor/skills` -> `cursor`
- `.gemini/skills` and `dist/gemini/.gemini/skills` -> `gemini_cli`
- `.github/skills` and `dist/github/.github/skills` -> `github_copilot`
- `.kiro/skills` and `dist/kiro/.kiro/skills` -> `kilo_code`

For each supported root it emits one variant per direct child skill directory that contains `SKILL.md`, canonicalizes repo-relative paths through `core/skills/identity.rs`, and maps each generated variant back to `source/skills/<name>` when that source-of-truth path exists.

Warnings remain strict: the detector scans broader agent-specific roots and emits `unsupported_agent_variant` when it sees a `SKILL.md` layout that does not match one of the supported single-skill directory shapes. Upstream-only tools that do not have a safe app-catalog mapping yet, such as Trae or Qoder in the observed `impeccable` layout, remain warnings rather than being auto-imported under a guessed agent key.

## Interactions

Used by `service.rs` after fetch and by `list_external_sources()` when cached repos already exist. This module only detects and classifies; it does not fetch git refs or write mirrors, but it must stay aligned with `git_tree.rs` path parsing and the generated-layout rule table.
