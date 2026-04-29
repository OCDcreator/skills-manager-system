# Frontend Bundle Optimization Design

## Summary

This pass reduces first-load pressure in the React frontend without changing product scope or the existing visual language. The current production build emits a single `index-*.js` bundle around `1.45 MB` minified (`454 KB` gzip), and the largest real contributors are not business logic files but eager imports:

- `App.tsx` statically imports every top-level view, so all page code ships in the first bundle.
- `AppShell.tsx` always mounts the project assistant launcher, whose static import chain pulls in the assistant panel and markdown renderer.
- `SkillDetailPanel.tsx` and `AssistantMarkdown.tsx` both import `highlight.js` and `marked`, which currently bundle the full highlight language set into the app.

The goal is not to chase a vanity number by fragmenting everything. It is to move genuinely non-initial code out of the main path while keeping the app structure understandable.

## Product Rules

- Default first render should still land on the Skills page.
- Existing navigation, assistant launcher behavior, and skill-detail rendering must continue to work.
- The optimization should favor view-level and interaction-level lazy loading over fine-grained micro-splitting.
- Shared markdown/highlight dependencies should become a dedicated secondary chunk rather than staying in the main entry bundle.
- No architecture regressions: avoid turning large files into many thin pass-through wrappers.

## Chosen Approach

### 1. Lazy-load non-default top-level views

`App.tsx` keeps `SkillsView` eager because it is the default landing surface. All other top-level views become `React.lazy(...)` imports resolved only when the user navigates to them. The shell stays mounted; only the view body suspends.

This cuts the main bundle by removing Git, Scenes, Projects, Settings, Agents, and External Sources code from the initial path.

### 2. Lazy-load skill detail rendering only after selection

The Skills page currently imports `SkillDetailPanel` immediately, which drags markdown/highlight support into the initial bundle even when no skill is selected. The page should render a lightweight placeholder until a skill is selected, and only then load the detail panel module.

This preserves the side-panel layout while removing markdown parsing and syntax highlighting from the default render path.

### 3. Lazy-load the assistant panel on demand

The floating launcher remains present so the app still looks and behaves the same, but the heavy assistant panel module loads only when the user opens it. This pulls assistant markdown/highlight code out of the main bundle without changing the launcher affordance.

### 4. Add one deliberate vendor markdown chunk

Vite should place `marked` and `highlight.js` into a shared `vendor-markdown` chunk via `manualChunks`. That keeps the heavy markdown stack out of the main bundle while avoiding an explosion of tiny vendor files.

## Tradeoffs

- First navigation to a lazy view will show a short loading fallback.
- First skill-detail open and first assistant open will also suspend briefly while their chunks load.
- This pass optimizes first-load pressure more than total downloaded bytes across a long session. A user who touches every surface will still eventually download most code.

These tradeoffs are acceptable because the user explicitly asked for lower main-bundle pressure and because the loading boundaries align with natural user actions.

## Validation

Success should be measured with real production output, not assumptions:

- `npm run build` must show a smaller main entry chunk and additional lazy chunks.
- The new chunk layout should demonstrate that markdown/highlight code moved out of the initial entry path.
- `npm run verify` must still pass.
