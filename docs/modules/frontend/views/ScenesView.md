# Scenes View

> **Source**: `src/views/ScenesView.tsx`
> **Status**: [DRAFT]

## Overview

Page-level view for scene management. Supports creating, editing, deleting, and applying scenes.

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

Manages local state for scene config, editing mode, creation form, and apply results. Auto-loads scene config on mount. SceneCard sub-component renders individual scenes with inline edit, delete, and apply actions.

## Interactions

- `context/AppContext` — reads repoPath, scanResult, agentInventory
- `lib/scenes` — all API calls
- `i18n` keys: `scenes.*`
