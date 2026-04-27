# Agent Selection Helpers

> **Source**: `src/lib/agent-selection.ts`
> **Status**: [REVIEW]

## Overview

Provides frontend-only helpers for editable agent drafts, dirty checking, and effective skill preview calculation.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentConfigDraft` | Editable per-agent frontend draft shape. |
| `AgentSkillPreviewItem` / `AgentSelectionPreview` | Derived preview types for the agent card. |
| `draftFromAgent` / `draftToConfig` | Convert between inventory DTOs and save payloads. |
| `isAgentDraftDirty` | Compares a draft against the persisted inventory baseline. |
| `toggleId` | Shared sorted toggle helper for IDs. |
| `resolveAgentSelectionPreview` | Computes deduped direct+scene skill preview with exclusions and global hard-disable flags. |

## Core Logic

Normalizes ID arrays, preserves trimmed path input, and resolves `direct ∪ scenes - exclusions` in the same shape the UI needs for unsaved previews without replacing Rust as the source of sync truth. Scene-derived contributions now honor the same selection-mode helper used by the Scenes UI, so legacy scenes and explicit-empty scenes preview the same way they sync.
