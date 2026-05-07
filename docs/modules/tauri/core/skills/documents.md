# Skill Documents

> **Source**: `src-tauri/src/core/skills/documents.rs`
> **Status**: [REVIEW]

## Overview

Reads a single skill directory's `SKILL.md`, validates the relative path, and returns document content plus metadata.

## Public Surface

| Export | Purpose |
|---|---|
| `SkillDocument` | Serializable detail payload for the frontend reader. |
| `read_skill_document` | Loads one repo-relative skill document. |

## Core Logic

The module canonicalizes the requested relative path through `identity.rs`, accepts both `custom/*` and `external/*` paths including managed mirrors under `external/managed/...`, reads the raw markdown text, parses metadata for fallback name/description, and returns the content unchanged. `managed_source` remains `None` here because managed-mirror enrichment happens during scanning, not document reads.

## Interactions

Must stay aligned with scanner id generation and the frontend `SkillDetailPanel` contract.

## Current Note

The current branch change for this module is formatting-only; runtime behavior is unchanged.
