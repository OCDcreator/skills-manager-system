# ProjectAssistantSourceRail

> **Source**: `src/components/assistant/ProjectAssistantSourceRail.tsx`
> **Status**: [DRAFT]

## Overview

Right-side context rail showing scope label, index freshness, document count, chunk count, warnings, and latest-answer sources.

## Import Relationships

```text
Upstream: src/components/assistant/ProjectAssistantPanel.tsx
Downstream: src/lib/assistant.ts (types only)
```

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectAssistantSourceRail` | Context status and source list display. |

## Core Logic

Renders loading, unavailable, or live status states. Scope label is split on ` + ` and rendered as individual chips. Timestamps are formatted into local date/time strings with invalid-date fallback. Lists source files with title and path when available. Surfaces context scan warnings with an amber alert section when `status.warnings` is non-empty. The rail renders as a bordered card with `self-start` so it uses its natural height without stretching to the chat column.

## Data Flow

- Receives `status`, `statusError`, `isLoadingStatus`, and `sources` as props
- Pure display component with no internal state or side effects

## Interactions

i18n keys: `assistant.contextTitle`, `assistant.statusLoading`, `assistant.statusUnavailable`, `assistant.documentCount`, `assistant.chunkCount`, `assistant.indexedAt`, `assistant.warningsTitle`, `assistant.sourcesTitle`, `assistant.sourcesEmpty`

## Configuration

None.
