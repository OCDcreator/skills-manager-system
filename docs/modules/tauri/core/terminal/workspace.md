# terminal::workspace

> **Source**: `src-tauri/src/core/terminal/workspace.rs`
> **Status**: [DRAFT]

## Overview

Bootstraps the Project Assistant config workspace and derives launcher
preferences from persisted settings.

## Responsibilities

- create the config-root `agents.md` guide on first run
- create `.codex/.claude/.opencode/.kimi` `skills` directories inside the app
  config workspace
- seed the built-in `my-skills` management and external-source intake skills
- derive the default-vs-saved terminal working-directory preference payload

## Notes

This module is intentionally separate from `session.rs`. PTY session state owns
runtime process management, while `workspace.rs` owns the file-backed config
workspace that the launcher should open by default.
