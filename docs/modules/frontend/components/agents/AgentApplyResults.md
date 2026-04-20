# AgentApplyResults

> **Source**: `src/components/agents/AgentApplyResults.tsx`
> **Status**: [REVIEW]

## Overview

Displays the latest manual apply result for each supported agent target.

## Import Relationships

```text
Upstream: src/views/AgentsView.tsx
Downstream: src/lib/tauri.ts, src/i18n/index.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `AgentApplyResults` | Result list for the latest agent-sync apply response. |

## Core Logic

Shows an empty-state message before the first apply, then renders one result card per agent with status badge, counts, summary message, and target-path text.

## Data Flow

Consumes the `ApplyAgentSyncResponse` shape from `AppContext`/`src/lib/tauri.ts`.

## Interactions

Must stay aligned with the `AgentApplyStatus` values returned by the Rust sync command.

## Configuration

Status coloring is derived locally from the result status.

## Change Notes

Do not move apply execution logic into this component; it should remain a pure renderer.
