# Assistant Document Chunking

> **Source**: `src-tauri/src/core/assistant/chunking.rs`
> **Status**: [DRAFT]

## Overview

Splits raw document content into heading-based chunks so that retrieval can score and excerpt at the section level rather than the whole-document level.

## Import Relationships

```text
Upstream: src-tauri/src/core/assistant/retrieval.rs
Downstream: none
```

## Public Surface

| Export | Purpose |
|---|---|
| `DocumentChunk` | Internal chunk type with optional heading and body text. |
| `split_into_chunks` | Split markdown content at `#`/`##`/`###` heading boundaries. |

## Core Logic

Iterates lines, flushing accumulated body text whenever a markdown heading is encountered. Each chunk carries the heading that precedes it. Content before any heading becomes a heading-less chunk. Falls back to the full content as a single chunk when no headings are found.

## Data Flow

Raw markdown string → line-by-line heading scan → ordered chunks with heading + body.

## Interactions

Used by `retrieval.rs` during `retrieve_relevant_chunks`. Heading depth is limited to `#`, `##`, `###` to match typical project doc structure.

## Configuration

None.
