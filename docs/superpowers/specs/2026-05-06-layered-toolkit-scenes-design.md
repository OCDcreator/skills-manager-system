# Layered Toolkit Scenes Design

## Summary

This design redefines scenes in Skills Manager System as reusable skill toolkits rather than only global switchable scenarios. A scene describes a task-oriented package of skills, optional default agents, ordering, and human-readable intent. The same scene can be referenced by global agent configuration and project-level agent configuration.

Project-level configuration inherits global scene and direct-skill selections, then adds project-specific scenes, direct skills, and project-local exclusions. Project exclusions are local overrides only: they do not mutate global agent configuration, scene definitions, or any other project.

The product goal is a layered model:

```text
Scene            = reusable toolkit
Global layer     = default toolkits and direct skills for each agent
Project layer    = inherited global layer plus project-specific additions and exclusions
Resolution       = computed final skills for one apply target
Apply            = explicit filesystem sync
```

## Product Rules

- A scene is a reusable toolkit template, not a storage location.
- A scene can be selected by global agent configuration.
- A scene can also be selected by project-level agent configuration.
- Project assignments inherit global agent selections before adding project-specific selections.
- Project exclusions only affect the current project.
- Global disabled skills remain a hard gate everywhere.
- Direct skill selections remain available for precise one-off choices that do not deserve a new scene.
- Scene selection and editing must not write to agent directories by itself.
- Apply actions remain explicit and scoped: apply global, apply project, or apply one agent target.
- The UI should show resolved skill previews before apply, including why each skill is included or excluded.

## Relationship To The Upstream Skills Manager

The upstream Skills Manager treats scenarios primarily as global skill sets. That model is useful for quick global switching, but it does not match this project goal. Skills Manager System keeps the strong parts of the upstream design while changing the core semantics:

- Keep explicit apply instead of making browse/edit actions mutate disk.
- Keep applied/not-applied status, but scope it to global, project, and agent targets.
- Keep project workspaces separate from global sync.
- Keep per-agent selection clarity.
- Change scenes from "the current global setup" into reusable toolkits that can be layered.

## Data Model

The existing scene model remains the toolkit definition:

```json
{
  "id": "obsidian-plugin-dev",
  "name": "Obsidian Plugin Dev",
  "description": "Skills for Obsidian plugin implementation and validation",
  "skillSelectionMode": "onlySelected",
  "selectedSkillIds": ["custom:obsidian-plugin-autodebug"],
  "disabledSkillIds": [],
  "enabledAgentKeys": ["codex", "opencode"],
  "skillOrder": ["custom:obsidian-plugin-autodebug"]
}
```

Agent global configuration continues to reference scenes:

```json
{
  "codex": {
    "enabled": true,
    "selectedSkillIds": ["custom:searxng"],
    "selectedSceneIds": ["general-coding"],
    "excludedSkillIds": []
  }
}
```

Project configuration should gain scene references and exclusions per project agent:

```json
{
  "projects": {
    "C:/Users/lt/Desktop/Write/custom-project/opencodian": {
      "name": "OpenCodian",
      "agents": {
        "codex": {
          "selectedSkillIds": ["custom:release-checklist"],
          "selectedSceneIds": ["obsidian-plugin-dev"],
          "excludedSkillIds": ["external:vendor/noisy-skill"]
        }
      }
    }
  }
}
```

The project `excludedSkillIds` list overrides inherited global skills and project-added skills for that project only.

## Resolution Rules

For a global agent target:

```text
resolved global skills
= global direct skills
+ skills from global selected scenes
- global agent exclusions
- globally disabled skills
```

For a project agent target:

```text
resolved project skills
= resolved global skills for the same agent
+ project direct skills
+ skills from project selected scenes
- project exclusions
- globally disabled skills
```

Deduplication is by stable `skillId`. Exclusions win over both direct and scene-provided inclusions. Project exclusions do not remove the skill from global targets or other projects.

If a project enables an agent that has no global configuration, the inherited global set is empty and the project layer still works independently.

## UI Design

### Scenes

The Scenes page is the toolkit editor. It should answer:

- What is this toolkit for?
- Which skills belong to it?
- Which agents are good default consumers?
- What order should skills appear in?

It should not imply that selecting a scene immediately applies it anywhere.

### Agents

The Agents page configures the global layer per agent:

- direct skills
- selected scenes
- exclusions
- path override
- effective preview
- apply global or apply selected agent

The preview should label sources such as `Global scene`, `Global direct`, `Excluded`, and `Globally disabled`.

### Projects

The Projects page configures the project layer per project and per agent:

- inherited global layer summary
- project selected scenes
- project direct skills
- project exclusions
- effective project preview
- apply project or apply selected project agent

The inherited global section should be visible but not directly edited from the project screen. Project edits should never mutate global configuration.

## Apply Semantics

Apply remains a user-initiated filesystem mutation.

- Applying global writes the resolved global layer to global agent skill directories.
- Applying a project writes the resolved project layer to project-local agent skill directories.
- Applying a single agent scopes the write to that one target.
- Editing a scene, selecting a scene, or viewing a project only updates configuration.

The app should keep enough apply metadata to show whether a target is current, stale, or never applied.

## Safety

- App-managed target manifests remain the authority for deletion and replacement.
- Unmanaged target content must not be deleted.
- Project exclusions only affect project-local resolution.
- A missing or deleted scene should be shown as a stale reference in the owning global/project config.
- A missing skill reference should be ignored during sync but surfaced in preview.
- Global disabled skills should be filtered everywhere and called out in preview when referenced.

## Validation

Rust tests should cover:

- Global resolution from direct skills plus selected scenes.
- Project resolution inherits global direct and scene-derived skills.
- Project scenes add skills on top of inherited global skills.
- Project exclusions remove inherited global skills only for that project.
- Global disabled skills are filtered from both global and project resolution.
- Missing scene and missing skill references do not panic.
- Project apply writes to project-local target directories without changing global target directories.

Frontend tests should cover:

- Scene editing does not imply apply.
- Project preview labels inherited, project-added, excluded, and globally disabled skills.
- Project exclusions are presented as local overrides.
- Apply status distinguishes global and project targets.

## Out Of Scope

- Marketplace behavior.
- AI search.
- Automatic background apply.
- Cross-project exclusion rules.
- Scene inheritance between scenes.
- Renaming "scene" in the product UI unless a later UX pass chooses a clearer term such as "Toolkit".
