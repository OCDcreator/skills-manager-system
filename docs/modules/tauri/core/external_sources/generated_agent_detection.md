# Generated Agent Bundle Detection

> **Source**: `src-tauri/src/core/external_sources/generated_agent_detection.rs`
> **Status**: [REVIEW]

## Overview

Owns the generated agent bundle rule table and warning scan used by external-source detection.

## Public Surface

| Export | Purpose |
|---|---|
| `GENERATED_AGENT_BUNDLE_KIND` | Internal detection kind for supported generated bundle variants. |
| `collect_supported_generated_variants` | Enumerates supported generated agent variant directories under the optional source subpath. |
| `detect_unknown_generated_agent_variants` | Emits warnings for generated-looking skill directories that do not match supported agent layouts. |
| `scoped_path` | Builds the repo-relative scan root for root-level or subpath-scoped detection. |

## Core Logic

The module keeps generated-bundle detection separate from generic repository fallback detection. It scans the configured source subpath, if present, then applies the same supported layout table for `.agents`, `.claude`, `.opencode`, `.cursor`, `.gemini`, `.github`, and `.kiro` skill roots. Supported variants are direct child skill directories with `SKILL.md`, and source-of-truth metadata is linked from the matching `source/skills/<name>` directory when present.

The warning path scans broader generated agent roots and reports `unsupported_agent_variant` for nested or unknown generated layouts. This keeps the MVP strict: unsupported generated agents are visible as warnings instead of being imported under guessed agent keys.

## Interactions

Used by `detect.rs` before `generic_skill_detection.rs`. It depends on caller-provided git-tree or worktree listing callbacks, so it does not fetch, inspect remotes, or write mirrors itself.
