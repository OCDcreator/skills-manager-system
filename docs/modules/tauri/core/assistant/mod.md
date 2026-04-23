# Assistant Core Domain

> **Source**: `src-tauri/src/core/assistant/mod.rs`
> **Status**: [DRAFT]

## Overview

Public assistant API that combines context scanning, retrieval, and answer generation into a single `ask_workspace_question` entry point.

## Import Relationships

```text
Upstream: src-tauri/src/commands/assistant.rs
Downstream: src-tauri/src/core/assistant/context.rs, src-tauri/src/core/assistant/retrieval.rs, src-tauri/src/core/assistant/response.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `ask_workspace_question` | Top-level entry: scan, retrieve, build answer. |
| `load_workspace_context` | Discover project root and load context bundle. |
| `load_context_from_root` | Load context from an explicit root path. |
| `AssistantContextStatus` | Scan status DTO. |
| `AssistantContextBundle` | Status plus loaded documents. |
| `AssistantContextDocument` | Single loaded document. |
| `AssistantAnswerResponse` | Full answer payload DTO. |
| `AssistantSource` | Matched source file DTO. |
| `AssistantRetrievedChunk` | Scored text chunk DTO. |
| `retrieve_relevant_chunks` | Standalone retrieval function. |

## Core Logic

`ask_workspace_question` validates the question, loads workspace context, runs retrieval, and assembles a deterministic template answer.

## Data Flow

Question string → context scan → keyword tokenization → chunk scoring → top-match selection → template answer assembly.

## Interactions

Context scope must match the design spec: `AGENTS.md`, `docs/**/*.md`, `package.json`, `src-tauri/Cargo.toml`.

## Configuration

None.
