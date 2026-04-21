# CLI Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a machine-first `skills-manager` CLI that runs headlessly, shares config/state with the desktop app, and covers the current `settings`, `skills`, `agents`, `scenes`, `projects`, and `git` workflows.

**Architecture:** Keep `src-tauri/src/core/*` as the only business-logic source of truth, add a small shared `app_runtime` layer for config-dir/repo/output/locking concerns, and add a dedicated CLI binary that parses commands with `clap` derive. Tauri desktop commands remain thin adapters, while the CLI talks to the same runtime/core modules and emits stable JSON by default with optional `--pretty`.

**Tech Stack:** Rust 2021, Cargo features/bin targets, `clap` derive, Tauri 2, serde/serde_json, existing Rust core stores under `src-tauri/src/core`, React/TypeScript docs guards

---

## File Map

- `src-tauri/Cargo.toml` — add explicit CLI binary target, feature split, and CLI-only dependencies.
- `src-tauri/src/lib.rs` — gate desktop-only modules and exports behind the `desktop` feature so the library can build for CLI-only mode.
- `src-tauri/src/main.rs` — keep desktop binary entrypoint and gate it behind the `desktop` feature.
- `src-tauri/src/app_runtime/mod.rs` — shared runtime module boundary.
- `src-tauri/src/app_runtime/context.rs` — config-dir resolution, repo override resolution, sync-mode resolution, path normalization, advisory locking entrypoint.
- `src-tauri/src/app_runtime/output.rs` — unified JSON/pretty response types, warning/error schema, exit-status mapping.
- `src-tauri/src/cli/mod.rs` — CLI module boundary.
- `src-tauri/src/cli/main.rs` — `skills-manager` binary entrypoint.
- `src-tauri/src/cli/args.rs` — `clap` derive structs/enums for root args and subcommands.
- `src-tauri/src/cli/commands/settings.rs` — CLI adapter for settings commands.
- `src-tauri/src/cli/commands/skills.rs` — CLI adapter for skill queries and skill-state mutations.
- `src-tauri/src/cli/commands/agents.rs` — CLI adapter for inventory and sync commands.
- `src-tauri/src/cli/commands/scenes.rs` — CLI adapter for scene config and apply commands.
- `src-tauri/src/cli/commands/projects.rs` — CLI adapter for project config and full apply command.
- `src-tauri/src/cli/commands/git.rs` — CLI adapter for git operations and sync-script command.
- `src-tauri/src/core/git/types.rs` — extend git operation result structure if needed so CLI can surface stderr/details without lossy string parsing.
- `src-tauri/src/core/git/operations.rs` — preserve stdout/stderr/exit-status in operation results and keep stable failure mapping inputs.
- `src-tauri/src/commands/*.rs` — optionally switch repeated config-dir/repo resolution to the new runtime helpers without changing behavior.
- `src-tauri/src/core/settings.rs` — update write path only if required to cooperate with advisory locking or normalized path output.
- `src-tauri/src/core/skills/state.rs` — update write path only if required to cooperate with advisory locking or normalized path output.
- `src-tauri/src/core/agents/config.rs` — update write path only if required to cooperate with advisory locking or normalized path output.
- `src-tauri/src/core/scenes/config.rs` — update write path only if required to cooperate with advisory locking or normalized path output.
- `src-tauri/src/core/projects/store.rs` — update write path only if required to cooperate with advisory locking or normalized path output.
- `docs/modules/tauri/**` — module docs for every new or modified Rust source file.
- `docs/README.md` or `AGENTS.md` — contributor note for CLI build/install commands if needed.

## Task 1: Add Cargo Feature Split And A Real CLI Binary Target

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/main.rs`
- Test: `src-tauri/src/lib.rs`

- [ ] **Step 1: Write the failing compile expectation for CLI-only mode**

The current package has only the desktop binary shape and unconditional `tauri` usage in `src-tauri/src/lib.rs`, so a CLI-only build should fail before the feature split is added.

Run: `cargo check --manifest-path src-tauri/Cargo.toml --no-default-features --features cli`

Expected: FAIL with missing `cli` feature definitions, missing binary target, or desktop-only `tauri` coupling still being compiled.

- [ ] **Step 2: Add explicit features and CLI dependency wiring in `src-tauri/Cargo.toml`**

Add a feature layout like:

```toml
[features]
default = ["desktop"]
desktop = ["dep:tauri", "dep:tauri-plugin-dialog"]
cli = ["dep:clap"]

[[bin]]
name = "skills-manager"
path = "src/cli/main.rs"
required-features = ["cli"]
```

And move desktop-only crates to optional dependencies while adding CLI parsing:

```toml
[dependencies]
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
walkdir = "2.5"
dirs = "6.0"
clap = { version = "4.5", features = ["derive"], optional = true }
tauri = { version = "2.10.0", features = [], optional = true }
tauri-plugin-dialog = { version = "2", optional = true }
```

- [ ] **Step 3: Gate desktop-only library code**

Update `src-tauri/src/lib.rs` so desktop-specific modules and `run()` only compile with the `desktop` feature:

```rust
#[cfg(feature = "desktop")]
mod commands;
mod core;

#[cfg(feature = "desktop")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![/* existing commands */])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

And gate `src-tauri/src/main.rs`:

```rust
#![cfg_attr(all(feature = "desktop", not(debug_assertions)), windows_subsystem = "windows")]

#[cfg(feature = "desktop")]
fn main() {
    app_lib::run();
}
```

- [ ] **Step 4: Run the CLI-only compile check again**

Run: `cargo check --manifest-path src-tauri/Cargo.toml --no-default-features --features cli`

Expected: FAIL now only because `src/cli/main.rs` and CLI modules do not exist yet, not because desktop/Tauri code is still hard-coupled.

- [ ] **Step 5: Commit**

Run:

```bash
git add src-tauri/Cargo.toml src-tauri/src/lib.rs src-tauri/src/main.rs
git commit -m "feat: add cargo feature split for desktop and cli builds"
```

## Task 2: Add Shared Runtime Context, Output Schema, And Advisory Locking

**Files:**
- Create: `src-tauri/src/app_runtime/mod.rs`
- Create: `src-tauri/src/app_runtime/context.rs`
- Create: `src-tauri/src/app_runtime/output.rs`
- Modify: `src-tauri/src/core/settings.rs`
- Modify: `src-tauri/src/core/skills/state.rs`
- Modify: `src-tauri/src/core/agents/config.rs`
- Modify: `src-tauri/src/core/scenes/config.rs`
- Modify: `src-tauri/src/core/projects/store.rs`
- Test: `src-tauri/src/app_runtime/context.rs`
- Test: `src-tauri/src/app_runtime/output.rs`

- [ ] **Step 1: Write failing runtime tests first**

Add focused Rust tests covering:

- config-dir resolution from app identifier
- `--config-dir` override
- `--repo` override precedence over persisted settings
- JSON path normalization to forward slashes
- warning object serialization
- partial status -> exit code `8`
- advisory lock acquisition around a temp config dir

Suggested test names:

```rust
#[test]
fn resolves_production_identifier_config_dir() {}

#[test]
fn repo_override_wins_over_saved_setting() {}

#[test]
fn windows_paths_normalize_to_forward_slashes() {}

#[test]
fn partial_status_maps_to_exit_code_8() {}

#[test]
fn advisory_lock_blocks_second_mutation_writer() {}
```

- [ ] **Step 2: Run the focused runtime tests to verify RED**

Run: `cargo test --manifest-path src-tauri/Cargo.toml app_runtime --no-default-features --features cli -- --nocapture`

Expected: FAIL because `app_runtime` modules and test targets do not exist yet.

- [ ] **Step 3: Implement `AppRuntimeContext` and lock wrapper**

In `src-tauri/src/app_runtime/context.rs`, add a focused context like:

```rust
pub struct AppRuntimeContext {
    pub config_dir: PathBuf,
    pub repo_override: Option<PathBuf>,
    pub pretty: bool,
    pub quiet: bool,
}

impl AppRuntimeContext {
    pub fn resolve_repo_path(&self) -> Result<PathBuf> { /* settings fallback */ }
    pub fn resolve_sync_mode(&self, raw: Option<&str>) -> Result<AgentSyncMode> { /* copy/symlink */ }
    pub fn with_config_lock<T>(&self, f: impl FnOnce() -> Result<T>) -> Result<T> { /* advisory lock */ }
}
```

Use a config-root lock file such as `.skills-manager-system.lock`. Mutation commands must execute store/core writes inside `with_config_lock`.

- [ ] **Step 4: Implement unified output types**

In `src-tauri/src/app_runtime/output.rs`, add stable schema types:

```rust
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CliMeta {
    pub config_dir: String,
    pub repo_path: Option<String>,
    pub timestamp: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct CliWarning {
    pub code: String,
    pub message: String,
    pub target: Option<String>,
    pub details: Option<serde_json::Value>,
}
```

And response builders for `success`, `partial`, and `error`, plus an exit-code helper.

- [ ] **Step 5: Keep stores compatible with runtime locking**

Do not move business logic into the runtime layer. Only adjust store write helpers if needed so locked mutation commands still serialize the exact same files. Keep file formats unchanged.

- [ ] **Step 6: Re-run the runtime tests and verify GREEN**

Run: `cargo test --manifest-path src-tauri/Cargo.toml app_runtime --no-default-features --features cli -- --nocapture`

Expected: PASS for runtime config-dir, path normalization, warning schema, exit-code mapping, and lock behavior.

- [ ] **Step 7: Commit**

Run:

```bash
git add src-tauri/src/app_runtime src-tauri/src/core/settings.rs src-tauri/src/core/skills/state.rs src-tauri/src/core/agents/config.rs src-tauri/src/core/scenes/config.rs src-tauri/src/core/projects/store.rs
git commit -m "feat: add shared cli runtime context and output schema"
```

## Task 3: Build The CLI Skeleton With `clap` Derive And Read-Only Commands

**Files:**
- Create: `src-tauri/src/cli/mod.rs`
- Create: `src-tauri/src/cli/main.rs`
- Create: `src-tauri/src/cli/args.rs`
- Create: `src-tauri/src/cli/commands/settings.rs`
- Create: `src-tauri/src/cli/commands/skills.rs`
- Create: `src-tauri/src/cli/commands/agents.rs`
- Modify: `src-tauri/src/core/skills/scan.rs`
- Modify: `src-tauri/src/core/skills/state.rs`
- Modify: `src-tauri/src/core/agents/discovery.rs`
- Test: `src-tauri/src/cli/args.rs`

- [ ] **Step 1: Write failing CLI parse tests**

Add parse tests like:

```rust
#[test]
fn parses_global_pretty_and_settings_subcommand() {}

#[test]
fn parses_skills_list_with_repo_override() {}

#[test]
fn parses_agents_list_without_mutation_flags() {}
```

Also add a smoke command test expectation:

Run: `cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager -- settings get-repo-path`

Expected: FAIL before handlers exist.

- [ ] **Step 2: Implement root CLI arg tree**

In `src-tauri/src/cli/args.rs`, define a `clap` derive tree:

```rust
#[derive(Parser)]
pub struct CliArgs {
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub pretty: bool,
    #[arg(long)]
    pub quiet: bool,
    #[arg(long)]
    pub no_color: bool,
    #[arg(long)]
    pub config_dir: Option<PathBuf>,
    #[arg(long)]
    pub repo: Option<PathBuf>,
    #[command(subcommand)]
    pub command: RootCommand,
}
```

Add subcommand enums for `settings`, `skills`, and `agents` read-only cases first.

- [ ] **Step 3: Implement `settings get-repo-path`, `settings get-sync-mode`, `skills scan`, `skills state`, `skills list`, `skills doc`, and `agents list`**

Wire each command through `AppRuntimeContext` plus existing core modules:

- `settings` -> `SettingsStore`
- `skills scan` -> `scan_repo_skills`
- `skills state` -> `SkillStateStore::load_for_repo`
- `skills list` -> join scan + disabled IDs into one list with `enabled` booleans
- `skills doc` -> existing document loader
- `agents list` -> existing agent discovery

`skills list` should emit joined view-model entries like:

```json
{
  "id": "custom:searxng",
  "name": "SearXNG",
  "sourceType": "custom",
  "enabled": true
}
```

- [ ] **Step 4: Add the CLI entrypoint**

In `src-tauri/src/cli/main.rs`, parse args, build `AppRuntimeContext`, dispatch commands, print JSON by default, and return proper exit codes:

```rust
fn main() {
    let args = CliArgs::parse();
    let exit = run_cli(args);
    std::process::exit(exit.code());
}
```

- [ ] **Step 5: Run parse tests and read-only smoke commands**

Run:

```bash
cargo test --manifest-path src-tauri/Cargo.toml cli --no-default-features --features cli -- --nocapture
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager -- --help
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager -- settings get-repo-path
```

Expected:

- parse tests PASS
- `--help` prints the nested command tree
- `settings get-repo-path` returns valid JSON even when unset

- [ ] **Step 6: Commit**

Run:

```bash
git add src-tauri/src/cli src-tauri/src/core/skills/scan.rs src-tauri/src/core/skills/state.rs src-tauri/src/core/agents/discovery.rs
git commit -m "feat: add cli skeleton and read-only commands"
```

## Task 4: Add Mutation Commands For Settings, Skills, And Agents

**Files:**
- Modify: `src-tauri/src/cli/args.rs`
- Modify: `src-tauri/src/cli/commands/settings.rs`
- Modify: `src-tauri/src/cli/commands/skills.rs`
- Modify: `src-tauri/src/cli/commands/agents.rs`
- Modify: `src-tauri/src/core/agents/sync.rs`
- Modify: `src-tauri/src/core/agents/target_sync.rs`
- Test: `src-tauri/src/cli/commands/settings.rs`
- Test: `src-tauri/src/cli/commands/skills.rs`
- Test: `src-tauri/src/cli/commands/agents.rs`

- [ ] **Step 1: Write failing mutation tests first**

Add command tests for:

- `settings set-repo-path`
- `settings set-sync-mode`
- `skills enable`
- `skills disable`
- `agents enable`
- `agents disable`
- `agents set-path`
- `agents clear-path`
- `agents sync`

Include one Windows symlink failure mapping test that expects `windows_symlink_privilege_required` when the underlying error mentions Developer Mode.

- [ ] **Step 2: Run the mutation test slice to verify RED**

Run: `cargo test --manifest-path src-tauri/Cargo.toml cli::commands --no-default-features --features cli -- --nocapture`

Expected: FAIL because mutation handlers and error mapping are still missing.

- [ ] **Step 3: Implement locked mutation handlers**

Wrap every mutation command in `context.with_config_lock(...)`.

Examples:

```rust
pub fn set_repo_path(ctx: &AppRuntimeContext, path: &Path) -> Result<CliResponse> {
    ctx.with_config_lock(|| {
        let settings = SettingsStore::new(ctx.config_dir.clone()).save_repo_path(Some(path))?;
        Ok(success("settings set-repo-path", json!({ "settings": settings }), ctx))
    })
}
```

And:

```rust
pub fn set_skill_enabled(ctx: &AppRuntimeContext, skill_id: &str, enabled: bool) -> Result<CliResponse> {
    ctx.with_config_lock(|| {
        let repo = ctx.resolve_repo_path()?;
        let snapshot = SkillStateStore::new(ctx.config_dir.clone()).set_skill_enabled(&repo, skill_id, enabled)?;
        Ok(success("skills enable", json!({ "state": snapshot }), ctx))
    })
}
```

- [ ] **Step 4: Preserve stable error mapping**

Map known failures into the approved codes:

- `repo_path_not_configured`
- `invalid_sync_mode`
- `agent_not_found`
- `skill_not_found`
- `windows_symlink_privilege_required`
- `config_write_failed`

For `agents sync`, classify conflict-bearing results as `partial` and exit code `8`.

- [ ] **Step 5: Run mutation smoke checks**

Run:

```bash
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager -- settings set-sync-mode copy
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager -- skills state
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager -- agents list
```

Expected:

- mutation commands return updated snapshots in JSON
- read-after-write reflects the new state
- no desktop runtime is required

- [ ] **Step 6: Commit**

Run:

```bash
git add src-tauri/src/cli/args.rs src-tauri/src/cli/commands/settings.rs src-tauri/src/cli/commands/skills.rs src-tauri/src/cli/commands/agents.rs src-tauri/src/core/agents/sync.rs src-tauri/src/core/agents/target_sync.rs
git commit -m "feat: add cli mutation commands for settings skills and agents"
```

## Task 5: Wire Scene, Project, And Git Commands To The Existing Core

**Files:**
- Create: `src-tauri/src/cli/commands/scenes.rs`
- Create: `src-tauri/src/cli/commands/projects.rs`
- Create: `src-tauri/src/cli/commands/git.rs`
- Modify: `src-tauri/src/cli/args.rs`
- Modify: `src-tauri/src/core/git/types.rs`
- Modify: `src-tauri/src/core/git/operations.rs`
- Modify: `src-tauri/src/cli/main.rs`
- Test: `src-tauri/src/cli/commands/scenes.rs`
- Test: `src-tauri/src/cli/commands/projects.rs`
- Test: `src-tauri/src/cli/commands/git.rs`

- [ ] **Step 1: Write failing adapter tests**

Add tests for:

- `scenes list/create/update/delete/set-active/clear-active/set-skills/set-agents/set-skill-order/apply`
- `projects list/add/update/remove/apply`
- `git status/diff/log/fetch/pull/push/commit/sync-external`

Include one explicit regression test asserting that `projects apply` has no path parameter and uses the full stored snapshot.

- [ ] **Step 2: Extend git operation result detail if needed**

If the current `GitOperationResult` lacks enough structure for CLI error details, change it to preserve stdout/stderr/status without breaking existing desktop usage. For example:

```rust
pub struct GitOperationResult {
    pub success: bool,
    pub message: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub exit_code: Option<i32>,
}
```

Then keep desktop adapters backward-compatible by continuing to display `message`.

- [ ] **Step 3: Implement scene and project adapters**

Reuse existing stores/managers:

- `SceneConfigStore`
- `apply_scene`
- `ProjectConfigStore`
- `apply_project_assignments`

Mutation commands must hold the config lock. `projects apply` must not accept a path argument in Phase 1.

- [ ] **Step 4: Implement git adapters with structured failures**

Use existing git core functions. On failures, include stderr and operation name in `error.details`, for example:

```json
{
  "code": "git_command_failed",
  "message": "git push failed",
  "details": {
    "operation": "push",
    "stderr": "fatal: could not read Username",
    "exitCode": 128
  }
}
```

- [ ] **Step 5: Run focused CLI command tests**

Run:

```bash
cargo test --manifest-path src-tauri/Cargo.toml cli::commands::scenes --no-default-features --features cli -- --nocapture
cargo test --manifest-path src-tauri/Cargo.toml cli::commands::projects --no-default-features --features cli -- --nocapture
cargo test --manifest-path src-tauri/Cargo.toml cli::commands::git --no-default-features --features cli -- --nocapture
```

Expected: PASS for scene/project/git adapter tests.

- [ ] **Step 6: Commit**

Run:

```bash
git add src-tauri/src/cli/commands/scenes.rs src-tauri/src/cli/commands/projects.rs src-tauri/src/cli/commands/git.rs src-tauri/src/cli/args.rs src-tauri/src/core/git/types.rs src-tauri/src/core/git/operations.rs src-tauri/src/cli/main.rs
git commit -m "feat: add cli scene project and git command groups"
```

## Task 6: Update Docs, Contributor Guidance, And Final Verification

**Files:**
- Create: `docs/modules/tauri/app_runtime/mod.md`
- Create: `docs/modules/tauri/app_runtime/context.md`
- Create: `docs/modules/tauri/app_runtime/output.md`
- Create: `docs/modules/tauri/cli/mod.md`
- Create: `docs/modules/tauri/cli/main.md`
- Create: `docs/modules/tauri/cli/args.md`
- Create: `docs/modules/tauri/cli/commands/settings.md`
- Create: `docs/modules/tauri/cli/commands/skills.md`
- Create: `docs/modules/tauri/cli/commands/agents.md`
- Create: `docs/modules/tauri/cli/commands/scenes.md`
- Create: `docs/modules/tauri/cli/commands/projects.md`
- Create: `docs/modules/tauri/cli/commands/git.md`
- Modify: `docs/modules/README.md`
- Modify: `docs/modules/tauri/README.md`
- Modify: `docs/modules/tauri/lib.md`
- Modify: `AGENTS.md`
- Modify: `docs/README.md`

- [ ] **Step 1: Add module docs for every new Rust file**

Create one-to-one docs for each new `app_runtime` and `cli` source file. Each doc must explain:

- responsibility
- upstream/downstream imports
- public surface
- key invariants
- interactions with the approved CLI JSON schema

- [ ] **Step 2: Add contributor build/install notes**

Update contributor-facing docs so they explicitly mention:

- default desktop build behavior
- CLI-only build command:

```bash
cargo build --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager
```

- how to verify the binary:

```bash
./src-tauri/target/debug/skills-manager --help
```

- where PATH registration or wrapper installation would fit later

- [ ] **Step 3: Run the required gates**

Run:

```bash
cargo check --manifest-path src-tauri/Cargo.toml --no-default-features --features cli
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --features cli
cargo check --manifest-path src-tauri/Cargo.toml
npm run verify
```

Expected:

- CLI-only build passes
- CLI-only test suite passes
- desktop/default build still passes
- module-doc checks pass
- repo verify gate passes

- [ ] **Step 4: Final manual smoke**

Run:

```bash
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager -- settings get-repo-path
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager -- skills list
cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager -- agents list
```

Expected:

- valid JSON on stdout
- forward-slash paths in JSON
- no Tauri runtime dependency

- [ ] **Step 5: Commit**

Run:

```bash
git add docs/modules docs/README.md AGENTS.md
git commit -m "docs: document cli runtime build and verification workflow"
```

## Self-Review Checklist

- Spec coverage: the plan covers Cargo/bin strategy, Tauri-compatible config-dir resolution, `clap` derive parsing, shared runtime/output schema, advisory locking, read-only and mutation CLI commands, `skills list`, no-arg `projects apply`, structured git failures, docs, and verification.
- Placeholder scan: no `TODO`, `TBD`, or “implement later” markers remain.
- Type consistency: response payloads stay camelCase, warning/error codes stay snake_case, and `projects apply` consistently refers to the full stored snapshot rather than a single-project path.
