# Autopilot Round Roadmap — `b1-backlog-slice`

## Queue

### [DONE] B1 - Highest-priority queued bug or backlog slice

- **Lane**: Bugfix / backlog
- **Goal**: Land the CLI Foundation Phase 1a slice: Cargo feature split, shared runtime/output layer, `clap` CLI skeleton, and read-only commands with validated JSON output.
- **Priority entrypoints**:
- `AGENTS.md`
- `docs/superpowers/specs/2026-04-21-cli-foundation-design.md`
- `docs/superpowers/plans/2026-04-21-cli-foundation.md`
- `src-tauri/Cargo.toml`
- `src-tauri/src/app_runtime/`
- `src-tauri/src/cli/`
- `src-tauri/src/core/`
- **Constraints**:
  - Stay inside plan Tasks 1-3 before moving on
  - Keep desktop/default build healthy while enabling CLI-only builds
  - Use TDD for new runtime/CLI tests and keep command files thin
- **Acceptance**:
  - Cargo feature split and explicit CLI binary compile cleanly
  - Shared runtime/output modules exist and pass focused tests
  - `settings get-repo-path`, `skills list`, and `agents list` work headlessly with JSON output
  - Focused CLI/runtime tests pass, plus `cargo check --manifest-path src-tauri/Cargo.toml --no-default-features --features cli`

### [DONE] B1.2 - Prepare mutation-ready runtime contracts

- **Goal**: If Phase 1a finishes early within the lane budget, harden error mapping and lock behavior without yet expanding to all mutation command groups.
- **Acceptance**:
  - Warning/error schema stays stable
  - Advisory lock behavior is covered by focused tests
  - No scene/project/git mutation surface ships in this lane unless Phase 1a is already green

## Lane state

- This roadmap is lane-local.
- When it has no remaining `[NEXT]` or `[QUEUED]` items, the controller switches to `b2-backlog-slice`.
