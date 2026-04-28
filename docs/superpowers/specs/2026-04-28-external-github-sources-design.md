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

The hard-coded `github` path segment is intentional Phase 1 scope, not an attempt at a permanent multi-provider abstraction.

- future support for non-GitHub remotes must come with an explicit migration plan for persisted mirror paths and `skillId` values
- Phase 1 should not over-generalize this path grammar before a second provider actually exists

The same "keep the schema stable" rule also applies to the current frontend filtering model:

- `SourceFilter` remains `all | custom | external` in Phase 1
- source summaries remain the existing two visible buckets plus `all`
- managed mirrors are not split into a separate source filter or section in Phase 1
- managed/manual distinction is expressed through additive badges and detail metadata inside the existing `external` bucket

## External Source Types

Phase 1 should classify each source into one of these kinds:

- `pure_skill_repo` - a repository that directly exposes one or more stable skill directories
- `multi_skill_repo` - a repository containing multiple separate skills in a native format
- `generated_agent_bundle` - a repository with source material plus generated per-agent output directories
- `unsupported` - repository does not expose a recognizable import target for the current rules

`pbakaus/impeccable` is the reference example for `generated_agent_bundle`.

The type is primarily a detection result that shapes the import workflow and warning text. It does not need to become user-editable configuration in Phase 1.

Forward-compatibility rule:

- if a future app version introduces a new stored source kind, older readers should degrade unknown kinds to `unsupported`

## Data Model

Phase 1 should introduce three related entities.

### `ExternalSource`

Represents a managed upstream GitHub repository.

Suggested fields:

```json
{
  "schemaVersion": 1,
  "id": "src_01",
  "repoUrl": "git@github.com:pbakaus/impeccable.git",
  "defaultBranch": "main",
  "cachedRepoPath": "external-sources/src_01/repo",
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
- `cachedRepoPath` is app-owned, stored relative to the app data root, and not user-facing configuration
- `status` is a summary such as `ok`, `warning`, or `error`
- `warnings` provide structured detection and update issues
- `id` should be deterministic from the canonical repo URL, for example `src_<sha256-prefix>`
- `schemaVersion` must start at `1` for Phase 1 persistence

Canonical GitHub URL normalization rules:

- treat GitHub owner and repo names as case-insensitive for source identity
- normalize accepted GitHub remotes into a canonical lowercase `github.com/<owner>/<repo>` identity tuple before ID generation
- strip a trailing `.git`
- strip a trailing slash
- treat `https://github.com/<owner>/<repo>`, `git@github.com:<owner>/<repo>`, and `ssh://git@github.com/<owner>/<repo>` as the same logical repository after normalization

Minimal status-state guidance:

- `ok` means fetch and detection completed without blocking issues
- `warning` means the source is usable but has actionable issues such as `variant_disappeared`, partial detection, or stale imports
- `error` means the latest fetch or detection attempt failed and the source cannot currently provide trustworthy update or import results
- a successful later fetch or detection run may move `warning` or `error` back to `ok`

Priority and examples:

- `error` wins over `warning`, and `warning` wins over `ok`
- fetch failure, unreadable cache state, or mirror-record corruption that blocks trusted import/update should yield `error`
- `variant_disappeared`, manifest-integrity mismatch on one imported mirror, or partial detection should yield `warning` when the rest of the source remains usable
- "newer upstream commit exists for an otherwise healthy import" is not itself a source `warning`; it is represented through import-level `updateAvailable`
- `unsupported_agent_variant` should surface in diagnostics but does not by itself escalate a healthy source above `ok`

Status recomputation rule:

- recompute `status` fresh from the newest fetch and detection result rather than incrementally transitioning from the prior state
- if the newest run has a blocking fetch or detection failure, result is `error`
- else if the newest run succeeds but yields any source-level warnings, result is `warning`
- else result is `ok`

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
- `sourceOfTruthPath` is diagnostic and explanatory metadata in Phase 1; it is shown in detail views and logs but does not drive import path resolution
- variant matching across re-detection should use the stable tuple `(sourceId, agentKey, variantPath)`
- variants whose upstream agent identifier does not map to the app's managed-agent catalog are not importable in Phase 1 and should surface only as source-level detection warnings such as `unsupported_agent_variant`
- `sourceOfTruthPath` is refreshed on each successful detection pass and always reflects the latest detected snapshot rather than the first-seen value
- `variantPath` must be canonicalized with the same safety rules used for repo-relative skill paths: reject parent traversal and absolute paths, normalize separators, and strip trailing separators
- a change to `sourceOfTruthPath` alone does not count as `variant_disappeared`; only the tracked `variantPath` identity governs that warning in Phase 1

### `ImportedExternalSkill`

Represents a read-only mirror currently installed into `my-skills`.

Suggested fields:

```json
{
  "importId": "imp_01",
  "externalSourceId": "src_01",
  "agentKey": "codex",
  "upstreamVariantPath": "dist/agents/.agents/skills/impeccable",
  "pinnedCommit": "abcdef123456",
  "pinnedVariantFingerprint": "sha256:abcd1234",
  "skillId": "external:managed/github/pbakaus__impeccable/codex/impeccable",
  "mirrorRelativePath": "external/managed/github/pbakaus__impeccable/codex/impeccable",
  "lastCheckedCommit": "abcdef123456",
  "importedAt": "2026-04-28T10:05:00Z",
  "warnings": [],
  "updateAvailable": false
}
```

Rules:

- this is the durable bridge back to the upstream source
- `importId` is the unique persisted identity for the logical import and replaces any ambiguous generic `id` naming
- `skillId` must be the exact ID the scanner will later emit for this mirror
- `mirrorRelativePath` is the canonical repo-relative location of the managed mirror
- `pinnedCommit` records exactly what was imported
- `pinnedVariantFingerprint` is the stored fingerprint of the imported variant snapshot used for later update comparisons
- `lastCheckedCommit` records the newest upstream default-branch commit that has been fetched for comparison, even when the user has not imported that revision yet
- `localMirrorPath` is derived at runtime from the current configured repository root plus `mirrorRelativePath`; Phase 1 should not persist absolute mirror paths in durable records
- `warnings` is the import-level warning collection for states such as `variant_disappeared` or `upstream_history_rewritten`
- mirror contents are regenerated by update, not edited in place

Fingerprint rule:

- `pinnedVariantFingerprint` should be a SHA-256 hash over the canonical file list plus file-content hashes for the imported variant snapshot
- fingerprinting excludes the app-written `.skills-manager-source.json` sidecar so that upstream-content comparison stays stable
- canonical fingerprint serialization should sort canonical relative file paths lexicographically, then hash a deterministic byte stream of repeated entries:
  - `<relative-path>\\n<file-content-sha256>\\n`
- directory metadata, mtimes, and local filesystem permissions are excluded from the fingerprint
- when evaluating fetched upstream content, compute file-content hashes from the cached git commit's blob bytes rather than platform-specific working-tree line endings so `core.autocrlf` does not change fingerprints across Windows and macOS
- `id` should be stable for the logical import, for example a deterministic hash of `(sourceId, agentKey, mirrorRelativePath)`

## Storage Model

Phase 1 should add app-local state for external-source metadata instead of overloading the existing skill-state store.

Recommended persistent files:

- `external-sources.json` - source records plus import records
- app-owned cache root under the shared config or app-data directory
- a managed import subtree inside `my-skills`, for example:
  - `external/managed/github/<owner>__<repo>/<agent-key>/<variant-key>/`

Recommended `external-sources.json` top-level shape:

```json
{
  "schemaVersion": 1,
  "sources": [],
  "imports": []
}
```

The import subtree should be visibly separate from the user's hand-managed `custom/` and cloned `external/` content so the ownership model stays obvious, but it must stay under `external/` so the existing scanner and persisted ID model remain valid.

Recommended rule:

- `custom/` stays user-authored
- existing `external/` stays repo-native upstream content the user manages manually
- `external/managed/github/` becomes the reserved subtree for app-owned mirrors from managed GitHub sources

Managed-subtree Git policy:

- `external/managed/` is local app-managed state in Phase 1, not repo-authored source material
- on first managed import, the app should check whether the configured repository ignores `external/managed/` and warn if it does not
- if the app cannot safely confirm or apply that ignore policy, it must warn before import rather than silently creating long-term working-tree noise

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
  "schemaVersion": 1,
  "managed": true,
  "importId": "imp_01",
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

Warning schema:

- both source-level and import-level warnings should use a structured shape such as:
  - `{ code, severity, message }`
- Phase 1 warning codes include values such as `variant_disappeared`, `upstream_history_rewritten`, `unsupported_agent_variant`, and `integrity_mismatch`
- import-level warnings are persisted snapshots derived from the latest successful evaluation pass for that import; each evaluation pass overwrites the warning set rather than appending forever
- source-level warnings are persisted snapshots derived from the latest successful source evaluation pass

### Shared Skill Identity Module

Phase 1 should treat stable skill identity as one shared concern, not scattered path checks.

Rules:

- extract both `source_type` resolution and `build_skill_id()` into a shared helper module under `core/skills`
- extract repo-relative path canonicalization into that same shared helper surface
- scanner, document reader, and external-source importer must all call that shared module
- no Phase 1 caller may inline its own `"custom/"` or `"external/"` prefix logic for skill identity
- no Phase 1 caller may build skill IDs from unnormalized path strings
- tests for the shared helper must cover both scanner and document-reader style inputs

Canonicalization rules:

- reject parent-directory traversal such as `..`
- strip leading `./`
- collapse repeated path separators
- strip a trailing separator
- reject rooted or drive-qualified paths

Formal `skillId` grammar:

- base grammar remains `<source_type>:<source_relative_path>`
- `source_type` is `custom` or `external` in Phase 1
- `source_relative_path` is the canonical repo-relative directory path with forward slashes
- managed GitHub mirrors therefore use the Phase 1 form:
  - `external:managed/github/<repo-slug>/<agent-key>/<variant-key>`

### Stable ID Contract

Phase 1 must not let importer and scanner invent IDs independently.

Rules:

- the repo-relative path is the source of truth for local skill identity
- canonical repo-relative paths must always use forward slashes and no redundant path segments before `skillId` generation
- importer must derive `skillId` using the same shared helper the scanner and document reader use
- `external-sources.json` stores `skillId` so the app can reverse-map a scanned skill back to its import record
- update and delete operations must fail if the manifest `skillId`, stored `skillId`, and computed `skillId` do not agree

### Mirror Path And Collision Rules

Managed mirror paths must be deterministic and collision-resistant.

Recommended pattern:

- `external/managed/github/<owner>__<repo>/<agent-key>/<variant-key>/`

Where:

- `<owner>__<repo>` is a sanitized repository slug
- `<variant-key>` defaults to the last path segment of the upstream variant
- if a repository exposes multiple variants for the same agent that would collide, append a lowercase SHA-256 hex suffix derived from the upstream variant path, starting with the first 10 hex characters
- if that suffix still collides, extend by 4 additional hex characters until the local path is unique
- if the last path segment is empty or invalid, fall back to `variant-<hash-prefix>`

Repository-slug collision rule:

- if two canonical GitHub repository URLs sanitize to the same `<owner>__<repo>` slug, append a lowercase SHA-256 hex suffix derived from the canonical repo URL, starting with the first 10 hex characters and extending by 4 characters on collision
- hash extension may grow up to the full 64 hex characters
- if the final path segment would still exceed filesystem safety limits, truncate the human-readable slug portion and keep the hash-derived suffix stable

Path-budget rule:

- design Phase 1 mirror-relative paths to stay within a conservative 180-character budget so the user-configured repository root still has headroom on Windows
- if a candidate path would exceed that budget, truncate human-readable slug portions before extending hash material
- if the path still cannot fit safely, abort import with a structured path-too-long error

Slug sanitization rules:

- lowercase the slug input
- replace characters outside `[a-z0-9._-]` with `-`
- collapse repeated `-`
- strip leading and trailing punctuation where needed
- if the sanitized segment matches a Windows reserved name such as `con`, `prn`, `aux`, `nul`, any `com1` through `com9`, or any `lpt1` through `lpt9`, append a stable hash suffix
- apply that reserved-name rule even when the segment would otherwise have an extension-like suffix, because Windows still treats names such as `con.txt` as reserved

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
- if the target exists with a sidecar manifest but without a matching import record, treat it as a corrupted managed mirror and stop with a repair-required conflict rather than as healthy managed or ordinary unmanaged content
- preserve a small metadata file inside the mirror or alongside it so the origin remains discoverable
- mirror replacement must be atomic: stage into a temp directory, validate the staged `SKILL.md` plus manifest, then swap into place

Repair path for corrupted managed mirrors:

- the app should offer an explicit repair or re-import action for corrupted managed mirrors
- repair may either re-import into the same mirror path after clearing the corrupted directory or remove the corrupted mirror so a fresh import can be created

Recommended staging behavior:

- prefer the existing ignored temp area under `my-skills/.tmp-skills/` for import and update staging when the configured repository already ignores it in `.gitignore`
- if the configured repository does not ignore `.tmp-skills/`, fall back to an app-data staging directory outside the repo instead of mutating the user's `.gitignore` automatically
- never replace a working mirror until the staged copy is complete and valid
- on failure, keep the old mirror untouched and report the failure

Rollback backup rule:

- before replacing an existing live mirror, move the old mirror into a same-volume temporary backup sibling
- only delete that backup after post-swap validation and import-record write succeed
- if post-swap validation fails, restore from that backup and leave the prior persisted import record intact

Atomicity rule for app-data fallback:

- app-data staging may be used for content assembly, but the final atomic swap must always happen through a same-volume temporary sibling under the destination mirror's parent directory
- if the app cannot obtain a same-volume final-swap location, it must abort the mutation rather than claiming an atomic update

Phase 1 should use real Git ignore semantics when deciding whether repo-local staging is safe:

- prefer `git check-ignore` against the repo-local `.tmp-skills/` path when `git` is available and the configured repository is a valid Git working tree
- if that check confirms the path is ignored, repo-local staging is allowed
- if `git check-ignore` is unavailable or inconclusive, use app-data staging

Cleanup rules:

- repo-local or app-data staging cleanup is best-effort
- cleanup failure must not block a successful import or update from being reported, but it must emit a warning

Startup reconciliation posture:

- on startup, the app may perform a best-effort reconciliation scan for leaked staging directories, leftover backup siblings, and orphaned managed manifests
- reconciliation should never silently overwrite a live mirror
- when safe automatic cleanup is unclear, surface integrity warnings and require an explicit user repair action instead

Staging isolation rule:

- every import or update operation should use its own unique staging subdirectory, for example namespaced by `sourceId` or `importId` plus an operation nonce
- unique staging paths are required even though Phase 1 serializes mutations, so crash recovery and diagnostics can attribute leftovers to a specific operation

## Update Workflow

Recommended update flow:

1. app fetches the upstream default branch for a source
2. app compares `lastFetchedCommit` and imported `pinnedCommit` values against the new upstream commit
3. if the selected variant still exists and the upstream commit is newer, mark `updateAvailable`
4. user explicitly chooses update
5. app replaces the local mirror from the newer upstream variant content
6. app updates `pinnedCommit`, timestamps, and source fetch metadata

Data-flow rule:

- a source fetch updates only source-level fields such as `lastFetchedCommit`, source warnings, and source status
- import-level fields such as `lastCheckedCommit`, `updateAvailable`, `warnings`, and any detected fingerprint are updated only when that specific import is evaluated against the fetched source snapshot
- a fetch may trigger such evaluations immediately in the same user action, but the design treats them as explicit import-level updates rather than implicit blanket rewrites of all imports

Rules:

- update is explicit, never automatic
- update must fail safely if the variant disappeared or became invalid
- update should explain whether the failure came from network, detection, or local conflict
- Phase 1 does not attempt a three-way merge because mirrors are read-only
- update must stage and validate the replacement mirror before swapping it into place
- `lastCheckedCommit` may advance on fetch even when `pinnedCommit` does not
- when the tracked variant no longer exists at the newest fetched commit, `updateAvailable` stays `false` and the import record must surface a structured warning such as `variant_disappeared`
- the UI should show a warning state for disappeared variants rather than an update action

Default-branch and rewritten-history rules:

- if GitHub reports a different default branch on a later fetch, update `defaultBranch` and treat that as the new tracked branch for future checks
- a previously imported local mirror remains valid even if its `pinnedCommit` is no longer reachable from the new default-branch history
- if upstream force-push or branch replacement means the old `pinnedCommit` is orphaned, surface a source warning such as `upstream_history_rewritten` rather than invalidating the local mirror
- `updateAvailable` is based on whether the tracked variant is successfully detected at the current default-branch HEAD and that detected importable snapshot differs from the currently pinned import; it does not depend on Git ancestry between the old and new commits

Recommended Phase 1 formula:

```text
updateAvailable =
  variant_detected_at(lastCheckedCommit) &&
  detected_variant_fingerprint(lastCheckedCommit, variantPath) != pinnedVariantFingerprint &&
  importWarnings does not contain variant_disappeared
```

Lifecycle rule for `variant_disappeared`:

- the warning persists until a later fetch and detection pass finds the same tracked variant again or the user removes the imported mirror record
- the warning is derived from current source state and is not user-dismissible persistent UI state
- a path change for the upstream variant counts as `variant_disappeared` in Phase 1, even if the upstream content looks semantically similar

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
- the top-level `external` source count in Phase 1 intentionally includes both manual and managed entries
- where counts are shown for the `external` bucket, the UI should be free to append a non-filtering breakdown such as `external (5 manual, 3 managed)`
- no separate managed-only filter pill is added in Phase 1; the `External Sources` view is the primary management surface for imported mirrors
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

Recommended implementation sequence:

1. extract the shared skill-identity helper and migrate scanner plus document reader
2. extend Rust and TypeScript DTOs with optional `managedSource` metadata
3. add scanner enrichment for `.skills-manager-source.json`
4. add external-source persistence plus cache management
5. add import, update, and warning flows
6. add the dedicated `External Sources` UI and managed badges in existing skill surfaces

## Scanner Integration

The existing skill scanner should not scan cached external repositories.

Instead:

- scanner input remains the configured `my-skills` repository
- imported mirrors are written into a dedicated managed subtree under `external/`
- scanner continues to classify imported mirrors as `external`
- scan-result enrichment should attach managed-origin metadata only when both of these are true:
  - the skill directory contains `.skills-manager-source.json`
  - the app-local import record for the matching `skillId` exists and agrees with the manifest
- if the manifest exists but the import record is missing or mismatched, return the skill as plain `external` plus integrity warning metadata rather than as a healthy managed mirror

This avoids changing the current `custom | external` schema in Rust structs, TypeScript types, filters, persisted selection IDs, and i18n.

Recommended DTO addition:

- extend `SkillSummary` and `SkillDocument` with optional import metadata such as:
  - `managedSource: null | { kind: "github_import", importId, repoUrl, pinnedCommit, agentKey, updateAvailable }`

This is an additive DTO change rather than a breaking `sourceType` migration.

### Frontend Presentation Rules

Phase 1 should make the managed/manual distinction visible without changing the source-filter schema.

Rules:

- `SkillList` cards under the existing `external` section should render an additional badge when `managedSource != null`
- `SkillDetailPanel` should show origin metadata and update state when `managedSource != null`
- `buildSourceSummaries()` continues to report only `custom`, `external`, and `all`
- Phase 1 does not add a separate managed count pill; imported-mirror counts belong on the `External Sources` view instead
- cleanup warnings from successful import or update operations should surface in the immediate operation response and structured diagnostics/logging, but they are not persisted as long-lived source or import warnings
- import-level state priority in the skills UI should be `integrity mismatch` > `variant disappeared` > `update available` > `healthy managed`

Recommended i18n grouping:

- use `externalSources.*` for the new dedicated view and source-management actions
- use `skills.badges.*` for managed/manual origin badges rendered inside the existing skills UI
- use `skills.detail.managedSource.*` for origin labels shown in `SkillDetailPanel`
- use `skills.warnings.*` for managed-mirror integrity and disappeared-variant warnings

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
- deleting an imported mirror is blocked when its `skillId` is still referenced by any saved scene, agent, or project configuration
- Phase 1 does not cascade-delete those references automatically; the user must detach the references first
- `remove source + imported mirrors` is all-or-nothing in Phase 1; if any imported mirror under that source is still referenced, block the whole destructive action rather than partially deleting a subset
- reference detection is based on `skillId`, because scenes, agents, and projects persist skill references by `skillId`
- the UI should list the blocking scene, agent, and project names when a destructive removal is rejected

### Duplicate External Skills In Agent Sync

Phase 1 does not try to deduplicate a manual `external` skill and a managed imported mirror that happen to represent similar upstream content.

Rules:

- duplicate-looking skills remain distinct if they have different stable `skillId` values
- agent sync relies on the existing `managed_entry_name(skillId)` behavior, so two different skill IDs deploy to different managed target entry names
- agent sync target naming is derived from stable `skillId`, not upstream skill folder names, so manual and managed external entries do not collide merely because they share a display name or leaf directory name
- unmanaged target conflicts continue to be handled by the existing target-manifest conflict rules
- Phase 1 may warn in the UI when a managed mirror and a manual external skill share the same leaf directory name or the same parsed skill name, but it does not block assignment solely on semantic similarity

### Concurrency And State Writes

Phase 1 should use the same mutation-safety posture as the rest of the app, but make it explicit for this new domain.

Rules:

- writes to `external-sources.json` must use advisory config locking plus atomic write-rename
- source fetch, import, update, and remove operations should serialize per source ID
- a fetch that refreshes `lastCheckedCommit` must not race with an import or update that rewrites the same import record
- mirror staging and swap must happen under a per-source or per-import critical section
- source operations must treat the source record and managed mirror tree as one mutation unit under the same source-level lock
- source operations do not mutate scenes, agents, or projects; those stores remain eventually consistent readers of scanned skill IDs rather than participants in a cross-store transaction

Recommended mutation ordering:

- import: stage -> validate -> swap mirror into place -> atomically write import record
- update: stage -> validate -> swap replacement -> validate live mirror -> atomically write updated import record
- remove: validate live mirror -> delete mirror -> atomically remove import record

Phase 1 intentionally allows `external_sources` persistence to be stricter than older stores.

Reason:

- external-source mutations are longer-lived multi-step operations that combine git fetches, metadata updates, and staged filesystem swaps
- those operations have a materially higher corruption risk than the app's current short single-file preference writes

Follow-up note:

- broad adoption of advisory locking and atomic write-rename across `settings`, `skills`, `scenes`, `agents`, and `projects` is desirable later, but it is not a prerequisite for this Phase 1 design
- single-file `external-sources.json` is an intentional Phase 1 trade-off: global mutation serialization is acceptable because source add/fetch/import/update/remove operations are low-frequency user actions, and one file keeps backup, inspection, and recovery simpler than early sharding
- successful source removal should also remove the source cache directory
- a best-effort orphaned-cache sweep is reasonable at startup or on a manual maintenance action, but it is not required for the core Phase 1 workflow

Locking contract:

- use a cross-process advisory lock file under the app config directory, dedicated to external-source mutations
- desktop and CLI must both honor that same lock file
- per-source serialization is a logical rule enforced while holding the global file lock: one source mutation runs at a time
- lock acquisition should use a bounded timeout and return a structured conflict error when the lock cannot be obtained in time
- Phase 1 defaults to immediate rejection with a structured "external source busy" error rather than background queuing

### Manifest Integrity Failure Handling

Manifest integrity mismatches should not be handled ad hoc.

Rules:

- import validates staged manifest plus computed `skillId` before first swap into the repo
- importer validates the computed path-derived `skillId`, stored import-record `skillId`, and manifest `skillId` before every update or delete
- updater validates the existing live mirror before staging, validates the staged replacement before swap, and validates the live mirror again after swap before persisting the updated import record
- remove validates live mirror metadata before deletion
- importer aborts the mutation if any of those values disagree
- if a post-swap validation unexpectedly fails, restore the previous mirror from the staged backup and leave the persisted import record unchanged
- scanner does not block the whole repository scan on one malformed managed mirror
- when scanner detects a managed mirror with integrity mismatch, it should still return the skill as an `external` entry but attach warning metadata such as `managedSource.integrity = "mismatch"`
- desktop and any future CLI should surface a repair-oriented message such as "managed mirror metadata is inconsistent; re-import or remove this mirror"

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
- concurrent fetch plus import against the same source ID
- shared-helper regressions between scanner, document reader, and importer
- repo-local `.tmp-skills/` staging vs app-data fallback staging
- large source inventories and large variant lists at least at smoke-test scale

Broader repo validation should include:

1. targeted Rust tests for new core modules
2. targeted frontend tests where meaningful
3. `node scripts/check-module-doc-coverage.mjs`
4. `node scripts/check-module-doc-diff.mjs --range <base>...HEAD`
5. `npm run check:architecture`
6. `npm run verify`

I18n planning must be part of Phase 1 UI work.

At minimum, plan keys for:

- managed GitHub mirror badges
- manual external badges where they are contrasted with managed mirrors
- update-available and variant-disappeared states
- imported-from and pinned-commit labels
- destructive source-removal confirmations

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
