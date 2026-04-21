# CLI Skills Command Tests

> **Source**: `src-tauri/src/cli/commands/skills_tests.rs`
> **Status**: [REVIEW]

## Overview

Focused tests for the Phase 1a `skills` CLI adapter behavior.

## Import Relationships

```text
Upstream: src-tauri/src/cli/mod.rs test module wiring
Downstream: src-tauri/src/cli/commands/skills.rs, src-tauri/src/core/skills/state.rs, tempfile
```

## Public Surface

Not applicable; this module is test-only.

## Core Logic

The tests create temporary config and repository roots, verify that `skills list` joins scan results with disabled state, and verify that `skills doc` can resolve a stable skill id into its `SKILL.md` document.

## Interactions

Must stay aligned with `SkillsCommand`, the CLI JSON response shape, and the `SkillStateStore` repo-scoped disabled ID format.
