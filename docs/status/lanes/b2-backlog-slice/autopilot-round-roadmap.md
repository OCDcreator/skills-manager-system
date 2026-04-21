# Autopilot Round Roadmap — `b2-backlog-slice`

## Queue

### [DONE] B2 - Next queued bug or backlog slice

- **Lane**: Bugfix / backlog
- **Goal**: Land the CLI Foundation Phase 1b/Phase 2 slice: mutation commands, scene/project/git command groups, docs coverage, and full verification.
- **Priority entrypoints**:
- `docs/superpowers/plans/2026-04-21-cli-foundation.md`
- `src-tauri/src/cli/commands/`
- `src-tauri/src/core/git/`
- `src-tauri/src/core/scenes/`
- `src-tauri/src/core/projects/`
- `docs/modules/tauri/`
- **Constraints**:
  - Reuse the Phase 1a runtime and arg tree; do not redesign the command surface
  - `projects apply` must stay full-snapshot with no path argument
  - Mutation commands must use the advisory config lock
- **Acceptance**:
  - Settings/skills/agents mutation commands work headlessly with stable JSON and exit codes
  - Scene, project, and git command groups are implemented and tested
  - Module docs and contributor build notes are updated
  - `npm run verify` and full Rust tests pass

## Lane state

- This roadmap is lane-local.
- When it has no remaining `[NEXT]` or `[QUEUED]` items, the controller switches to `b3-checkpoint`.
