# External Source Compatibility Design

## Summary

This pass hardens external GitHub source detection so fetched repositories can be classified and imported even when the cache repo is not checked out into a populated worktree. The current implementation fetches refs into a cache repo, then scans on-disk folders under that repo. That produces false `unsupported` results when the worktree is empty and also misses newer generated layouts such as root-level hidden agent folders.

The goal is not to support every arbitrary upstream repository shape. The goal is to make Phase 1 reliably support declared, already-generated agent bundles without running upstream build pipelines.

## Product Rules

- Detection must read from fetched git data, not depend on a populated worktree.
- The app must continue supporting the existing generated bundle roots under `dist/agents/...`.
- The app must also support root-level hidden agent roots used by current upstreams:
  - `.agents/skills/*`
  - `.claude/skills/*`
  - `.opencode/skills/*`
- Import still requires an already-present variant directory with `SKILL.md`; this pass does not execute upstream build steps.
- Unknown layouts should still degrade to `unsupported` plus warnings rather than being guessed into importability.

## Chosen Approach

### 1. Detect from git tree snapshots

Variant detection should inspect the fetched default-branch `HEAD` tree through git object commands instead of `std::fs` directory walking. This aligns detection with the existing fingerprint/export pipeline, which already reads git blobs directly.

### 2. Replace hard-coded path checks with a rule table

Generated variant matching should move onto a small table of supported roots keyed by agent. The first ruleset should include both the legacy `dist/agents/...` layout and the newer root-level hidden directories.

### 3. Keep warning behavior strict

The detector should still emit `unsupported_agent_variant` warnings when it finds `SKILL.md` files under scanned agent areas that do not match one of the supported single-skill directory shapes.

## Scope Limits

- No arbitrary upstream build execution
- No generic heuristic candidate state yet
- No repo-specific adapter mechanism yet
- No persistence schema change for source kind or import records

## Validation

- Add regression tests proving detection works against fetched git refs without relying on worktree files.
- Add regression tests for root-level `.agents/.claude/.opencode` variant roots.
- Run focused Rust tests for the external-source domain.
- Run the repo verify gate and module-doc checks after code and docs updates.
