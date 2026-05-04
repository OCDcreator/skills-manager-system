# AgentGlobalSkillList

> **Source**: `src/components/agents/AgentGlobalSkillList.tsx`
> **Status**: [REVIEW]

## Overview

Renders the sidecar inventory and explicit management actions for skills already present in an agent's effective global skills directory.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentGlobalSkillList` | Displays app-managed and existing unmanaged target skill entries for one agent and exposes single-entry plus batch take-over / delete / import actions. |

## Core Logic

Counts managed versus unmanaged entries, reports target scan errors, highlights taken-over entries separately from synced ones, and lists per-entry actions. The managed/unmanaged count pills also act as local visibility filters, so operators can isolate either status bucket without changing inventory data. A multi-select mode adds checkboxes to target entries and exposes batch delete, take-over, import, and import-delete commands alongside those status pills. Batch take-over and import actions only use selected unmanaged/importable entries, while delete can apply to any selected visible entry. The secondary path label is hidden when it would just repeat the bold title, and symlink target paths use a two-line clamp so managed and unmanaged links can both be inspected without wasting vertical space on duplicated labels.

The sidecar shell is a full-height flex column with hidden overflow, and the scrollable list uses `flex-1` instead of a fixed max-height so it fills the wrapper height without stretching the surrounding grid row. The list remembers its per-agent scroll position across refreshes and shows a filter-specific empty state when the current status pill hides every entry. The component never mutates files directly: it forwards explicit button clicks back to `AgentsView`, which owns confirmation dialogs and Tauri command calls.

## Data Flow

Receives `AgentInventoryItem.targetSkillEntries`, the current in-flight action key, and single-entry/batch action callbacks directly from `AgentsView`.

## Interactions

Keep aligned with `AgentTargetSkillEntry` in `src/lib/tauri.ts`, target inventory scanning in Rust, and `agents.globalSkills.*` i18n keys.
