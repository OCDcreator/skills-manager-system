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
- prefer Windows-native launch targets such as `.cmd` and `.exe` before a bare
  npm shim name so PTY startup does not hand `CreateProcessW` a non-launchable
  script file
- on macOS, keep the bare executable first but also try common Homebrew and
  `/usr/local` absolute paths so Finder-launched GUI sessions with sparse
  `PATH` values can still find installed CLIs
