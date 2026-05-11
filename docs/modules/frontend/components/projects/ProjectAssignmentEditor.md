# Project Assignment Editor

> **Source**: `src/components/projects/ProjectAssignmentEditor.tsx`
> **Status**: [REVIEW]

## Overview

Multi-agent project-layer editor. Renders a vertical agent card grid, a primary project-skill checklist with three-state indicators, and a project-scene panel. Skills and scenes can be toggled in batch for all selected agents at once.

## Props

| Prop | Type | Purpose |
|---|---|---|
| `skills` | `SkillSummary[]` | Pre-filtered skill list for the editor. |
| `agents` | `AgentInventoryItem[]` | Pre-filtered agent list for the selector grid. |
| `scenes` | `SceneEntry[]` | Available scenes. |
| `agentDrafts` | `Record<string, ProjectAgentDraft>` | Per-agent draft state keyed by agent key. |
| `selectedAgentKeys` | `string[]` | Currently selected agent keys. |
| `skillQuery` / `agentQuery` | `string` | Free-text search state. |
| `skillPathFilter` | `SkillPathFilter` | Repository-path bucket filter. |
| `externalGroupFilter` | `ExternalGroupFilter` | External source group filter. |
| `skillSelectionFilter` | `ProjectSkillSelectionFilter` | `all` / `selected` / `unselected`. |
| `agentStatusFilter` | `ProjectAgentStatusFilter` | `all` / `enabled` / `disabled`. |
| `skillPathSummaries` | `SkillPathSummary[]` | Bucket counts for the toolbar. |
| `externalGroupSummaries` | `ExternalGroupSummary[]` | Group counts for the toolbar. |
| `onSkillQueryChange` … `onAgentStatusFilterChange` | callbacks | Forward filter changes to parent. |
| `onToggleAgent` | `(agentKey: string) => void` | Toggle an agent in/out of selection. |
| `onToggleProjectSkill` | `(skillId: string) => void` | Batch-toggle a skill for all selected agents. |
| `onToggleProjectScene` | `(sceneId: string) => void` | Batch-toggle a scene for all selected agents. |

## Responsibilities

- receives already-filtered `skills`, `agents`, and `scenes`
- renders a vertical grid of `AgentSelectorCard` components (click-to-select, not checkbox-based)
- shows `all / enabled / disabled` pills for agent status filtering
- displays selection count and contextual hint text above the agent grid
- renders project-skill toggles as a dense grid with three-state indicators (`all` → solid check, `some` → minus, `none` → empty)
- renders project-scene toggles with the same three-state pattern
- disables skill and scene interaction when no agents are selected (reduced opacity)
- delegates the skill search, path filters, selected-state filters, and external group popover to `ProjectSkillFilterToolbar`
- gives agent and skill searches explicit placeholder text and a search icon
- caps the agent grid and skill/scene panels with viewport-aware maximum heights
- uses the shared `skill-markdown-scroll` surface for all scroll areas
- remembers scroll positions independently across remounts via `useRememberedScrollPosition`
- weights the desktop workbench toward skills (1.45fr) while keeping scenes in a narrow secondary column (0.75fr)

## Internal Components

| Component | Purpose |
|---|---|
| `AgentSelectorCard` | Single agent card with brand icon, name, `projectSkillsDirRule`, and selection indicator. |
| `ProjectChecklistPanel` | Titled panel wrapper with primary/secondary overflow variants. |
| `ProjectSkillList` | Three-state skill grid that reads `agentDrafts` to compute per-skill state. |
| `SearchField` | Compact labeled search input with icon. |

## Three-State Logic

Both `getSkillCheckState` and `getSceneCheckState` return `"all" | "some" | "none"` by checking how many selected agents include the given ID. When no agents are selected, both return `"none"`.

## Interaction Notes

The agent selector uses a vertical responsive grid (`repeat(auto-fill, minmax(10rem, 1fr))`) instead of a horizontal scroll strip. Clicking an agent card toggles its membership in `selectedAgentKeys`. The skill and scene toggles operate in batch mode: clicking a skill/scene applies the toggle across all selected agents. If the item is already selected by all agents, clicking deselects it from all; if some or none have it, clicking selects it for all.

Filter state types come from `src/lib/project-filters.ts`, while draft mutation types come from `src/lib/project-draft.ts`. The parent view uses `filterProjectSkillsForMultiAgent` and batch functions like `toggleProjectAgentSkillForAgents` to drive multi-agent state.
