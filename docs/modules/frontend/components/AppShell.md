# AppShell

> **Source**: `src/components/AppShell.tsx`
> **Status**: [REVIEW]

## Overview

Provides the shared page chrome for the desktop app, including the header, navigation buttons, global error banner, and main content container.

## Import Relationships

```text
Upstream: src/App.tsx
Downstream: src/context/AppContext.tsx, src/i18n/index.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `AppShell` | Layout component accepting `children` and an optional content-width class override. |

## Core Logic

The shell reads `activeView`, `errorMessage`, and `setActiveView` from context. It renders the translated app title/subtitle, five view buttons (skills, agents, git, scenes, settings), and an error strip when a context-level error exists. A caller may override the shared content-width class so a specific page can use a wider responsive frame.

## Data Flow

User clicks on navigation buttons call `setActiveView`, which changes the context state consumed by `src/App.tsx`.

## Interactions

Depends on i18n keys under `app.*` and `nav.*`. Navigation choices must stay aligned with the `AppView` union.

## Configuration

Styling is Tailwind class based and scoped to this component. If no width override is supplied, the shell uses `max-w-7xl`.

## Change Notes

Do not add view-specific business logic here; this component should remain shared layout and navigation only. Width overrides should stay at the container-policy level rather than introducing per-view UI logic into the shell.
