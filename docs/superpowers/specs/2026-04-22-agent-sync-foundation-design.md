# Agent Sync Foundation Design

## Summary

Phase 3 adds the first real distribution workflow to `skills-manager-system`: manually syncing the repository's currently enabled skills into supported agents' global skills directories.

This phase intentionally stays narrow:

- only 3 agents: `Codex`, `Claude Code`, `OpenCode`
- only global agent skills directories
- only manual apply
- only copy mode
- no Git UI
- no scene management
- no project-local assignment
- no background watch or auto-sync

The goal is not to build the final agent-management framework. The goal is to deliver one reliable, bounded vertical slice that can safely copy enabled skills into real agent directories without touching unmanaged content.

## Product Goal

Deliver the smallest useful agent-sync workflow on top of the existing phase-one and phase-two foundations:

1. Detect or configure each supported agent's global skills directory.
2. Let the user enable or disable an agent as a sync target.
3. Compute `enabled skills = scanned skills - disabledSkillIds`.
4. Manually apply those enabled skills into enabled agents' global skills directories.
5. Track app-managed output with manifest and ledger data so re-apply can update stale managed entries without deleting foreign content.

## In Scope

- built-in agent catalog for `codex`, `claude_code`, and `opencode`
- default-path discovery for those three agents
- per-agent path override
- per-agent enabled/disabled target state
- manual apply button and result summary
- copy-only sync implementation
- manifest/ledger tracking for safe re-apply and cleanup
- thin Tauri commands and Rust core-domain implementation under `src-tauri/src/core/agents/`
- frontend `Agents` view plus stable agent-focused components
- Rust tests for config persistence, discovery behavior, and sync safety
- module-document updates required by the repo guard

## Out of Scope

- symlink implementation
- scene-based target selection
- project-local assignment such as `.claude/skills` or `.opencode/skills`
- Git status, diff, commit, push, pull, or update-script workflows
- more than the three listed agents
- background auto-sync or filesystem watching
- bulk per-skill overrides inside the agent-sync page
- writing any agent settings into `settings.json`

## Recommended Approach

Use a dedicated `agents` Rust subdomain that composes the existing skills scan and skill-state modules instead of folding agent sync into settings or scan logic.

### Why this boundary is correct

- `settings.rs` currently owns one app-level setting: `repo_path`. It should stay focused.
- `skills/scan.rs` answers discovery questions for the repository. It should not absorb agent-path detection or sync side effects.
- `skills/state.rs` already owns repo-scoped enable/disable persistence. Agent sync should consume that state, not redefine it.
- agent catalog, target config, path discovery, and safe apply rules form a new domain with its own persistence and safety invariants.

This keeps the phase-three code small but expandable. Future symlink mode, scene overlays, and project-local assignment can add new behavior around the same agent domain instead of rewriting it.

## Architecture

### Frontend

The frontend keeps the current shell/navigation pattern and adds one new top-level surface:

- `src/App.tsx` adds `Agents` to the existing view switch.
- `src/components/AppShell.tsx` adds the new navigation button and keeps global error display.
- `src/context/AppContext.tsx` remains the orchestration boundary for app-wide Tauri-backed state, now extended with:
  - agent inventory snapshot
  - per-agent config mutation actions
  - manual apply action
  - latest apply result
- `src/views/AgentsView.tsx` only assembles the page structure.
- `src/components/agents/*` render the agent summary, agent cards, and apply results.
- `src/lib/tauri.ts` exposes the new Tauri DTOs and commands.

The page layer should not calculate sync rules or path detection itself. It renders normalized backend state and calls context actions.

### Backend

Rust introduces a separate `agents` subdomain under `src-tauri/src/core/`:

- `catalog.rs` — built-in agent definitions and stable metadata
- `config.rs` — persistent `agent-config.json` load/save for enabled state and path overrides
- `discovery.rs` — path-resolution and inventory snapshot assembly
- `manifest.rs` — target-local manifest and app-local ledger helpers for safe copy-only reconciliation
- `sync.rs` — manifest/ledger handling plus copy-only apply logic

Thin Tauri commands live in `src-tauri/src/commands/agents.rs` and call into the new core modules.

The existing skills-domain modules remain unchanged in responsibility:

- `scan.rs` still discovers repository skills
- `state.rs` still persists disabled skill IDs
- agent sync consumes both to compute the current enabled-skill set

## Supported Agents

Phase 3 supports exactly these built-in agents:

- `codex` → display name `Codex`
- `claude_code` → display name `Claude Code`
- `opencode` → display name `OpenCode`

The catalog should be explicit and small. Do not introduce a generic adapter marketplace or a 10+ tool matrix in this phase.

## Path Detection Model

Each built-in agent has:

- a stable key
- a display name
- a default global skills-directory rule
- a default detect-directory rule

Recommended default rules:

- `Codex`
  - default skills dir: `~/.codex/skills`
  - detect dir: `~/.codex`
- `Claude Code`
  - default skills dir: `~/.claude/skills`
  - detect dir: `~/.claude`
- `OpenCode`
  - default skills dir rule: `.config/opencode/skills`
  - detect dir rule: `.config/opencode`
  - for `.config/...` rules, discovery should check both `HOME/.config/...` and platform config-dir equivalents, reusing the proven reference-project boundary without importing its full tool system

### Detection output

The frontend should receive, for each agent:

- `key`
- `displayName`
- `enabled`
- `defaultSkillsDir`
- `detectedSkillsDir | null`
- `pathOverride | null`
- `effectiveSkillsDir | null`
- `pathMode`

Where:

- `pathMode = "override"` when a manual override is set
- `pathMode = "detected"` when no override exists and a default path was detected on disk
- `pathMode = "missing"` when neither override nor detected default path is available

### Missing-path behavior

If the default path is not detected:

- the agent still appears in inventory
- the user may manually provide an override path
- `effectiveSkillsDir` remains `null` until detection or override makes the target path concrete
- manual apply must not guess and create output in an undetected default location

This keeps Windows/macOS behavior safe and explicit.

## Agent Config Persistence

Agent-target configuration must not live in `settings.json`.

Phase 3 adds a separate `agent-config.json` under the app config directory.

Recommended shape:

```json
{
  "agents": {
    "codex": {
      "enabled": false,
      "pathOverride": null
    },
    "claude_code": {
      "enabled": false,
      "pathOverride": null
    },
    "opencode": {
      "enabled": false,
      "pathOverride": null
    }
  }
}
```

### Default enable rule

Built-in agents default to `enabled: false`.

Reason:

- global manual sync writes into real agent directories
- the user should explicitly opt in per target before the first apply
- this avoids surprising writes on first launch while still keeping the workflow simple

## Enabled Skill Computation

The sync source for phase 3 is the repository's current enabled-skill set:

1. scan the configured repository via the existing `scan_repo_skills`
2. load repo-scoped disabled IDs via the existing `SkillStateStore`
3. compute:

```text
enabled skills = all scanned skills - disabledSkillIds
```

Disabled skills remain part of the browser but are excluded from sync output.

## Target Entry Naming

The app must avoid target-name collisions across `custom/` and recursive `external/` skills.

Phase 3 should therefore derive a stable managed target directory name from the stable skill ID instead of using the display name directly.

Recommended pattern:

- `custom:searxng` → `custom--searxng`
- `external:awesome-claude-skills/frontend-design` → `external--awesome-claude-skills--frontend-design`

Rules:

- replace `/` and `\` with `--`
- keep the result a single safe directory component
- keep the mapping deterministic across refreshes and re-apply

This makes repeated apply predictable and avoids accidental cross-source collisions.

## Manifest And Ledger Model

Phase 3 must track app-managed output with both a target-local manifest and an app-local ledger.

### Target-local manifest

Each agent target directory gets a hidden manifest file, for example:

```text
<agent-skills-dir>/.skills-manager-system-manifest.json
```

Recommended shape:

```json
{
  "appId": "skills-manager-system",
  "agentKey": "codex",
  "entries": {
    "custom--searxng": {
      "skillId": "custom:searxng",
      "relativePath": "custom/searxng"
    }
  }
}
```

This manifest is the source of truth for what the app currently manages in that specific target directory.

### App-local ledger

The app config directory also stores `agent-sync-ledger.json`.

Recommended shape:

```json
{
  "agents": {
    "codex": {
      "lastAppliedTargetDir": "C:/Users/example/.codex/skills"
    }
  }
}
```

The ledger is intentionally lightweight. Its job is to remember where the app last applied for each agent so a future apply can attempt safe cleanup if the target directory changed because of a new override.

## Apply Algorithm

Manual apply runs only when the user clicks the apply action.

For each enabled agent:

1. resolve the current inventory entry
2. if `effectiveSkillsDir` is `null`, return a skipped result with a path-missing message
3. if the ledger points at a different previous target dir, attempt old-target cleanup using that target's manifest
4. load the current target manifest, or start empty if none exists
5. compute desired managed entries from the enabled skill set
6. remove managed entries that were present in the manifest but are no longer desired
7. copy desired skills into target entry directories
8. write the updated manifest, or remove it if no managed entries remain
9. update the ledger with the current target dir

### Safety rules

These are mandatory:

- the app may update or remove entries that are recorded in its manifest
- the app must never delete content that is not recorded in its manifest
- if a desired target entry path already exists but is not app-managed, apply must skip that entry and report a conflict instead of overwriting it
- stale manifest cleanup must remove only previously managed entries, never the whole target directory

## Copy Mode Only

Phase 3 implements copy-only sync.

That means:

- directories are copied recursively
- `.git` is ignored during copy
- existing managed target entries are replaced by removing the old managed entry and copying the fresh source

Symlink behavior may be anticipated in data structures or comments, but phase 3 must not expose or execute symlink mode.

## Apply Result Model

Manual apply should return a per-agent summary that the UI can render directly.

Recommended fields:

- `key`
- `displayName`
- `targetDir | null`
- `status` (`success`, `partial`, `skipped`, `failed`)
- `writtenCount`
- `removedCount`
- `conflictCount`
- `message`

The aggregate response should also include the enabled-skill count used for that apply.

This is enough for a first useful manual workflow without building a job queue or live progress system.

## UI Design

### App Shell

Add a third navigation destination:

- `Skills`
- `Agents`
- `Settings`

### Agents View

The page should contain:

1. a summary panel showing:
   - current repository path
   - enabled-skill count derived from scan + disabled IDs
   - enabled-agent count
   - copy-only/manual-apply messaging
2. an apply button for the current enabled targets
3. one card per supported agent showing:
   - display name
   - enabled toggle
   - path status badge (`override`, `detected`, `missing`)
   - default path
   - current override input
   - effective target path
   - save override action
   - reset override action
4. a result section for the most recent apply

### UX rules

- if no repository path is configured, the page still shows agent inventory, but apply is disabled
- if no agents are enabled, apply is disabled
- if enabled agents exist but all are path-missing, apply is allowed but returns skipped results with explicit guidance, or the button may be disabled with clear text; either is acceptable as long as the state is explicit
- the page should show that this phase syncs only enabled skills and only to global directories

## Data Flow

### Inventory flow

1. app starts
2. frontend loads repo path and agent inventory
3. inventory is built from:
   - built-in agent catalog
   - persisted `agent-config.json`
   - runtime path discovery
4. UI renders inventory cards

### Config mutation flow

1. user toggles an agent or saves/resets an override
2. frontend calls a thin Tauri command
3. Rust updates `agent-config.json`
4. Rust rebuilds the inventory snapshot
5. frontend replaces its current agent inventory state

### Manual apply flow

1. user clicks apply
2. Rust loads repo path from `settings.json`
3. Rust scans repo skills and loads skill-state
4. Rust computes enabled skills
5. Rust applies copy-only sync to enabled agents using manifest/ledger safety rules
6. frontend renders result summaries

## Error Handling

### Inventory

- config-file parse failure should surface an error rather than silently discarding data
- path-missing is not an error; it is a normal inventory state

### Config mutation

- invalid override path input should return a clear validation message
- failed config writes should keep prior UI state until a fresh snapshot is returned

### Apply

- missing repo path should fail fast with a user-readable message
- missing agent path should become a skipped agent result, not a process-wide crash
- individual agent copy failures should not prevent other agents from applying; return per-agent status
- conflicts with unmanaged target content should be reported as partial/skipped rather than overwritten

## Module Boundaries

Expected source changes:

- Create: `src-tauri/src/core/agents/mod.rs`
- Create: `src-tauri/src/core/agents/catalog.rs`
- Create: `src-tauri/src/core/agents/config.rs`
- Create: `src-tauri/src/core/agents/discovery.rs`
- Create: `src-tauri/src/core/agents/manifest.rs`
- Create: `src-tauri/src/core/agents/sync.rs`
- Create: `src-tauri/src/core/agents/sync_tests.rs`
- Modify: `src-tauri/src/core/mod.rs`
- Create: `src-tauri/src/commands/agents.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`
- Create: `src/views/AgentsView.tsx`
- Create: `src/components/agents/AgentSyncSummary.tsx`
- Create: `src/components/agents/AgentTargetCard.tsx`
- Create: `src/components/agents/AgentApplyResults.tsx`
- Modify: `src/App.tsx`
- Modify: `src/components/AppShell.tsx`
- Modify: `src/context/AppContext.tsx`
- Modify: `src/lib/tauri.ts`
- Modify: `src/i18n/en.json`
- Modify: `src/i18n/zh.json`

Expected module-doc changes:

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

## Validation Strategy

Phase-three validation should proceed from focused Rust tests to the full repo gate:

1. `cargo test --manifest-path src-tauri/Cargo.toml agents -- --nocapture`
2. `npm run verify`
3. `cargo test --manifest-path src-tauri/Cargo.toml`

Rust tests should cover at least:

- agent config round-trip
- default path detection and missing-path behavior
- path override taking precedence
- apply copying only enabled skills
- disabled skills being excluded from output
- re-apply cleaning stale managed entries
- unmanaged target content not being deleted

## Future Expansion Path

This phase deliberately leaves clean room for later additions:

- symlink mode can add a second sync strategy inside `sync.rs`
- scene/project targeting can layer on top of the same inventory and manifest safety model
- additional agents can extend the catalog one definition at a time instead of requiring a framework rewrite

The key phase-three success criterion is not “maximum flexibility”. It is safe, comprehensible copy-only distribution to a very small supported target set.
