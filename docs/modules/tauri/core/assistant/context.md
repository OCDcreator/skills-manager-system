# Assistant Context Discovery

> **Source**: `src-tauri/src/core/assistant/context.rs`
> **Status**: [DRAFT]

## Overview

Discovers the workspace root, filters assistant-allowed files, reads document content, and returns context status plus loaded documents.

## Import Relationships

```text
Upstream: src-tauri/src/core/assistant/mod.rs
Downstream: walkdir, time, serde, anyhow
```

## Public Surface

| Export | Purpose |
|---|---|
| `load_context_from_root` | Scan from an explicit root path. |
| `load_workspace_context` | Scan from `current_dir()`, walking up to find project root. |
| `AssistantContextStatus` | Scan status with counts and warnings. |
| `AssistantContextDocument` | Loaded document with path, title, content. |
| `AssistantContextBundle` | Status plus documents. |

## Core Logic

- `discover_project_root` walks ancestors looking for both `AGENTS.md` and `src-tauri/Cargo.toml`.
- `collect_context_paths` gathers direct files and walks `docs/**/*.md` recursively.
- `title_from_path` extracts the first `# ` heading or falls back to the filename.
- Non-readable files become warnings instead of aborting the scan.
- Chunk count is derived from paragraph-split content for status reporting.

## Data Flow

Root path → ancestor walk → file collection → content read → status + documents.

## Notes

- Allowed files: `AGENTS.md`, `docs/**/*.md`, `package.json`, `src-tauri/Cargo.toml`
- Uses `Path::join` with separate segments for Windows compatibility

## Configuration

None.
