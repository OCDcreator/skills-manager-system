# Open Source Knowledge Base Design

## Summary

This design defines a separate Git repository for an AI-maintained knowledge base that corresponds to the user's local GitHub open-source project collection. The knowledge base does not replace the existing `open-source-project` index. It references that index as the source of truth for repository identity, local clone paths, category, remote URL, and default branch.

The product goal is not a manually curated Markdown library. The goal is an automated project memory system:

```text
Project discovery     = read open-source-project index
Normalized snapshot   = committed generated cache for CI-visible validation
Initial knowledge     = AI-generated project purpose summary
Usage evidence        = commands, config files, deployment attempts, failures, fixes
Status transition     = evidence-driven, with limited manual decisions
Documentation         = regenerated from source docs + evidence
Gatekeeping           = local + CI verification, not personal discipline
Cross-device sync     = Git + per-host event logs + local path adapters
```

The user should not need to write documentation whenever a project is cloned. New projects start as `unused` with AI-generated summaries. Projects become `using` automatically only when strong usage evidence exists. Projects become `abandoned` only after explicit human confirmation.

## Goals

- Create a standalone `project-knowledge-base` repository that can be cloned on Windows and macOS.
- Reuse the existing `open-source-project` index instead of manually maintaining duplicate repository inventory.
- Commit a generated normalized index snapshot so CI can validate knowledge entries without local machine paths.
- Automatically generate a short purpose document for every discovered project.
- Keep only three user-facing states: `using`, `unused`, and `abandoned`.
- Allow `unused -> using` automatic promotion from strong real-world evidence.
- Require human confirmation for `using -> abandoned` because abandonment is a product decision.
- Require agents to read and update the knowledge base when deploying, configuring, debugging, or validating an open-source project.
- Record setup pitfalls, fixes, and verification commands automatically as evidence events.
- Support Windows and macOS paths without committing machine-specific absolute paths.
- Use local and CI gates so freshness, schema, coverage, and evidence rules are enforced.

## Non-Goals

- Do not build a web app or desktop UI in the first version.
- Do not require hand-written long documentation for all cloned repositories.
- Do not manually duplicate the full `open-source-project` manifest inside the knowledge repository. A generated sanitized snapshot is allowed for CI and cross-device consistency checks.
- Do not auto-abandon projects based on inactivity.
- Do not require CI to run platform-specific setup commands that depend on local machine state.
- Do not treat README summaries as proof that a project is in use.

## Repository Boundary

The knowledge base should be a separate repository, tentatively named `project-knowledge-base`.

`open-source-project` remains responsible for:

- Cloning and syncing GitHub repositories.
- Maintaining the canonical project inventory.
- Recording project name, category, remote URL, default branch, and local relative path.
- Exporting machine-readable indexes such as Windows `projects.json` or macOS SQLite/JSON.

`project-knowledge-base` is responsible for:

- Tracking usage state and lifecycle evidence.
- Generating and storing project purpose summaries.
- Recording deployment/configuration/debugging experience.
- Tracking freshness policy and verified refs.
- Enforcing documentation and evidence gates.

## Directory Structure

```text
project-knowledge-base/
├── knowledge.yaml
├── snapshots/
│   └── open-source-project.normalized.json
├── docs/
│   ├── generated/
│   │   └── lean-ctx.md
│   ├── using/
│   │   └── lean-ctx.md
│   ├── abandoned/
│   │   └── some-project.md
│   └── notes/
├── events/
│   ├── windows-lt.jsonl
│   └── macbook.jsonl
├── templates/
│   ├── generated.md
│   ├── using.md
│   └── abandoned.md
├── scripts/
│   ├── sync-index.mjs
│   ├── record-event.mjs
│   ├── use-project.mjs
│   ├── generate-docs.mjs
│   ├── verify-ci.mjs
│   ├── verify-local.mjs
│   └── report-stale.mjs
├── .github/workflows/verify.yml
├── package.json
└── README.md
```

Machine-local configuration must not be committed:

```text
.local/
└── config.json
```

`.local/config.json` provides host-specific paths to the local `open-source-project` root and index. If `machine_id` is missing, `record-event.mjs` should initialize a stable generated value and write it back to `.local/config.json`.

## Cross-Platform Local Configuration

The repository must avoid committed Windows or macOS absolute paths. Host-specific paths live in `.local/config.json`, which is ignored by Git.

Windows example:

```json
{
  "machine_id": "windows-lt-8f3b7c2e",
  "host": "windows-lt",
  "platform": "windows",
  "openSourceRoot": "C:\\Users\\lt\\Desktop\\Write\\open-source-project",
  "openSourceIndex": "C:\\Users\\lt\\Desktop\\Write\\open-source-project\\_index-export\\projects.json"
}
```

macOS example:

```json
{
  "machine_id": "macbook-a41d9e7f",
  "host": "macbook",
  "platform": "mac",
  "openSourceRoot": "/Volumes/SDD2T/obsidian-vault-write/open-source-project",
  "openSourceIndex": "/Volumes/SDD2T/obsidian-vault-write/open-source-project/_index/projects.db"
}
```

The v1 implementation should normalize both Windows JSON and macOS SQLite/JSON sources into `snapshots/open-source-project.normalized.json`. All verification scripts consume this normalized snapshot. Direct SQLite support remains an adapter concern, not a verifier concern.

Normalized snapshot shape:

```json
{
  "schema_version": 1,
  "generated_at": "2026-05-06T12:00:00Z",
  "source_host": "windows-lt",
  "projects": [
    {
      "key": "AI-tools-agents/lean-ctx",
      "name": "lean-ctx",
      "category": "AI-tools-agents",
      "remote_url": "https://github.com/yvgude/lean-ctx.git",
      "default_branch": "main",
      "path_relative": "AI-tools-agents/lean-ctx"
    }
  ]
}
```

The snapshot is generated, committed, and treated as a CI-visible cache of the external index. It is not hand-edited and does not become the source of truth for cloning or syncing repositories.

## Project Identity

Project identity should be stable across Windows and macOS.

The stable key is `category/name` from the normalized snapshot. Human-facing docs may use the project name, but machine checks should use `key` to avoid collisions.

The knowledge base must validate that every project entry exists in the normalized snapshot by matching:

- `remote_url`
- `key`
- `name`
- `category`

The knowledge base may cache remote URL and category in generated docs for readability, but `open-source-project` remains the source of truth.

## Lifecycle States

Only three user-facing states are allowed:

```yaml
status: using | unused | abandoned
```

### `unused`

The project has been discovered but is not known to be in active use.

Rules:

- Created automatically when a project appears in the external index.
- Must have an AI-generated purpose summary.
- Does not require setup verification.
- Does not require human confirmation.
- May become `using` automatically when strong evidence appears.

### `using`

The project is actively used in the user's workflow.

Rules:

- May be promoted automatically from `unused` when strong evidence exists.
- Must have real evidence, not just README reading or conversation mention.
- Must have freshness policy and verification metadata.
- Must have generated or maintained documentation under `docs/using/`.
- Must not be demoted automatically to `unused`; stale or inactivity should be represented as flags.

### `abandoned`

The project has been intentionally rejected or discontinued.

Rules:

- Requires explicit human confirmation.
- Must record abandonment reason.
- Should record replacement or decision context when available.
- May trigger a future reconsideration suggestion if new strong usage evidence appears, but automatic restoration to `using` should require confirmation.

## Automatic State Transitions

```text
discovered in index
→ unused

unused + strong usage evidence
→ using

using + stale freshness
→ using + stale

using + no recent events
→ using + inactive

using + explicit user abandonment
→ abandoned

abandoned + new strong evidence
→ abandoned + reconsider_suggested
```

Strong evidence examples:

- A successful deployment or setup command.
- A successful validation command, such as an MCP server listed as connected.
- A configuration file in an active project referencing the tool.
- A successful CLI, MCP, API, or plugin invocation during a task.
- A verified fix for a setup or runtime failure.

Weak evidence examples:

- The project was cloned.
- A README was summarized.
- A conversation mentioned the project name.
- An agent searched the project without using it.

Weak evidence may create a `using_candidate` recommendation internally, but it must not change the public `status` to `using`.

### Deterministic Promotion Criteria

Automatic `unused -> using` promotion is allowed only when all of these conditions are true:

- The project exists in `snapshots/open-source-project.normalized.json`.
- The project has at least one generated summary with source metadata.
- The project has qualifying strong evidence from the last 180 days.
- The evidence references the same project `key` as `knowledge.yaml`.
- The evidence has passed event schema validation.

Qualifying strong evidence is one of:

- One `command_success` event with `verification: true` and a non-empty `command`, `output_summary`, and `observed_ref`.
- One `config_file` event plus one `command_success` event for the same project and platform within 30 days.
- One `setup_failure_resolved` event with a non-empty `resolution` and `verified_by` command.
- One `agent_tool_success` event showing a successful CLI, MCP, API, or plugin invocation with `verification: true`.

Non-qualifying events may be stored, but they must not promote a project by themselves.

## Knowledge Manifest

`knowledge.yaml` is the authoritative materialized state and policy file. Events are append-only evidence that justify automatic transitions and regenerate documentation. If `knowledge.yaml` and events disagree, verification should fail until `generate-docs` or a state sync task reconciles them.

Example:

```yaml
schema_version: 1
source_index:
  adapter: open-source-project
  required: true

projects:
  AI-tools-agents/lean-ctx:
    name: lean-ctx
    status: using
    status_source: evidence
    doc: docs/using/lean-ctx.md
    generated_summary: docs/generated/lean-ctx.md
    freshness:
      strategy: version
      verified_ref: "lean-ctx 3.4.7"
      checked_at: "2026-05-06"
      fail_on_stale: true
    automation:
      ai_may_update: true
      require_sources: true
      require_command_evidence_for_setup: true
    supported_platforms:
      - windows
      - mac
```

For `unused` projects:

```yaml
projects:
  dev-libraries/some-library:
    name: some-library
    status: unused
    generated_summary: docs/generated/some-library.md
    automation:
      ai_may_update: true
```

For `abandoned` projects:

```yaml
projects:
  AI-tools-agents/old-tool:
    name: old-tool
    status: abandoned
    doc: docs/abandoned/old-tool.md
    decision:
      decided_at: "2026-05-06"
      reason: "Windows support was unreliable"
      replacement: "lean-ctx"
      confirmed_by_user: true
```

## Event Log Model

Events are append-only JSONL records. They preserve original evidence and reduce Git conflicts between devices.

Each host writes to its own file:

```text
events/windows-lt.jsonl
events/macbook.jsonl
```

Host identity must use a stable `machine_id` generated once in `.local/config.json`. Display host names are allowed to change, but `machine_id` should remain stable. Event IDs should be UUIDv7 or ULID values generated by `record-event.mjs`, not timestamp counters.

Common required fields for every event:

```json
{
  "schema_version": 1,
  "event_id": "01HX0000000000000000000000",
  "project_key": "AI-tools-agents/lean-ctx",
  "project_name": "lean-ctx",
  "platform": "windows",
  "machine_id": "windows-lt-8f3b7c2e",
  "host_label": "windows-lt",
  "event_kind": "command_success",
  "source_task": "Verify OpenCode project-level lean-ctx configuration",
  "source_type": "command_output",
  "observed_at": "2026-05-06T12:34:56Z"
}
```

Allowed `event_kind` values for v1:

- `readme_summary`
- `command_success`
- `setup_failure_resolved`
- `setup_failure_open`
- `setup_limitation_accepted`
- `config_file`
- `agent_tool_success`
- `abandonment_decision`

Event-specific schemas must be enforced by `verify:evidence`.

Event schema table:

| `event_kind` | Required event-specific fields | Optional fields | Promotion eligible |
| --- | --- | --- | --- |
| `readme_summary` | `summary`, `source_files`, `confidence` | `install_hints`, `entrypoints` | No |
| `command_success` | `command`, `output_summary`, `observed_ref`, `verification` | `working_directory`, `duration_ms` | Yes, only when `verification: true` |
| `setup_failure_resolved` | `error_summary`, `root_cause`, `resolution`, `verified_by` | `failed_command`, `output_summary` | Yes |
| `setup_failure_open` | `error_summary`, `failed_command`, `next_action` | `output_summary`, `issue_url` | No |
| `setup_limitation_accepted` | `limitation`, `accepted_reason`, `scope` | `workaround`, `review_after` | No |
| `config_file` | `path`, `summary` | `matched_keys`, `repo_relative` | Yes only when paired with `command_success` |
| `agent_tool_success` | `tool_name`, `operation`, `output_summary`, `observed_ref`, `verification` | `endpoint`, `duration_ms` | Yes, only when `verification: true` |
| `abandonment_decision` | `reason`, `confirmed_by_user`, `decided_at` | `replacement`, `reconsider_after` | No |

`source_files` and `path` values should be repository-relative when possible. `confidence` for `readme_summary` must be `low`, `medium`, or `high`.

Command success event:

```json
{
  "schema_version": 1,
  "event_id": "01HX0000000000000000000001",
  "project_key": "AI-tools-agents/lean-ctx",
  "project_name": "lean-ctx",
  "platform": "windows",
  "machine_id": "windows-lt-8f3b7c2e",
  "host_label": "windows-lt",
  "event_kind": "command_success",
  "source_type": "command_output",
  "source_task": "Verify OpenCode project-level lean-ctx configuration",
  "command": "opencode mcp list",
  "output_summary": "lean-ctx connected",
  "observed_ref": "lean-ctx 3.4.7",
  "verification": true,
  "observed_at": "2026-05-06T12:34:56Z"
}
```

Failure and resolution event:

```json
{
  "schema_version": 1,
  "event_id": "01HX0000000000000000000002",
  "project_key": "AI-tools-agents/lean-ctx",
  "project_name": "lean-ctx",
  "platform": "windows",
  "machine_id": "windows-lt-8f3b7c2e",
  "host_label": "windows-lt",
  "event_kind": "setup_failure_resolved",
  "source_type": "command_output",
  "source_task": "Verify OpenCode lean-ctx plugin behavior",
  "error_summary": "PowerShell which lean-ctx failed",
  "root_cause": "PowerShell does not provide Unix which, but the OpenCode plugin runs through Bun Shell",
  "resolution": "Simulated the plugin with Bun Shell and confirmed which lean-ctx resolves to the Cargo binary",
  "verified_by": "bun -e 'import { $ } from \"bun\"; await $`which lean-ctx`.quiet()'",
  "observed_at": "2026-05-06T12:40:00Z"
}
```

Config evidence event:

```json
{
  "schema_version": 1,
  "event_id": "01HX0000000000000000000003",
  "project_key": "AI-tools-agents/lean-ctx",
  "project_name": "lean-ctx",
  "platform": "windows",
  "machine_id": "windows-lt-8f3b7c2e",
  "host_label": "windows-lt",
  "event_kind": "config_file",
  "source_type": "file_inspection",
  "source_task": "Inspect OpenCode project configuration",
  "path": "skills-manager-system/.opencode/opencode.json",
  "summary": "Project-level OpenCode MCP config enables lean-ctx as a local server",
  "observed_at": "2026-05-06T12:45:00Z"
}
```

## AI Documentation Generation

Documentation should be generated from three source classes:

- Upstream sources: README, docs, release notes, package metadata, repository config.
- Local evidence: event logs, command outputs, config file paths, verification commands.
- Conversation-derived task context: user intent and agent conclusions from actual work.

Generated text must identify source type. It must not present guesses as verified facts.

Generated summary docs for `unused` projects should include:

- What the project appears to do.
- Main use cases.
- Installation or entrypoint hints when obvious.
- Key files or docs worth reading later.
- Confidence level and source files used.

`using` docs should include:

- Purpose in the user's workflow.
- Current use locations.
- Install and configuration steps that were actually verified.
- Common commands.
- Pitfalls and resolutions.
- Verification records.
- Freshness policy.
- Platform-specific notes.

`abandoned` docs should include:

- Why it was abandoned.
- What replaced it, if anything.
- Known pitfalls.
- Conditions for future reevaluation.

## Agent Protocol

Integrated agents that use an open-source project must follow this executable protocol:

```text
1. Run `npm run use-project -- <project-key>` or call the equivalent script API.
2. The wrapper locates the project in project-knowledge-base.
3. If absent, it syncs the normalized snapshot and generates an unused summary.
4. The wrapper prints the docs that must be read before setup or debugging.
5. During deployment/configuration/debugging, the agent records events with `npm run record-event -- ...`.
6. If a failure is resolved, the agent records a `setup_failure_resolved` event.
7. The agent runs `npm run generate` to refresh affected docs and materialized state.
8. The agent runs `npm run verify:local -- <project-key>` before reporting success.
```

The gate is aimed at integrated agent behavior, not user discipline. The knowledge base cannot technically prevent arbitrary external agents from making claims. It can provide wrappers and verification commands that project-level agent instructions require before reporting setup success.

Project-level agent instructions should state that setup success is not reportable unless `record-event`, `generate`, and scoped `verify:local` have succeeded for the project.

## Freshness Strategy

Freshness is mixed-strategy because the open-source projects have different release practices.

Allowed strategies:

```yaml
strategy: commit | tag | version | manual
```

Rules:

- `commit`: compare the local clone's fetched default-branch HEAD with `verified_ref`. Local verification may run `git fetch` when configured; CI validates only the committed metadata and stale report.
- `tag`: compare the latest reachable tag in the local clone with `verified_ref`. The tag sorting strategy must be recorded as `semver` or `git-date`.
- `version`: run a configured command on each supported platform and compare output with that platform's `verified_ref`.
- `manual`: use `checked_at` plus a configured maximum age.

Freshness metadata can be platform-specific:

```yaml
freshness:
  strategy: version
  platforms:
    windows:
      command: lean-ctx --version
      verified_ref: "lean-ctx 3.4.7"
      checked_at: "2026-05-06"
    mac:
      command: lean-ctx --version
      verified_ref: "lean-ctx 3.4.7"
      checked_at: "2026-05-06"
  fail_on_stale: true
```

`using` projects default to `fail_on_stale: true`. Lower-risk tools can opt into warning-only behavior with `fail_on_stale: false`.

Stale status must not automatically rewrite project docs. Instead, the next agent that uses the project must refresh upstream context, rerun verification, and update evidence.

Stale failures are scoped:

- `verify:local -- --project <key>` fails stale `using` projects when `fail_on_stale` is true.
- Full local `verify:local` fails stale `using` projects when `--strict-freshness` is set.
- CI fails stale metadata only when the changed files touch that project's `knowledge.yaml` entry, docs, or events. Otherwise CI reports stale projects as warnings to avoid blocking unrelated documentation work.

## Verification Gates

Local and CI verification reuse the same core validators but run different entrypoints because local verification can access host-specific paths and CI cannot.

`package.json` should expose these commands:

```json
{
  "scripts": {
    "sync:index": "node scripts/sync-index.mjs",
    "record-event": "node scripts/record-event.mjs",
    "use-project": "node scripts/use-project.mjs",
    "generate": "node scripts/generate-docs.mjs",
    "verify:local": "node scripts/verify-local.mjs",
    "verify:ci": "node scripts/verify-ci.mjs",
    "report:stale": "node scripts/report-stale.mjs"
  }
}
```

```text
npm run verify:local
├── sync local index adapter
├── compare local normalized index with committed snapshot
├── verify:index
├── verify:schema
├── verify:generated
├── verify:coverage
├── verify:evidence
└── verify:freshness

npm run verify:ci
├── read committed normalized snapshot
├── verify:index
├── verify:schema
├── verify:generated
├── verify:coverage
├── verify:evidence
└── verify:freshness-metadata
```

### `verify:index`

- Every project in `knowledge.yaml` must exist in `snapshots/open-source-project.normalized.json`.
- External snapshot projects absent from `knowledge.yaml` are warning-only by default.
- `STRICT_INDEX=1` can make missing knowledge entries fail.
- `verify:local` must fail when the current local index normalizes to content different from the committed snapshot.
- The repair path for snapshot drift is `npm run sync:index`, which regenerates `snapshots/open-source-project.normalized.json` before docs and verification are rerun.

### `verify:schema`

- `status` must be one of `using`, `unused`, or `abandoned`.
- `using` entries must define documentation, freshness, automation policy, and evidence requirements.
- `unused` entries must define or point to a generated summary.
- `abandoned` entries must define a confirmed decision.

### `verify:generated`

- Every project listed in `knowledge.yaml` must have a generated summary unless explicitly ignored.
- Every external snapshot project must have a generated summary only when `STRICT_INDEX=1` or after `sync-index` has materialized it into `knowledge.yaml`.
- Generated summaries must include source metadata and generation timestamp.
- Generated summaries may be regenerated automatically.

### `verify:coverage`

- `using` docs must contain required sections.
- `abandoned` docs or decisions must contain abandonment reason and replacement/reevaluation notes.
- `unused` docs are allowed to be summary-only.

### `verify:evidence`

- Automatic `unused -> using` promotion must be supported by strong evidence.
- Setup success claims must include command or config evidence.
- `setup_failure_resolved` events must include resolution evidence.
- `setup_failure_open` events must include `next_action` and must not promote projects to `using`.
- `setup_limitation_accepted` events must include `accepted_reason` and must not promote projects to `using`.
- Events must include `event_id`, `project_key`, `platform`, `machine_id`, `host_label`, `event_kind`, `source_task`, `source_type`, and `observed_at`.
- Event-specific fields must match the schema for their `event_kind`.

### `verify:freshness`

- `using` projects are checked against their freshness strategy.
- Project-scoped local verification fails for that project when it is stale and `fail_on_stale` is true.
- Full local verification fails stale `using` projects only when `--strict-freshness` is set.
- CI fails stale metadata only when changed files touch that project's `knowledge.yaml` entry, docs, or events.
- CI skips machine-specific command execution but validates freshness metadata and stale reports generated by local runs.

## Local And CI Responsibilities

`verify:local` can access machine-specific paths and commands:

- Read Windows or macOS `open-source-project` indexes.
- Inspect local repository HEADs and tags.
- Run tool version commands.
- Validate local config evidence.

`verify:ci` should be deterministic and path-independent:

- Validate YAML/JSON schema.
- Validate event format.
- Validate required docs and sections.
- Validate that generated summaries are present.
- Validate that committed freshness metadata is coherent.
- Validate that the committed normalized snapshot exists and matches its schema.
- Avoid relying on local clone paths that only exist on the user's machines.

## Privacy And Redaction

Conversation-derived and command-derived evidence must be safe to commit.

Rules:

- Event records must store summaries, not raw terminal logs, unless raw output is explicitly marked safe.
- Events must not include secrets, tokens, API keys, private keys, cookies, or credentials.
- Absolute local paths should be avoided in committed events unless the path is already public project context. Prefer repository-relative paths.
- `verify:evidence` should scan for common secret patterns and fail on likely leaks.
- Conversation-derived evidence must use concise redacted summaries and should not include full chat transcripts.

## Cross-Device Sync

The knowledge base syncs through Git.

Conflict-minimizing rules:

- Each host appends to its own event file.
- Events are the source of truth for evidence.
- Generated docs can be regenerated if conflicts occur.
- `knowledge.yaml` should change less often than event files.
- If conflicts happen, preserve events first and regenerate docs second.

Recommended sync flow:

```text
git pull
npm run sync:index
npm run generate
npm run verify:local
git add knowledge.yaml docs/ events/
git commit
git push
```

Agents may automate this flow, but should not commit unless explicitly requested by the user in the current session.

## Acceptance Criteria

- A new project in `open-source-project` can be discovered and receive an AI-generated `unused` summary without manual writing.
- A project with strong command/config evidence can be promoted from `unused` to `using` automatically.
- A `using` project cannot pass verification without evidence and freshness metadata.
- A `using` project cannot become `abandoned` without explicit user confirmation.
- The repository contains a generated normalized snapshot that lets CI validate project references without local machine paths.
- `verify:local` can use `.local/config.json`; `verify:ci` does not require `.local/config.json`.
- Windows and macOS can use different `.local/config.json` files while sharing the same committed knowledge repo.
- Windows and macOS write separate event logs to reduce Git conflicts.
- Local verification can use machine paths; CI verification remains path-independent.
- Agent setup/debugging work must leave event evidence and update generated docs before claiming success.
- Stale `using` projects are detected by commit/tag/version/manual strategy.
- Stale failures are scoped so unrelated CI changes are not blocked by untouched stale projects.
- Event records are schema-validated and redacted before commit.
- The system supports mostly automatic maintenance, with human input reserved for abandonment and exceptional decisions.

## Implementation Defaults

- The first implementation should normalize both Windows and macOS sources into JSON before verification. Direct SQLite support can be added later, but the core scripts should consume one normalized index shape.
- `docs/generated/**` should be fully regenerated. `docs/using/**` and `docs/abandoned/**` should use managed sections for generated content and preserve clearly marked human decision sections.
- Freshness failures should follow scoped behavior: project-scoped local verification fails for the stale project, full local verification fails only with `--strict-freshness`, and CI fails only when changed files touch the stale project's state, docs, or events.
- `report:stale` should provide a non-failing overview for routine review.
- `verify:local` should fail on normalized snapshot drift and tell the user to run `npm run sync:index`.
- Conversation-derived evidence should be written directly by agents as event records during the task, not imported later from opaque chat transcripts.
