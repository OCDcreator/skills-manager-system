# Agent Per-Target Sync Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace global enabled-skill agent sync with per-agent skill and scene assignment, while preserving global hard-disable behavior and manifest-safe cleanup.

**Architecture:** Extend the existing `core/agents` domain instead of creating a parallel sync system. Persist per-agent skill, scene, and exclusion arrays in `agent-config.json`; resolve desired skills inside Rust sync; keep Tauri commands thin; and split the larger frontend work into focused agent selector components plus a reusable navigation guard.

**Tech Stack:** Tauri 2, Rust 2021, React 19, TypeScript 5, Vite 7, Tailwind CSS 3, i18next, serde_json

---

## File Map

- `src-tauri/src/core/agents/config.rs` — extend persisted agent entries and add whole-entry save.
- `src-tauri/src/core/agents/discovery.rs` — expose per-agent selection arrays in inventory.
- `src-tauri/src/core/agents/sync.rs` — resolve per-agent direct, scene, and excluded skill sets.
- `src-tauri/src/core/agents/sync_tests.rs` — lock new sync semantics.
- `src-tauri/src/commands/agents.rs` — add `set_agent_configuration` and scoped apply.
- `src-tauri/src/cli/commands/agent_sync.rs` — pass unscoped sync for CLI.
- `src/lib/tauri.ts` — add DTO fields and new invoke wrapper.
- `src/context/AppContext.tsx` — expose saved agent config and scoped apply without growing navigation logic.
- `src/context/navigation-guard.ts` — own unsaved-navigation guard state and actions.
- `src/components/UnsavedChangesDialog.tsx` — render the save/discard/stay prompt.
- `src/components/AppShell.tsx` — request guarded navigation and show the dialog.
- `src/views/AgentsView.tsx` — draft editor orchestration.
- `src/components/agents/AgentTargetCard.tsx` — per-agent card layout.
- `src/components/agents/AgentSkillSelector.tsx` — direct skill selection and exclusions.
- `src/components/agents/AgentSceneSelector.tsx` — scene checkbox selection.
- `src/components/agents/AgentSelectionSummary.tsx` — effective dedupe/exclusion preview.
- `src/i18n/en.json` / `src/i18n/zh.json` — updated labels and prompts.
- `docs/modules/**` — one-to-one module docs for every new or modified source file.

## Tasks

- [ ] Write Rust failing tests for per-agent selection, global hard disable, duplicate scene/direct dedupe, exclusions, scoped apply, and disabled-agent cleanup.
- [ ] Extend `AgentConfigEntry` with selected skill IDs, selected scene IDs, and excluded skill IDs, using serde defaults for backwards compatibility.
- [ ] Update inventory DTOs and commands to round-trip the new fields.
- [ ] Replace global desired-skill resolution with per-agent resolution in `sync.rs`.
- [ ] Add scoped sync support so saving one agent can sync only that agent.
- [ ] Split frontend navigation guard state out of `AppContext` and wire the shell prompt.
- [ ] Convert Agents page to a draft editor with per-agent save and save-all flows.
- [ ] Update i18n strings, module docs, and design docs to match the new semantics.
- [ ] Run focused Rust tests, `cargo check`, frontend build, module-doc checks, architecture checks, and the full verify gate.
