# AssistantLauncherMenu

> **Source**: `src/components/assistant/AssistantLauncherMenu.tsx`
> **Status**: [DRAFT]

## Overview

Renders the version-one Project Assistant launcher surface: four approved CLI
choices, a working-directory picker, launch validation feedback, and the single
launch action.

## Public Surface

| Export | Purpose |
|---|---|
| `AssistantLauncherMenu` | CLI-selection and working-directory launcher UI. |

## Core Logic

Keeps the set of supported CLI keys explicit in one local constant:
`codex`, `opencode`, `claude_code`, and `kimi`. The browse action uses the
Tauri dialog plugin to pick a directory, normalizes the returned path to
forward slashes, and leaves launch orchestration to the parent panel.

## Interactions

i18n keys: `assistant.launcherTitle`, `assistant.launcherBody`,
`assistant.cwdPlaceholder`, `assistant.launchCta`
