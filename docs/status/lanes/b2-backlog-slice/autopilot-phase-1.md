# Autopilot Phase 1 — CLI Foundation Phase 1b/2

> **Status**: [DONE]
> **Lane**: `b2-backlog-slice`
> **Attempt**: 4
> **Commit Subject**: `autopilot: round 4 - land cli mutation and workflow commands`

## Scope

- Executed `[NEXT] B2 - Next queued bug or backlog slice`.
- Landed settings, skills, and agents mutation commands with advisory config locking and stable JSON/error codes.
- Added scene, project, and git command groups without redesigning the Phase 1a runtime or arg tree.
- Kept `projects apply` as a no-argument full-snapshot apply command.
- Updated module docs and contributor CLI build notes.

## Changed Files

- `AGENTS.md`
- `docs/README.md`
- `docs/modules/tauri/app_runtime/output.md`
- `docs/modules/tauri/cli/args.md`
- `docs/modules/tauri/cli/command_groups.md`
- `docs/modules/tauri/cli/commands/agent_sync.md`
- `docs/modules/tauri/cli/commands/agents.md`
- `docs/modules/tauri/cli/commands/agents_tests.md`
- `docs/modules/tauri/cli/commands/git.md`
- `docs/modules/tauri/cli/commands/git_tests.md`
- `docs/modules/tauri/cli/commands/projects.md`
- `docs/modules/tauri/cli/commands/projects_tests.md`
- `docs/modules/tauri/cli/commands/scenes.md`
- `docs/modules/tauri/cli/commands/scenes_tests.md`
- `docs/modules/tauri/cli/commands/settings.md`
- `docs/modules/tauri/cli/commands/skill_mutations.md`
- `docs/modules/tauri/cli/commands/skills.md`
- `docs/modules/tauri/cli/commands/skills_tests.md`
- `docs/modules/tauri/cli/mod.md`
- `docs/modules/tauri/core/git/operations.md`
- `docs/modules/tauri/core/git/types.md`
- `docs/status/lanes/b2-backlog-slice/autopilot-round-roadmap.md`
- `src-tauri/src/app_runtime/output.rs`
- `src-tauri/src/cli/args.rs`
- `src-tauri/src/cli/command_groups.rs`
- `src-tauri/src/cli/commands/agent_sync.rs`
- `src-tauri/src/cli/commands/agents.rs`
- `src-tauri/src/cli/commands/agents_tests.rs`
- `src-tauri/src/cli/commands/git.rs`
- `src-tauri/src/cli/commands/git_tests.rs`
- `src-tauri/src/cli/commands/projects.rs`
- `src-tauri/src/cli/commands/projects_tests.rs`
- `src-tauri/src/cli/commands/scenes.rs`
- `src-tauri/src/cli/commands/scenes_tests.rs`
- `src-tauri/src/cli/commands/settings.rs`
- `src-tauri/src/cli/commands/skill_mutations.rs`
- `src-tauri/src/cli/commands/skills.rs`
- `src-tauri/src/cli/commands/skills_tests.rs`
- `src-tauri/src/cli/mod.rs`
- `src-tauri/src/core/git/operations.rs`
- `src-tauri/src/core/git/types.rs`

## Validation

- `cargo test --manifest-path src-tauri/Cargo.toml cli::commands --no-default-features --features cli -- --nocapture` — passed.
- `cargo test --manifest-path src-tauri/Cargo.toml cli --no-default-features --features cli -- --nocapture` — passed.
- `cargo test --manifest-path src-tauri/Cargo.toml app_runtime --no-default-features --features cli -- --nocapture` — passed.
- CLI smoke command batch for settings mutation, skills list, agents list, scenes list, no-arg projects apply, git status, and rejected `projects apply <path>` — passed.
- `node scripts/check-module-doc-coverage.mjs` — passed.
- `node scripts/check-module-doc-diff.mjs --range 0da02431a9bf181efb556d95f2ce1eb0e3c2bef6...HEAD` — passed.
- `cargo check --manifest-path src-tauri/Cargo.toml --no-default-features --features cli` — passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --features cli` — passed.
- `npm run lint` — passed with one existing `react-refresh/only-export-components` warning in `src/context/AppContext.tsx`.
- `cargo check --manifest-path src-tauri/Cargo.toml` — passed.
- `npm run verify` — passed; reported existing architecture warning for `src/components/scenes/SceneCard.tsx` near line limit and Vite chunk-size warning.
- `npm run build` — passed; build marker `sha256:04dbc91660c432a76876588ebfec83e1d0c9fafcab272bf3841aef5d85696a93`.

## Vulture

- Vulture command was not configured for this lane; no dead-code observability command was run.

## Next Recommended Slice

- No `[QUEUED]` items remain in `b2-backlog-slice`; continue to lane `b3-checkpoint`.
