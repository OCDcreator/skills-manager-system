# terminal::launcher

> **Source**: `src-tauri/src/core/terminal/launcher.rs`
> **Status**: [DRAFT]

## Overview

Validates terminal launch input and translates approved CLI keys into real
local executable candidates.

## Responsibilities

- reject missing or invalid working directories
- reject zero-sized terminal dimensions
- map `codex`, `opencode`, `claude_code`, and `kimi` onto candidate binary
  names without coupling that logic to the frontend
