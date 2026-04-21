# CLI Foundation Design

## Summary

This design adds an official headless CLI to `skills-manager-system` so large models and scripts can operate the project without the desktop UI.

The CLI is machine-first:

- default output is JSON
- human-readable output is opt-in via `--pretty`
- commands are stable and composable
- side effects return structured snapshots instead of loose text

The CLI must be able to manage the same repository and configuration state as the desktop app:

- settings
- skills
- agents
- scenes
- projects
- git workflows

The CLI must run independently from the desktop app, but its changes must remain visible to the desktop app because both surfaces read and write the same repository and config files.

## Product Goal

Deliver an official automation entrypoint that allows a model or script to:

1. inspect current project state
2. modify skill-manager configuration
3. apply agent sync
4. apply scenes and project assignments
5. run git workflows against the managed skills repository

The CLI should become the canonical non-UI interface for unattended model workflows.

## In Scope

- a standalone Rust CLI binary for `skills-manager-system`
- full command groups for `settings`, `skills`, `agents`, `scenes`, `projects`, and `git`
- default JSON responses with stable fields
- `--pretty` human-readable output
- shared config-file access with the desktop app
- shared business logic through existing Rust core modules where practical
- a shared runtime/context layer for config-dir, repo-path, sync-mode, and output concerns
- focused tests for CLI-facing behavior and runtime resolution
- documentation for the design, CLI usage surface, and new source modules

## Out of Scope

- background daemon or IPC bridge between CLI and desktop app
- persistent file watching or automatic desktop refresh
- remote API server mode
- interactive TUI
- shell wrapper scripts as the primary implementation
- duplicating business logic in a second service layer

## Recommended Approach

Use a standalone Rust CLI binary that reuses the current Rust `core` domain modules and shares the same config files as the desktop app.

### Why this is the right boundary

- The project already has meaningful business logic under `src-tauri/src/core/`.
- The current Tauri command layer is useful but depends on Tauri runtime types and should not become the CLI runtime boundary.
- The correct reuse target is the domain core plus a new shared runtime/context layer, not the UI adapter layer.
- Shared config files guarantee that desktop and CLI remain in sync without needing a separate sync service.

This keeps the CLI truly headless while preserving the current "thin command layer + core business logic" architecture rule.

## Architecture

### High-Level Shape

Recommended source layout:

- `src-tauri/src/core/*` — existing business logic remains the source of truth
- `src-tauri/src/app_runtime/*` — new shared runtime/context helpers
- `src-tauri/src/commands/*` — Tauri adapter layer for the desktop app
- `src-tauri/src/cli/*` — CLI argument and output layer
- `src-tauri/src/cli/main.rs` — CLI entrypoint

### Cargo Strategy

Phase 1 should stay in the current `src-tauri` package rather than splitting into a new workspace.

Recommended strategy:

- keep the existing desktop binary from `src-tauri/src/main.rs`
- add an explicit CLI binary target for `skills-manager`
- keep shared domain modules in the library target
- gate Tauri-specific desktop code behind a `desktop` Cargo feature
- gate CLI-only parsing dependencies behind a `cli` Cargo feature

Recommended Cargo shape:

- `default = ["desktop"]`
- `desktop` enables `tauri` and `tauri-plugin-dialog`
- `cli` enables `clap`

Recommended binary layout:

- desktop binary keeps `src-tauri/src/main.rs`
- CLI binary is added explicitly through `[[bin]]` so the command name and source path are stable

The important detail is not the exact file path. The important detail is that the CLI binary must compile without requiring desktop-only runtime wiring.

### Developer Build Experience

Because `default = ["desktop"]` keeps the current desktop build behavior, CLI-only builds should be documented explicitly for contributors.

At minimum, contributor-facing docs should state:

- default `cargo build` builds the desktop target set
- CLI-only builds use `--no-default-features --features cli`
- combined development builds may opt into both features when needed

### Core Responsibilities

#### `core`

`core` continues to own domain behavior:

- settings persistence
- skill scanning, state, and document loading
- agent inventory, config, and sync
- scene config and apply logic
- project config and assignment apply logic
- git operations

The CLI must call into these modules instead of reimplementing their rules.

#### `app_runtime`

`app_runtime` is a shared environment layer, not a second business layer.

Phase 1 should start small:

- `context.rs` — define a shared `AppRuntimeContext`
- `output.rs` — build consistent JSON and pretty responses

Config-dir, repo-path, sync-mode, and locking helpers may start as private helpers inside `context.rs`. Split them into more files only if the module crosses the repo's file-size guardrails or gains stable independent responsibilities.

This layer should centralize environment concerns that are currently repeated in desktop command adapters.

#### `commands`

`src-tauri/src/commands/*` remains the Tauri adapter layer:

- accept Tauri parameters
- obtain runtime paths through shared runtime helpers
- call `core`
- map errors into frontend-friendly strings or DTOs

#### `cli`

`src-tauri/src/cli/*` is the headless adapter layer:

- parse command-line arguments
- obtain config-dir and repo resolution from shared runtime helpers
- call `core`
- render JSON or pretty output
- set exit codes

The CLI layer must stay thin and must not write raw JSON config files directly.

### Argument Parsing

Phase 1 should use `clap` derive mode.

Reason:

- the command tree is nested and non-trivial
- the CLI needs stable help output and validation
- global options plus per-command flags fit `clap` derive well
- hand-written parsing would add risk without adding value

## Command Tree

The CLI should expose one root command, tentatively named `skills-manager`.

### `settings`

- `skills-manager settings get-repo-path`
- `skills-manager settings set-repo-path <path>`
- `skills-manager settings get-sync-mode`
- `skills-manager settings set-sync-mode <copy|symlink>`

### `skills`

- `skills-manager skills list`
- `skills-manager skills scan`
- `skills-manager skills doc <skill-id|relative-path>`
- `skills-manager skills state`
- `skills-manager skills enable <skill-id>`
- `skills-manager skills disable <skill-id>`

### `agents`

- `skills-manager agents list`
- `skills-manager agents enable <agent-key>`
- `skills-manager agents disable <agent-key>`
- `skills-manager agents set-path <agent-key> <path>`
- `skills-manager agents clear-path <agent-key>`
- `skills-manager agents sync [--mode copy|symlink]`

### `scenes`

- `skills-manager scenes list`
- `skills-manager scenes create <id> --name <name> [--description <text>]`
- `skills-manager scenes update <id> [--name <name>] [--description <text>]`
- `skills-manager scenes delete <id>`
- `skills-manager scenes set-active <id>`
- `skills-manager scenes clear-active`
- `skills-manager scenes set-skills <id> --disabled <csv>`
- `skills-manager scenes set-agents <id> --enabled <csv>`
- `skills-manager scenes set-skill-order <id> --order <csv>`
- `skills-manager scenes apply <id>`

### `projects`

- `skills-manager projects list`
- `skills-manager projects add <project-path> --name <name> [--skills <csv>] [--agents <csv>]`
- `skills-manager projects update <project-path> [--name <name>] [--skills <csv>] [--agents <csv>]`
- `skills-manager projects remove <project-path>`
- `skills-manager projects apply`

### `git`

- `skills-manager git status`
- `skills-manager git diff [--staged]`
- `skills-manager git log [--limit <n>]`
- `skills-manager git fetch`
- `skills-manager git pull`
- `skills-manager git push`
- `skills-manager git commit -m <message>`
- `skills-manager git sync-external`

### Global Options

- `--json` — explicit JSON mode even though JSON is the default
- `--pretty` — render human-readable output
- `--config-dir <path>` — use an alternate config root for testing or isolation
- `--repo <path>` — override repo path without persisting it
- `--quiet` — suppress non-essential output
- `--no-color` — disable pretty-mode color

## Output Contract

### Default Mode

Default stdout output is JSON.

- stdout carries the response payload
- stderr carries warnings, logs, and progress messages when needed
- JSON shape must remain stable across commands

### Unified Success Shape

```json
{
  "ok": true,
  "status": "success",
  "command": "agents sync",
  "data": {},
  "warnings": [],
  "meta": {
    "configDir": "C:/Users/example/AppData/Roaming/skills-manager-system",
    "repoPath": "C:/Users/lt/Desktop/Write/custom-project/my-skills",
    "timestamp": "2026-04-21T12:34:56Z",
    "version": "0.1.0"
  }
}
```

### Unified Partial Shape

```json
{
  "ok": true,
  "status": "partial",
  "command": "agents sync",
  "data": {
    "results": []
  },
  "warnings": [
    {
      "code": "agent_target_missing",
      "message": "No detected default path or override is configured for this agent.",
      "target": "cursor"
    }
  ],
  "meta": {}
}
```

### Warning Object Shape

Warnings should use a stable object structure so scripts and models can reason about partial success without parsing free-form text.

Recommended warning shape:

```json
{
  "code": "agent_target_missing",
  "message": "No detected default path or override is configured for this agent.",
  "target": "cursor",
  "details": {}
}
```

Rules:

- `code` uses stable snake_case
- `message` is human-readable
- `target` is optional and identifies the affected skill, agent, scene, project, or git operation
- `details` is optional structured context for automation consumers

### Unified Error Shape

```json
{
  "ok": false,
  "status": "error",
  "command": "scenes apply",
  "error": {
    "code": "scene_not_found",
    "message": "Scene 'focus' does not exist.",
    "details": {
      "sceneId": "focus"
    }
  },
  "warnings": [],
  "meta": {}
}
```

### Output Rules

- query commands should return the current snapshot or list directly under `data`
- mutation commands should return the updated snapshot where practical
- execution commands should return aggregate results and per-target detail
- `--pretty` should be rendered from the same internal response object, not from a separate ad hoc path
- all paths emitted in JSON should use forward slashes, including Windows paths, so model consumers do not need to handle backslash escaping
- JSON field names should use camelCase for payload objects, matching the current Rust serde DTO style; stable classifier values such as error codes and warning codes should use snake_case

## Error Model And Exit Codes

### Status Values

- `success`
- `partial`
- `error`

### Exit Codes

- `0` — success
- `1` — general failure
- `2` — argument or validation error
- `3` — missing configuration, such as unset repo path
- `4` — target not found
- `5` — external command failure, such as git or sync script
- `6` — filesystem or permission failure
- `7` — state conflict or invalid operation
- `8` — partial success

### Stable Error Codes

Use stable snake_case error codes, for example:

- `repo_path_not_configured`
- `scene_not_found`
- `skill_not_found`
- `agent_not_found`
- `project_not_found`
- `invalid_sync_mode`
- `git_command_failed`
- `sync_script_failed`
- `config_write_failed`
- `filesystem_permission_denied`

The `message` field is for humans. The `code` and `details` fields are for models and scripts.

### Error Mapping Strategy

The current `core` layer mostly uses `anyhow::Result`, so Phase 1 must define a stable adapter mapping rather than passing raw error strings through unchanged.

Recommended rule:

- CLI command adapters map known failure classes into stable snake_case codes
- unknown failures fall back to `internal_error`
- `details` carries structured command-specific context when available

Examples:

- missing repo path -> `repo_path_not_configured`
- unsupported sync mode -> `invalid_sync_mode`
- known Windows symlink permission failure -> `windows_symlink_privilege_required`
- git command failure -> `git_command_failed`
- sync script failure -> `sync_script_failed`

For git and sync-script failures, `details` should include at least:

- exit status when available
- stderr text
- invoked operation name

## Config Directory And Desktop Sync Strategy

### Shared Storage

By default, the CLI must use the same application config directory as the desktop app.

That means the CLI and desktop app share the same persisted state for:

- repo path
- agent sync mode
- agent target config
- scene config
- project config
- sync ledger and related runtime state files

### Override Support

`--config-dir <path>` is supported only for testing, CI, or isolated runs.

The default user path should always match the desktop app.

### Repo Resolution

Repo path resolution priority:

1. CLI `--repo <path>` override
2. persisted repo path in settings

The `--repo` override must not rewrite persisted settings unless the user explicitly runs `settings set-repo-path`.

### Tauri-Compatible Config Resolution

The default config-dir resolver must match the desktop app exactly. It must not stop at `dirs::config_dir()`.

For this project, the resolver must be based on the Tauri application identifier:

- production identifier: `com.ocdcreator.skills-manager-system`
- dev identifier: `com.ocdcreator.skills-manager-system.dev`

At minimum, the resolver must produce the same final application directory as Tauri for the active identifier, including the macOS case where the app-specific directory lives under `~/Library/Application Support/<identifier>`.

Phase 1 should include focused tests that lock this behavior down against the project identifiers above.

### Concurrent Mutation Safety

Current stores such as `SettingsStore` and `SkillStateStore` use read-modify-write without locking.

Phase 1 should therefore add an advisory config lock for mutation commands.

Recommended strategy:

- create a lock file under the config root, such as `.skills-manager-system.lock`
- mutation commands acquire an exclusive advisory lock before reading and writing config-backed state
- read-only commands do not need to hold the lock

This is sufficient for desktop-plus-CLI coordination without introducing a daemon.

### Consistency Model

- No background daemon is required.
- No desktop process dependency is required.
- Consistency comes from shared storage, not IPC.
- advisory locking prevents concurrent mutation conflicts across desktop and CLI writers
- Writes should continue to use store/core write paths rather than direct JSON edits from the CLI layer.

## MVP Scope

The first phase should cover all six command groups so a model can complete real workflows without falling back to the UI.

### Phase 1

#### Phase 1a

- shared runtime/context layer
- CLI root entrypoint
- default JSON and `--pretty`
- read-only commands for `settings`, `skills`, and `agents list`
- config-dir and repo override behavior
- Cargo feature and binary layout

#### Phase 1b

- mutation commands for `settings`, `skills`, and `agents`
- `scenes` command group
- `projects` command group
- `git` command group
- advisory locking for mutations

### Phase 2

- stronger pretty output
- file-based batch input helpers such as `--skills-file`
- richer error and warning taxonomy
- CLI usage guide and schema examples
- diagnostic commands such as `doctor`

### Phase 3

- shell completion
- dry-run support
- compound automation commands
- finer-grained diff and preview behavior
- optional single-project apply command if the project domain later needs it

## Distribution

Phase 1 does not need a polished end-user installer flow, but it should define how the binary is obtained.

Recommended initial distribution:

- CI produces standalone CLI artifacts for supported platforms
- local developers can build from source with Cargo
- desktop bundling of the CLI binary is optional and can be deferred until the command surface stabilizes

If the project later decides to package the CLI together with the desktop app, that should be treated as a distribution task rather than a prerequisite for the runtime architecture.

## Implementation Order

Recommended sequence:

1. define Cargo features and explicit CLI binary target
2. add shared runtime/context modules
3. add CLI root binary and output framework with `clap` derive
4. wire Phase 1a read-only commands
5. wire mutation commands plus advisory locking
6. wire `scenes`, `projects apply`, and `git`
7. add pretty-mode polish, examples, and docs

This sequence keeps the work incremental and avoids turning the CLI into one oversized file.

## Expected Source Changes

Expected new or modified source areas:

- create `src-tauri/src/app_runtime/`
- create `src-tauri/src/app_runtime/context.rs`
- create `src-tauri/src/app_runtime/output.rs`
- modify `src-tauri/Cargo.toml` for explicit CLI binary target, features, and dependencies
- modify `src-tauri/src/lib.rs` only as needed to keep desktop integration aligned
- modify `src-tauri/src/commands/*.rs` to consume shared runtime helpers where repetition exists
- create `src-tauri/src/cli/`
- create `src-tauri/src/cli/main.rs`
- create CLI command modules under `src-tauri/src/cli/commands/`

If Cargo layout requires a dedicated binary target or crate split, keep the change minimal and aligned with the current repo structure rather than introducing an unnecessary workspace explosion.

## Validation Strategy

Validation should move from focused checks to broader verification:

1. focused Rust tests for shared runtime helpers
2. focused Rust tests for CLI argument parsing and output shape
3. targeted command tests against temporary config directories
4. `cargo check --manifest-path src-tauri/Cargo.toml`
5. `cargo test --manifest-path src-tauri/Cargo.toml`
6. `npm run verify`

Key scenarios to cover:

- Tauri-compatible config-dir resolution for production and dev identifiers
- config-dir override behavior
- repo override behavior
- default JSON output shape
- pretty output still driven by the unified response object
- forward-slash path normalization in JSON output
- settings mutations round-trip through shared files
- concurrent mutation lock behavior
- `skills list` returns the joined scan-plus-state view
- scene/project/agent commands return updated snapshots
- `projects apply` operates on the full persisted snapshot rather than a single project argument
- git command failures map to stable error codes
- Windows symlink failures map to a stable explicit code and message

## Design Constraints

- CLI must stay independent from desktop runtime
- core business logic must not be duplicated
- `app_runtime` must not become a second business center
- CLI command files must remain thin
- config writes must continue through store/core modules
- output schema must remain stable for model consumption
- default config-dir resolution must stay Tauri-compatible for this app identifier

## Future Expansion Path

This design leaves room for:

- richer diagnostics
- dry-run and preview flows
- command composition helpers
- more advanced model-oriented automation wrappers

The first success criterion is simpler: a reliable machine-first CLI that can fully operate the current project without the desktop UI while staying synchronized with it through shared state.
