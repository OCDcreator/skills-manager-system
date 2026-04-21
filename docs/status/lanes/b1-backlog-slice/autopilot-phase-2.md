# Autopilot Phase 2

> **Status**: [DONE]
> **Lane**: `b1-backlog-slice`
> **Slice**: `B1.2 - Prepare mutation-ready runtime contracts`
> **Date**: `2026-04-21`

## Scope

- Hardened the CLI runtime contract for upcoming mutation commands without adding scene, project, git, settings mutation, skill mutation, or agent mutation surfaces.
- Split config locking into a focused `app_runtime::config_lock` module backed by a cross-platform advisory file lock instead of lock-file creation as the lock itself.
- Added typed config-lock errors and stable CLI mappings for lock conflicts, config-write failures, and invalid sync-mode inputs.
- Locked warning/error schema behavior with focused serialization and mapping tests.

## Changed Files

- Runtime contracts: `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `src-tauri/src/app_runtime/mod.rs`, `src-tauri/src/app_runtime/context.rs`, `src-tauri/src/app_runtime/config_lock.rs`, `src-tauri/src/app_runtime/output_errors.rs`
- Module docs: `docs/modules/tauri/app_runtime/mod.md`, `docs/modules/tauri/app_runtime/context.md`, `docs/modules/tauri/app_runtime/config_lock.md`, `docs/modules/tauri/app_runtime/output_errors.md`
- Lane tracking: `docs/status/lanes/b1-backlog-slice/autopilot-round-roadmap.md`, `docs/status/lanes/b1-backlog-slice/autopilot-phase-2.md`

## Validation

### Targeted

- `cargo test --manifest-path src-tauri/Cargo.toml app_runtime --no-default-features --features cli -- --nocapture` ✅ baseline before the slice, 7 tests passed.
- `cargo test --manifest-path src-tauri/Cargo.toml app_runtime --no-default-features --features cli -- --nocapture` ✅ after the slice, 14 tests passed.
- `cargo check --manifest-path src-tauri/Cargo.toml --no-default-features --features cli` ✅
- `node scripts/check-module-doc-coverage.mjs && node scripts/check-module-doc-diff.mjs --range origin/main...HEAD` ✅

### Configured

- `npm run lint` ✅ with the pre-existing `react-refresh/only-export-components` warning in `src/context/AppContext.tsx`.
- `cargo check --manifest-path src-tauri/Cargo.toml` ✅
- `npm run verify` ✅
- `npm run build` ✅ (`dist/assets/index-CyY97gYi.js`, sha256 `09b7fccb98c55172beb9d94db5deb0e301e9f7dacf55c03cfd4debb370bd8dae`)

### Notes

- `npm run verify` still reports the pre-existing architecture warning for `src/components/scenes/SceneCard.tsx` at 324 lines, but no blocking architecture issues remain.
- Frontend production builds still emit the existing Vite chunk-size warning for `dist/assets/index-CyY97gYi.js`; the build succeeds.
- No mutation command groups were added in this lane.

## Vulture

- Not configured for this lane; no dead-code observability command was available to run.

## Next Recommended Slice

- No remaining queued item exists in `b1-backlog-slice`; the controller can advance to `b2-backlog-slice`.
