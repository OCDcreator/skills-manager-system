# Agent Ordering Design

## Summary

The Agents surface currently renders in discovery order, which makes two things feel off:

1. enabled and disabled agents are mixed together
2. the user cannot define a stable global order that matches their workflow

This phase adds a persisted global agent ordering model for `skills-manager-system`.

The new behavior is:

- enabled agents appear above disabled agents by default
- users can open an ordering modal from the floating Agent navigation rail
- users can drag to reorder agents
- the chosen order persists across app restarts
- every UI surface that renders agent lists uses the same resolved order

This is a global presentation preference, not per-agent domain state.

## Product Goal

Deliver a small but durable global ordering system for agents:

1. make enabled agents surface above disabled agents everywhere
2. let the user customize the order of agents with drag-and-drop
3. persist that ordering in app settings
4. apply the resolved order consistently across `Agents`, `Scenes`, `Projects`, and the floating navigation rail
5. keep the current module boundaries intact: settings persistence in Rust, thin Tauri commands, and reusable ordering helpers in the frontend

## In Scope

- global persisted agent ordering
- default ordering rule of enabled-first and disabled-last
- a new floating-nav button above the existing jump-to-top arrow
- an ordering modal with drag-and-drop interaction
- frontend ordering helpers reused by all agent-list surfaces
- Tauri commands and settings-store updates required for persistence
- i18n strings for the new button, modal, and save/cancel actions
- regression tests for settings round-trip, ordering behavior, and the new nav entry
- module documentation updates required by repo guardrails

## Out of Scope

- changing the meaning of `enabled` or sync eligibility
- per-view or per-project custom agent orders
- changing scene data formats to store scene-local agent order
- introducing a drag-and-drop dependency library
- changing skill ordering logic
- auto-group headers such as “Enabled” and “Disabled” in each page
- bulk enable/disable actions

## Recommended Approach

Use the existing app settings store as the persistence boundary and keep the resolved ordering logic in a shared frontend helper.

### Why settings are the right home

- agent order is a global UI preference, similar in scope to `repoPath` and `agentSyncMode`
- it is not owned by any single agent configuration entry
- it should apply everywhere the app renders agent lists
- storing it in `settings.json` keeps CLI, desktop, and future config inspection aligned on one source of truth

### Why not store order in agent config

The existing per-agent config store owns agent-specific state:

- enabled flag
- path override
- direct skill picks
- selected scenes
- exclusions

Global list order cuts across all agents and all pages. Adding it to per-agent config would blur the boundary and create an awkward “which entry owns the list order?” problem.

### Why not use browser-only storage

Local browser storage would be fast to implement, but this repo already has a Tauri-backed settings model. Reusing that model keeps persistence consistent and testable, and avoids a split between desktop config state and frontend-only preferences.

## Architecture

### Backend

Rust extends the existing settings model:

- `src-tauri/src/core/settings.rs`
  - add `agent_order: Vec<String>` to `AppSettings`
  - add `save_agent_order(&self, agent_order: &[String]) -> Result<AppSettings>`
  - preserve backward compatibility for older `settings.json` files
- `src-tauri/src/commands/settings.rs`
  - add `get_agent_order`
  - add `set_agent_order`
- `src-tauri/src/lib.rs`
  - register both new commands

No new Rust domain module is needed. This feature belongs to app settings rather than scan, sync, or scene logic.

### Frontend

The frontend gets one shared ordering helper and one new modal component:

- `src/lib/agent-order.ts`
  - pure helpers for resolving the final order
  - pure helpers for drag reorder operations
- `src/components/agents/AgentOrderModal.tsx`
  - modal shell
  - drag-and-drop list
  - save/cancel behavior
- `src/context/AppContext.tsx`
  - load and persist the global agent order once
  - expose `agentOrder`, `setAgentOrder`, and `sortedAgentInventory`
- `src/components/agents/AgentFloatingNav.tsx`
  - new top button above the up-arrow
  - prop contract extends to include `onOpenOrderModal: () => void`
  - callback to open the modal
- `src/views/AgentsView.tsx`
  - consume sorted agents from context
  - open the ordering modal and persist updates through context
- `src/views/ScenesView.tsx`
  - use context-provided sorted agents for scene agent toggles
- `src/views/ProjectsView.tsx`
  - use context-provided sorted agents for project agent checklists

Because multiple views now need the same persisted order immediately and consistently, this phase should lift the ordering state into `AppContext` instead of letting each page perform its own asynchronous settings fetch.

## Persistence Model

`settings.json` gains a new field:

```json
{
  "repoPath": "C:/Users/lt/Desktop/Write/custom-project/my-skills",
  "agentSyncMode": "copy",
  "agentOrder": ["codex", "claude_code", "opencode", "cursor"]
}
```

### Persistence rules

- `agentOrder` stores only agent keys
- missing field loads as `[]`
- unknown keys are ignored when resolving order
- newly introduced agent keys that are not in the saved array append after saved keys within their enabled/disabled group

This keeps upgrades safe as the catalog evolves.

## Ordering Rules

Every consumer should use the same resolver:

1. partition agents into enabled and disabled groups
2. sort each group by user-defined order when present
3. for agents missing from the persisted order, preserve their catalog/inventory order after known keys in the same group
4. concatenate enabled group first, disabled group second

### Important consequence

The persisted order is global, but enabled status still wins the top-level grouping.

If the user drags a disabled agent near the top of the disabled section, it stays below all enabled agents until the agent is enabled again.

This matches the requested rule that enabled-above-disabled is the rational default everywhere.

## UI Design

### Floating Nav Entry

The floating Agent nav gains one new circular button above the existing jump-to-top arrow:

- icon: settings/order-oriented affordance
- tooltip: open agent order settings
- same rail style and hover behavior as the existing rail nodes

It opens the ordering modal and does not scroll anywhere.

Because the new button becomes the first rail item, the existing jump-to-top arrow shifts down by one slot in the nav item index sequence. The magnetic proximity animation should be visually rechecked after insertion so the top controls still feel balanced.

### Ordering Modal

The modal should be compact and purpose-built:

- title
- short explanatory copy that enabled agents always render above disabled ones
- two stacked sections:
  - enabled agents
  - disabled agents
- each item shows brand icon, display name, and a drag handle affordance
- footer actions:
  - cancel
  - save order

### Drag Interaction

Use native HTML5 drag-and-drop for the first version:

- press and drag a row
- drop it onto another row in the same rendered list
- update draft order immediately in the modal
- persist only when the user clicks save

This avoids adding a third-party dependency while still meeting the requested interaction model.

### Cross-group behavior

The modal renders enabled agents first and disabled agents second, matching the final app behavior.

Drag behavior should not allow bypassing the global enabled/disabled grouping rule. Reordering affects relative order, not group membership.

## Data Flow

### Agents View

`AgentsView` becomes the primary interaction owner of the ordering workflow:

- consume `agentOrder` and `sortedAgentInventory` from `AppContext`
- render cards and floating-nav anchors from `sortedAgentInventory`
- open the ordering modal from the nav callback
- persist the updated order through the context action that wraps `set_agent_order`

### Scenes View

`ScenesView` currently reads `agentInventory` directly from context. It should instead consume the already-resolved `sortedAgentInventory` from context so the agent toggles stay aligned with the rest of the app without a second settings fetch.

### Projects View

`ProjectsView` should consume the same context-provided `sortedAgentInventory` so project assignment checklists match the rest of the app without a second settings fetch.

### Future Surfaces

Any later page that renders agent lists should consume the same helper instead of implementing local sorting logic.

## Failure Handling

- if loading `agentOrder` fails, pages fall back to enabled-first plus inventory order and show the existing error surface
- if saving fails, keep the modal open, preserve the draft order, and show an inline error
- invalid or stale keys in persisted settings should never hard-fail the UI

## Testing Strategy

### Rust

Add focused tests to `src-tauri/src/core/settings.rs`:

- loading older settings without `agentOrder` defaults to an empty vector
- saving `agentOrder` round-trips cleanly
- saving `repoPath` or `agentSyncMode` preserves `agentOrder`

### Frontend Node Tests

Add a dedicated ordering test file, for example `scripts/agent-order.test.mjs`:

- enabled agents sort ahead of disabled agents
- persisted known keys determine relative order within each group
- unknown persisted keys are ignored
- newly added agents not in the saved order append predictably

Extend the existing `scripts/agent-layout.test.mjs`:

- floating nav contains a dedicated ordering control above the jump-top entry

### Verification

Run:

- `npm run check:module-docs`
- `npm run check:architecture`
- focused node tests for layout and ordering
- `npm run build`
- `cargo check --manifest-path src-tauri/Cargo.toml`

Full `npm run verify` is ideal if the current repo state allows it.

## Affected Files

Expected write set for implementation:

- `src/components/agents/AgentFloatingNav.tsx`
- `src/components/agents/AgentOrderModal.tsx`
- `src/views/AgentsView.tsx`
- `src/views/ScenesView.tsx`
- `src/views/ProjectsView.tsx`
- `src/lib/agent-order.ts`
- `src/lib/tauri.ts`
- `src/i18n/en.json`
- `src/i18n/zh.json`
- `src-tauri/src/core/settings.rs`
- `src-tauri/src/commands/settings.rs`
- `src-tauri/src/lib.rs`
- matching module docs under `docs/modules/...`
- focused test files under `scripts/`

## Open Decisions Resolved

- ordering is global across all views, not Agents-only
- drag-and-drop is the chosen interaction
- enabled agents always render above disabled agents
- persistence belongs in app settings, not per-agent config

## Success Criteria

This phase is complete when:

1. every agent-list surface renders enabled agents above disabled agents
2. users can open a modal from the floating nav and drag to reorder agents
3. saving the modal persists that order across app restarts
4. the same resolved order appears in `Agents`, `Scenes`, `Projects`, and the floating nav
5. tests, module docs, and repo checks are updated together
