# Agent Ordering Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a persisted global agent order that always renders enabled agents above disabled agents, exposes a drag-to-reorder modal from the Agent floating nav, and applies the same order across Agents, Scenes, and Projects.

**Architecture:** Extend the existing Rust settings store with `agentOrder`, expose thin Tauri commands and TypeScript wrappers, and let `AppContext` own one global copy of the persisted order plus the resolved `sortedAgentInventory`. Reuse a pure frontend ordering helper and the repo’s existing native HTML5 drag-and-drop pattern so the new modal and all consumer views stay aligned without adding a new dependency.

**Tech Stack:** Tauri 2, Rust 2021, React 19, TypeScript 5, Vite 7, Tailwind CSS 3, i18next, serde_json

---

## File Map

- `src-tauri/src/core/settings.rs` — add persisted `agent_order`, save helper, and focused unit tests.
- `src-tauri/src/commands/settings.rs` — add thin `get_agent_order` / `set_agent_order` commands.
- `src-tauri/src/lib.rs` — register the new settings commands.
- `src/lib/tauri.ts` — add `AgentKey`-based wrappers for the new commands.
- `src/lib/agent-order.ts` — resolve enabled-first global ordering and handle draft drag reorder updates.
- `src/context/app-context-types.ts` — add `agentOrder`, `sortedAgentInventory`, and `saveAgentOrder`.
- `src/context/AppContext.tsx` — load one global agent order, derive sorted inventory, and persist updates.
- `src/components/agents/AgentOrderModal.tsx` — new modal with native drag-and-drop ordering UI.
- `src/components/agents/AgentFloatingNav.tsx` — add the order button above jump-top and wire `onOpenOrderModal`.
- `src/views/AgentsView.tsx` — use `sortedAgentInventory`, host the modal, and save the updated order.
- `src/views/ScenesView.tsx` — swap `agentInventory` for `sortedAgentInventory`.
- `src/views/ProjectsView.tsx` — swap `agentInventory` for `sortedAgentInventory`.
- `src/i18n/en.json` / `src/i18n/zh.json` — add nav, modal, and explanatory copy.
- `scripts/agent-order.test.mjs` — focused resolver tests.
- `scripts/agent-layout.test.mjs` — assert the new floating-nav order button contract.
- `docs/modules/frontend/components/agents/AgentOrderModal.md` — new module doc.
- `docs/modules/frontend/components/agents/AgentFloatingNav.md`
- `docs/modules/frontend/context/AppContext.md`
- `docs/modules/frontend/context/app-context-types.md`
- `docs/modules/frontend/lib/agent-order.md` — new module doc.
- `docs/modules/frontend/lib/tauri.md`
- `docs/modules/frontend/views/AgentsView.md`
- `docs/modules/frontend/views/ScenesView.md`
- `docs/modules/frontend/views/ProjectsView.md`
- `docs/modules/tauri/core/settings.md`
- `docs/modules/tauri/commands/settings.md`
- `docs/modules/tauri/lib.md`

### Task 1: Persist Global Agent Order In Rust Settings

**Files:**
- Modify: `src-tauri/src/core/settings.rs`
- Modify: `src-tauri/src/commands/settings.rs`
- Modify: `src-tauri/src/lib.rs`
- Test: `src-tauri/src/core/settings.rs`

- [ ] **Step 1: Write the failing Rust tests**

Add these tests to `src-tauri/src/core/settings.rs` inside the existing `#[cfg(test)] mod tests` block:

```rust
    #[test]
    fn load_backfills_missing_agent_order() {
        let dir = tempdir().unwrap();
        fs::write(
            dir.path().join("settings.json"),
            r#"{"repoPath":"/tmp/my-skills","agentSyncMode":"copy"}"#,
        )
        .unwrap();

        let settings = SettingsStore::new(dir.path().to_path_buf()).load().unwrap();

        assert!(settings.agent_order.is_empty());
    }

    #[test]
    fn save_agent_order_round_trips() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        store
            .save_agent_order(&[
                "codex".to_string(),
                "claude_code".to_string(),
                "opencode".to_string(),
            ])
            .unwrap();

        assert_eq!(
            store.load().unwrap().agent_order,
            vec![
                "codex".to_string(),
                "claude_code".to_string(),
                "opencode".to_string(),
            ]
        );
    }

    #[test]
    fn save_repo_path_preserves_agent_order() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        store
            .save_agent_order(&["codex".to_string(), "cursor".to_string()])
            .unwrap();
        store.save_repo_path(Some(Path::new("C:/tmp/my-skills"))).unwrap();

        assert_eq!(
            store.load().unwrap().agent_order,
            vec!["codex".to_string(), "cursor".to_string()]
        );
    }

    #[test]
    fn save_agent_sync_mode_preserves_agent_order() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        store
            .save_agent_order(&["codex".to_string(), "cursor".to_string()])
            .unwrap();
        store.save_agent_sync_mode(AgentSyncMode::Symlink).unwrap();

        assert_eq!(
            store.load().unwrap().agent_order,
            vec!["codex".to_string(), "cursor".to_string()]
        );
    }
```

- [ ] **Step 2: Run the Rust settings tests and confirm they fail**

Run: `cargo test --manifest-path src-tauri/Cargo.toml settings -- --nocapture`

Expected: FAIL with compile errors because `AppSettings` does not yet contain `agent_order` and `SettingsStore::save_agent_order` does not exist.

- [ ] **Step 3: Add the persisted field and save helper**

Update `src-tauri/src/core/settings.rs` so `AppSettings` and `SettingsStore` include the new ordering support:

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct AppSettings {
    pub repo_path: Option<String>,
    pub agent_sync_mode: AgentSyncMode,
    pub agent_order: Vec<String>,
}

impl SettingsStore {
    pub fn save_agent_order(&self, agent_order: &[String]) -> Result<AppSettings> {
        let mut settings = self.load()?;
        settings.agent_order = agent_order.to_vec();
        self.save(&settings)
    }
}
```

Update `src-tauri/src/commands/settings.rs` to add these thin commands:

```rust
#[tauri::command]
pub fn get_agent_order(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let config_dir = app_config_dir(&app)?;
    SettingsStore::new(config_dir)
        .load()
        .map(|settings| settings.agent_order)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_agent_order(
    app: tauri::AppHandle,
    agent_order: Vec<String>,
) -> Result<Vec<String>, String> {
    let config_dir = app_config_dir(&app)?;
    SettingsStore::new(config_dir)
        .save_agent_order(&agent_order)
        .map(|settings| settings.agent_order)
        .map_err(|error| error.to_string())
}
```

Register both commands in `src-tauri/src/lib.rs` next to the existing settings handlers:

```rust
            commands::settings::get_repo_path,
            commands::settings::set_repo_path,
            commands::settings::get_agent_sync_mode,
            commands::settings::set_agent_sync_mode,
            commands::settings::get_agent_order,
            commands::settings::set_agent_order,
```

- [ ] **Step 4: Re-run the Rust settings tests**

Run: `cargo test --manifest-path src-tauri/Cargo.toml settings -- --nocapture`

Expected: PASS for the existing settings tests plus the four new `agent_order` tests.

- [ ] **Step 5: Commit the Rust settings slice**

Run:

```bash
git add src-tauri/src/core/settings.rs src-tauri/src/commands/settings.rs src-tauri/src/lib.rs
git commit -m "feat: persist global agent order settings"
```

### Task 2: Add Frontend API And Global Ordering State

**Files:**
- Modify: `src/lib/tauri.ts`
- Create: `src/lib/agent-order.ts`
- Modify: `src/context/app-context-types.ts`
- Modify: `src/context/AppContext.tsx`
- Test: `scripts/agent-order.test.mjs`
- Modify: `package.json`

- [ ] **Step 1: Write the failing ordering test first**

Create `scripts/agent-order.test.mjs` with these tests:

```javascript
import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

test("agent-order helper keeps enabled-first sorting logic", () => {
  const source = fs.readFileSync(path.resolve("src/lib/agent-order.ts"), "utf8");

  assert.match(source, /if \(left\.enabled !== right\.enabled\) \{/);
  assert.match(source, /return left\.enabled \? -1 : 1/);
  assert.match(source, /const leftIndex = indexMap\.get\(left\.key as AgentKey\)/);
  assert.match(source, /if \(leftIndex != null && rightIndex != null\) \{/);
});

test("agent-order helper merges persisted keys with inventory keys", () => {
  const source = fs.readFileSync(path.resolve("src/lib/agent-order.ts"), "utf8");

  assert.match(source, /export function mergeAgentOrderWithInventory/);
  assert.match(source, /const preserved = persistedOrder\.filter/);
  assert.match(source, /const missing = agents\.map\(\(agent\) => agent\.key\)/);
  assert.match(source, /return \[\.\.\.preserved, \.\.\.missing\] as AgentKey\[\]/);
});

test("agent-order helper exposes drag reorder support", () => {
  const source = fs.readFileSync(path.resolve("src/lib/agent-order.ts"), "utf8");

  assert.match(source, /export function applyDraggedAgentOrder/);
  assert.match(source, /const next = currentOrder\.filter\(\(key\) => key !== draggedKey\)/);
  assert.match(source, /next\.splice\(targetIndex, 0, draggedKey\)/);
});
```

- [ ] **Step 2: Run the new ordering test and confirm it fails**

Run: `node --test scripts/agent-order.test.mjs`

Expected: FAIL because `src/lib/agent-order.ts` does not exist yet.

- [ ] **Step 3: Add wrappers, helper functions, and AppContext state**

Update `src/lib/tauri.ts` with these wrappers:

```ts
export const getAgentOrder = () =>
  invoke<AgentKey[]>("get_agent_order");

export const setAgentOrder = (agentOrder: AgentKey[]) =>
  invoke<AgentKey[]>("set_agent_order", { agentOrder });
```

Create `src/lib/agent-order.ts` with this core logic:

```ts
import type { AgentInventoryItem, AgentKey } from "./tauri";

function orderIndex(agentOrder: AgentKey[]) {
  return new Map(agentOrder.map((key, index) => [key, index]));
}

export function resolveSortedAgentInventory<T extends Pick<AgentInventoryItem, "key" | "enabled">>(
  agents: T[],
  agentOrder: AgentKey[],
) {
  const indexMap = orderIndex(agentOrder);

  return [...agents].sort((left, right) => {
    if (left.enabled !== right.enabled) {
      return left.enabled ? -1 : 1;
    }

    const leftIndex = indexMap.get(left.key as AgentKey);
    const rightIndex = indexMap.get(right.key as AgentKey);

    if (leftIndex != null && rightIndex != null) {
      return leftIndex - rightIndex;
    }
    if (leftIndex != null) return -1;
    if (rightIndex != null) return 1;
    return 0;
  });
}

export function applyDraggedAgentOrder(
  currentOrder: AgentKey[],
  draggedKey: AgentKey,
  targetKey: AgentKey,
) {
  const next = currentOrder.filter((key) => key !== draggedKey);
  const targetIndex = next.indexOf(targetKey);

  if (targetIndex === -1) {
    next.push(draggedKey);
    return next;
  }

  next.splice(targetIndex, 0, draggedKey);
  return next;
}

export function mergeAgentOrderWithInventory(
  agents: Pick<AgentInventoryItem, "key">[],
  persistedOrder: AgentKey[],
) {
  const knownKeys = new Set(agents.map((agent) => agent.key));
  const preserved = persistedOrder.filter((key) => knownKeys.has(key));
  const missing = agents.map((agent) => agent.key).filter((key) => !preserved.includes(key));
  return [...preserved, ...missing] as AgentKey[];
}
```

Update `src/context/app-context-types.ts`:

```ts
  agentOrder: AgentKey[];
  sortedAgentInventory: AgentInventoryItem[];
  saveAgentOrder: (agentOrder: AgentKey[]) => Promise<void>;
```

Update `src/context/AppContext.tsx` to load and expose the new state:

```ts
  const [agentOrder, setAgentOrder] = useState<api.AgentKey[]>([]);

  const refreshAgentOrder = useCallback(async () => {
    try {
      const nextOrder = await api.getAgentOrder();
      setAgentOrder(nextOrder);
    } catch (error) {
      setErrorMessage(errorMessageFrom(error));
    }
  }, []);

  const saveAgentOrder = useCallback(async (nextOrder: api.AgentKey[]) => {
    try {
      const savedOrder = await api.setAgentOrder(nextOrder);
      setAgentOrder(savedOrder);
      setErrorMessage(null);
    } catch (error) {
      const message = errorMessageFrom(error);
      setErrorMessage(message);
      throw error instanceof Error ? error : new Error(message);
    }
  }, []);

  const sortedAgentInventory = useMemo(
    () => resolveSortedAgentInventory(agentInventory, mergeAgentOrderWithInventory(agentInventory, agentOrder)),
    [agentInventory, agentOrder],
  );
```

Also call `refreshAgentOrder()` in the same startup effect family as `refreshAgents()`, and expose `agentOrder`, `sortedAgentInventory`, and `saveAgentOrder` from the context value.

Use this exact startup effect in `src/context/AppContext.tsx`:

```ts
  useEffect(() => {
    queueMicrotask(() => void refreshAgentOrder());
  }, [refreshAgentOrder]);
```

- [ ] **Step 4: Wire the new test script and verify it passes**

Update `package.json`:

```json
    "test:agent-order": "node --test scripts/agent-order.test.mjs",
    "verify": "npm run check:module-docs && npm run check:architecture && npm run test:architecture && npm run test:git-layout && npm run test:agent-layout && npm run test:assistant-layout && npm run test:agent-order && npm run build && cargo check --manifest-path src-tauri/Cargo.toml && cargo test --manifest-path src-tauri/Cargo.toml"
```

Run: `npm run test:agent-order`

Expected: PASS for the three resolver tests.

- [ ] **Step 5: Commit the ordering-state slice**

Run:

```bash
git add src/lib/tauri.ts src/lib/agent-order.ts src/context/app-context-types.ts src/context/AppContext.tsx scripts/agent-order.test.mjs package.json
git commit -m "feat: add global agent ordering state"
```

### Task 3: Add The Floating-Nav Order Button And Drag Modal

**Files:**
- Create: `src/components/agents/AgentOrderModal.tsx`
- Modify: `src/components/agents/AgentFloatingNav.tsx`
- Modify: `src/i18n/en.json`
- Modify: `src/i18n/zh.json`
- Modify: `scripts/agent-layout.test.mjs`

- [ ] **Step 1: Extend the layout test before the UI change**

Append this test to `scripts/agent-layout.test.mjs`:

```javascript
test("Agent floating nav exposes an order button above the jump-top control", () => {
  const navSource = fs.readFileSync(
    path.resolve("src/components/agents/AgentFloatingNav.tsx"),
    "utf8",
  );

  assert.match(navSource, /kind: "action"/);
  assert.match(navSource, /actionKey: "open-order-modal"/);
  assert.match(navSource, /onOpenOrderModal/);
});
```

- [ ] **Step 2: Run the layout test and confirm it fails**

Run: `node --test scripts/agent-layout.test.mjs`

Expected: FAIL because the nav does not yet have an action item or `onOpenOrderModal`.

- [ ] **Step 3: Add the modal component and nav action item**

Create `src/components/agents/AgentOrderModal.tsx` with this shape:

```tsx
import { GripVertical, X } from "lucide-react";
import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import type { AgentInventoryItem, AgentKey } from "../../lib/tauri";
import { applyDraggedAgentOrder, mergeAgentOrderWithInventory, resolveSortedAgentInventory } from "../../lib/agent-order";
import { AgentBrandIcon } from "./AgentBrandIcon";

interface AgentOrderModalProps {
  agents: AgentInventoryItem[];
  initialOrder: AgentKey[];
  isSaving: boolean;
  onClose: () => void;
  onSave: (agentOrder: AgentKey[]) => Promise<void>;
}

export function AgentOrderModal({ agents, initialOrder, isSaving, onClose, onSave }: AgentOrderModalProps) {
  const { t } = useTranslation();
  const [draftOrder, setDraftOrder] = useState<AgentKey[]>(
    mergeAgentOrderWithInventory(agents, initialOrder),
  );
  const [draggedKey, setDraggedKey] = useState<AgentKey | null>(null);

  const orderedAgents = useMemo(
    () => resolveSortedAgentInventory(agents, draftOrder),
    [agents, draftOrder],
  );

  const handleDrop = (targetKey: AgentKey) => {
    if (!draggedKey || draggedKey === targetKey) return;
    setDraftOrder((current) => applyDraggedAgentOrder(current, draggedKey, targetKey));
    setDraggedKey(null);
  };

  return (
    <div className="fixed inset-0 z-[70] flex items-center justify-center bg-slate-950/70 p-4 backdrop-blur-sm">
      <div className="w-full max-w-xl rounded-2xl border border-slate-800 bg-slate-900 shadow-2xl shadow-slate-950/60">
        <div className="flex items-center justify-between border-b border-slate-800 px-5 py-4">
          <div>
            <h2 className="text-base font-semibold text-slate-100">{t("agents.orderModal.title")}</h2>
            <p className="mt-1 text-sm text-slate-400">{t("agents.orderModal.description")}</p>
          </div>
          <button className="rounded p-2 text-slate-400 hover:bg-slate-800 hover:text-slate-100" onClick={onClose} type="button">
            <X className="h-4 w-4" />
          </button>
        </div>
        <div className="space-y-4 px-5 py-4">
          {[
            { key: "enabled", label: t("agents.orderModal.enabledSection"), items: orderedAgents.filter((agent) => agent.enabled) },
            { key: "disabled", label: t("agents.orderModal.disabledSection"), items: orderedAgents.filter((agent) => !agent.enabled) },
          ].map((section) => (
            <div key={section.key}>
              <div className="mb-2 text-xs font-medium uppercase tracking-wide text-slate-500">{section.label}</div>
              <div className="space-y-2">
                {section.items.map((agent) => (
                  <div
                    key={agent.key}
                    className="flex items-center gap-3 rounded-xl border border-slate-800 bg-slate-950/60 px-3 py-3 text-sm text-slate-200"
                    draggable
                    onDragEnd={() => setDraggedKey(null)}
                    onDragOver={(event) => event.preventDefault()}
                    onDragStart={() => setDraggedKey(agent.key)}
                    onDrop={(event) => {
                      event.preventDefault();
                      handleDrop(agent.key);
                    }}
                  >
                    <GripVertical className="h-4 w-4 cursor-grab text-slate-500 active:cursor-grabbing" />
                    <div className="grid h-8 w-8 place-items-center rounded-full border border-slate-700 bg-slate-900">
                      <AgentBrandIcon agentKey={agent.key} className="size-[65%]" />
                    </div>
                    <span className="flex-1">{agent.displayName}</span>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
        <div className="flex justify-end gap-2 border-t border-slate-800 px-5 py-4">
          <button className="rounded-lg border border-slate-700 px-4 py-2 text-sm text-slate-200" onClick={onClose} type="button">
            {t("agents.orderModal.cancel")}
          </button>
          <button className="rounded-lg bg-sky-500 px-4 py-2 text-sm font-semibold text-slate-950 disabled:opacity-60" disabled={isSaving} onClick={() => void onSave(draftOrder)} type="button">
            {isSaving ? t("agents.orderModal.saving") : t("agents.orderModal.save")}
          </button>
        </div>
      </div>
    </div>
  );
}
```

Update `src/components/agents/AgentFloatingNav.tsx` so the nav item union includes an action entry:

```tsx
interface AgentFloatingNavProps {
  agents: Pick<AgentInventoryItem, "key" | "displayName">[];
  onOpenOrderModal: () => void;
}

interface AgentFloatingNavItem {
  href?: string;
  label: string;
  kind: "jump" | "agent" | "action";
  actionKey?: "open-order-modal";
  agentKey?: string;
  direction?: "up" | "down";
}
```

Build `navItems` with the new first item:

```tsx
      {
        label: t("agents.sideNav.order"),
        kind: "action",
        actionKey: "open-order-modal",
      },
```

And render the action branch with a button instead of an anchor:

```tsx
                {item.kind === "action" ? (
                  <button
                    className="relative ml-auto block h-14 w-14 text-right transition-all duration-300 ease-out focus-visible:outline-none"
                    onBlur={() => setActiveIndex(null)}
                    onClick={onOpenOrderModal}
                    onFocus={() => setActiveIndex(index)}
                    onMouseEnter={() => setActiveIndex(index)}
                    style={{ opacity: motion.opacity, transform: `translateX(${motion.pull}px)` }}
                    title={item.label}
                    type="button"
                  >
                    {/* reuse the same label bubble */}
                  </button>
                ) : (
                  <a ... />
                )}
```

Add i18n keys:

```json
"agents.sideNav.order": "Order agents",
"agents.orderModal.title": "Agent order",
"agents.orderModal.description": "Drag to reorder agents. Enabled agents always stay above disabled agents.",
"agents.orderModal.enabledSection": "Enabled agents",
"agents.orderModal.disabledSection": "Disabled agents",
"agents.orderModal.cancel": "Cancel",
"agents.orderModal.save": "Save order",
"agents.orderModal.saving": "Saving…"
```

- [ ] **Step 4: Re-run the layout test and confirm it passes**

Run: `node --test scripts/agent-layout.test.mjs`

Expected: PASS for the existing hover-hit-area checks plus the new nav-order-button assertion.

- [ ] **Step 5: Commit the modal and nav slice**

Run:

```bash
git add src/components/agents/AgentOrderModal.tsx src/components/agents/AgentFloatingNav.tsx src/i18n/en.json src/i18n/zh.json scripts/agent-layout.test.mjs
git commit -m "feat: add agent ordering modal"
```

### Task 4: Integrate Sorted Agents Across Views

**Files:**
- Modify: `src/views/AgentsView.tsx`
- Modify: `src/views/ScenesView.tsx`
- Modify: `src/views/ProjectsView.tsx`

- [ ] **Step 1: Update AgentsView to consume sorted inventory and host the modal**

In `src/views/AgentsView.tsx`, change the context destructure:

```tsx
    agentInventory,
    agentOrder,
    sortedAgentInventory,
    saveAgentOrder,
```

Add modal state and save orchestration:

```tsx
  const [isOrderModalOpen, setIsOrderModalOpen] = useState(false);
  const [isSavingAgentOrder, setIsSavingAgentOrder] = useState(false);

  const handleSaveAgentOrder = useCallback(async (nextOrder: api.AgentKey[]) => {
    setIsSavingAgentOrder(true);
    try {
      await saveAgentOrder(nextOrder);
      setIsOrderModalOpen(false);
    } finally {
      setIsSavingAgentOrder(false);
    }
  }, [saveAgentOrder]);
```

Replace all `agentInventory`-ordered rendering with `sortedAgentInventory`:

```tsx
      <AgentFloatingNav
        agents={sortedAgentInventory}
        onOpenOrderModal={() => setIsOrderModalOpen(true)}
      />
```

And:

```tsx
            {sortedAgentInventory.map((agent) => {
```

Mount the modal near the bottom of the view:

```tsx
      {isOrderModalOpen ? (
        <AgentOrderModal
          agents={agentInventory}
          initialOrder={agentOrder}
          isSaving={isSavingAgentOrder}
          onClose={() => setIsOrderModalOpen(false)}
          onSave={handleSaveAgentOrder}
        />
      ) : null}
```

- [ ] **Step 2: Update ScenesView and ProjectsView to use sorted agents**

In `src/views/ScenesView.tsx`:

```tsx
  const { repoPath, scanResult, sortedAgentInventory, refreshAgents, refreshSkills } = useAppContext();
  const agents = sortedAgentInventory;
```

In `src/views/ProjectsView.tsx`:

```tsx
  const { repoPath, scanResult, sortedAgentInventory } = useAppContext();
  const agents = sortedAgentInventory;
```

These are the only list-ordering changes needed in those views because the downstream components already accept ordered `agents` arrays.

- [ ] **Step 3: Run the focused frontend checks**

Run:

```bash
npm run test:agent-order
node --test scripts/agent-layout.test.mjs
```

Expected: PASS for both suites.

- [ ] **Step 4: Commit the integration slice**

Run:

```bash
git add src/views/AgentsView.tsx src/views/ScenesView.tsx src/views/ProjectsView.tsx
git commit -m "feat: apply global agent ordering across views"
```

### Task 5: Sync Module Docs And Verify The Whole Slice

**Files:**
- Modify: `docs/modules/frontend/components/agents/AgentFloatingNav.md`
- Create: `docs/modules/frontend/components/agents/AgentOrderModal.md`
- Modify: `docs/modules/frontend/context/AppContext.md`
- Modify: `docs/modules/frontend/context/app-context-types.md`
- Create: `docs/modules/frontend/lib/agent-order.md`
- Modify: `docs/modules/frontend/lib/tauri.md`
- Modify: `docs/modules/frontend/views/AgentsView.md`
- Modify: `docs/modules/frontend/views/ScenesView.md`
- Modify: `docs/modules/frontend/views/ProjectsView.md`
- Modify: `docs/modules/tauri/core/settings.md`
- Modify: `docs/modules/tauri/commands/settings.md`
- Modify: `docs/modules/tauri/lib.md`

- [ ] **Step 1: Update and add module docs**

Use these content anchors while updating docs:

```md
- `AppContext` now owns `agentOrder`, `sortedAgentInventory`, and `saveAgentOrder`, loading one settings-backed order list and reusing it across all views.
- `AgentFloatingNav` now accepts `onOpenOrderModal` and renders an action node above the jump-top control.
- `AgentOrderModal` provides native HTML5 drag-and-drop ordering and preserves the enabled-first grouping rule.
- `settings.rs` now persists `agentOrder` beside `repoPath` and `agentSyncMode`.
- `commands/settings.rs` now exposes `get_agent_order` and `set_agent_order` as thin wrappers.
```

- [ ] **Step 2: Run repo checks**

Run:

```bash
npm run check:module-docs
npm run check:architecture
npm run test:agent-order
node --test scripts/agent-layout.test.mjs
npm run build
cargo check --manifest-path src-tauri/Cargo.toml
```

Expected:

- `check:module-docs`: OK
- `check:architecture`: no blocking issues
- ordering/layout tests: PASS
- `build`: PASS
- `cargo check`: PASS

- [ ] **Step 3: Run full verify if the worktree state allows it**

Run: `npm run verify`

Expected: PASS, unless pre-existing unrelated failures remain in the repo. If verify fails for an unrelated pre-existing issue, capture the exact failing command and stop before cleanup.

- [ ] **Step 4: Commit docs and verification updates**

Run:

```bash
git add docs/modules scripts package.json
git commit -m "docs: sync agent ordering module docs"
```

## Self-Review

- Spec coverage: this plan covers persisted `agentOrder`, enabled-first sorting, the new floating-nav button, the drag modal, all-view integration, tests, and docs.
- Placeholder scan: no task contains `TODO`, `TBD`, or “implement later”.
- Type consistency: plan uses `agentOrder` in settings/TS, `sortedAgentInventory` in context/views, and `onOpenOrderModal` in the nav contract consistently.
