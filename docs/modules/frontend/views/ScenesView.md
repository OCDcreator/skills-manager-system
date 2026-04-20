# Scenes View

> **Source**: `src/views/ScenesView.tsx`
> **Status**: [DRAFT]

## Overview

Page-level view for scene management. Supports creating, editing, deleting, duplicating, configuring, and applying scenes.

## Import Relationships

```text
Upstream: context/AppContext, lib/scenes
Downstream: App.tsx (routing)
```

## Public Surface

| Export | Purpose |
|---|---|
| `ScenesView` | Scene management page component |

## Core Logic

Manages local state for scene config, editing mode, configure mode, creation form, and apply results. Auto-loads scene config on mount. Delegates per-scene rendering to `components/scenes/SceneCard` and wires toggle handlers for disabled skill IDs, enabled agent keys, and skill priority ordering via `handleMoveSkill`.

## Interactions

- `context/AppContext` — reads repoPath, scanResult, agentInventory
- `components/scenes/SceneCard` — scene card UI and configure panels
- `lib/scenes` — all API calls
- `i18n` keys: `scenes.*`
