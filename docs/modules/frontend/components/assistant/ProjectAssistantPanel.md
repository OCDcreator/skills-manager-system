# ProjectAssistantPanel

> **Source**: `src/components/assistant/ProjectAssistantPanel.tsx`
> **Status**: [DRAFT]

## Overview

Wide floating chat panel with a main chat area and right context rail. Wider (up to 860px) and taller than the initial prototype, with a responsive fixed height. Assistant messages render Markdown; user messages stay plain text. Manages local message state, ask lifecycle, loading, and error handling.

## Import Relationships

```text
Upstream: src/components/assistant/ProjectAssistantLauncher.tsx
Downstream: src/lib/assistant.ts, src/components/assistant/AssistantMarkdown.tsx, src/components/assistant/ProjectAssistantSourceRail.tsx
```

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectAssistantPanel` | Chat panel with message flow and context rail. |

## Core Logic

Maintains local `messages`, `sources`, `draft`, `isAsking`, and `askError` state. Enter sends the message (Shift+Enter inserts newline). On submit, calls `askProjectAssistant()`, appends user and assistant messages, and updates sources. Errors are shown inline without clearing prior messages. Welcome message is sourced from i18n (`assistant.welcomeMessage`).

## Data Flow

- Receives `status`, `statusError`, `isLoadingStatus` from launcher
- Calls `askProjectAssistant(question)` from `src/lib/assistant.ts`
- Passes status and sources into `ProjectAssistantSourceRail`

## Interactions

i18n keys: `assistant.title`, `assistant.subtitle`, `assistant.welcomeMessage`, `assistant.closePanel`, `assistant.inputPlaceholder`, `assistant.inputHint`, `assistant.answerLoading`, `assistant.send`

## Change Notes

Panel uses `items-start` on the grid so the right rail uses its natural height, while the left chat column uses `self-stretch` and `h-full` to fill the panel down to the bottom.

## Configuration

None.
