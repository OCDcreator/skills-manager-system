# ProjectAssistantPanel

> **Source**: `src/components/assistant/ProjectAssistantPanel.tsx`
> **Status**: [DRAFT]

## Overview

Wide floating assistant panel that now acts as a mode switch between the
version-one CLI launcher menu and the active embedded terminal session.

## Import Relationships

```text
Upstream: src/components/assistant/ProjectAssistantLauncher.tsx
Downstream: src/lib/terminal.ts, src/components/assistant/AssistantLauncherMenu.tsx, src/components/assistant/AssistantTerminalSession.tsx
```

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectAssistantPanel` | Floating terminal-launcher panel orchestrator. |

## Core Logic

Maintains the selected CLI key, chosen working directory, launch input, launch
error, default config-workspace path, and the active terminal session snapshot.
On mount, asks the backend for both launcher preferences and any already-running
session so reopening the panel can reattach to the single session while still
remembering the last non-default working directory. When no session exists,
renders `AssistantLauncherMenu`; once a session starts, renders
`AssistantTerminalSession`.

## Data Flow

Calls `getTerminalLauncherPreferences()`, `setTerminalWorkingDirectoryPreference()`,
`getTerminalSession()`, and `startTerminalSession()` from `src/lib/terminal.ts`.
The panel persists the working-directory preference on launch and close, then
passes launch/session state into the two dedicated assistant subcomponents.

## Interactions

i18n keys: `assistant.title`, `assistant.subtitle`, `assistant.closePanel`

## Configuration

None.
