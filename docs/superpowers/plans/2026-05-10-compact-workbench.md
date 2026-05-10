# Compact Workbench Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the compact workbench responsive model so `skills-manager-system` remains genuinely usable at `900px` while preserving the existing wide desktop workbench above `1280px`.

**Architecture:** The rollout starts at the shell layer and then applies page-specific compaction rules. `AppShell` and `foundation.css` own the breakpoint map (`<768`, `768-899`, `900-1279`, `>=1280`), while `Skills`, `Agents`, `Scenes`, and the remaining views adopt compact-first content priorities without introducing a new global responsive state system.

**Tech Stack:** React 19, TypeScript, Tailwind utility classes, shared CSS in `src/styles/foundation.css`, Node `--test` source-contract tests, `npm run build`, `npm run verify`

---

## File Structure

### Shell and shared responsive foundation

- Modify: `src/components/AppShell.tsx`
- Modify: `src/styles/foundation.css`
- Modify: `src/App.tsx`
- Modify: `src/components/assistant/ProjectAssistantLauncher.tsx`
- Modify: `src/components/UnsavedChangesDialog.tsx`
- Modify: `scripts/app-shell-design.test.mjs`
- Modify: `docs/modules/frontend/App.md`
- Modify: `docs/modules/frontend/components/AppShell.md`
- Modify: `docs/modules/frontend/components/assistant/ProjectAssistantLauncher.md`
- Modify: `docs/modules/frontend/components/UnsavedChangesDialog.md`
- Modify: `docs/modules/frontend/styles/foundation.md`

### Skills compact behavior

- Modify: `src/views/SkillsView.tsx`
- Modify: `src/components/skills/SkillList.tsx`
- Modify: `src/components/skills/SkillFilters.tsx`
- Modify: `src/i18n/en.json`
- Modify: `src/i18n/zh.json`
- Create: `scripts/skills-layout.test.mjs`
- Modify: `docs/modules/frontend/views/SkillsView.md`
- Modify: `docs/modules/frontend/components/skills/SkillList.md`
- Modify: `docs/modules/frontend/components/skills/SkillFilters.md`

### Agents compact behavior

- Modify: `src/views/AgentsView.tsx`
- Modify: `src/components/agents/AgentTargetsSection.tsx`
- Modify: `src/components/agents/AgentFloatingNav.tsx`
- Create: `src/components/agents/AgentCompactTabs.tsx`
- Modify: `src/i18n/en.json`
- Modify: `src/i18n/zh.json`
- Modify: `scripts/agent-layout.test.mjs`
- Modify: `docs/modules/frontend/views/AgentsView.md`
- Modify: `docs/modules/frontend/components/agents/AgentTargetsSection.md`
- Modify: `docs/modules/frontend/components/agents/AgentFloatingNav.md`
- Create: `docs/modules/frontend/components/agents/AgentCompactTabs.md`

### Scenes and baseline compact pages

- Modify: `src/views/ScenesView.tsx`
- Modify: `src/components/scenes/SceneCard.tsx`
- Modify: `src/views/GitView.tsx`
- Modify: `src/components/git/GitFileList.tsx`
- Modify: `src/components/git/GitDiffViewer.tsx`
- Modify: `src/views/ProjectsView.tsx`
- Modify: `src/components/projects/ProjectLayerWorkbench.tsx`
- Modify: `src/components/projects/SavedProjectsSection.tsx`
- Modify: `src/views/ExternalSourcesView.tsx`
- Modify: `src/components/external-sources/AddExternalSourceForm.tsx`
- Modify: `src/components/external-sources/ExternalSourceList.tsx`
- Modify: `src/views/SettingsView.tsx`
- Modify: `src/components/RepoPathForm.tsx`
- Modify: `scripts/scene-layout.test.mjs`
- Modify: `scripts/git-layout.test.mjs`
- Modify: `scripts/project-assignment-layout.test.mjs`
- Modify: `scripts/external-sources-ui.test.mjs`
- Create: `scripts/settings-layout.test.mjs`
- Modify: `docs/modules/frontend/views/ScenesView.md`
- Modify: `docs/modules/frontend/components/scenes/SceneCard.md`
- Modify: `docs/modules/frontend/views/GitView.md`
- Modify: `docs/modules/frontend/components/git/GitFileList.md`
- Modify: `docs/modules/frontend/components/git/GitDiffViewer.md`
- Modify: `docs/modules/frontend/views/ProjectsView.md`
- Modify: `docs/modules/frontend/components/projects/ProjectLayerWorkbench.md`
- Modify: `docs/modules/frontend/components/projects/SavedProjectsSection.md`
- Modify: `docs/modules/frontend/views/ExternalSourcesView.md`
- Modify: `docs/modules/frontend/components/external-sources/AddExternalSourceForm.md`
- Modify: `docs/modules/frontend/components/external-sources/ExternalSourceList.md`
- Modify: `docs/modules/frontend/views/SettingsView.md`
- Modify: `docs/modules/frontend/components/RepoPathForm.md`

### Test registration and final verification

- Modify: `package.json`

## Task 1: Shell Compact Breakpoint Layer

**Files:**
- Modify: `src/components/AppShell.tsx`
- Modify: `src/styles/foundation.css`
- Modify: `src/App.tsx`
- Modify: `src/components/assistant/ProjectAssistantLauncher.tsx`
- Modify: `src/components/UnsavedChangesDialog.tsx`
- Modify: `scripts/app-shell-design.test.mjs`
- Modify: `docs/modules/frontend/App.md`
- Modify: `docs/modules/frontend/components/AppShell.md`
- Modify: `docs/modules/frontend/components/assistant/ProjectAssistantLauncher.md`
- Modify: `docs/modules/frontend/components/UnsavedChangesDialog.md`
- Modify: `docs/modules/frontend/styles/foundation.md`

- [ ] **Step 1: Add failing shell breakpoint assertions**

In `scripts/app-shell-design.test.mjs`, add a new test block near the existing `foundation` assertions:

```js
test("foundation defines the compact rail breakpoint map", () => {
  const source = readSource("src/styles/foundation.css");

  assert.match(source, /@media\\s*\\(min-width:\\s*900px\\)/);
  assert.match(source, /@media\\s*\\(min-width:\\s*1280px\\)/);
  assert.match(source, /grid-template-columns:\\s*4\\.5rem minmax\\(0, 1fr\\)/);
  assert.match(source, /grid-template-columns:\\s*16\\.5rem minmax\\(0, 1fr\\)/);
  assert.match(source, /\\.app-shell__nav-label\\s*\\{/);
  assert.match(source, /display:\\s*none;/);
});

test("shell launcher and unsaved dialog stay viewport-safe in compact mode", () => {
  const launcherSource = readSource("src/components/assistant/ProjectAssistantLauncher.tsx");
  const dialogSource = readSource("src/components/UnsavedChangesDialog.tsx");

  assert.match(launcherSource, /max-\\[1279px\\]:bottom-4/);
  assert.match(launcherSource, /max-\\[1279px\\]:right-4/);
  assert.match(dialogSource, /max-w-\\[min\\(32rem,calc\\(100vw-2rem\\)\\)\\]/);
});
```

- [ ] **Step 2: Run the shell test and verify it fails for the right reason**

Run:

```powershell
node --test scripts/app-shell-design.test.mjs
```

Expected: FAIL because `foundation.css`, `ProjectAssistantLauncher.tsx`, and `UnsavedChangesDialog.tsx` do not yet contain the new compact breakpoint or viewport-safe class contracts.

- [ ] **Step 3: Add shell markup hooks for compact navigation**

In `src/components/AppShell.tsx`, update the nav item label span and mobile/desktop brand text so CSS can hide or reveal them by breakpoint:

```tsx
<button
  key={item.view}
  aria-current={isActive ? "page" : undefined}
  className="app-shell__nav-item"
  data-active={isActive ? "true" : "false"}
  onClick={() => setActiveView(item.view)}
  title={t(`tooltip.nav.${item.view}`)}
>
  <Icon className="app-shell__nav-icon" aria-hidden="true" />
  <span className="app-shell__nav-label">{t(`nav.${item.view}`)}</span>
</button>
```

Keep the existing accessible names and `title` tooltips. Do not remove the mobile header; it remains the `<900px` shell.

- [ ] **Step 4: Implement the breakpoint map in shared CSS**

In `src/styles/foundation.css`, replace the old single desktop handoff with explicit `900px` and `1280px` bands:

```css
@media (min-width: 900px) {
  .app-shell {
    display: grid;
    grid-template-columns: 4.5rem minmax(0, 1fr);
  }

  .app-shell__sidebar {
    display: flex;
    align-self: start;
    gap: 1rem;
    padding: 1rem 0.75rem;
  }

  .app-shell__mobile-header {
    display: none;
  }

  .app-shell__nav {
    flex-direction: column;
  }

  .app-shell__nav-label,
  .app-shell__subtitle {
    display: none;
  }

  .app-shell__nav-item {
    justify-content: center;
    padding: 0.65rem;
  }
}

@media (min-width: 1280px) {
  .app-shell {
    grid-template-columns: 16.5rem minmax(0, 1fr);
  }

  .app-shell__sidebar {
    gap: 1.45rem;
    padding: 1.25rem;
  }

  .app-shell__nav-label,
  .app-shell__subtitle {
    display: block;
  }

  .app-shell__nav-item {
    justify-content: flex-start;
    padding: 0.55rem 0.75rem;
  }
}
```

Keep the existing mobile header behavior below `900px`, preserve focus-ring buffer rules, and keep content padding tighter in the `900px` compact band than in the `>=1280px` band.

- [ ] **Step 5: Make auxiliary shell surfaces compact-safe**

In `src/components/assistant/ProjectAssistantLauncher.tsx` and `src/components/UnsavedChangesDialog.tsx`, apply viewport-safe classes:

```tsx
<div
  className="fixed bottom-6 right-6 z-40 group max-[1279px]:bottom-4 max-[1279px]:right-4"
  data-project-assistant-launcher
>
```

```tsx
<button
  className="relative flex h-14 w-14 items-center justify-center rounded-full border border-sky-400/40 bg-slate-900 text-sky-300 shadow-2xl shadow-sky-950/40 transition hover:border-sky-300 hover:text-sky-100 max-[1279px]:h-12 max-[1279px]:w-12"
  onClick={() => setIsOpen((current) => !current)}
  title={t("assistant.launcherLabel")}
  type="button"
>
```

```tsx
<section className="w-full max-w-[min(32rem,calc(100vw-2rem))] rounded-2xl border border-slate-700 bg-slate-900 p-6 shadow-2xl">
```

No responsive JavaScript is needed here.

- [ ] **Step 6: Keep the top-level width policy aligned with the shell**

In `src/App.tsx`, keep the wide pages on the wider content container, but make the intent explicit with a view list that still includes `skills`, `agents`, `projects`, and `sources`:

```tsx
const useWideWorkbenchWidth =
  activeView === "skills" ||
  activeView === "agents" ||
  activeView === "projects" ||
  activeView === "sources";

const contentWidthClassName = useWideWorkbenchWidth
  ? "max-w-[min(96vw,1800px)]"
  : "max-w-7xl";
```

Do not add a new global responsive state here.

- [ ] **Step 7: Re-run focused shell verification**

Run:

```powershell
node --test scripts/app-shell-design.test.mjs
npm run build
```

Expected: PASS. The shell test should now find the `900px` / `1280px` contract, and the frontend build should stay green.

- [ ] **Step 8: Update shell module docs and commit**

Update the listed module docs to describe:

- the four-band shell map
- icon-only compact rail between `900px` and `1279px`
- launcher/dialog compact constraints

Run:

```powershell
node scripts/check-module-doc-coverage.mjs
git add src/components/AppShell.tsx src/styles/foundation.css src/App.tsx src/components/assistant/ProjectAssistantLauncher.tsx src/components/UnsavedChangesDialog.tsx scripts/app-shell-design.test.mjs docs/modules/frontend/App.md docs/modules/frontend/components/AppShell.md docs/modules/frontend/components/assistant/ProjectAssistantLauncher.md docs/modules/frontend/components/UnsavedChangesDialog.md docs/modules/frontend/styles/foundation.md
git commit -m "feat: add compact shell breakpoint layer"
```

## Task 2: Skills Compact Drawer And Tighter Scanning

**Files:**
- Modify: `src/views/SkillsView.tsx`
- Modify: `src/components/skills/SkillList.tsx`
- Modify: `src/components/skills/SkillFilters.tsx`
- Modify: `src/i18n/en.json`
- Modify: `src/i18n/zh.json`
- Create: `scripts/skills-layout.test.mjs`
- Modify: `docs/modules/frontend/views/SkillsView.md`
- Modify: `docs/modules/frontend/components/skills/SkillList.md`
- Modify: `docs/modules/frontend/components/skills/SkillFilters.md`

- [ ] **Step 1: Add failing Skills compact layout tests**

Create `scripts/skills-layout.test.mjs` with:

```js
import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

function read(relativePath) {
  return fs.readFileSync(path.resolve(relativePath), "utf8");
}

test("SkillsView defines wide, compact-drawer, and narrow-stacked detail surfaces", () => {
  const source = read("src/views/SkillsView.tsx");

  assert.match(source, /data-skills-detail-wide/);
  assert.match(source, /data-skills-detail-drawer/);
  assert.match(source, /data-skills-detail-stacked/);
  assert.match(source, /min-\\[1280px\\]:grid-cols-\\[minmax\\(0,1fr\\)_clamp\\(24rem,30vw,42rem\\)\\]/);
});

test("SkillList tightens the card grid before the wide breakpoint", () => {
  const source = read("src/components/skills/SkillList.tsx");

  assert.match(source, /minmax\\(15rem,1fr\\)/);
  assert.match(source, /min-\\[1280px\\]:auto-rows-\\[13\\.5rem\\]/);
  assert.match(source, /max-\\[1279px\\]:auto-rows-\\[11rem\\]/);
});
```

- [ ] **Step 2: Run the new test and confirm it fails**

Run:

```powershell
node --test scripts/skills-layout.test.mjs
```

Expected: FAIL because `SkillsView.tsx` and `SkillList.tsx` do not yet expose the compact detail surfaces or the tighter mid-width grid.

- [ ] **Step 3: Add explicit detail-surface state in `SkillsView`**

In `src/views/SkillsView.tsx`, add a compact detail-open flag and a shared detail body:

```tsx
const [isCompactDetailOpen, setIsCompactDetailOpen] = useState(false);

const handleSelectSkill = async (skill: SkillSummary) => {
  await selectSkill(skill);
  setIsCompactDetailOpen(true);
};

const detailPanel = selectedSkill ? (
  <Suspense fallback={<SkillDetailPlaceholder message={t("skills.detail.loadingDocument")} />}>
    <SkillDetailPanel
      document={selectedDocument}
      isEnabled={selectedSkillEnabled}
      skill={selectedSkill}
    />
  </Suspense>
) : (
  <SkillDetailPlaceholder
    message={isInitialSkillLoad ? t("skills.detail.loadingInitial") : t("skills.selectPrompt")}
  />
);
```

Wire the list selection to `handleSelectSkill`, not directly to `selectSkill`.

- [ ] **Step 4: Render wide, compact, and narrow detail surfaces**

In `SkillsView.tsx`, keep the wide `<aside>` column, add a compact drawer, and add a narrow stacked section:

```tsx
<div className="grid gap-6 min-[1280px]:grid-cols-[minmax(0,1fr)_clamp(24rem,30vw,42rem)]">
  <div className="min-w-0 space-y-6">
    {/* filters, warnings, list */}
  </div>

  <aside
    className="hidden min-[1280px]:block"
    data-skills-detail-wide
  >
    {detailPanel}
  </aside>
</div>

{selectedSkill ? (
  <div
    className={`hidden min-[900px]:max-[1279px]:block ${isCompactDetailOpen ? "" : "pointer-events-none opacity-0"}`}
    data-skills-detail-drawer
  >
    <div className="fixed inset-y-0 right-0 z-40 w-[min(30rem,calc(100vw-2rem))] border-l border-slate-800 bg-slate-950/98 p-4 shadow-2xl">
      <div className="mb-3 flex items-center justify-between">
        <h2 className="text-sm font-semibold text-slate-100">{selectedSkill.name}</h2>
        <button
          className="rounded-lg border border-slate-700 px-3 py-2 text-xs text-slate-200"
          onClick={() => setIsCompactDetailOpen(false)}
          type="button"
        >
          {t("skills.detail.close")}
        </button>
      </div>
      {detailPanel}
    </div>
  </div>
) : null}

{selectedSkill ? (
  <section className="min-[900px]:hidden" data-skills-detail-stacked>
    {detailPanel}
  </section>
) : null}
```

Keep the compact drawer page-local, not a modal.

- [ ] **Step 5: Tighten the list and filter rhythm**

In `src/components/skills/SkillList.tsx`, shrink the mid-width card contract:

```tsx
<section className="flex max-h-[clamp(32rem,calc(100vh-6rem),64rem)] flex-col overflow-hidden rounded-2xl border border-slate-800 bg-slate-900 p-4">
```

```tsx
<div className="grid gap-3 max-[1279px]:auto-rows-[11rem] min-[1280px]:auto-rows-[13.5rem] [grid-template-columns:repeat(auto-fit,minmax(15rem,1fr))] min-[1280px]:[grid-template-columns:repeat(auto-fit,minmax(18rem,1fr))]">
```

In `src/components/skills/SkillFilters.tsx`, make the search and refresh controls wrap cleanly:

```tsx
<div className="flex flex-wrap gap-3">
  <input className="min-w-[16rem] flex-1 rounded-xl border border-slate-700 bg-slate-950 px-4 py-3 text-sm text-slate-100 outline-none focus:border-sky-400" />
  <button className="rounded-xl border border-slate-700 bg-slate-800 px-4 py-3 text-sm text-slate-100 disabled:opacity-60" />
</div>
```

Add `skills.detail.close` to both locale files.

- [ ] **Step 6: Re-run focused Skills verification**

Run:

```powershell
node --test scripts/skills-layout.test.mjs
node --test scripts/skills-filters.test.mjs
npm run build
```

Expected: PASS. The new layout test should pass, the existing filters test should stay green, and the build should succeed.

- [ ] **Step 7: Update module docs and commit**

Update the Skills docs to explain:

- wide persistent detail
- `900px` to `1279px` drawer
- `<900px` stacked detail
- tighter card density in the compact band

Run:

```powershell
node scripts/check-module-doc-coverage.mjs
git add src/views/SkillsView.tsx src/components/skills/SkillList.tsx src/components/skills/SkillFilters.tsx src/i18n/en.json src/i18n/zh.json scripts/skills-layout.test.mjs docs/modules/frontend/views/SkillsView.md docs/modules/frontend/components/skills/SkillList.md docs/modules/frontend/components/skills/SkillFilters.md
git commit -m "feat: compact the Skills workbench"
```

## Task 3: Agent Tab Bar And Compact Single-Agent Focus

**Files:**
- Modify: `src/views/AgentsView.tsx`
- Modify: `src/components/agents/AgentTargetsSection.tsx`
- Modify: `src/components/agents/AgentFloatingNav.tsx`
- Create: `src/components/agents/AgentCompactTabs.tsx`
- Modify: `src/i18n/en.json`
- Modify: `src/i18n/zh.json`
- Modify: `scripts/agent-layout.test.mjs`
- Modify: `docs/modules/frontend/views/AgentsView.md`
- Modify: `docs/modules/frontend/components/agents/AgentTargetsSection.md`
- Modify: `docs/modules/frontend/components/agents/AgentFloatingNav.md`
- Create: `docs/modules/frontend/components/agents/AgentCompactTabs.md`

- [ ] **Step 1: Add failing agent compact tests**

In `scripts/agent-layout.test.mjs`, add:

```js
test("AgentsView switches from floating rail padding to compact tab focus below 1280px", () => {
  const viewSource = fs.readFileSync(path.resolve("src/views/AgentsView.tsx"), "utf8");
  const navSource = fs.readFileSync(path.resolve("src/components/agents/AgentFloatingNav.tsx"), "utf8");
  const sectionSource = fs.readFileSync(path.resolve("src/components/agents/AgentTargetsSection.tsx"), "utf8");

  assert.match(viewSource, /min-\\[1280px\\]:pr-12/);
  assert.match(viewSource, /AgentCompactTabs/);
  assert.match(navSource, /max-\\[1279px\\]:hidden/);
  assert.match(sectionSource, /activeAgentKey/);
});
```

- [ ] **Step 2: Run the agent layout test and watch it fail**

Run:

```powershell
node --test scripts/agent-layout.test.mjs
```

Expected: FAIL because the view still uses unconditional right padding, has no compact tabs, and the floating rail does not retreat below `1280px`.

- [ ] **Step 3: Add a compact tab bar component**

Create `src/components/agents/AgentCompactTabs.tsx` with:

```tsx
import { useTranslation } from "react-i18next";
import type { AgentInventoryItem } from "../../lib/tauri";

interface AgentCompactTabsProps {
  activeAgentKey: string | null;
  agents: Pick<AgentInventoryItem, "key" | "displayName">[];
  onSelect: (agentKey: string) => void;
}

export function AgentCompactTabs({ activeAgentKey, agents, onSelect }: AgentCompactTabsProps) {
  const { t } = useTranslation();

  return (
    <nav className="min-[1280px]:hidden" aria-label={t("agents.compactTabs.label")}>
      <div className="skill-markdown-scroll -mx-1 flex gap-2 overflow-x-auto px-1 pb-1">
        {agents.map((agent) => (
          <button
            key={agent.key}
            className={`rounded-full border px-3 py-2 text-sm whitespace-nowrap ${
              activeAgentKey === agent.key
                ? "border-sky-400/70 bg-sky-400/10 text-sky-100"
                : "border-slate-700 bg-slate-900 text-slate-300"
            }`}
            onClick={() => onSelect(agent.key)}
            type="button"
          >
            {agent.displayName}
          </button>
        ))}
      </div>
    </nav>
  );
}
```

Add `agents.compactTabs.label` to both locale files.

- [ ] **Step 4: Wire the active compact agent in `AgentsView`**

In `src/views/AgentsView.tsx`, add active-agent state and move the right padding behind the wide breakpoint:

```tsx
const [activeAgentKey, setActiveAgentKey] = useState<string | null>(null);

useEffect(() => {
  if (!activeAgentKey && sortedAgentInventory.length > 0) {
    setActiveAgentKey(sortedAgentInventory[0].key);
  }
}, [activeAgentKey, sortedAgentInventory]);
```

```tsx
return (
  <div className="space-y-6 min-[1280px]:pr-12">
    <AgentFloatingNav
      agents={floatingNavAgents}
      onOpenOrderModal={() => setIsOrderModalOpen(true)}
    />

    <AgentCompactTabs
      activeAgentKey={activeAgentKey}
      agents={sortedAgentInventory}
      onSelect={setActiveAgentKey}
    />
```

Pass `activeAgentKey` into `AgentTargetsSection`.

- [ ] **Step 5: Hide the floating rail in compact mode and focus the target list**

In `src/components/agents/AgentFloatingNav.tsx`, add the compact hide class:

```tsx
className="pointer-events-none fixed right-3 top-1/2 z-50 hidden w-96 max-w-[calc(100vw-1.5rem)] -translate-y-1/2 bg-transparent min-[1280px]:block"
```

In `src/components/agents/AgentTargetsSection.tsx`, add the new prop and compact visibility contract:

```tsx
interface AgentTargetsSectionProps {
  activeAgentKey: string | null;
  // existing props...
}
```

```tsx
const isCompactActive = activeAgentKey === null || activeAgentKey === agent.key;

<div
  id={`agent-sync-target-${agent.key}`}
  className={`${isCompactActive ? "block" : "hidden"} min-[1280px]:block scroll-mt-8`}
  key={agent.key}
>
```

Keep the existing `min-[1380px]` sidecar split; compact mode will naturally stack below that breakpoint.

- [ ] **Step 6: Re-run focused agent verification**

Run:

```powershell
node --test scripts/agent-layout.test.mjs
npm run build
```

Expected: PASS. The new compact tab bar contract should exist, and the build should still succeed.

- [ ] **Step 7: Update agent docs and commit**

Update docs to describe:

- floating rail only at `>=1280px`
- compact tab bar below `1280px`
- one-agent-at-a-time compact focus

Run:

```powershell
node scripts/check-module-doc-coverage.mjs
git add src/views/AgentsView.tsx src/components/agents/AgentTargetsSection.tsx src/components/agents/AgentFloatingNav.tsx src/components/agents/AgentCompactTabs.tsx src/i18n/en.json src/i18n/zh.json scripts/agent-layout.test.mjs docs/modules/frontend/views/AgentsView.md docs/modules/frontend/components/agents/AgentTargetsSection.md docs/modules/frontend/components/agents/AgentFloatingNav.md docs/modules/frontend/components/agents/AgentCompactTabs.md
git commit -m "feat: add compact agent tab navigation"
```

## Task 4: Scene Compact Stacking And Active-Scene Focus

**Files:**
- Modify: `src/views/ScenesView.tsx`
- Modify: `src/components/scenes/SceneCard.tsx`
- Modify: `scripts/scene-layout.test.mjs`
- Modify: `docs/modules/frontend/views/ScenesView.md`
- Modify: `docs/modules/frontend/components/scenes/SceneCard.md`

- [ ] **Step 1: Add failing Scenes compact assertions**

In `scripts/scene-layout.test.mjs`, add:

```js
test("ScenesView stacks the create form before inputs become cramped", () => {
  const viewSource = fs.readFileSync(path.resolve("src/views/ScenesView.tsx"), "utf8");
  const cardSource = fs.readFileSync(path.resolve("src/components/scenes/SceneCard.tsx"), "utf8");

  assert.match(viewSource, /grid gap-3 rounded-2xl border border-slate-800 bg-slate-900 p-4 min-\\[900px\\]:grid-cols-\\[minmax\\(0,1fr\\)_minmax\\(0,1fr\\)_auto\\]/);
  assert.match(cardSource, /data-scene-active-config/);
});
```

- [ ] **Step 2: Run the Scenes test and verify failure**

Run:

```powershell
node --test scripts/scene-layout.test.mjs
```

Expected: FAIL because `ScenesView.tsx` still uses the old horizontal create row and `SceneCard.tsx` has no explicit compact active-config hook.

- [ ] **Step 3: Convert the create form into an adaptive grid**

In `src/views/ScenesView.tsx`, replace the fixed row with:

```tsx
<div className="grid gap-3 rounded-2xl border border-slate-800 bg-slate-900 p-4 min-[900px]:grid-cols-[minmax(0,1fr)_minmax(0,1fr)_auto]">
  <div className="min-w-0">
    {/* id input */}
  </div>
  <div className="min-w-0">
    {/* name input */}
  </div>
  <button
    className="flex items-center justify-center gap-2 rounded-lg bg-sky-600 px-4 py-2 text-sm text-white hover:bg-sky-500 disabled:opacity-50 max-[899px]:w-full min-[900px]:self-end"
    disabled={creating || !newId.trim() || !newName.trim()}
    onClick={() => void handleCreate()}
    type="button"
  >
    <Plus className="h-4 w-4" />
    {t("scenes.create.button")}
  </button>
</div>
```

- [ ] **Step 4: Keep non-active scenes summary-first**

In `src/components/scenes/SceneCard.tsx`, mark the configuration surface and tighten compact visibility:

```tsx
<section
  className={`${isConfiguring ? "block" : "hidden"} mt-4 min-[1280px]:block`}
  data-scene-active-config={isConfiguring ? "true" : "false"}
>
  <SceneSkillChooser ... />
</section>
```

If the current file already uses a different wrapper around `SceneSkillChooser`, keep that structure and apply the same compact visibility rule instead of introducing a duplicate configuration block.

- [ ] **Step 5: Re-run focused scene verification**

Run:

```powershell
node --test scripts/scene-layout.test.mjs
npm run build
```

Expected: PASS. The create form should now stack cleanly and active scene configuration should stay explicit.

- [ ] **Step 6: Update scene docs and commit**

Update the view/component docs to capture:

- adaptive create form
- active-scene-first compact configuration

Run:

```powershell
node scripts/check-module-doc-coverage.mjs
git add src/views/ScenesView.tsx src/components/scenes/SceneCard.tsx scripts/scene-layout.test.mjs docs/modules/frontend/views/ScenesView.md docs/modules/frontend/components/scenes/SceneCard.md
git commit -m "feat: stack scenes for compact workbench"
```

## Task 5: Baseline Compact Pages, Test Registration, And Full Verification

**Files:**
- Modify: `src/views/GitView.tsx`
- Modify: `src/components/git/GitFileList.tsx`
- Modify: `src/components/git/GitDiffViewer.tsx`
- Modify: `src/views/ProjectsView.tsx`
- Modify: `src/components/projects/ProjectLayerWorkbench.tsx`
- Modify: `src/components/projects/SavedProjectsSection.tsx`
- Modify: `src/views/ExternalSourcesView.tsx`
- Modify: `src/components/external-sources/AddExternalSourceForm.tsx`
- Modify: `src/components/external-sources/ExternalSourceList.tsx`
- Modify: `src/views/SettingsView.tsx`
- Modify: `src/components/RepoPathForm.tsx`
- Modify: `scripts/git-layout.test.mjs`
- Modify: `scripts/project-assignment-layout.test.mjs`
- Modify: `scripts/external-sources-ui.test.mjs`
- Create: `scripts/settings-layout.test.mjs`
- Modify: `package.json`
- Modify: all listed baseline-page module docs

- [ ] **Step 1: Add failing baseline compact tests**

Update and create tests with these assertions:

```js
// scripts/git-layout.test.mjs
assert.match(source, /grid gap-6 xl:grid-cols-\\[280px_1fr\\]/);
assert.doesNotMatch(source, /lg:grid-cols-\\[280px_1fr\\]/);
```

```js
// scripts/project-assignment-layout.test.mjs
assert.match(workbenchSource, /min-\\[1380px\\]:grid-cols-\\[minmax\\(0,1fr\\)_clamp\\(22rem,28vw,34rem\\)\\]/);
assert.match(workbenchSource, /max-\\[1379px\\]:space-y-6/);
```

```js
// scripts/external-sources-ui.test.mjs
assert.match(viewSource, /xl:flex-row/);
assert.match(listSource, /grid gap-4/);
```

```js
// scripts/settings-layout.test.mjs
import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

test("SettingsView and RepoPathForm stay single-column and overflow-safe", () => {
  const settingsSource = fs.readFileSync(path.resolve("src/views/SettingsView.tsx"), "utf8");
  const formSource = fs.readFileSync(path.resolve("src/components/RepoPathForm.tsx"), "utf8");

  assert.match(settingsSource, /space-y-6 rounded-2xl border border-slate-800 bg-slate-900 p-6/);
  assert.match(formSource, /min-w-0/);
  assert.match(formSource, /break-all|truncate|overflow-x-auto/);
});
```

- [ ] **Step 2: Run the baseline tests and confirm failure**

Run:

```powershell
node --test scripts/git-layout.test.mjs
node --test scripts/project-assignment-layout.test.mjs
node --test scripts/external-sources-ui.test.mjs
node --test scripts/settings-layout.test.mjs
```

Expected: at least one FAIL in each area because the current layouts still assume earlier wide breakpoints or do not yet expose the compact-safe contract.

- [ ] **Step 3: Compact the baseline pages**

Apply these concrete layout changes:

```tsx
// src/views/GitView.tsx
<div className="grid gap-6 xl:grid-cols-[280px_1fr]">
```

```tsx
// src/components/projects/ProjectLayerWorkbench.tsx
<div className="space-y-6 max-[1379px]:space-y-6 min-[1380px]:grid min-[1380px]:items-stretch min-[1380px]:grid-cols-[minmax(0,1fr)_clamp(22rem,28vw,34rem)]">
```

```tsx
// src/views/ExternalSourcesView.tsx
<div className="flex flex-col gap-4 xl:flex-row xl:items-start xl:justify-between">
```

```tsx
// src/views/SettingsView.tsx
<section className="space-y-6 rounded-2xl border border-slate-800 bg-slate-900 p-6">
```

In `RepoPathForm.tsx`, make the path field and result rows use `min-w-0` plus path-safe overflow handling:

```tsx
<div className="min-w-0 rounded-xl border border-slate-700 bg-slate-950 px-4 py-3 text-sm text-slate-100 overflow-x-auto">
  {repoPath}
</div>
```

Use the existing component structure; do not rewrite the business logic.

- [ ] **Step 4: Register the new layout tests**

In `package.json`, add:

```json
"test:skills-layout": "node --test scripts/skills-layout.test.mjs",
"test:settings-layout": "node --test scripts/settings-layout.test.mjs",
```

Then add both scripts to the `verify` pipeline after the related page tests:

```json
"verify": "npm run check:gitnexus-freshness && npm run check:lean-ctx && npm run check:module-docs && npm run check:architecture && npm run test:architecture && npm run test:git-layout && npm run test:agent-layout && npm run test:agent-brand-icon && npm run test:bundle-splitting && npm run test:assistant-layout && npm run test:assistant-terminal && npm run test:agent-order && npm run test:skills-filters && npm run test:skills-layout && npm run test:scene-layout && npm run test:project-draft && npm run test:project-assignment-layout && npm run test:settings-layout && npm run test:scroll-memory && npm run test:external-sources-ui && npm run test:app-shell-design && npm run build && cargo check --manifest-path src-tauri/Cargo.toml && cargo test --manifest-path src-tauri/Cargo.toml"
```

- [ ] **Step 5: Run the focused baseline suite**

Run:

```powershell
node --test scripts/git-layout.test.mjs
node --test scripts/project-assignment-layout.test.mjs
node --test scripts/external-sources-ui.test.mjs
node --test scripts/settings-layout.test.mjs
npm run build
```

Expected: PASS. These tests should now enforce the baseline compact behavior for the remaining views.

- [ ] **Step 6: Update module docs and run full verification**

Update each affected baseline-page module doc to explain the compact stacking behavior and any breakpoint-specific layout split that remains.

Run:

```powershell
node scripts/check-module-doc-coverage.mjs
npm run check:module-docs
npm run check:architecture
node --test scripts/app-shell-design.test.mjs
node --test scripts/skills-layout.test.mjs
node --test scripts/agent-layout.test.mjs
node --test scripts/scene-layout.test.mjs
node --test scripts/git-layout.test.mjs
node --test scripts/project-assignment-layout.test.mjs
node --test scripts/external-sources-ui.test.mjs
node --test scripts/settings-layout.test.mjs
npm run build
npm run verify
```

Expected: every command exits `0`.

- [ ] **Step 7: Commit the baseline compact pass**

Run:

```powershell
git add src/views/GitView.tsx src/components/git/GitFileList.tsx src/components/git/GitDiffViewer.tsx src/views/ProjectsView.tsx src/components/projects/ProjectLayerWorkbench.tsx src/components/projects/SavedProjectsSection.tsx src/views/ExternalSourcesView.tsx src/components/external-sources/AddExternalSourceForm.tsx src/components/external-sources/ExternalSourceList.tsx src/views/SettingsView.tsx src/components/RepoPathForm.tsx scripts/git-layout.test.mjs scripts/project-assignment-layout.test.mjs scripts/external-sources-ui.test.mjs scripts/settings-layout.test.mjs package.json docs/modules/frontend/views/GitView.md docs/modules/frontend/components/git/GitFileList.md docs/modules/frontend/components/git/GitDiffViewer.md docs/modules/frontend/views/ProjectsView.md docs/modules/frontend/components/projects/ProjectLayerWorkbench.md docs/modules/frontend/components/projects/SavedProjectsSection.md docs/modules/frontend/views/ExternalSourcesView.md docs/modules/frontend/components/external-sources/AddExternalSourceForm.md docs/modules/frontend/components/external-sources/ExternalSourceList.md docs/modules/frontend/views/SettingsView.md docs/modules/frontend/components/RepoPathForm.md
git commit -m "feat: finish compact workbench baseline pages"
```

## Self-Review Checklist

- [ ] Spec coverage: Task 1 implements the shell map and migration strategy; Task 2 implements the Skills drawer/stacked detail plan; Task 3 implements the Agent tab-bar replacement for the floating rail; Task 4 implements adaptive Scenes behavior; Task 5 covers Git, Projects, Sources, Settings, and final verification.
- [ ] No placeholder wording remains in task steps, commands, or code snippets.
- [ ] New source files have matching module docs: `AgentCompactTabs.tsx` and its doc are paired.
- [ ] New test files are registered in `package.json` before final verification.
- [ ] Compact behavior never depends on a new global responsive state store.
- [ ] The `900px` to `1279px` band is implemented as icon rail plus compact page behavior, not as a compressed copy of the wide desktop layout.

## Execution Choice

Plan complete and saved to `docs/superpowers/plans/2026-05-10-compact-workbench.md`. Two execution options:

1. **Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration.
2. **Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints.

Which approach?
