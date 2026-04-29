# External Source Compatibility Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make external-source detection read fetched git data and support both legacy and current generated agent-bundle path layouts.

**Architecture:** Keep imports and persistence unchanged, but move detection onto git-tree reads so cached repos do not need a populated worktree. Represent supported generated layouts as a rule table so new path roots can be added without rewriting the detector.

**Tech Stack:** Rust, git CLI object inspection, Tauri core modules, module-doc guard scripts

---

### Task 1: Lock the expected behavior with failing tests

**Files:**
- Modify: `src-tauri/src/core/external_sources/detect.rs`
- Test: `src-tauri/src/core/external_sources/detect.rs`

- [ ] **Step 1: Write failing tests**

Add tests that prove:
- a fetched cache repo with no checked-out files can still detect variants from the fetched `HEAD`
- root-level `.agents/skills/impeccable`, `.claude/skills/impeccable`, and `.opencode/skills/impeccable` are recognized as supported variants

- [ ] **Step 2: Run focused Rust tests to verify failure**

Run: `cargo test --manifest-path src-tauri/Cargo.toml detect_external_source -- --nocapture`

Expected: FAIL because the current detector depends on the worktree and only recognizes `dist/agents/...`

### Task 2: Implement git-tree-backed detection and path-rule expansion

**Files:**
- Create: `src-tauri/src/core/external_sources/git_tree.rs`
- Modify: `src-tauri/src/core/external_sources/detect.rs`
- Modify: `src-tauri/src/core/external_sources/mod.rs`

- [ ] **Step 1: Add git-tree helper primitives**

Expose small helpers for listing directories and locating `SKILL.md` entries from a git ref without checking out files.

- [ ] **Step 2: Rebuild detection on top of the helper and rule table**

Switch detection from `std::fs` walking to git-tree inspection, keep source-of-truth mapping against `source/skills/*`, and expand supported roots to both legacy and root-level hidden agent directories.

- [ ] **Step 3: Run focused Rust tests to verify pass**

Run: `cargo test --manifest-path src-tauri/Cargo.toml detect_external_source -- --nocapture`

Expected: PASS

### Task 3: Sync docs and verify the repo gates

**Files:**
- Modify: `docs/modules/tauri/core/external_sources/detect.md`
- Modify: `docs/modules/tauri/core/external_sources/git_repo.md`
- Modify: `docs/modules/tauri/core/external_sources/mod.md`
- Create: `docs/modules/tauri/core/external_sources/git_tree.md`

- [ ] **Step 1: Update module docs**

Document the new git-tree helper module, the new detection rule-table behavior, and the fact that fetched refs are now the detection source of truth.

- [ ] **Step 2: Run module-doc guard commands**

Run: `node scripts/check-module-doc-coverage.mjs`

Expected: PASS

Run: `node scripts/check-module-doc-diff.mjs --range HEAD`

Expected: PASS

- [ ] **Step 3: Run repo verification**

Run: `npm run verify`

Expected: PASS
