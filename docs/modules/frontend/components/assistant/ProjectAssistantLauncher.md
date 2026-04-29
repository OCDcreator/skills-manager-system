# ProjectAssistantLauncher

> **Source**: `src/components/assistant/ProjectAssistantLauncher.tsx`
> **Status**: [DRAFT]

## Overview

Renders the fixed floating assistant button in the bottom-right corner, owns open/close state, preloads assistant context status before the panel is used, and lazy-loads the heavy assistant panel only when the user opens it.

## Import Relationships

```text
Upstream: src/components/AppShell.tsx
Downstream: src/lib/assistant.ts, src/components/assistant/ProjectAssistantPanel.tsx
```

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectAssistantLauncher` | Floating robot button with panel toggle. |

## Core Logic

On first open, prefetches `getAssistantContextStatus()` and passes status and errors into the panel. The launcher button toggles between a robot icon and a close icon. When the panel is closed, the hint chip stays collapsed by default and only fades/slides in while the floating launcher group is hovered or keyboard-focused. The fixed wrapper also exposes a stable `data-*` anchor so other floating UI, such as the Agent Sync icon rail, can keep clear of the launcher instead of overlapping it.

The panel module itself is loaded through `React.lazy(...)` and shown behind a fixed-position fallback shell. That preserves the floating affordance while keeping assistant markdown/rendering code out of the initial bundle.

## Data Flow

- Calls `getAssistantContextStatus()` from `src/lib/assistant.ts`
- Passes status and status errors into the lazy `ProjectAssistantPanel`
- Mounts from `src/components/AppShell.tsx`

## Interactions

i18n keys: `assistant.launcherLabel`, `assistant.launcherHint`

## Configuration

None.
