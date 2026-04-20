# Agent Sync Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add manual copy-only global agent sync for `Codex`, `Claude Code`, and `OpenCode`, using the current repository's enabled skills and safe manifest/ledger-based cleanup rules.

**Architecture:** Add a dedicated Rust `core/agents` subdomain for catalog, config, discovery, and sync; expose thin Tauri commands for inventory/config/apply; extend `AppContext` with agent inventory and apply actions; and add a focused `Agents` page with stable agent-specific components. Keep `settings.rs` limited to `repo_path`, reuse `scan.rs` and `state.rs` as the source of enabled-skill truth, and update module docs for every touched source file.

**Tech Stack:** Tauri 2, Rust 2021, React 19, TypeScript 5, Vite 7, Tailwind CSS 3, i18next, serde_json

---

## File Map

- `src-tauri/src/core/agents/catalog.rs` — built-in definitions for `codex`, `claude_code`, and `opencode`.
- `src-tauri/src/core/agents/config.rs` — `agent-config.json` persistence for enabled flags and path overrides.
- `src-tauri/src/core/agents/discovery.rs` — path resolution, detection status, and inventory snapshot assembly.
- `src-tauri/src/core/agents/manifest.rs` — target-local manifest and app-local ledger helpers plus copy-only target reconciliation.
- `src-tauri/src/core/agents/sync.rs` — enabled-skill computation and high-level manual apply orchestration.
- `src-tauri/src/core/agents/sync_tests.rs` — focused Rust tests for the phase-three sync workflow.
- `src-tauri/src/core/agents/mod.rs` — public exports for the new agents subdomain.
- `src-tauri/src/core/mod.rs` — exports `agents`.
- `src-tauri/src/commands/agents.rs` — thin inventory/config/apply commands.
- `src-tauri/src/commands/mod.rs` — exports `agents`.
- `src-tauri/src/lib.rs` — registers agent commands.
- `src/lib/tauri.ts` — DTOs and invoke wrappers for agent inventory/config/apply.
- `src/context/AppContext.tsx` — app-wide orchestration for agent inventory and apply.
- `src/views/AgentsView.tsx` — page composition only.
- `src/components/agents/AgentSyncSummary.tsx` — summary card and apply button.
- `src/components/agents/AgentTargetCard.tsx` — per-agent config card and override form.
- `src/components/agents/AgentApplyResults.tsx` — latest apply result rendering.
- `src/App.tsx` — adds the new `agents` view.
- `src/components/AppShell.tsx` — adds `Agents` nav item.
- `src/i18n/en.json` / `src/i18n/zh.json` — text for agent inventory, copy-only labels, and result summaries.
- `docs/modules/**` — module-doc coverage for all new/modified source files above.

## Task 1: Lock Failing Rust Tests For The New Agents Domain

**Files:**
- Create: `src-tauri/src/core/agents/mod.rs`
- Create: `src-tauri/src/core/agents/catalog.rs`
- Create: `src-tauri/src/core/agents/config.rs`
- Create: `src-tauri/src/core/agents/discovery.rs`
- Create: `src-tauri/src/core/agents/manifest.rs`
- Create: `src-tauri/src/core/agents/sync.rs`
- Create: `src-tauri/src/core/agents/sync_tests.rs`
- Modify: `src-tauri/src/core/mod.rs`

- [ ] **Step 1: Add the new module skeleton and compile-failing tests first**

Create the `agents` module files with test modules that define the required behavior before full implementation:

- `config.rs`
  - `load_returns_defaults_when_config_missing`
  - `set_agent_enabled_round_trips`
  - `path_override_round_trips`
- `discovery.rs`
  - `detected_default_path_becomes_effective_path`
  - `missing_default_path_reports_missing_mode`
  - `path_override_takes_precedence_over_detected_path`
- `sync.rs`
  - `apply_copies_only_enabled_skills`
  - `disabled_skills_are_not_written`
  - `reapply_removes_stale_managed_entries`
  - `unmanaged_entries_are_not_deleted`
  - `path_override_target_is_used`

The initial tests may fail to compile because `AgentConfigStore`, `AgentInventoryItem`, `apply_agent_sync`, and manifest helpers do not exist yet. That is acceptable and expected.

- [ ] **Step 2: Run the focused Rust target and confirm RED**

Run: `cargo test --manifest-path src-tauri/Cargo.toml agents -- --nocapture`

Expected:

- failing compile or failing tests in the new `agents` module
- failures specifically caused by missing implementation, not typo-level issues

## Task 2: Implement Catalog, Config, And Discovery Without Touching Sync Yet

**Files:**
- Modify: `src-tauri/src/core/agents/catalog.rs`
- Modify: `src-tauri/src/core/agents/config.rs`
- Modify: `src-tauri/src/core/agents/discovery.rs`
- Modify: `src-tauri/src/core/agents/mod.rs`
- Modify: `src-tauri/src/core/mod.rs`

- [ ] **Step 1: Implement the small built-in catalog**

Add a focused catalog with exactly three agents and stable metadata:

- `codex`
- `claude_code`
- `opencode`

The catalog should expose the agent key, display name, default skills-dir rule, and detect-dir rule. Do not add general custom-agent support in this phase.

- [ ] **Step 2: Implement `agent-config.json` persistence**

In `config.rs`, add:

- load default-empty snapshot behavior
- validation and normalization for path override input
- `set_agent_enabled`
- `set_agent_path_override`
- `clear_agent_path_override`

Store only:

- `enabled`
- `pathOverride`

Do not move `repo_path` out of `settings.rs`.

- [ ] **Step 3: Implement inventory discovery**

In `discovery.rs`, add:

- path candidate resolution using the catalog rules
- `.config/...` dual-candidate handling for OpenCode
- `pathMode` calculation
- `defaultSkillsDir`, `detectedSkillsDir`, `effectiveSkillsDir`
- final inventory DTOs that combine catalog + config + runtime path checks

Keep missing-path behavior explicit by leaving `effectiveSkillsDir = null` when neither override nor detected default is available.

- [ ] **Step 4: Re-run the focused tests and get config/discovery GREEN before sync work**

Run: `cargo test --manifest-path src-tauri/Cargo.toml agents -- --nocapture`

Expected:

- config and discovery tests pass
- sync tests still fail until Task 3 is complete

## Task 3: Implement Copy-Only Manual Apply With Manifest And Ledger Safety

**Files:**
- Modify: `src-tauri/src/core/agents/sync.rs`

- [ ] **Step 1: Reuse the existing skill scan and skill-state modules as inputs**

Inside `sync.rs`, compose:

- `scan_repo_skills` from `src-tauri/src/core/skills/scan.rs`
- `SkillStateStore` from `src-tauri/src/core/skills/state.rs`

Compute:

`enabled skills = scanned skills - disabledSkillIds`

Do not re-encode enable-state rules inside the frontend.

- [ ] **Step 2: Implement manifest and ledger persistence**

Add:

- target-local manifest file under each agent skills directory
- app-local ledger file under the app config directory

The manifest should track only app-managed entries for that one target directory. The ledger should remember the last applied target directory per agent.

- [ ] **Step 3: Implement copy-only apply rules**

Add copy-only sync behavior with these rules:

- only enabled agents are considered
- enabled agent with no effective path returns a skipped result
- existing managed entries may be replaced
- stale managed entries may be removed
- unmanaged entries must never be deleted or overwritten
- unmanaged name conflicts should be reported as skipped or partial
- `.git` must not be copied

Derive target entry directory names from stable skill IDs, not raw display names.

- [ ] **Step 4: Re-run the focused agent tests to get GREEN**

Run: `cargo test --manifest-path src-tauri/Cargo.toml agents -- --nocapture`

Expected:

- all new config/discovery/sync tests pass
- targeted output is clean enough to continue to command wiring

## Task 4: Expose Thin Tauri Commands And TypeScript Wrappers

**Files:**
- Create: `src-tauri/src/commands/agents.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src/lib/tauri.ts`

- [ ] **Step 1: Add thin Rust commands**

Create commands for:

- `get_agent_inventory`
- `set_agent_enabled`
- `set_agent_path_override`
- `clear_agent_path_override`
- `apply_agent_sync`

Command behavior:

- config and discovery logic stay in `core/agents`
- apply orchestration stays in `core/agents/sync.rs`
- commands only load the app config dir / repo path and map errors to strings

- [ ] **Step 2: Register the new commands**

Update:

- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/lib.rs`

Do not let command registration sprawl beyond normal file-size bounds.

- [ ] **Step 3: Add TypeScript DTOs and wrappers**

In `src/lib/tauri.ts`, add TypeScript interfaces and invoke wrappers for:

- agent inventory items
- inventory snapshot
- apply result summary
- apply response

Keep names close to the Rust DTOs to reduce mismatch risk.

- [ ] **Step 4: Run a compile pass before frontend UI work**

Run:

- `cargo check --manifest-path src-tauri/Cargo.toml`
- `npm run build`

Expected:

- Rust command surface compiles
- TypeScript compiles with the new API surface even though the new page is not wired yet

## Task 5: Add The Agents View And Stable Agent Components

**Files:**
- Create: `src/views/AgentsView.tsx`
- Create: `src/components/agents/AgentSyncSummary.tsx`
- Create: `src/components/agents/AgentTargetCard.tsx`
- Create: `src/components/agents/AgentApplyResults.tsx`
- Modify: `src/context/AppContext.tsx`
- Modify: `src/App.tsx`
- Modify: `src/components/AppShell.tsx`

- [ ] **Step 1: Extend `AppContext` with agent inventory orchestration**

Add app-level state and actions for:

- `agentInventory`
- `isLoadingAgents`
- `isApplyingAgentSync`
- `updatingAgentKey`
- `lastAgentApplyResult`
- `refreshAgents`
- `setAgentEnabled`
- `setAgentPathOverride`
- `clearAgentPathOverride`
- `applyAgentSync`

Keep heavy I/O behind Tauri commands and keep pure display calculations out of the view where reasonable.

- [ ] **Step 2: Add the new `Agents` view to shell navigation**

Update:

- `src/App.tsx`
- `src/components/AppShell.tsx`

So the app now routes among:

- `skills`
- `agents`
- `settings`

- [ ] **Step 3: Build the page from stable components**

Add:

- `AgentSyncSummary.tsx` for repo summary + apply action
- `AgentTargetCard.tsx` for per-agent toggle/path override editing
- `AgentApplyResults.tsx` for latest per-agent result summaries
- `AgentsView.tsx` to assemble the page and pass props only

The page should display inventory even when repo path is unset, but apply must clearly require a configured repo path.

- [ ] **Step 4: Run the frontend compile pass**

Run: `npm run build`

Expected:

- no missing props or DTO mismatches
- new agent page compiles cleanly

## Task 6: Add i18n Text And Update Module Docs

**Files:**
- Modify: `src/i18n/en.json`
- Modify: `src/i18n/zh.json`
- Create: `docs/modules/tauri/core/agents/mod.md`
- Create: `docs/modules/tauri/core/agents/catalog.md`
- Create: `docs/modules/tauri/core/agents/config.md`
- Create: `docs/modules/tauri/core/agents/discovery.md`
- Create: `docs/modules/tauri/core/agents/manifest.md`
- Create: `docs/modules/tauri/core/agents/sync.md`
- Create: `docs/modules/tauri/core/agents/sync_tests.md`
- Create: `docs/modules/tauri/commands/agents.md`
- Create: `docs/modules/frontend/views/AgentsView.md`
- Create: `docs/modules/frontend/components/agents/AgentSyncSummary.md`
- Create: `docs/modules/frontend/components/agents/AgentTargetCard.md`
- Create: `docs/modules/frontend/components/agents/AgentApplyResults.md`
- Modify: `docs/modules/tauri/core/mod.md`
- Modify: `docs/modules/tauri/commands/mod.md`
- Modify: `docs/modules/tauri/lib.md`
- Modify: `docs/modules/frontend/App.md`
- Modify: `docs/modules/frontend/components/AppShell.md`
- Modify: `docs/modules/frontend/context/AppContext.md`
- Modify: `docs/modules/frontend/lib/tauri.md`

- [ ] **Step 1: Add user-facing copy for agent inventory and apply results**

Include strings for:

- navigation
- summary labels
- copy-only/manual apply wording
- path mode badges
- override buttons
- apply status/result labels
- missing-path guidance

- [ ] **Step 2: Add module docs for every new source file**

Create one-to-one docs for each new `agents` source file and each new frontend agents component/view file.

- [ ] **Step 3: Update docs for modified source files**

Refresh the existing docs so they mention:

- the new `agents` view in `App`/`AppShell`
- new agent orchestration in `AppContext`
- new Tauri DTOs and commands
- new `agents` module export and registration

## Task 7: Run Focused And Full Verification

**Files:**
- No source additions; verification only

- [ ] **Step 1: Re-run the focused Rust suite**

Run: `cargo test --manifest-path src-tauri/Cargo.toml agents -- --nocapture`

Expected:

- PASS for config, discovery, and sync tests

- [ ] **Step 2: Run the repo verify gate**

Run: `npm run verify`

Expected:

- module-doc coverage passes
- module-doc diff check passes
- architecture checks pass
- frontend build passes
- `cargo check` passes
- `cargo test` passes

- [ ] **Step 3: Re-run the full Rust suite explicitly**

Run: `cargo test --manifest-path src-tauri/Cargo.toml`

Expected:

- PASS for the entire Rust test suite

- [ ] **Step 4: Final scope audit**

Confirm before handoff:

- no Git UI functionality was added
- no scene/project-local assignment was added
- no auto-sync/background watch was added
- `settings.rs` still only owns `repo_path`
- sync remains copy-only
- only `Codex`, `Claude Code`, and `OpenCode` are supported
- all touched source files have matching module docs

## Self-Review Checklist

- Spec coverage: the plan covers catalog/config/discovery/sync, thin commands, frontend inventory/apply UX, i18n, docs, and required validation.
- Placeholder scan: no `TODO`, `TBD`, or “implement later” markers remain.
- Type consistency: the plan consistently uses inventory/config/apply concepts and keeps `settings.rs` separate from agent config persistence.
