# External Source Git Command Helpers

> **Source**: `src-tauri/src/core/external_sources/git_command.rs`
> **Status**: [REVIEW]

## Overview

Owns low-level Git process construction and output handling for the external-source Git modules.

## Public Surface

| Export | Purpose |
|---|---|
| `git_cmd` | Builds a Git command pinned to one repository path with noninteractive environment defaults. |
| `run_git` | Runs a Git command and returns trimmed UTF-8 stdout or stderr as the operation error. |
| `run_git_bytes` | Runs a Git command and returns raw stdout bytes or stderr as the operation error. |
| `readable_git_error` | Converts Git stderr into a short user-facing summary while keeping sanitized details. |

## Core Logic

The helper delegates executable resolution to `core::command_resolution::git_command()` so external-source code gets the same Windows/macOS Git fallback behavior as the rest of the application. It keeps `GIT_TERMINAL_PROMPT=0` and `LC_ALL=C` centralized for repository-scoped commands. Nonzero Git stderr is mapped into recognizable summaries for missing repositories, authentication failures, missing branches/refs, and network failures, with URL credentials and common GitHub token prefixes masked before details are returned. Spawn failures still use the resolver's candidate-aware message so missing Git installs are actionable.

## Interactions

Used by `git_repo.rs`, `git_tree.rs`, and `git_export.rs`. It is intentionally limited to process setup and output conversion; repository synchronization, tree parsing, and export semantics stay in the domain modules that call it.
