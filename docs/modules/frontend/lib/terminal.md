# terminal.ts

> **Source**: `src/lib/terminal.ts`
> **Status**: [DRAFT]

## Overview

Provides the typed Tauri invoke wrapper for the Project Assistant terminal
launcher and embedded session runtime.

## Responsibilities

- define the frontend `CliKey`, launcher preference, launch input, session
  snapshot, and drain DTOs
- expose invoke wrappers for launcher-preference load/save plus start, stop,
  write, resize, drain, and snapshot
- isolate terminal-runtime traffic from the older retrieval-oriented
  `src/lib/assistant.ts`
