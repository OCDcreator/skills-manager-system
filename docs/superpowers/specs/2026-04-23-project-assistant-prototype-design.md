# Project Assistant Prototype Design

## Status

Approved for design documentation on 2026-04-23.

This document defines the first prototype for a floating project assistant in
Skills Manager System. The prototype validates the user experience and local
project-document retrieval layer before connecting a cloud model.

## Goals

- Add a bottom-right floating robot launcher that is available across app views.
- Expand the launcher into a wide floating chat window.
- Show real project-document context status in the assistant window.
- Answer questions with real document retrieval plus a deterministic template response.
- Preserve a clean boundary for later cloud LLM integration.

## Non-Goals

- Do not call a real cloud model in the first prototype.
- Do not stream assistant responses yet.
- Do not index or read source code files for context in the first prototype.
- Do not add long-term assistant memory.
- Do not add provider, model, API key, or base URL settings in this prototype.

## User Experience

The assistant appears as a robot launcher fixed to the bottom-right corner of
the app shell. Clicking it opens a wide floating panel instead of navigating to a
new page. The panel should feel like a project sidekick rather than a generic
support bubble.

The approved layout is a wide assistant window with two regions:

- Main chat area: welcome message, user questions, assistant responses, loading
  state, empty state, and error state.
- Right context rail: context scope, index freshness, document count, and the
  sources used for the latest answer.

The right rail uses the approved "context status plus sources" direction. It
must make the assistant's grounding visible: what it reads, whether the index is
ready, and which files informed the latest response.

## Context Scope

The prototype scans project documentation and lightweight project metadata:

- `AGENTS.md`
- `docs/**/*.md`
- `package.json`
- `src-tauri/Cargo.toml`

The prototype intentionally excludes source files such as `src/**/*.tsx` and
`src-tauri/src/**/*.rs`. This keeps performance, privacy, and relevance risks
contained while still allowing the assistant to answer questions about project
purpose, workflow, architecture rules, module documentation, and verification.

## Retrieval Behavior

The first prototype uses simple local retrieval rather than embeddings or a
cloud model:

1. Scan the context scope and build a lightweight in-memory document index.
2. Split text into stable chunks with source path and title metadata.
3. Tokenize the user question with a conservative keyword matcher.
4. Score chunks by keyword overlap, filename relevance, heading matches, and
   exact phrase matches when available.
5. Return the top relevant chunks and source files.

The retrieval layer should be deterministic and fast. If no relevant chunks are
found, the assistant should say that the current project documents do not
contain enough evidence and suggest asking a narrower question.

## Template Response Behavior

The prototype answer generator is intentionally deterministic. It should not
pretend to be a cloud model. It should summarize the retrieved evidence with a
clear framing such as:

"I found related project documentation in these files..."

Responses should:

- Mention the most relevant source files.
- Summarize matching content in concise bullets or short paragraphs.
- Avoid unsupported claims when retrieval confidence is low.
- Encourage follow-up questions when the result is partial.

This response shape mirrors the future cloud-model payload:

- `answer`: user-facing answer text.
- `sources`: source files used for the answer.
- `retrievedChunks`: matched chunks with scores and excerpts.
- `contextSummary`: context scope and index status.

When a real cloud model is added later, the scan and retrieval layer can stay in
place while the deterministic answer generator is replaced with an LLM-backed
answer generator.

## Frontend Architecture

The assistant should be mounted from `src/components/AppShell.tsx` so it is
available in every view. Assistant UI state should stay local to the assistant
components unless a later feature needs cross-view coordination.

Planned frontend modules:

- `src/components/assistant/ProjectAssistantLauncher.tsx`: floating robot
  button, open/close affordance, and panel mounting.
- `src/components/assistant/ProjectAssistantPanel.tsx`: chat panel layout,
  message list, right context rail, input, loading state, and errors.
- `src/lib/assistant.ts`: typed Tauri API wrapper for assistant status and ask
  calls.

All new UI strings must be added to both `src/i18n/en.json` and
`src/i18n/zh.json`.

## Backend Architecture

Tauri commands stay thin and delegate to a new assistant core domain.

Planned backend modules:

- `src-tauri/src/commands/assistant.rs`: command entrypoints and error mapping.
- `src-tauri/src/core/assistant/mod.rs`: public assistant core API.
- `src-tauri/src/core/assistant/context.rs`: context file discovery and status.
- `src-tauri/src/core/assistant/retrieval.rs`: chunking, tokenization, scoring,
  and source selection.
- `src-tauri/src/core/assistant/response.rs`: deterministic template response
  generation.

The command layer should expose:

- `get_assistant_context_status`: returns scan scope, indexed document count,
  warnings, and index readiness.
- `ask_project_assistant`: accepts a user question and returns answer text,
  context summary, retrieved chunks, and sources.

The command layer must not contain path scanning, chunk scoring, or response
generation logic.

## Data Contracts

The initial response types should be cloud-model-ready without requiring the
cloud model now:

- `AssistantContextStatus`
  - `scopeLabel`
  - `indexedDocumentCount`
  - `indexedChunkCount`
  - `lastIndexedAt`
  - `warnings`
- `AssistantQuestionRequest`
  - `question`
- `AssistantAnswerResponse`
  - `answer`
  - `contextSummary`
  - `sources`
  - `retrievedChunks`
- `AssistantSource`
  - `path`
  - `title`
  - `score`
- `AssistantRetrievedChunk`
  - `path`
  - `heading`
  - `excerpt`
  - `score`

Field names should follow the existing frontend TypeScript style while Rust
types can use idiomatic snake_case with serde renames where needed.

## Error Handling

The assistant should handle these states explicitly:

- No configured project path or unreadable project root.
- Context file missing, unreadable, or too large.
- Empty question.
- No retrieval matches.
- Backend command failure.

The UI should keep the panel open on errors and show a recoverable message. A
failed ask should not clear prior messages or sources.

## Testing And Verification

Implementation should include focused validation for the new behavior:

- Rust tests for context discovery and retrieval scoring.
- Frontend build/typecheck through the existing project gate.
- Module documentation coverage checks for every new source file.
- Architecture checks to ensure no oversized or generic `utils`/`helpers` files
  are introduced.

The minimum verification gate is `npm run verify`.

## Module Documentation Requirements

Every new frontend and Rust source file in this design needs a matching module
document:

- `docs/modules/frontend/...` for `src/**/*.ts` and `src/**/*.tsx`
- `docs/modules/tauri/...` for `src-tauri/src/**/*.rs`

The implementation is not complete unless source changes and module documents
are updated together.

## Future Cloud Model Integration

The next phase can add a real cloud model by introducing a provider-backed
answer generator after retrieval:

1. Keep context discovery and retrieval unchanged.
2. Build a prompt from the question, retrieved chunks, and project instructions.
3. Send the prompt to the configured cloud model.
4. Render model output while keeping sources from the retrieval layer.

Future settings can include provider, model, API key, base URL, streaming
toggle, context limit, and whether to include lightweight metadata files.

## Open Design Decisions Resolved

- Prototype approach: half-real prototype with real retrieval and template
  answers.
- Visual layout: wide bottom-right floating assistant window.
- Right rail: context status plus latest answer sources.
- Context scope: project documentation plus lightweight project metadata.
- First answer strategy: deterministic retrieval-backed template response.
