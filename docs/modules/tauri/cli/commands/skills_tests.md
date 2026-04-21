# CLI Skills Command Tests

> **Source**: `src-tauri/src/cli/commands/skills_tests.rs`
> **Status**: [REVIEW]

## Overview

Focused tests for `skills` CLI adapter query and mutation behavior.

## Import Relationships

```text
Upstream: src-tauri/src/cli/mod.rs test module wiring
Downstream: src-tauri/src/cli/commands/skills.rs, src-tauri/src/core/skills/state.rs, tempfile
```

## Public Surface

| Export | Purpose |
|---|---|
| tests | Cover `skills list`, `skills doc`, `skills enable`, and `skills disable`. |

## Core Logic

The tests create temporary config and repository roots, verify that `skills list` joins scan results with disabled state, verify that `skills doc` can resolve a stable skill id into its `SKILL.md` document, and assert enable/disable writes through `SkillStateStore`.

## Interactions

Must stay aligned with `SkillsCommand`, `skill_mutations.rs`, the CLI JSON response shape, and the `SkillStateStore` repo-scoped disabled ID format.
