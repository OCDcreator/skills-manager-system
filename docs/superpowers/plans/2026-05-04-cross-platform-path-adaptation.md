# Cross Platform Path Adaptation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make persisted user-facing paths stable across Windows and macOS while preserving existing sync and launcher behavior.

**Architecture:** Add one Rust core helper module for portable path string persistence. Wire settings, agent path overrides, and project assignment paths through that helper, then document the module-doc mapping.

**Tech Stack:** Rust, Tauri core modules, serde JSON stores, Node/Rust verification gates.

---

### Task 1: Portable Path Persistence

**Files:**
- Create: `src-tauri/src/core/platform_paths.rs`
- Modify: `src-tauri/src/core/mod.rs`
- Modify: `src-tauri/src/core/projects/project_paths.rs`
- Modify: `src-tauri/src/core/projects/store.rs`
- Modify: `src-tauri/src/core/agents/config.rs`
- Modify: `src-tauri/src/core/settings.rs`
- Create: `docs/modules/tauri/core/platform_paths.md`
- Modify: `docs/modules/tauri/core/projects/project_paths.md`

- [ ] Add failing tests for Windows slash normalization and trailing separator trimming in project path normalization.
- [ ] Add failing tests for settings and agent path override persistence using the shared normalized path format.
- [ ] Implement `normalize_portable_absolute_path` and `portable_path_string`.
- [ ] Wire project assignment, agent override, repo path, and assistant working directory saves through the shared helper.
- [ ] Update module docs.
- [ ] Run `cargo test --manifest-path src-tauri/Cargo.toml projects settings agents`, `node scripts/check-module-doc-coverage.mjs`, `node scripts/check-module-doc-diff.mjs`, `npm run check:architecture`, and `npm run verify`.
