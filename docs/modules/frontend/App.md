# App

> **Source**: `src/App.tsx`
> **Status**: [REVIEW]

## Overview

Owns the top-level React composition for the desktop shell and routes the active top-level view.

## Public Surface

| Export | Purpose |
|---|---|
| `App` | Default application component mounted from `src/main.tsx`. |

## Core Logic

`AppBody` reads `activeView` from `AppContext`, selects the matching page component, and passes a width policy into `AppShell`. `skills`, `agents`, `projects`, and the new `sources` page use the wider `max-w-[min(96vw,1800px)]` container so their multi-column layouts do not collapse early.

## Interactions

Must stay aligned with the `AppView` union in `src/context/navigation-guard.ts`, the nav buttons in `src/components/AppShell.tsx`, and the page exports under `src/views/`.
