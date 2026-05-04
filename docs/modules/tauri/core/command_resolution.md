# Command Resolution Core

> **Source**: `src-tauri/src/core/command_resolution.rs`
> **Status**: [DRAFT]

## Overview

Builds platform-aware executable candidates for Git and other CLI launches.

## Public Surface

| Export | Purpose |
|---|---|
| `candidate_executable_paths` | Returns candidate executable paths for one command name. |
| `command_candidate_strings` | Returns the same candidates as strings for launcher DTOs. |
| `resolved_command` | Creates a `std::process::Command`, preferring an existing absolute candidate and otherwise falling back to the bare command. |
| `git_command` | Convenience wrapper for resolved Git commands. |
| `command_output` | Executes a command through its candidate list and attaches candidate diagnostics to spawn failures. |
| `CommandResolution` | Captures attempted candidates and formats readable spawn diagnostics. |

## Core Logic

Windows candidates keep `.cmd`, `.exe`, then bare command order for npm-style
CLI shims. Non-Windows platforms keep the bare command first. macOS also adds
common Homebrew, `/usr/local`, system bin, and sbin locations so GUI-launched
apps with sparse `PATH` values can still find installed tools. Resolution keeps
the PATH-resolvable candidate first and only falls back to absolute locations
when PATH cannot resolve the command.

## Interactions

Used by Git core modules for Git subprocess creation and by the terminal
launcher for CLI candidate lists. Callers keep their existing environment and
argument setup while attaching `CommandResolution` diagnostics to spawn errors.
