# Skill Scan Cache Tests

> **Source**: `src-tauri/src/core/skills/cache_tests.rs`
> **Status**: [REVIEW]

## Overview

Focused regression tests for cached and incremental skill scanning.

## Public Surface

This module is test-only and exports no production API.

## Core Logic

The tests build temporary `my-skills` style repositories, seed the cache through the public cached scan functions, then verify three startup-critical behaviors: unchanged fingerprints reuse cached metadata, changed `SKILL.md` files are rebuilt, and immediate cached loads can return the previous snapshot without walking the repo tree. A small integration-style test also proves the managed-source-aware cached scan writes the cache file while returning the normal scan payload.

## Interactions

Keep these tests aligned with `cache.rs` fingerprint semantics, `scan.rs` traversal expectations, and the startup hydration path in `commands/skills.rs` plus `AppContext.tsx`.
