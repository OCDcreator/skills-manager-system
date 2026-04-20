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
| `AppShell` | Layout component accepting `children` through `PropsWithChildren`. |

## Core Logic

The shell reads `activeView`, `errorMessage`, and `setActiveView` from context. It renders the translated app title/subtitle, two view buttons, and an error strip when a context-level error exists.

## Data Flow

User clicks on navigation buttons call `setActiveView`, which changes the context state consumed by `src/App.tsx`.

## Interactions

Depends on i18n keys under `app.*` and `nav.*`. Navigation choices must stay aligned with the `AppView` union.

## Configuration

Styling is Tailwind class based and scoped to this component.

## Change Notes

Do not add view-specific business logic here; this component should remain shared layout and navigation only.
