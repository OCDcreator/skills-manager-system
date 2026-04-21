# Project Docs Guide

`docs/` holds repo-maintenance documentation that should stay aligned with the current codebase.

## Start Here

- `modules/README.md` documents source-to-module-doc mapping rules.
- `modules/_WORKFLOW.md` documents the required sync workflow for code and module docs.
- `modules/_TEMPLATE.md` is the template for new source-module docs.
- `superpowers/` contains design specs and implementation plans from Superpowers workflows.

## Directory Rules

- Source-adjacent docs live under `docs/modules/`.
- Planning artifacts may stay under `docs/superpowers/`.
- Do not add module-specific notes outside `docs/modules/`; they bypass the guardrail.
- When a source module changes, update its mapped module doc in the same branch.

## CLI Build Notes

- Default Cargo and Tauri commands build the desktop app through the `desktop` feature.
- Build the headless CLI only with `cargo build --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager`.
- Verify the generated binary with `./src-tauri/target/debug/skills-manager --help`.
- PATH registration or wrapper installation belongs in a future installer task; this repo currently only verifies the binary artifact.
