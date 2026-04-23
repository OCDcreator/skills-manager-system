# Assistant Response

> **Source**: `src-tauri/src/core/assistant/response.rs`
> **Status**: [DRAFT]

## Overview

Assembles deterministic template answers from retrieved chunks, including source deduplication and no-match handling.

## Import Relationships

```text
Upstream: src-tauri/src/core/assistant/mod.rs
Downstream: src-tauri/src/core/assistant/context.rs (types), src-tauri/src/core/assistant/retrieval.rs (types)
```

## Public Surface

| Export | Purpose |
|---|---|
| `build_answer` | Build a template answer from context and chunks. |
| `AssistantAnswerResponse` | Full answer payload DTO. |
| `AssistantSource` | Deduplicated source file DTO. |

## Core Logic

- No-match case: returns a helpful message suggesting narrower questions.
- Match case: lists up to 3 source excerpts as bullets, framed as "I found project documentation related to..."
- `sources_from_chunks` deduplicates sources by path, keeping the highest-scoring entry.

## Data Flow

Context bundle + chunks → source deduplication → template assembly → response DTO.

## Interactions

Response shape mirrors the future cloud-model payload: `answer`, `context_summary`, `sources`, `retrieved_chunks`.

## Configuration

None.
