# ProjectAssistantLauncher

> **Source**: `src/components/assistant/ProjectAssistantLauncher.tsx`
> **Status**: [DRAFT]

## Overview

Renders the fixed floating assistant button in the bottom-right corner, owns
open/close state, and lazy-loads the assistant panel only when the user opens
it.

## Import Relationships

```text
Upstream: src/components/AppShell.tsx
Downstream: src/components/assistant/ProjectAssistantPanel.tsx
```

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectAssistantLauncher` | Floating robot button with panel toggle. |

## Core Logic

The launcher button toggles between a robot icon and a close icon. When the
panel is closed, the hint chip stays collapsed by default and only fades/slides
in while the floating launcher group is hovered or keyboard-focused. The fixed
wrapper also exposes a stable `data-*` anchor so other floating UI can keep
clear of the launcher instead of overlapping it.

The panel module itself is loaded through `React.lazy(...)`. The launcher no
longer prefetches retrieval status because the assistant surface now opens into
the terminal-launcher workflow instead of the older document-chat prototype.

## Data Flow

Passes only `onClose` into the lazy `ProjectAssistantPanel`.
- Mounts from `src/components/AppShell.tsx`

## Interactions

i18n keys: `assistant.launcherLabel`, `assistant.launcherHint`

## Configuration

None.
