# External Source Detection

> **Source**: `src-tauri/src/core/external_sources/detect.rs`
> **Status**: [REVIEW]

## Overview

Classifies cached repositories and enumerates both supported generated agent variants and generic `SKILL.md` fallback candidates from the same scan pass.

## Public Surface

| Export | Purpose |
|---|---|
| `DetectedExternalVariant` | One detected generated skill variant plus optional source-of-truth metadata. |
| `DetectionResult` | Detection kind, variants, and warnings bundle. |
| `detect_external_source_variants` | Scans a cached repository for supported generated layouts, preferring fetched git refs over the worktree. |
| `detect_external_source_variants_at_ref` | Scans a known commit/ref without consulting remotes. |
| `detect_external_source_variants_at_ref_in_root` | Scans a known commit/ref under an optional repo-relative source subpath. |
| `detect_external_source_variants_from_worktree` | Scans local files as the fallback for temp repos and caches without a persisted commit. |
| `detect_external_source_variants_from_worktree_in_root` | Worktree fallback that honors the same optional source subpath. |

## Core Logic

Detection is rule-table-driven for supported generated bundles and subpath-aware for both git-object and worktree reads. The broad `detect_external_source_variants` helper still resolves fetched upstream `HEAD` for fetch-time flows, then reads variant directories from git object data via `git_tree.rs` so cached repos do not need a checked-out worktree. Startup snapshot assembly calls the explicit subpath-aware `*_at_ref_in_root` helper with the persisted `lastFetchedCommit`, avoiding remote `HEAD` resolution on read-only list operations. If no fetched ref can be resolved, callers can fall back to the local filesystem for temp-repo tests and other non-fetched directories.

Supported roots now include both the legacy generated bundle layout under `dist/agents/...` and root-level hidden agent folders such as `.agents/skills`, `.claude/skills`, and `.opencode/skills`. The aligned rule table also recognizes additional upstream layouts that map cleanly onto the app's existing agent catalog:

- `.cursor/skills` and `dist/cursor/.cursor/skills` -> `cursor`
- `.gemini/skills` and `dist/gemini/.gemini/skills` -> `gemini_cli`
- `.github/skills` and `dist/github/.github/skills` -> `github_copilot`
- `.kiro/skills` and `dist/kiro/.kiro/skills` -> `kilo_code`
- `.codex/skills` and `dist/codex/.codex/skills` -> `codex`

For each supported generated root it emits one variant per direct child skill directory that contains `SKILL.md`, canonicalizes repo-relative paths through `core/skills/identity.rs`, maps each generated variant back to `source/skills/<name>` when that source-of-truth path exists, and records extra metadata for the UI: detection class, suggested target agents, and an optional detected-agent hint. Exact rule-table hits and alias-compatible hits remain distinguishable so the frontend can explain why a candidate was found.

The generic fallback now runs alongside supported-layout detection instead of only after total generated-layout failure. It scans the configured repo-relative subpath when present, otherwise the repo root, recursively finds every remaining `SKILL.md` directory, removes paths already claimed by supported generated rules, skips noisy trees like `.git`, `node_modules`, `target`, `coverage`, `.next`, and `build`, and excludes `source/skills/*` authoring directories so source-of-truth content does not appear as duplicate import candidates. A scan root with its own `SKILL.md` still becomes one importable root variant with variant path `.`, and generic candidates continue to use the neutral `skill_repository` key so ordinary skill repositories do not masquerade as generated agent exports.

Warnings remain strict: the detector scans broader agent-specific roots and emits `unsupported_agent_variant` when it sees a `SKILL.md` layout that does not match one of the supported single-skill directory shapes. Upstream-only tools that do not have a safe app-catalog mapping yet, such as Trae or Qoder in the observed `impeccable` layout, remain warnings rather than being auto-imported under a guessed agent key.

## Interactions

Used by `service.rs` after fetch and by `list_external_sources()` when cached repos already exist. This module only detects and classifies; it does not fetch git refs or write mirrors, but it must stay aligned with `git_tree.rs` path parsing and the generated-layout rule table.
