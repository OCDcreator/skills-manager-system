# ProjectAssistantLauncher

> **Source**: `src/components/assistant/ProjectAssistantLauncher.tsx`
> **Status**: [DRAFT]

## Overview

Renders the fixed floating assistant button in the bottom-right corner, owns open/close state, and preloads assistant context status before the panel is used.

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

On first open, prefetches `getAssistantContextStatus()` and passes status and errors into the panel. The launcher button toggles between a robot icon and a close icon. When the panel is closed, the hint chip stays collapsed by default and only fades/slides in while the floating launcher group is hovered or keyboard-focused.

## Data Flow

- Calls `getAssistantContextStatus()` from `src/lib/assistant.ts`
- Passes status and status errors into `ProjectAssistantPanel`
- Mounts from `src/components/AppShell.tsx`

## Interactions

i18n keys: `assistant.launcherLabel`, `assistant.launcherHint`

## Configuration

None.
