# Skill Metadata

> **Source**: `src-tauri/src/core/skills/metadata.rs`
> **Status**: [REVIEW]

## Overview

Parses optional YAML frontmatter from a skill `SKILL.md` file to extract name and description metadata.

## Import Relationships

```text
Upstream: src-tauri/src/core/skills/scan.rs, src-tauri/src/core/skills/documents.rs
Downstream: std::fs, serde_yaml
```

## Public Surface

| Export | Purpose |
|---|---|
| `SkillMetadata` | Optional parsed name and description values. |
| `parse_skill_metadata` | Reads a `SKILL.md` path and returns parsed metadata or defaults. |

## Core Logic

The parser trims leading whitespace, requires an opening `---`, finds the next `---`, and attempts to deserialize only `name` and `description`. Missing or invalid frontmatter returns default metadata instead of failing the scan.

## Data Flow

Scanner and document reader pass `SKILL.md` paths in; metadata flows into summaries and detail payloads.

## Interactions

Parsing behavior affects skill names shown in `SkillList` and `SkillDetailPanel`.

## Configuration

No configurable fields; only `name` and `description` are currently read.

## Change Notes

If new frontmatter fields are surfaced, update Rust structs, TypeScript payloads, UI rendering, and parser tests.
