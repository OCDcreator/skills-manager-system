# Assistant API Wrapper

> **Source**: `src/lib/assistant.ts`
> **Status**: [DRAFT]

## Overview

Typed Tauri invoke wrappers and DTOs for the project assistant context status and ask calls.

## Import Relationships

```text
Upstream: src/components/assistant/ProjectAssistantLauncher.tsx, src/components/assistant/ProjectAssistantPanel.tsx, src/components/assistant/ProjectAssistantSourceRail.tsx
Downstream: @tauri-apps/api/core
```

## Public Surface

| Export | Purpose |
|---|---|
| `AssistantContextStatus` | DTO for context scan status. |
| `AssistantSource` | DTO for a matched source file. |
| `AssistantRetrievedChunk` | DTO for a scored text chunk. |
| `AssistantAnswerResponse` | DTO for the full answer payload. |
| `getAssistantContextStatus` | Invoke wrapper for `get_assistant_context_status`. |
| `askProjectAssistant` | Invoke wrapper for `ask_project_assistant`. |

## Core Logic

Thin typed wrappers over `@tauri-apps/api/core.invoke`. No business logic.

## Data Flow

Frontend calls invoke wrappers → Tauri commands → Rust core domain.

## Interactions

Must stay in sync with `src-tauri/src/commands/assistant.rs` command names and DTO field names.

## Configuration

None.
