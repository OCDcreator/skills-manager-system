# Generic Skill Repository Detection

> **Source**: `src-tauri/src/core/external_sources/generic_skill_detection.rs`
> **Status**: [REVIEW]

## Overview

Detects ordinary skill repository layouts after generated agent bundle detection has had first chance to classify the source.

## Public Surface

| Export | Purpose |
|---|---|
| `GENERIC_SKILL_REPOSITORY_KIND` | Internal detection kind for ordinary skill repository sources. |
| `GENERIC_SKILL_AGENT_KEY` | Neutral variant key used for ordinary skill repository imports. |
| `detect_generic_skill_variants` | Emits one root variant or one variant per direct child skill directory under the configured scan root. |

## Core Logic

The detector accepts injected tree/worktree callbacks from `detect.rs`, so it does not know whether the source is being read from git objects or a fallback filesystem tree. If the scan root itself has `SKILL.md`, it emits the root as a single variant. Otherwise it emits direct child directories that contain `SKILL.md` and intentionally ignores deeper nested matches for the MVP.

Generic variants use the neutral `skill_repository` key so ordinary skill repositories do not masquerade as Codex-specific exports. Existing managed imports that were created with an older agent key remain readable through their stored manifests and records; the detector only affects newly listed variants.

## Interactions

Used only by `detect.rs` after generated bundle rules return no supported variants. Import/export/update behavior remains owned by `imports.rs`, `git_export.rs`, and `git_repo.rs`.
