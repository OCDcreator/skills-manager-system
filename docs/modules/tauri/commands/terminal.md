# terminal.rs

> **Source**: `src-tauri/src/commands/terminal.rs`
> **Status**: [DRAFT]

## Overview

Thin Tauri command layer for the Project Assistant terminal runtime.

## Responsibilities

- expose snapshot, start, drain, write, resize, and stop commands
- delegate all session logic to `core::terminal::session::TerminalState`
- convert backend errors into string responses that the frontend can surface
