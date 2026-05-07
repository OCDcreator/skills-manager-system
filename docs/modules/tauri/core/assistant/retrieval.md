# Assistant Retrieval

> **Source**: `src-tauri/src/core/assistant/retrieval.rs`
> **Status**: [DRAFT]

## Overview

Splits documents into heading-based chunks, tokenizes user questions (with CJK bigram support), scores each chunk by keyword overlap, and returns the top-matching chunks.

## Import Relationships

```text
Upstream: src-tauri/src/core/assistant/mod.rs
Downstream: src-tauri/src/core/assistant/chunking.rs, src-tauri/src/core/assistant/context.rs (types only)
```

## Public Surface

| Export | Purpose |
|---|---|
| `retrieve_relevant_chunks` | Score documents and return top matches. |
| `AssistantRetrievedChunk` | Scored chunk DTO with path, heading, excerpt, score. |

## Core Logic

- `split_into_chunks` (imported from `chunking.rs`) splits document content at markdown headings, producing chunks with optional heading and body text.
- `tokenize` handles English and CJK separately: English words are lowercased and filtered to length >= 3; CJK sequences produce overlapping bigrams for matching.
- Scoring: path match +5, title match +4, heading match +3, body match +2 per token.
- Minimum score threshold is 2. Maximum 4 results returned.
- Results sorted by score descending, then path ascending.

## Data Flow

Question + documents → chunk splitting → tokenization → per-chunk scoring → sort + truncate → chunks.

## Interactions

Scoring weights, chunk splitting rules, and CJK bigram generation may need tuning when the context scope changes.

## Configuration

`MIN_SCORE` and `MAX_RESULTS` are module-level constants.

## Current Note

The current branch change for this module is formatting-only; runtime behavior is unchanged.
