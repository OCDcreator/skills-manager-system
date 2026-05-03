# terminal.rs

> **Source**: `src-tauri/src/commands/terminal.rs`
> **Status**: [DRAFT]

## Overview

Thin Tauri command layer for the Project Assistant terminal runtime.

## Responsibilities

- expose launcher-preference load/save plus snapshot, start, drain, write,
  resize, and stop commands
- resolve the desktop app config directory for launcher-preference commands
- bootstrap the terminal config workspace before saving preferences
- delegate all session logic to `core::terminal::session::TerminalState`
- convert backend errors into string responses that the frontend can surface
