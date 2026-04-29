# Project Assistant Terminal Launcher Design

## Status

Approved for design documentation on 2026-04-29.

This document defines the first terminal-launcher phase for the floating
Project Assistant in Skills Manager System. The goal is to evolve the existing
bottom-right assistant entry point from a document-retrieval prototype into a
practical local CLI launcher without overloading the first version with prompt
injection, multi-session management, or agent-specific automation.

## Goals

- Reuse the existing bottom-right Project Assistant launcher instead of adding
  a new page.
- Let the user choose one of four installed CLIs: Codex, OpenCode, Claude
  Code, or Kimi Code.
- Require the user to choose a working directory before launch.
- Expand the same floating assistant panel into a single interactive terminal
  session after launch.
- Run each CLI inside a real PTY-backed terminal session rather than a fake
  log viewer.
- Preserve clear boundaries so later phases can add prompt injection,
  project-context helpers, or multi-session support without rewriting the
  terminal core.

## Non-Goals

- Do not support multiple terminal tabs or concurrent assistant sessions in
  this version.
- Do not inject a startup prompt, skill, system instructions, or project
  context automatically.
- Do not translate current assistant chat messages into terminal input.
- Do not add background detached sessions that survive independently after the
  active panel is closed.
- Do not add terminal history management, saved session transcripts, or
  session restore.
- Do not add a generic agent marketplace or arbitrary custom command entry
  point.

## Approved User Experience

The Project Assistant remains a fixed bottom-right launcher that is available
across all main app views. Clicking the launcher opens a floating assistant
panel, but the first visible state is no longer the document chat prototype by
default. Instead, the panel first acts as a launcher menu for four approved
local CLIs.

The approved first-run interaction is:

1. User clicks the floating robot launcher.
2. A compact floating panel opens with four CLI choices:
   - Codex
   - OpenCode
   - Claude Code
   - Kimi Code
3. The launcher requires the user to choose a working directory before launch.
4. After the user selects one CLI and a directory, the same floating panel
   expands into a larger terminal surface.
5. The expanded panel hosts one live PTY-backed terminal session for the chosen
   CLI.

The terminal view should clearly show:

- active CLI name
- selected working directory
- terminal output area
- direct interactive input path through the embedded terminal
- stop and restart controls
- explicit startup or runtime errors when launch fails

The panel must feel like a practical local tool surface, not a decorative chat
shell. The user should be able to type directly into the embedded terminal and
continue the CLI session naturally.

## Single-Session Rule

Version one supports exactly one active terminal session at a time.

If no session is active, the floating panel shows the launcher menu. Once a
session starts, the panel shows the terminal workspace for that session. A new
launch request must replace the current session rather than opening a second
tab or a second floating panel.

The implementation should model session state explicitly:

- `idle`: launcher menu visible, no running session
- `starting`: launch requested, waiting for PTY process startup
- `running`: session active and interactive
- `stopped`: process exited normally or was stopped by the user
- `failed`: process failed to start or PTY setup failed

This explicit state model avoids a fragile UI where "open panel" and "running
terminal" are incorrectly treated as the same thing.

## Launch Scope

Version one intentionally limits launch targets to four fixed CLIs:

- `codex`
- `opencode`
- `claude_code`
- `kimi`

The frontend may reuse the repo's existing agent naming and icon system where
useful, but terminal launch support must be treated as a distinct capability.
Being present in the agent catalog does not automatically mean the terminal
launcher can run that tool.

The launch request contract should include:

- `cliKey`
- `workingDirectory`
- `cols`
- `rows`

The backend should reject unknown CLI keys, missing directories, unreadable
directories, and zero-size terminal dimensions with clear user-facing errors.

## Frontend Architecture

The existing floating assistant surface is the correct host, but the current
assistant prototype files should not absorb terminal behavior as one large
mixed-responsibility component.

Planned frontend modules:

- `src/components/assistant/ProjectAssistantLauncher.tsx`
  - retains floating open/close behavior
  - remains responsible only for launcher affordance and lazy panel mounting
- `src/components/assistant/ProjectAssistantPanel.tsx`
  - becomes the high-level mode switch for launcher state versus terminal state
  - must not become a large mixed component containing full terminal behavior
- `src/components/assistant/AssistantLauncherMenu.tsx`
  - renders the four CLI choices
  - renders working-directory selection UI
  - handles launch request submission state
- `src/components/assistant/AssistantTerminalSession.tsx`
  - renders the expanded terminal shell
  - shows active CLI, working directory, status, stop, and restart controls
  - owns terminal container sizing and PTY session UI orchestration
- `src/lib/terminal.ts`
  - typed Tauri API wrapper for terminal commands
  - should stay separate from `src/lib/assistant.ts`

The frontend must not hide terminal concerns inside generic helpers. Terminal
state and launch actions belong to assistant/terminal-specific modules, not to
`utils.ts`, `helpers.ts`, or a bloated `tauri.ts`.

All new UI copy must be added to both:

- `src/i18n/en.json`
- `src/i18n/zh.json`

## Backend Architecture

The backend should introduce a dedicated terminal core domain instead of
packing PTY logic into the existing assistant retrieval modules.

Planned backend modules:

- `src-tauri/src/commands/terminal.rs`
  - thin Tauri command layer
  - validates parameters only at the surface
  - maps core errors into user-facing strings or structured responses
- `src-tauri/src/core/terminal/mod.rs`
  - public terminal-core entry points
- `src-tauri/src/core/terminal/model.rs`
  - request and response structures
- `src-tauri/src/core/terminal/launcher.rs`
  - maps approved CLI keys to launch behavior
  - validates launchable command shape
- `src-tauri/src/core/terminal/session.rs`
  - manages single active session lifecycle and status transitions
- `src-tauri/src/core/terminal/io.rs`
  - owns PTY input, output, and resize bridging

The command layer must remain thin. PTY creation, session lifecycle handling,
launch rules, and process I/O bridging belong in `core/terminal`.

The repo's existing assistant retrieval core:

- `src-tauri/src/core/assistant/context.rs`
- `src-tauri/src/core/assistant/retrieval.rs`
- `src-tauri/src/core/assistant/response.rs`

should remain untouched by version-one terminal runtime logic. The terminal
launcher is a sibling domain, not a subroutine of document retrieval.

## PTY Requirement

Version one must use a real PTY-backed interactive terminal rather than a
simple `stdout`/`stderr` log streamer with line-based `stdin`.

This requirement is essential because the target CLIs are interactive tools.
A fake pipe-based terminal surface is likely to break prompt repainting,
cursor movement, color output, raw keyboard handling, or interactive flows.

The PTY layer must support:

- writing user input to the active session
- forwarding terminal output to the frontend
- resizing the terminal when the floating panel changes size
- detecting process exit

If PTY support cannot be established on a platform, the app should fail
explicitly rather than silently downgrading to a broken pseudo-terminal.

## CLI Launch Contract

Version one launch behavior should be minimal and explicit.

For each supported CLI:

- launch the locally installed executable directly
- set the selected working directory as the process `cwd`
- do not prepend hidden instructions
- do not inject project files
- do not wrap the command in OpenCode-specific or repo-specific automation

Expected launch intent:

- Codex -> launch local `codex`
- OpenCode -> launch local `opencode`
- Claude Code -> launch the real locally validated Claude Code executable name
- Kimi -> launch local Kimi executable name used on this workstation

The Claude Code and Kimi executable names must be validated against the real
local environment during implementation. The design intentionally describes
their launch behavior by product identity rather than assuming a binary name
that might differ from the visible brand.

## Error Handling

Version one should distinguish launch failures clearly rather than collapsing
them into a generic assistant error.

Required error buckets:

- CLI executable not found
- selected working directory missing
- selected working directory unreadable or not a directory
- PTY creation failed
- process failed during launch
- process exited after launch
- user terminated the session

UI behavior requirements:

- keep the floating panel open when errors occur
- show the current session state clearly
- preserve the chosen CLI and directory when retry is reasonable
- expose a retry path after startup failure
- expose stop and restart actions when a session had launched successfully

The app should not pretend a session is interactive if the launch process has
already exited.

## Platform Boundaries

This repo targets Windows and macOS, but version-one implementation and manual
verification should focus first on the user's primary Windows environment.

The design must still preserve cross-platform structure:

- do not hardcode Windows-only path separators in the UI contract
- do not assume one shell binary for all CLIs
- keep PTY runtime and path handling compatible with a later macOS pass

The first explicit promise is:

- Windows: required for real implementation validation
- macOS: code structure should remain compatible, but v1 acceptance is centered
  on Windows verification unless the implementation round explicitly expands
  the test matrix

## Relationship To Existing Agent Catalog

The repo already tracks multiple agents through the agent catalog and discovery
system. That data can inform branding and ordering, but the terminal launcher
must not inherit agent-management assumptions blindly.

Examples:

- A tool may have a managed skills directory but still need independent CLI
  launch validation.
- A tool's display name and its executable name may differ.
- Terminal launch capability should be modeled separately from skill-sync
  capability.

This distinction keeps the terminal launcher from becoming coupled to agent-sync
internals that solve a different problem.

## Testing And Verification

Version-one implementation should include both focused automated checks and
real local smoke validation.

Required validation areas:

- frontend component and state behavior through the repo's normal build/test
  gates
- Rust tests for launch-contract validation and session-state transitions
- manual or scripted smoke validation that each supported installed CLI can be
  launched from the floating panel on Windows when available
- resize and stop/restart behavior validation for the active session
- module documentation coverage for all new source files

Minimum repo verification gate:

- `npm run verify`

Implementation should also include targeted launch verification for the active
Windows workstation because `npm run verify` alone cannot prove real PTY-backed
CLI interactivity.

## Module Documentation Requirements

Every new source file introduced by this design requires a matching module
document according to the repo guard:

- `docs/modules/frontend/...` for `src/**/*.ts` and `src/**/*.tsx`
- `docs/modules/tauri/...` for `src-tauri/src/**/*.rs`

The implementation is not complete unless new terminal files and their
corresponding module docs are added together and pass the repo's module-doc
checks.

## Version-One Acceptance Criteria

The design is correctly implemented only if all of the following are true:

- Clicking the floating Project Assistant launcher opens a CLI-selection panel.
- The selection panel offers exactly four approved CLIs in version one:
  Codex, OpenCode, Claude Code, and Kimi.
- The user must choose a working directory before launch.
- After launch, the same floating panel expands into one interactive terminal
  session.
- The terminal is PTY-backed and accepts real interactive input.
- The terminal view shows the active CLI and current working directory.
- The user can stop the active session.
- The user can restart the active session.
- Launch failures are surfaced with explicit, actionable errors.
- The version does not add multi-tab terminal sessions.
- The version does not inject startup prompts or skills automatically.

## Deferred Follow-Up Opportunities

These are intentionally deferred to later phases rather than version one:

- automatic project-context prompt injection
- repo-aware startup skills
- external-source analysis helpers
- skill recommendation tasks
- multiple concurrent sessions or terminal tabs
- session persistence or restore
- richer terminal history and transcript export
- directory presets such as current app repo, configured my-skills repo, or
  cached external source folders

## Open Design Decisions Resolved

- Entry surface: reuse the existing floating assistant launcher.
- Launch mode: show a four-choice CLI launcher first.
- Working directory: user chooses it before launch.
- Session count: single active session only.
- Terminal model: real PTY-backed interactive terminal.
- First version scope: no automatic prompt injection or skill bootstrapping.
