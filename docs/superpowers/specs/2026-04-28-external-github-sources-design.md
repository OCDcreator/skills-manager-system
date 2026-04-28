# External GitHub Sources Design

## Summary

This phase adds first-class management for external GitHub repositories that publish skills or agent-specific skill artifacts, without merging those upstream repositories directly into `my-skills`.

The recommended model is:

- manage the upstream GitHub repository as a separate external source
- fetch and inspect that repository inside an app-managed cache
- detect which artifacts are importable for which target agent
- import only the selected agent-specific artifact into `my-skills` as a read-only mirror under the existing `external/` tree
- update that mirror later by fetching the upstream default branch and re-importing explicitly

This design is driven by repositories like `pbakaus/impeccable`, which are not simple "one folder with one `SKILL.md`" repositories. They contain source-of-truth skill content plus generated per-agent distribution outputs. Those repositories update frequently, so directly copying the whole upstream repository into `my-skills/external/` would make updates noisy and would confuse the current recursive skill scanner.

## Product Goal

Allow the app to manage a GitHub repository URL as a durable upstream source and let the user import the correct skill artifact for a specific agent without losing updateability.

The first success criterion is:

1. user adds a GitHub HTTPS or SSH URL
2. app clones or fetches the repository into its own cache
3. app detects importable artifacts for supported agents
4. when the user is looking at a specific agent, the app shows only that agent's relevant artifacts
5. user imports one artifact into `my-skills` as a read-only mirror
6. later, the app can fetch upstream changes and offer an explicit mirror update

## Why Not Merge Into `my-skills`

Directly merging the entire upstream repository into `my-skills` is the wrong boundary for this phase.

Reasons:

- upstream repositories like `impeccable` are not pure skill repositories
- they may contain multiple generated artifacts for different agents
- they may contain source folders, dist folders, scripts, websites, tests, and reference files
- frequent upstream updates would pollute the `my-skills` history with unrelated churn
- the current scanner recursively treats `external/**/SKILL.md` as importable skills, which would surface duplicate or misleading entries from multi-output repositories

The app should therefore treat an upstream GitHub repository as source material, not as an already-ingested `my-skills` subtree.

## Product Rules

- Phase 1 supports only GitHub repository URLs over HTTPS or SSH.
- The app manages external repositories in its own cache directory, not inside `my-skills`.
- Imported artifacts are read-only mirrors inside `my-skills`; the user does not edit them in place.
- Imported artifacts stay within the existing `external/` scan root so Phase 1 does not introduce a third top-level skill source category.
- The import surface is agent-aware: the UI shows only variants relevant to the currently viewed agent.
- The default tracking target is the repository's default branch.
- Every import records the exact upstream commit it came from.
- Updating an imported mirror is always an explicit user action.
- Phase 1 does not push changes upstream, fork upstream repositories, or preserve local edits inside imported mirrors.
- Phase 1 does not run arbitrary upstream build pipelines. It only imports from detected already-present artifact paths.

## In Scope

- add and remove external GitHub sources
- validate GitHub HTTPS and SSH repository URLs
- clone and fetch external sources into an app-managed cache
- detect supported importable variants from cached repository contents
- expose agent-filtered variant lists in the UI
- import one selected variant into `my-skills` as a read-only managed mirror
- record source metadata such as repository URL, upstream path, default branch, and pinned commit
- detect when a newer upstream default-branch commit is available
- re-import a mirror from a newer upstream commit on explicit user action
- surface source health, detection warnings, import status, and update availability
- desktop source management plus desktop import and update flows

## Out of Scope

- non-GitHub git remotes
- local-folder sources
- arbitrary branch selection UI
- tag pinning UI
- editing imported mirrors and merging local edits back upstream
- background auto-refresh or scheduled update jobs
- arbitrary upstream build execution
- generalized package-manager-style dependency solving
- importing multiple agents from one click
- full CLI parity for source management

## Recommended Approach

Introduce a new domain named `external_sources` rather than stretching the current `skills` scanner to own upstream repository management.

This keeps the architecture clean:

- `my-skills` remains the final managed repository of assignable skills
- external-source management becomes a separate upstream-ingestion layer
- import converts one detected upstream variant into one stable local mirror

This is the right boundary because current skill scanning assumes it is reading a skills repository, while this new problem is about managing and normalizing upstream repositories before they become skills in the local system.

The design must still fit the current codebase reality:

- `sourceType` is currently a binary `custom | external`
- scanner roots are currently `custom/` and `external/`
- persisted `skillId` values already flow through skill-state, scenes, agents, and projects

Phase 1 should therefore avoid introducing a third source type such as `external_imported`. Imported mirrors should remain `external` skills from the scanner's perspective, with import-specific metadata layered on through a stable manifest contract.

## External Source Types

Phase 1 should classify each source into one of these kinds:

- `pure_skill_repo` - a repository that directly exposes one or more stable skill directories
- `multi_skill_repo` - a repository containing multiple separate skills in a native format
- `generated_agent_bundle` - a repository with source material plus generated per-agent output directories
- `unsupported` - repository does not expose a recognizable import target for the current rules

`pbakaus/impeccable` is the reference example for `generated_agent_bundle`.

The type is primarily a detection result that shapes the import workflow and warning text. It does not need to become user-editable configuration in Phase 1.

## Data Model

Phase 1 should introduce three related entities.

### `ExternalSource`

Represents a managed upstream GitHub repository.

Suggested fields:

```json
{
  "id": "src_01",
  "repoUrl": "git@github.com:pbakaus/impeccable.git",
  "defaultBranch": "main",
  "cachedRepoPath": "C:/Users/example/AppData/.../external-sources/src_01/repo",
  "detectedKind": "generated_agent_bundle",
  "lastFetchedCommit": "abcdef123456",
  "lastFetchedAt": "2026-04-28T10:00:00Z",
  "status": "ok",
  "warnings": []
}
```

Rules:

- `repoUrl` is the canonical user-provided remote
- `defaultBranch` is discovered after clone/fetch
- `cachedRepoPath` is app-owned and not user-facing configuration
- `status` is a summary such as `ok`, `warning`, or `error`
- `warnings` provide structured detection and update issues

### `ExternalVariant`

Represents one importable agent-specific artifact detected inside an external source.

Suggested fields:

```json
{
  "sourceId": "src_01",
  "agentKey": "codex",
  "displayName": "impeccable",
  "variantPath": "dist/agents/.agents/skills/impeccable",
  "sourceOfTruthPath": "source/skills/impeccable",
  "isGenerated": true,
  "metadataPath": "dist/agents/.agents/skills/impeccable/SKILL.md"
}
```

Rules:

- variants are detection outputs, not user-authored records
- `agentKey` must align with the app's existing managed-agent catalog
- a source may expose zero, one, or many variants
- only variants for the currently viewed agent are shown in the import UI

### `ImportedExternalSkill`

Represents a read-only mirror currently installed into `my-skills`.

Suggested fields:

```json
{
  "id": "ext_codex_impeccable",
  "externalSourceId": "src_01",
  "agentKey": "codex",
  "upstreamVariantPath": "dist/agents/.agents/skills/impeccable",
  "pinnedCommit": "abcdef123456",
  "skillId": "external:managed/github/pbakaus__impeccable/codex/impeccable",
  "mirrorRelativePath": "external/managed/github/pbakaus__impeccable/codex/impeccable",
  "localMirrorPath": "C:/Users/lt/Desktop/Write/custom-project/my-skills/external/managed/github/pbakaus__impeccable/codex/impeccable",
  "lastCheckedCommit": "abcdef123456",
  "importedAt": "2026-04-28T10:05:00Z",
  "updateAvailable": false
}
```

Rules:

- this is the durable bridge back to the upstream source
- `skillId` must be the exact ID the scanner will later emit for this mirror
- `mirrorRelativePath` is the canonical repo-relative location of the managed mirror
- `pinnedCommit` records exactly what was imported
- `lastCheckedCommit` records the newest upstream default-branch commit that has been fetched for comparison, even when the user has not imported that revision yet
- `localMirrorPath` lives inside a dedicated app-managed subtree under `external/`
- mirror contents are regenerated by update, not edited in place

## Storage Model

Phase 1 should add app-local state for external-source metadata instead of overloading the existing skill-state store.

Recommended persistent files:

- `external-sources.json` - source records plus import records
- app-owned cache root under the shared config or app-data directory
- a managed import subtree inside `my-skills`, for example:
  - `external/managed/github/<owner>__<repo>/<agent-key>/<variant-key>/`

The import subtree should be visibly separate from the user's hand-managed `custom/` and cloned `external/` content so the ownership model stays obvious, but it must stay under `external/` so the existing scanner and persisted ID model remain valid.

Recommended rule:

- `custom/` stays user-authored
- existing `external/` stays repo-native upstream content the user manages manually
- `external/managed/github/` becomes the reserved subtree for app-owned mirrors from managed GitHub sources

Legacy behavior remains explicit:

- existing `external/` content and `update.sh` / `update.bat` workflows remain user-managed and are not auto-migrated in Phase 1
- the app does not try to deduplicate manual `external/` entries against imported managed mirrors in Phase 1
- imported mirrors must expose enough origin metadata in the UI that users can distinguish them from manually maintained `external/` skills

### Managed Mirror Manifest

Every imported mirror must contain a small app-owned sidecar manifest, for example:

`external/managed/github/<owner>__<repo>/<agent-key>/<variant-key>/.skills-manager-source.json`

Suggested fields:

```json
{
  "version": 1,
  "managed": true,
  "importId": "ext_codex_impeccable",
  "sourceId": "src_01",
  "repoUrl": "git@github.com:pbakaus/impeccable.git",
  "agentKey": "codex",
  "variantPath": "dist/agents/.agents/skills/impeccable",
  "mirrorRelativePath": "external/managed/github/pbakaus__impeccable/codex/impeccable",
  "skillId": "external:managed/github/pbakaus__impeccable/codex/impeccable",
  "pinnedCommit": "abcdef123456"
}
```

Rules:

- this manifest is the contract between `core/external_sources` and `core/skills`
- importer writes it whenever a mirror is created or updated
- scanner reads it to enrich scanned `external` skills with origin metadata
- delete and update operations trust only this manifest plus the app-local import record, never path shape alone

### Stable ID Contract

Phase 1 must not let importer and scanner invent IDs independently.

Rules:

- the repo-relative path is the source of truth for local skill identity
- importer must derive `skillId` using the same shared helper the scanner and document reader use
- if needed, extract `build_skill_id()` into a shared module rather than duplicating its logic
- `external-sources.json` stores `skillId` so the app can reverse-map a scanned skill back to its import record
- update and delete operations must fail if the manifest `skillId`, stored `skillId`, and computed `skillId` do not agree

### Mirror Path And Collision Rules

Managed mirror paths must be deterministic and collision-resistant.

Recommended pattern:

- `external/managed/github/<owner>__<repo>/<agent-key>/<variant-key>/`

Where:

- `<owner>__<repo>` is a sanitized repository slug
- `<variant-key>` defaults to the last path segment of the upstream variant
- if a repository exposes multiple variants for the same agent that would collide, append a short sanitized hash derived from the upstream variant path

## Detection Strategy

Detection must be path-driven and explicit. Phase 1 should not attempt to infer arbitrary build systems.

For each fetched repository:

1. determine default branch and current HEAD commit
2. inspect a small set of known path patterns
3. classify repository kind
4. enumerate variants for known agents
5. parse `SKILL.md` from candidate variant directories where available
6. emit warnings when likely source material exists but no importable variant matches the current rules

Phase 1 can begin with a rule table rather than a plugin system.

Example rule shapes:

- direct skill directory patterns
- per-agent generated output patterns
- optional source-of-truth companion paths

For `generated_agent_bundle`, detection should prefer already-built agent targets over source directories, because import must remain agent-specific and build-free in Phase 1.

## Import Workflow

Recommended import flow:

1. user adds GitHub URL
2. app clones or fetches the source into cache
3. app runs detection and stores source metadata
4. user opens an agent page
5. app shows only variants matching that `agentKey`
6. user clicks import
7. app mirrors the selected variant into the managed `my-skills/external/managed/github/...` path
8. app writes import metadata linking mirror, source, variant path, and pinned commit
9. imported mirror becomes visible to existing skill scanning and assignment flows

Import behavior rules:

- if the target mirror path does not exist, create it
- if the target exists and is app-managed for the same import record, replace it
- if the target exists but is unmanaged, stop with a conflict
- preserve a small metadata file inside the mirror or alongside it so the origin remains discoverable
- mirror replacement must be atomic: stage into a temp directory, validate the staged `SKILL.md` plus manifest, then swap into place

Recommended staging behavior:

- use the existing ignored temp area under `my-skills/.tmp-skills/` for import and update staging
- never replace a working mirror until the staged copy is complete and valid
- on failure, keep the old mirror untouched and report the failure

## Update Workflow

Recommended update flow:

1. app fetches the upstream default branch for a source
2. app compares `lastFetchedCommit` and imported `pinnedCommit` values against the new upstream commit
3. if the selected variant still exists and the upstream commit is newer, mark `updateAvailable`
4. user explicitly chooses update
5. app replaces the local mirror from the newer upstream variant content
6. app updates `pinnedCommit`, timestamps, and source fetch metadata

Rules:

- update is explicit, never automatic
- update must fail safely if the variant disappeared or became invalid
- update should explain whether the failure came from network, detection, or local conflict
- Phase 1 does not attempt a three-way merge because mirrors are read-only
- update must stage and validate the replacement mirror before swapping it into place
- `lastCheckedCommit` may advance on fetch even when `pinnedCommit` does not

## UI Design

Phase 1 should avoid overloading the existing Skills view with upstream-repository concerns.

Recommended surfaces:

- a new `External Sources` view for adding URLs, fetching, inspection, and update status
- agent-page import panels that show variants relevant to the current agent
- imported-skill detail badges inside the existing Skills view, while keeping their scanner `sourceType` as `external`

### External Sources View

Each source card should show:

- repository URL
- detected kind
- default branch
- last fetched commit and time
- status and warnings
- counts of detected variants and active imports
- actions: fetch, inspect, remove source

### Agent Import Panel

When viewing a specific agent:

- show only variants for that agent
- show whether each variant is already imported
- show upstream repository identity and pinned commit
- offer import or update actions

This follows the user-approved rule that agent-specific upstream outputs should be selected from the perspective of the target agent, not from a generic skill list.

### Imported Skill Presentation

Inside existing skill browsing:

- imported mirrors should appear as normal assignable skills
- imported mirrors remain grouped under the existing `external` source bucket in Phase 1
- each imported mirror should show read-only origin metadata
- badges should distinguish `manual external` from `managed GitHub mirror`
- destructive actions should warn that deletion only removes the local mirror, not the upstream source

This avoids a Phase 1 rewrite of the binary `custom` / `external` filtering model and keeps existing persisted skill IDs valid.

## CLI Shape

Full CLI parity is deferred from MVP.

If a thin CLI follow-up is added later, recommended commands are:

- `skills-manager external-sources list`
- `skills-manager external-sources add <github-url>`
- `skills-manager external-sources fetch <source-id>`
- `skills-manager external-sources inspect <source-id>`
- `skills-manager external-sources import <source-id> --agent <agent-key> --variant <variant-path>`
- `skills-manager external-sources update <import-id>`
- `skills-manager external-sources remove <source-id>`

Any CLI surface must reuse the same source records and cache paths as the desktop app, and all orchestration must remain in `core/external_sources/`.

## Architecture

Recommended module shape:

- `src-tauri/src/core/external_sources/`
  - source records and persistence
  - git fetch/clone orchestration
  - detection and classification
  - import/update reconciliation
- `src-tauri/src/commands/external_sources.rs`
  - thin Tauri command adapter
- `src-tauri/src/cli/commands/external_sources.rs`
  - thin CLI adapter if CLI is included in this phase
- `src/lib/tauri.ts`
  - frontend bindings
- `src/views/ExternalSourcesView.tsx`
  - page-level orchestration
- `src/components/external-sources/`
  - source cards, add-source form, variant list, import status

Important boundary:

- current `core/skills` remains responsible for scanning already-imported local skills
- new `core/external_sources` owns upstream repository handling and mirror lifecycle
- thin command adapters only validate inputs, call `core/external_sources`, and map errors
- git orchestration, URL normalization, detection, manifest writing, collision resolution, and mirror reconciliation all belong in `core/external_sources`

This avoids turning `scan_repo_skills` into a mixed scanner-plus-git-ingestion module.

## Scanner Integration

The existing skill scanner should not scan cached external repositories.

Instead:

- scanner input remains the configured `my-skills` repository
- imported mirrors are written into a dedicated managed subtree under `external/`
- scanner continues to classify imported mirrors as `external`
- scanner gains a targeted enrichment step: when an `external` skill directory contains `.skills-manager-source.json`, attach managed-origin metadata to the DTO returned to frontend and any future CLI

This avoids changing the current `custom | external` schema in Rust structs, TypeScript types, filters, persisted selection IDs, and i18n.

Recommended DTO addition:

- extend `SkillSummary` and `SkillDocument` with optional import metadata such as:
  - `managedSource: null | { kind: "github_import", importId, repoUrl, pinnedCommit, agentKey, updateAvailable }`

This is an additive DTO change rather than a breaking `sourceType` migration.

## Safety And Ownership

Ownership rules must stay explicit.

- the app may freely replace or delete only app-managed imported mirrors
- the app must never delete cached upstream repositories that are still referenced by an import without first removing the import record
- removing a source with active imports should require confirmation and a clear policy
- a mirror counts as app-managed only when both the sidecar manifest and app-local import record agree

Recommended removal policy:

- remove source only: allowed if no active imports
- remove source and imported mirrors: explicit destructive action
- keep imported mirrors while removing source: not allowed in Phase 1 because updates and provenance would become ambiguous

### Concurrency And State Writes

Phase 1 should use the same mutation-safety posture as the rest of the app, but make it explicit for this new domain.

Rules:

- writes to `external-sources.json` must use advisory config locking plus atomic write-rename
- source fetch, import, update, and remove operations should serialize per source ID
- a fetch that refreshes `lastCheckedCommit` must not race with an import or update that rewrites the same import record
- mirror staging and swap must happen under a per-source or per-import critical section

## Validation Strategy

Focused Rust tests should cover:

- GitHub URL validation for HTTPS and SSH forms
- source persistence round trips
- repository kind classification
- variant detection for representative directory shapes
- agent filtering returns only variants for the requested agent
- import creates the expected managed mirror path
- re-import replaces an existing app-managed mirror
- update detection flips `updateAvailable` only when a newer upstream commit exists
- manifest and stored import record stay in sync
- source removal rules respect active imports

Focused integration scenarios should cover:

- generated bundle source with multiple agent outputs
- unsupported repository with clear warnings
- import conflict against unmanaged local directory
- variant disappears after upstream fetch
- mirror-path collision resolution
- atomic update rollback when staged validation fails

Broader repo validation should include:

1. targeted Rust tests for new core modules
2. targeted frontend tests where meaningful
3. `node scripts/check-module-doc-coverage.mjs`
4. `node scripts/check-module-doc-diff.mjs --range <base>...HEAD`
5. `npm run check:architecture`
6. `npm run verify`

## MVP Cut

To keep this phase focused, MVP should ship with:

- GitHub HTTPS/SSH source add/fetch
- repository classification
- known-pattern variant detection
- agent-filtered import UI
- read-only mirror import
- explicit update check and explicit re-import
- origin metadata display
- shared stable ID contract between importer and scanner
- managed mirror manifest plus atomic staging and swap

MVP should defer:

- editable forks
- arbitrary git remotes
- background sync
- arbitrary artifact-build pipelines
- broad generic detector plugins
- full desktop and CLI parity

## Future Expansion Path

This design leaves room for later features without forcing them into Phase 1:

- editable imported mirrors with "detach from upstream" semantics
- tag or commit pinning UI
- support for non-GitHub remotes
- richer detector rules for additional upstream repository patterns
- source-level health diagnostics
- optional auto-fetch notifications

The important thing in Phase 1 is to land the correct ownership model: upstream GitHub repositories are managed separately, while `my-skills` receives only the chosen agent-specific read-only mirrors.
