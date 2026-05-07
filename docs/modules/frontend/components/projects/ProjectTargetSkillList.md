# Project Target Skill List

> **Source**: `src/components/projects/ProjectTargetSkillList.tsx`
> **Status**: [REVIEW]

## Overview

Default-collapsed per-agent target-skill management card used inside the Projects summary inspector.

## Responsibilities

- shows the existing project-local target skill count before expansion
- expands into a taller internal scroll region for managed and unmanaged target entries
- labels each entry as managed or unmanaged and surfaces missing `SKILL.md` metadata
- sends managed rows to the draft-cancel action and unmanaged rows to the direct delete action
- keeps destructive affordances compact with the shared Lucide trash icon and localized button labels

## Current Note

The component renders target inventory only. It does not decide whether a row should mutate the draft or the filesystem; that ownership boundary stays in the Projects view action hook.
