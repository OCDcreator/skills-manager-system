# Skill Documents

> **Source**: `src-tauri/src/core/skills/documents.rs`
> **Status**: [REVIEW]

## Overview

Reads a single skill directory's `SKILL.md`, validates the requested relative path, and returns document content plus metadata for the frontend.

## Import Relationships

```text
Upstream: src-tauri/src/commands/skills.rs
Downstream: src-tauri/src/core/skills/metadata.rs, src-tauri/src/core/skills/scan.rs, std::fs
```

## Public Surface

| Export | Purpose |
|---|---|
| `SkillDocument` | Serializable document payload for the frontend detail panel. |
| `read_skill_document` | Loads content and metadata for a repo-relative skill directory. |

## Core Logic

The module rejects parent-directory, root, and prefix path components before reading `SKILL.md`. It classifies source type from `custom/` or `external/`, derives a fallback name from the directory name, and builds the stable skill id with `build_skill_id`. The returned `content` remains the raw document text, including YAML frontmatter when present, so the frontend can decide how to preview it.

## Data Flow

Command input supplies `relative_path`; the module reads the filesystem and metadata parser output, then returns a serialized `SkillDocument`.

## Interactions

Must stay aligned with scanner id generation, TypeScript `SkillDocument` fields, and scene skill matching.

## Configuration

Only `custom/` and `external/` relative paths are accepted.

## Change Notes

Path traversal validation is the security boundary for document reads; keep tests updated when path rules change. If the markdown preview contract changes, document whether frontmatter stays raw, is transformed for presentation, or is hidden before updating the frontend reader.
