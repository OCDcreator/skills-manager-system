# App

> **Source**: `src/App.tsx`
> **Status**: [REVIEW]

## Overview

Owns the top-level React composition for the desktop shell and routes the active top-level view. Only the default `SkillsView` stays eager; the other first-class pages are lazy-loaded so they do not inflate the initial bundle.

## Public Surface

| Export | Purpose |
|---|---|
| `App` | Default application component mounted from `src/main.tsx`. |

## Core Logic

`AppBody` reads `activeView` from `AppContext`, selects the matching page component, and passes a width policy into `AppShell`. `skills`, `agents`, `projects`, and `sources` use the wider `max-w-[min(96vw,1800px)]` container so their multi-column layouts do not collapse early. Non-default pages are wrapped in `Suspense` with a localized lightweight shell-level fallback, which keeps the top-level layout stable while the requested view chunk loads.

## Interactions

Must stay aligned with the `AppView` union in `src/context/navigation-guard.ts`, the nav buttons in `src/components/AppShell.tsx`, and the page exports under `src/views/`. If a new top-level page is added, decide explicitly whether it belongs in the eager path or the lazy path.
