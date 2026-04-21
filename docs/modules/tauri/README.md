# Tauri Module Docs

Rust backend modules under `src-tauri/src/` map here one-to-one.

Domain `mod.rs` files document module boundaries, while leaf files document runtime behavior, DTOs, persistence, and test-only modules that are part of the tracked source set.

## Mapping

- `src-tauri/src/lib.rs` -> `docs/modules/tauri/lib.md`
- `src-tauri/src/main.rs` -> `docs/modules/tauri/main.md`
- `src-tauri/src/app_runtime/foo.rs` -> `docs/modules/tauri/app_runtime/foo.md`
- `src-tauri/src/cli/foo.rs` -> `docs/modules/tauri/cli/foo.md`
- `src-tauri/src/cli/commands/foo.rs` -> `docs/modules/tauri/cli/commands/foo.md`
- `src-tauri/src/commands/foo.rs` -> `docs/modules/tauri/commands/foo.md`
- `src-tauri/src/core/domain/foo.rs` -> `docs/modules/tauri/core/domain/foo.md`

Keep this README in sync when backend source roots or high-level boundaries change.
