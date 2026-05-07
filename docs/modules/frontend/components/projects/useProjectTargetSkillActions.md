# Project Target Skill Actions Hook

> **Source**: `src/components/projects/useProjectTargetSkillActions.ts`
> **Status**: [REVIEW]

## Overview

Action hook for project-local target skill management in the Projects workbench.

## Responsibilities

- tracks the currently pending target delete action for row-level disabled state
- treats managed target entries as draft configuration and cancels them through `removeProjectManagedTargetSkill`
- confirms unmanaged target deletion before calling `deleteProjectTargetSkill`
- bumps the inspection refresh key after unmanaged deletion so the target inventory is rescanned
- reports localized success or error feedback through the Projects view status channel

## Current Note

This hook keeps the large Projects view from owning the target management branch directly while preserving the important product boundary: configuration edits and filesystem deletion are separate actions.
