# Project Layer Workbench

> **Source**: `src/components/projects/ProjectLayerWorkbench.tsx`
> **Status**: [REVIEW]

## Overview

Composes the editable Projects workbench: project identity, multi-agent project-layer editor, and the summary/inspection surface.

## Props

| Prop | Type | Purpose |
|---|---|---|
| `draft` | `ProjectDraft` | Full draft including `agents` map. |
| `duplicatePath` | `boolean` | Path collision flag. |
| `inferredName` | `string` | Auto-derived display name. |
| `inspectionError` / `isInspecting` | — | Target inspection state. |
| `isSaving` | `boolean` | Save-in-progress flag. |
| `agents` | `AgentInventoryItem[]` | Pre-filtered agent list. |
| `scenes` | `SceneEntry[]` | Available scenes. |
| `selectedAgentKeys` | `string[]` | Currently selected agents. |
| `skillQuery` / `agentQuery` | `string` | Free-text search state. |
| `skillPathFilter` / `externalGroupFilter` / `skillSelectionFilter` / `agentStatusFilter` | — | Editor filter state. |
| `skillPathSummaries` / `externalGroupSummaries` | — | Filter toolbar data. |
| `skills` | `SkillSummary[]` | Pre-filtered skill list. |
| `targetActionId` | `string \| null` | Active target action for spinner. |
| `summary` | object | Per-agent preview counts, unsupported keys. |
| `onToggleAgent` | `(agentKey: string) => void` | Toggle agent selection. |
| `onToggleProjectSkill` / `onToggleProjectScene` | callbacks | Batch toggle for skills/scenes. |
| `onSave` / `onCancelEdit` / `onBrowseProjectPath` | callbacks | Identity actions. |
| Filter change callbacks | various | Forward filter changes from editor. |

## Responsibilities

- keeps the Projects page view focused on data loading, save/apply handlers, and draft derivation
- passes identity save controls to `ProjectIdentityPanel`
- wires `ProjectAssignmentEditor` with `draft.agents` as `agentDrafts`, filtered skills, external group summaries, and scenes
- wires `ProjectAssignmentSummary` to the derived per-agent preview counts and unsupported saved agents
- forwards target skill management callbacks and pending action state to the summary inspector
- owns the responsive workbench breakpoint where the summary stays stacked until `1380px`
- moves the save action into `ProjectIdentityPanel` instead of the summary surface

## Interactions

`ProjectsView.tsx` computes draft state, filters, inspection, and summary data before passing them in. This component does not call Tauri APIs and does not mutate draft state directly; all mutations flow through callback props.

Its compact decision is narrower than the global shell rule: even after the shell leaves the `<1280` compact band, the workbench itself still stays single-column until `min-[1380px]`. That means `<900px`, `900px-1279px`, and `1280px-1379px` all share the same stacked identity/editor/summary flow, while `>=1380px` switches to the wide editor-plus-sticky-inspector grid.

The workbench passes `draft.agents` directly to `ProjectAssignmentEditor` as `agentDrafts`, enabling the multi-agent three-state skill/scene toggles. It does not track a single active agent; all selected agents share the same skill/scene editing surface.
