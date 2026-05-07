# Agent Skill Selection

> **Source**: `src-tauri/src/core/agents/selection.rs`
> **Status**: [REVIEW]

## Overview

Resolves each agent's desired skill list from direct selections, selected scenes, exclusions, project-local layers, and the global hard-disable gate.

## Public Surface

This module is `pub(crate)` to the agents domain and exposes selection context loading, structured global selection resolution, layered project selection resolution, and a compatibility helper for older sync callers.

## Core Logic

Loads repository skills, repo-scoped disabled skill IDs, and scene definitions, then computes each agent's global selection from direct skill references plus selected scenes. Scene membership flows through `SceneEntry::includes_skill`, so legacy all-except-disabled scenes and explicit selected-skill scenes stay compatible.

`resolve_agent_skill_selection` returns a `SkillResolutionResult` rather than a bare skill vector. Each `ResolvedSkillEntry` carries the `SkillSummary`, source labels, exclusion state, and global disabled state. Global labels distinguish direct agent picks from scene-derived picks.

`resolve_project_agent_skill_selection` starts from the non-excluded, non-disabled entries in the matching global agent result, adds the project agent's direct skills and scenes, then marks project exclusions on the final layered result. Project labels distinguish direct project picks from project-scene picks, and project exclusions stay scoped to that project target.

The result also carries stale-reference diagnostics. Missing scene IDs are reported when an agent still points at a deleted scene, missing skill IDs are reported for stale skill references, and globally disabled references are tracked separately so callers can explain why a referenced skill did not sync.

`resolve_agent_skills` remains as the compatibility wrapper for existing global sync orchestration. It calls the structured resolver and returns only entries that are neither excluded nor globally disabled, preserving the previous target-writing behavior while newer callers can inspect diagnostics and source labels.

## Interactions

Used by agent and project sync modules so high-level apply orchestration stays focused on ledger and target reconciliation rather than selection rules. Agent sync still consumes the wrapper; project sync consumes the structured global result plus the layered project resolver.

## Current Note

The current branch change for this module is formatting-only; runtime behavior is unchanged.
