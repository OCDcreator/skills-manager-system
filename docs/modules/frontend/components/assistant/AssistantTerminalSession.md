# AssistantTerminalSession

> **Source**: `src/components/assistant/AssistantTerminalSession.tsx`
> **Status**: [DRAFT]

## Overview

Hosts the embedded xterm surface for the active Project Assistant session and
bridges frontend terminal behavior to the PTY-backed Tauri backend.

## Public Surface

| Export | Purpose |
|---|---|
| `AssistantTerminalSession` | Expanded assistant terminal shell with stop/restart controls. |

## Core Logic

Creates an xterm instance on mount, loads the fit addon, forwards keyboard data
to `writeTerminalInput()`, polls `drainTerminalOutput()` on a short interval,
and resizes the PTY whenever the terminal container changes size. Stop and
restart actions remain explicit UI controls instead of hidden lifecycle hooks.

## Interactions

i18n keys: `assistant.activeSessionTitle`, `assistant.stop`,
`assistant.restart`
