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
| `detect_generic_skill_variants` | Emits one root variant or one variant per recursively discovered fallback skill directory under the configured scan root. |

## Core Logic

The detector accepts injected tree/worktree callbacks from `detect.rs`, so it does not know whether the source is being read from git objects or a fallback filesystem tree. If the scan root itself has `SKILL.md`, it emits the root as a single variant. It also collects recursive `SKILL.md` matches below the scan root, de-duplicates them, and filters out paths already claimed by supported generated-bundle rules.

This pass deliberately skips noisy infrastructure directories such as `.git`, `node_modules`, `target`, `coverage`, `.next`, and `build`, and it excludes `source/skills/*` authoring paths so repositories that ship both source-of-truth skills and generated outputs do not show duplicate import rows. Generic variants use the neutral `skill_repository` key, mark themselves as `generic_discovered`, and leave target-agent selection open for the frontend rather than guessing at import time.

## Interactions

Used only by `detect.rs` after generated bundle rules return no supported variants. Import/export/update behavior remains owned by `imports.rs`, `git_export.rs`, and `git_repo.rs`.
