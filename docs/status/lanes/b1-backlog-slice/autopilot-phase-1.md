# Autopilot Phase 1

> **Status**: [DONE]
> **Lane**: `b1-backlog-slice`
> **Slice**: `B1 - CLI Foundation Phase 1a`
> **Date**: `2026-04-21`

## Scope

- Landed the Phase 1a CLI foundation slice: Cargo desktop/CLI feature split, explicit `skills-manager` binary, shared runtime context/output layer, and read-only `settings`, `skills`, and `agents` command groups.
- Added focused Rust tests for runtime resolution, CLI parsing, and read-only skills/agents behavior.
- Cleared the existing frontend lint blockers that previously prevented validation, and backfilled the missing module-doc updates already required by the branch diff.

## Changed Files

- Rust runtime and CLI: `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `src-tauri/build.rs`, `src-tauri/src/lib.rs`, `src-tauri/src/main.rs`, `src-tauri/src/app_runtime/mod.rs`, `src-tauri/src/app_runtime/context.rs`, `src-tauri/src/app_runtime/output.rs`, `src-tauri/src/app_runtime/output_errors.rs`, `src-tauri/src/cli/mod.rs`, `src-tauri/src/cli/main.rs`, `src-tauri/src/cli/args.rs`, `src-tauri/src/cli/commands/settings.rs`, `src-tauri/src/cli/commands/skills.rs`, `src-tauri/src/cli/commands/skills_tests.rs`, `src-tauri/src/cli/commands/agents.rs`, `src-tauri/src/core/skills/scan.rs`, `src-tauri/src/core/agents/discovery.rs`, `src-tauri/src/core/agents/target_sync.rs`
- Frontend validation fixes: `src/components/RepoPathForm.tsx`, `src/components/agents/AgentTargetCard.tsx`, `src/context/AppContext.tsx`, `src/views/GitView.tsx`, `src/views/ProjectsView.tsx`, `src/views/ScenesView.tsx`
- Module docs: `docs/modules/tauri/README.md`, `docs/modules/tauri/lib.md`, `docs/modules/tauri/main.md`, `docs/modules/tauri/app_runtime/mod.md`, `docs/modules/tauri/app_runtime/context.md`, `docs/modules/tauri/app_runtime/output.md`, `docs/modules/tauri/app_runtime/output_errors.md`, `docs/modules/tauri/cli/mod.md`, `docs/modules/tauri/cli/main.md`, `docs/modules/tauri/cli/args.md`, `docs/modules/tauri/cli/commands/settings.md`, `docs/modules/tauri/cli/commands/skills.md`, `docs/modules/tauri/cli/commands/skills_tests.md`, `docs/modules/tauri/cli/commands/agents.md`, `docs/modules/tauri/core/skills/scan.md`, `docs/modules/tauri/core/agents/discovery.md`, `docs/modules/tauri/core/agents/target_sync.md`, `docs/modules/tauri/core/agents/sync_tests.md`, `docs/modules/tauri/core/scenes/manager.md`, `docs/modules/frontend/components/RepoPathForm.md`, `docs/modules/frontend/components/agents/AgentTargetCard.md`, `docs/modules/frontend/context/AppContext.md`, `docs/modules/frontend/views/GitView.md`, `docs/modules/frontend/views/ProjectsView.md`, `docs/modules/frontend/views/ScenesView.md`
- Lane tracking: `docs/status/lanes/b1-backlog-slice/autopilot-round-roadmap.md`, `docs/status/lanes/b1-backlog-slice/autopilot-phase-1.md`

## Validation

### Targeted

- `cargo check --manifest-path src-tauri/Cargo.toml --no-default-features --features cli` ✅
- `cargo test --manifest-path src-tauri/Cargo.toml app_runtime --no-default-features --features cli -- --nocapture` ✅
- `cargo test --manifest-path src-tauri/Cargo.toml cli --no-default-features --features cli -- --nocapture` ✅
- `cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager -- --config-dir <tmp> settings get-repo-path` ✅
- `cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager -- --config-dir <tmp> --repo <tmp-repo> skills list` ✅
- `cargo run --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager -- --config-dir <tmp> agents list` ✅
- `node scripts/check-module-doc-coverage.mjs` ✅
- `node scripts/check-module-doc-diff.mjs --range origin/main...HEAD` ✅

### Configured

- `npm run lint` ✅ with one non-blocking warning from `react-refresh/only-export-components` in `src/context/AppContext.tsx`
- `cargo check --manifest-path src-tauri/Cargo.toml` ✅
- `npm run verify` ✅
- `npm run build` ✅ (`dist/assets/index-CyY97gYi.js`)

### Notes

- `npm run verify` still reports the pre-existing architecture warning for `src/components/scenes/SceneCard.tsx` at 324 lines, but no blocking architecture issues remain.
- Frontend production builds still emit the existing Vite chunk-size warning for `dist/assets/index-CyY97gYi.js`; the build itself succeeds.

## Vulture

- Not configured for this lane; no dead-code observability command was available to run.

## Next Recommended Slice

- Execute `B1.2 - Prepare mutation-ready runtime contracts`: harden CLI error mapping and config-lock behavior for upcoming mutation command groups without expanding into scene/project/git mutations yet.
