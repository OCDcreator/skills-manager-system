# terminal::io

> **Source**: `src-tauri/src/core/terminal/io.rs`
> **Status**: [DRAFT]

## Overview

Owns the blocking PTY output pump used by the single active assistant session.

## Responsibilities

- read PTY bytes on a dedicated thread
- append decoded output into the shared terminal buffer
- stay ignorant of session policy, launch validation, and frontend timing
