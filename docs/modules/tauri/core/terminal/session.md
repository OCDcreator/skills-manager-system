# terminal::session

> **Source**: `src-tauri/src/core/terminal/session.rs`
> **Status**: [DRAFT]

## Overview

Owns the single active PTY-backed assistant session.

## Responsibilities

- replace an old session when a new launch begins
- track the PTY child, writer, master, and output buffer
- expose snapshot, drain, write, resize, and stop operations
- refresh the stored status when the child process exits

## Current Note

The current branch change for this module is formatting-only; runtime behavior is unchanged.
