# App

> **Source**: `src/App.tsx`
> **Status**: [REVIEW]

## Overview

Owns the top-level React application composition. It wires the global app provider around the shell and chooses the active page view.

## Import Relationships

```text
Upstream: src/main.tsx
Downstream: src/components/AppShell.tsx, src/context/AppContext.tsx, src/views/AgentsView.tsx, src/views/GitView.tsx, src/views/ScenesView.tsx, src/views/SettingsView.tsx, src/views/SkillsView.tsx
```

## Public Surface

| Export | Purpose |
|---|---|
| `App` | Default React component mounted by `src/main.tsx`. |

## Core Logic

`AppBody` reads `activeView` from context, chooses the current page view, and can pass a view-specific content-width policy into `AppShell` (the skills browser uses a wider responsive container than the other pages). `App` wraps that body in `AppProvider`.

## Data Flow

View selection flows from `AppContext` into `AppBody`; rendered views then read the same context for their own state.

## Interactions

Must stay in sync with `AppView` values in `src/context/AppContext.tsx` and navigation labels in `src/components/AppShell.tsx`.

## Configuration

The skills browser widens the shared shell to `max-w-[min(96vw,1800px)]`; other views keep the default `max-w-7xl`.

## Change Notes

When adding a new top-level view, update `AppView`, `AppShell` navigation, and this branch selection together.
