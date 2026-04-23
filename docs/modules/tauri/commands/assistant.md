# Assistant Commands

> **Source**: `src-tauri/src/commands/assistant.rs`
> **Status**: [DRAFT]

## Overview

Thin Tauri command wrappers for the project assistant. Delegates all logic to `core::assistant`.

## Import Relationships

```text
Upstream: src-tauri/src/lib.rs (generate_handler!)
Downstream: src-tauri/src/core/assistant/mod.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `get_assistant_context_status` | Returns context scan status without documents. |
| `ask_project_assistant` | Accepts a question and returns a retrieval-backed answer. |

## Core Logic

Command layer only. Converts `anyhow::Error` to `String` for the Tauri bridge. No path resolution, scoring, or response formatting.

## Data Flow

Frontend invoke → command → `core::assistant::load_workspace_context` or `ask_workspace_question` → serialized response.

## Interactions

Must stay in sync with `src/lib/assistant.ts` invoke names and DTO field names.

## Configuration

None.
