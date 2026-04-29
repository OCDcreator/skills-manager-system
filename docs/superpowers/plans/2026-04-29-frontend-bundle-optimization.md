# Frontend Bundle Optimization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Reduce first-load/main-bundle pressure by lazy-loading non-default views, deferring markdown-heavy panels until they are needed, and extracting markdown vendor code into a dedicated chunk.

**Architecture:** Keep the current single-shell app structure intact. The app continues to use `AppContext` plus the existing top-nav view switch, but only the default view stays eager. The markdown/highlight path becomes shared secondary code used by the skill detail panel and assistant panel, and `vite.config.ts` adds one focused manual chunk rule instead of broad chunk micromanagement.

**Tech Stack:** React 19, TypeScript 5, Vite 7, Tailwind CSS 3, react-i18next

---

## File Map

- `src/App.tsx` — convert top-level non-default views to `React.lazy` with a shared loading fallback.
- `src/views/SkillsView.tsx` — lazy-load `SkillDetailPanel` only when a skill is selected.
- `src/components/assistant/ProjectAssistantLauncher.tsx` — lazy-load `ProjectAssistantPanel` when opened.
- `vite.config.ts` — add a narrow `manualChunks` rule for markdown/highlight dependencies.
- `docs/modules/frontend/App.md`
- `docs/modules/frontend/views/SkillsView.md`
- `docs/modules/frontend/components/assistant/ProjectAssistantLauncher.md`
- `docs/modules/frontend/components/AppShell.md` if the launcher-loading behavior description changes indirectly enough to merit refresh.

## Tasks

- [ ] Record the current bundle baseline from the verified build output so the before/after comparison stays grounded.
- [ ] Update `src/App.tsx` to keep `SkillsView` eager and lazy-load the remaining top-level views behind a shared fallback.
- [ ] Update `src/views/SkillsView.tsx` so the placeholder panel remains eager but `SkillDetailPanel` loads only after a skill is selected.
- [ ] Update `src/components/assistant/ProjectAssistantLauncher.tsx` so the floating launcher remains eager while the assistant panel itself loads on first open.
- [ ] Add a narrow `vendor-markdown` `manualChunks` rule in `vite.config.ts` for `marked` and `highlight.js`.
- [ ] Refresh the affected module docs so they describe the new lazy-loading boundaries.
- [ ] Run `npm run build` and inspect the emitted chunk list to confirm the main entry shrank and the markdown stack moved out of the entry chunk.
- [ ] Run `npm run verify` to confirm no functional or repo-guard regressions were introduced.
