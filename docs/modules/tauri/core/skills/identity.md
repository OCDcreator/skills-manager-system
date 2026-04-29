# Skill Identity

> **Source**: `src-tauri/src/core/skills/identity.rs`
> **Status**: [REVIEW]

## Overview

Owns canonical repo-relative path normalization and source-type resolution for skill paths.

## Public Surface

| Export | Purpose |
|---|---|
| `canonicalize_repo_relative_path` | Normalizes separators and rejects traversal, root, and drive-qualified paths. |
| `resolve_source_type` | Resolves `custom` vs `external` from a canonical relative path. |
| `build_skill_id` | Builds a stable `{source_type}:{source_relative}` id. |
| `build_skill_id_from_relative_path` | Canonicalizes, resolves source type, and returns the stable skill id. |

## Core Logic

The canonicalizer converts backslashes to forward slashes, strips `.` segments, rejects `..`, absolute roots, and Windows drive-qualified paths, then rejoins the surviving path components. `resolve_source_type()` treats both plain external paths and managed mirror paths under `external/managed/...` as `external`.

## Interactions

This module is the canonical skill-path boundary for scanners, document reads, managed-mirror manifests, and external-source detection. Any change here affects skill ids across the repo.
