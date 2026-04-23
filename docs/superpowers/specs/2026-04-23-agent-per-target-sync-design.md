# Agent Per-Target Sync Design

## Summary

This phase replaces the old global skill-set sync behavior with per-agent skill assignment. Agent sync no longer means "copy every globally enabled skill to every enabled agent." It means "for each enabled agent, resolve that agent's own skill sources, then sync only that resolved set into that agent's target directory."

The global skill enable/disable switch remains a hard gate: a globally disabled skill is unavailable to every agent and every scene-derived agent selection.

## Product Rules

- Each agent keeps its own sync configuration.
- Disabling an agent remains destructive for app-managed entries: it removes previously synced skills that this app owns through the target manifest.
- Enabling an agent with no selected skills and no selected scenes produces an empty desired set and cleans that agent's app-managed entries.
- Direct per-agent skill selections are independent of the skill browser except for the global hard-disable gate.
- Per-agent scene selections can include multiple scenes.
- The final desired set is `direct skills ∪ selected scene skills - excluded skills`, with all inputs filtered through the global hard-disable gate.
- Duplicate skill sources are deduplicated by stable `skillId`.
- Excluded skills override both direct and scene-provided skills.
- Saving agent configuration automatically runs sync for the changed agent or changed agents.
- Leaving the Agents page with unsaved draft changes opens a prompt with save-and-sync, discard, and stay options.

## Data Model

`agent-config.json` extends each agent entry:

```json
{
  "enabled": true,
  "pathOverride": "C:/Users/example/.codex/skills",
  "selectedSkillIds": ["custom:searxng"],
  "selectedSceneIds": ["work"],
  "excludedSkillIds": ["external:vendor/noisy-skill"]
}
```

Older config files remain valid. Missing arrays default to empty arrays.

## Sync Resolution

For each agent:

1. Load all repository skills.
2. Load global disabled skill IDs from the existing skill-state store.
3. Load scene definitions from the existing scene-config store.
4. Add direct selected skills that are present and globally enabled.
5. For every selected scene, add skills included by that scene and globally enabled.
6. Remove every skill listed in the agent's excluded IDs.
7. Deduplicate by `skillId`.
8. Apply the manifest/ledger sync using only this resolved set.

Scene skill inclusion keeps the current scene model: a scene includes all skills except its `disabledSkillIds`, then global disabled IDs are applied as a hard gate.

## UI Design

The Agents page becomes a draft editor:

- Each agent card has a sync toggle, path override controls, direct skill selection, selected scenes, and an effective selection preview.
- Skill selection filters out globally disabled skills from the selectable set, while showing ignored existing selections as disabled/conflict text where useful.
- The effective preview shows synced count, excluded count, and deduped source behavior.
- A card-level save action persists that agent and syncs that agent.
- A page-level save action persists all dirty agents and syncs them.
- Navigation guard prevents losing unsaved draft changes.

## Safety

The existing target manifest and ledger safety rules remain unchanged:

- Only app-managed target entries can be removed or overwritten.
- Unmanaged target content is never deleted.
- Name conflicts with unmanaged target entries are reported as conflicts.
- Disabling an agent cleans only app-managed target entries.

## Validation

Focused Rust tests must cover:

- Per-agent selected skills do not fall back to all globally enabled skills.
- Global disabled skills are excluded even if directly selected.
- Scene plus direct duplicate selection writes the skill once.
- Excluded scene skills are removed on reapply.
- Scoped agent sync updates only the requested agent.
- Disabling an agent still cleans app-managed entries.

Frontend validation uses the existing build gate plus architecture and module-doc checks.
