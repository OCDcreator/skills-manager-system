# Project Assistant Prototype Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a bottom-right floating project assistant prototype that scans real project docs plus lightweight metadata, answers with deterministic retrieval-backed summaries, and shows context status plus sources in a wide floating panel.

**Architecture:** Keep the assistant isolated from the existing app-wide context. Add a new Rust `core::assistant` domain for project-root discovery, allowed-file scanning, chunk retrieval, and deterministic answer generation; keep Tauri commands thin; and mount a focused frontend assistant surface from `AppShell` via dedicated `components/assistant` modules rather than growing `AppContext`.

**Tech Stack:** Tauri 2, Rust 2021, React 19, TypeScript 5, Vite 7, Tailwind CSS 3, i18next, serde, walkdir, time

---

## File Map

- `src-tauri/src/core/assistant/mod.rs` — public assistant DTOs and top-level helper functions.
- `src-tauri/src/core/assistant/context.rs` — project-root discovery, allowed-file scanning, document loading, chunk count, status generation.
- `src-tauri/src/core/assistant/retrieval.rs` — chunk splitting, tokenization, scoring, and top-match selection.
- `src-tauri/src/core/assistant/response.rs` — deterministic answer assembly and no-match handling.
- `src-tauri/src/core/mod.rs` — register the new `assistant` core domain.
- `src-tauri/src/commands/assistant.rs` — `get_assistant_context_status` and `ask_project_assistant`.
- `src-tauri/src/commands/mod.rs` — register the new desktop command module.
- `src-tauri/src/lib.rs` — expose the new Tauri commands in `generate_handler!`.
- `src/lib/assistant.ts` — frontend DTOs and invoke wrappers for assistant status and ask calls.
- `src/components/assistant/ProjectAssistantLauncher.tsx` — floating robot button, open/close state, status prefetch.
- `src/components/assistant/ProjectAssistantPanel.tsx` — panel layout, message flow, ask lifecycle, loading and error handling.
- `src/components/assistant/ProjectAssistantSourceRail.tsx` — context scope, index freshness, warnings, and latest-answer sources.
- `src/components/AppShell.tsx` — mount the launcher so it is available in every app view.
- `src/i18n/en.json` / `src/i18n/zh.json` — assistant strings.
- `docs/modules/frontend/components/AppShell.md` — update shell docs for the mounted assistant entrypoint.
- `docs/modules/frontend/components/assistant/ProjectAssistantLauncher.md` — module doc for launcher behavior.
- `docs/modules/frontend/components/assistant/ProjectAssistantPanel.md` — module doc for panel orchestration.
- `docs/modules/frontend/components/assistant/ProjectAssistantSourceRail.md` — module doc for context-rail rendering.
- `docs/modules/frontend/lib/assistant.md` — module doc for assistant invoke wrapper.
- `docs/modules/tauri/lib.md` — update invoke handler registration docs.
- `docs/modules/tauri/commands/mod.md` — update desktop command index docs.
- `docs/modules/tauri/commands/assistant.md` — module doc for assistant commands.
- `docs/modules/tauri/core/mod.md` — update core domain index docs.
- `docs/modules/tauri/core/assistant/mod.md` — module doc for assistant public API.
- `docs/modules/tauri/core/assistant/context.md` — module doc for context discovery and scan rules.
- `docs/modules/tauri/core/assistant/retrieval.md` — module doc for chunking and scoring.
- `docs/modules/tauri/core/assistant/response.md` — module doc for deterministic answer formatting.

## Implementation Notes

- Treat this as a repo-local development feature first: scan the current workspace by discovering the project root from the current working directory and walking upward until both `AGENTS.md` and `src-tauri/Cargo.toml` are found.
- Do not reuse `AppContext` for assistant chat state; the current context file is already large.
- Do not add a global cache or embeddings index in the prototype. Rescan on demand and derive freshness from the latest scan timestamp.
- Keep the context scope strict: `AGENTS.md`, `docs/**/*.md`, `package.json`, and `src-tauri/Cargo.toml`.
- When no evidence is found, answer honestly and keep `sources` empty.

## Tasks

### Task 1: Build assistant context discovery and status

**Files:**
- Create: `src-tauri/src/core/assistant/mod.rs`
- Create: `src-tauri/src/core/assistant/context.rs`
- Modify: `src-tauri/src/core/mod.rs`
- Test: `src-tauri/src/core/assistant/context.rs` (inline `#[cfg(test)]`)

- [ ] **Step 1: Write failing context tests and public status types**

Add the new assistant core module with an explicit stub that compiles but fails the tests:

```rust
// src-tauri/src/core/assistant/mod.rs
pub mod context;

pub use context::{
    AssistantContextBundle,
    AssistantContextDocument,
    AssistantContextStatus,
    load_context_from_root,
    load_workspace_context,
};
```

```rust
// src-tauri/src/core/assistant/context.rs
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AssistantContextStatus {
    pub project_root: String,
    pub scope_label: String,
    pub indexed_document_count: usize,
    pub indexed_chunk_count: usize,
    pub last_indexed_at: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssistantContextDocument {
    pub path: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssistantContextBundle {
    pub status: AssistantContextStatus,
    pub documents: Vec<AssistantContextDocument>,
}

pub fn load_context_from_root(_root: &Path) -> Result<AssistantContextBundle> {
    bail!("red phase: assistant context scan failed")
}

pub fn load_workspace_context() -> Result<AssistantContextBundle> {
    let cwd = std::env::current_dir()?;
    load_context_from_root(&cwd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn assistant_context_status_counts_scannable_files() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        fs::create_dir_all(root.join("docs/modules")).unwrap();
        fs::create_dir_all(root.join("src-tauri")).unwrap();
        fs::write(root.join("AGENTS.md"), "# Agent Rules").unwrap();
        fs::write(root.join("docs/README.md"), "# Docs").unwrap();
        fs::write(root.join("docs/modules/intro.md"), "Verification lives here").unwrap();
        fs::write(root.join("package.json"), r#"{ "name": "assistant-demo" }"#).unwrap();
        fs::write(root.join("src-tauri/Cargo.toml"), "[package]\nname = \"assistant-demo\"").unwrap();
        fs::write(root.join("src/main.tsx"), "ignored").unwrap();

        let bundle = load_context_from_root(root).unwrap();

        assert_eq!(bundle.status.indexed_document_count, 5);
        assert_eq!(bundle.documents.len(), 5);
        assert!(bundle.status.scope_label.contains("AGENTS.md"));
        assert!(!bundle.status.project_root.is_empty());
    }

    #[test]
    fn assistant_context_ignores_non_allowed_files() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::create_dir_all(root.join("src-tauri")).unwrap();
        fs::write(root.join("AGENTS.md"), "# Agent Rules").unwrap();
        fs::write(root.join("docs/keep.md"), "# Keep").unwrap();
        fs::write(root.join("docs/skip.txt"), "skip").unwrap();
        fs::write(root.join("package.json"), r#"{ "name": "assistant-demo" }"#).unwrap();
        fs::write(root.join("src-tauri/Cargo.toml"), "[package]\nname = \"assistant-demo\"").unwrap();

        let bundle = load_context_from_root(root).unwrap();

        assert!(bundle.documents.iter().all(|document| !document.path.ends_with(".txt")));
        assert!(bundle.documents.iter().any(|document| document.path == "docs/keep.md"));
    }
}
```

Update the root core module:

```rust
// src-tauri/src/core/mod.rs
pub mod assistant;
pub mod agents;
pub mod git;
pub mod projects;
pub mod scenes;
pub mod settings;
pub mod skills;
```

- [ ] **Step 2: Run the targeted Rust tests and confirm they fail**

Run:

```bash
cargo test --manifest-path src-tauri/Cargo.toml assistant_context_status_counts_scannable_files -- --nocapture
```

Expected: FAIL with `red phase: assistant context scan failed`.

- [ ] **Step 3: Implement project-root discovery and allowed-file scanning**

Replace the stub with concrete root discovery, sorted file collection, content loading, and status generation:

```rust
// src-tauri/src/core/assistant/context.rs
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::Serialize;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use walkdir::WalkDir;

const CONTEXT_SCOPE_LABEL: &str = "AGENTS.md + docs/**/*.md + package.json + src-tauri/Cargo.toml";

fn discover_project_root(start: &Path) -> Result<PathBuf> {
    for candidate in start.ancestors() {
        if candidate.join("AGENTS.md").is_file() && candidate.join("src-tauri/Cargo.toml").is_file() {
            return Ok(candidate.to_path_buf());
        }
    }

    bail!("Could not discover project root from {}", start.display())
}

fn collect_context_paths(root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let direct_files = [
        root.join("AGENTS.md"),
        root.join("package.json"),
        root.join("src-tauri/Cargo.toml"),
    ];

    for path in direct_files {
        if path.is_file() {
            paths.push(path);
        }
    }

    let docs_root = root.join("docs");
    if docs_root.is_dir() {
        for entry in WalkDir::new(&docs_root).into_iter().filter_map(|entry| entry.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|value| value.to_str()) == Some("md") {
                paths.push(path.to_path_buf());
            }
        }
    }

    paths.sort();
    paths.dedup();
    paths
}

fn relative_display_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn title_from_path(path: &str, content: &str) -> String {
    content
        .lines()
        .find_map(|line| line.strip_prefix("# ").map(str::trim))
        .filter(|title| !title.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| path.rsplit('/').next().unwrap_or(path).to_string())
}

pub fn load_context_from_root(root: &Path) -> Result<AssistantContextBundle> {
    let project_root = discover_project_root(root)?;
    let mut documents = Vec::new();
    let mut warnings = Vec::new();

    for path in collect_context_paths(&project_root) {
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(error) => {
                warnings.push(format!("Failed to read {}: {}", path.display(), error));
                continue;
            }
        };

        let relative_path = relative_display_path(&project_root, &path);
        documents.push(AssistantContextDocument {
            title: title_from_path(&relative_path, &content),
            path: relative_path,
            content,
        });
    }

    let last_indexed_at = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .context("format assistant context timestamp")?;

    Ok(AssistantContextBundle {
        status: AssistantContextStatus {
            project_root: project_root.to_string_lossy().into_owned(),
            scope_label: CONTEXT_SCOPE_LABEL.to_string(),
            indexed_document_count: documents.len(),
            indexed_chunk_count: 0,
            last_indexed_at,
            warnings,
        },
        documents,
    })
}
```

- [ ] **Step 4: Re-run the targeted Rust tests and confirm they pass**

Run:

```bash
cargo test --manifest-path src-tauri/Cargo.toml assistant_context_ -- --nocapture
```

Expected: PASS for both assistant context tests.

- [ ] **Step 5: Record a checkpoint commit**

Run:

```bash
git add src-tauri/src/core/mod.rs src-tauri/src/core/assistant/mod.rs src-tauri/src/core/assistant/context.rs
git commit -m "feat: add assistant context scan status"
```

Expected: a clean checkpoint commit with only assistant context files staged.

### Task 2: Add chunk retrieval and deterministic answer generation

**Files:**
- Create: `src-tauri/src/core/assistant/retrieval.rs`
- Create: `src-tauri/src/core/assistant/response.rs`
- Modify: `src-tauri/src/core/assistant/mod.rs`
- Modify: `src-tauri/src/core/assistant/context.rs`
- Test: `src-tauri/src/core/assistant/retrieval.rs` (inline `#[cfg(test)]`)

- [ ] **Step 1: Add failing retrieval tests and response DTO stubs**

Create the retrieval and response files with tests that define the target behavior:

```rust
// src-tauri/src/core/assistant/retrieval.rs
use serde::Serialize;

use super::context::AssistantContextDocument;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssistantRetrievedChunk {
    pub path: String,
    pub title: String,
    pub heading: Option<String>,
    pub excerpt: String,
    pub score: usize,
}

pub fn retrieve_relevant_chunks(
    _question: &str,
    _documents: &[AssistantContextDocument],
) -> Vec<AssistantRetrievedChunk> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_documents() -> Vec<AssistantContextDocument> {
        vec![
            AssistantContextDocument {
                path: "AGENTS.md".into(),
                title: "Skills Manager System".into(),
                content: "## 开发命令\nnpm run verify\ncargo check --manifest-path src-tauri/Cargo.toml".into(),
            },
            AssistantContextDocument {
                path: "docs/README.md".into(),
                title: "Docs".into(),
                content: "# Docs\n模块文档覆盖必须和源码同步".into(),
            },
        ]
    }

    #[test]
    fn assistant_retrieval_prefers_verify_related_chunks() {
        let chunks = retrieve_relevant_chunks("how do I verify this project", &sample_documents());

        assert!(!chunks.is_empty());
        assert_eq!(chunks[0].path, "AGENTS.md");
        assert!(chunks[0].excerpt.contains("npm run verify"));
    }

    #[test]
    fn assistant_retrieval_returns_empty_for_irrelevant_questions() {
        let chunks = retrieve_relevant_chunks("what is the weather today", &sample_documents());

        assert!(chunks.is_empty());
    }
}
```

```rust
// src-tauri/src/core/assistant/response.rs
use serde::Serialize;

use super::context::AssistantContextBundle;
use super::retrieval::AssistantRetrievedChunk;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssistantSource {
    pub path: String,
    pub title: String,
    pub score: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssistantAnswerResponse {
    pub answer: String,
    pub context_summary: String,
    pub sources: Vec<AssistantSource>,
    pub retrieved_chunks: Vec<AssistantRetrievedChunk>,
}

pub fn build_answer(
    question: &str,
    context: &AssistantContextBundle,
    chunks: Vec<AssistantRetrievedChunk>,
) -> AssistantAnswerResponse {
    AssistantAnswerResponse {
        answer: format!("No answer implementation yet for {}", question),
        context_summary: context.status.scope_label.clone(),
        sources: Vec::new(),
        retrieved_chunks: chunks,
    }
}
```

Extend `mod.rs` with the new public API:

```rust
// src-tauri/src/core/assistant/mod.rs
pub mod context;
pub mod response;
pub mod retrieval;

use anyhow::{bail, Result};

pub use context::{AssistantContextBundle, AssistantContextDocument, AssistantContextStatus, load_context_from_root, load_workspace_context};
pub use response::{AssistantAnswerResponse, AssistantSource, build_answer};
pub use retrieval::{AssistantRetrievedChunk, retrieve_relevant_chunks};

pub fn ask_workspace_question(question: &str) -> Result<AssistantAnswerResponse> {
    if question.trim().is_empty() {
        bail!("Question is required");
    }

    let context = load_workspace_context()?;
    let chunks = retrieve_relevant_chunks(question, &context.documents);
    Ok(build_answer(question, &context, chunks))
}
```

- [ ] **Step 2: Run the targeted retrieval tests and confirm they fail**

Run:

```bash
cargo test --manifest-path src-tauri/Cargo.toml assistant_retrieval_ -- --nocapture
```

Expected: FAIL because `retrieve_relevant_chunks` returns no matches.

- [ ] **Step 3: Implement chunk scoring and template answers**

Replace the stubs with deterministic retrieval and response assembly:

```rust
// src-tauri/src/core/assistant/retrieval.rs
use std::collections::BTreeSet;

use serde::Serialize;

use super::context::AssistantContextDocument;

const MIN_SCORE: usize = 2;
const MAX_RESULTS: usize = 4;

fn tokenize(value: &str) -> BTreeSet<String> {
    value
        .split(|character: char| !character.is_alphanumeric())
        .map(|token| token.trim().to_lowercase())
        .filter(|token| token.len() >= 3)
        .collect()
}

fn excerpt_for_match(content: &str, token: &str) -> String {
    let token = token.to_lowercase();
    content
        .lines()
        .find(|line| line.to_lowercase().contains(&token))
        .or_else(|| content.lines().next())
        .unwrap_or("")
        .trim()
        .to_string()
}

pub fn retrieve_relevant_chunks(
    question: &str,
    documents: &[AssistantContextDocument],
) -> Vec<AssistantRetrievedChunk> {
    let tokens = tokenize(question);
    if tokens.is_empty() {
        return Vec::new();
    }

    let mut results = documents
        .iter()
        .filter_map(|document| {
            let content_lower = document.content.to_lowercase();
            let title_lower = document.title.to_lowercase();
            let path_lower = document.path.to_lowercase();

            let mut score = 0;
            for token in &tokens {
                if path_lower.contains(token) {
                    score += 5;
                }
                if title_lower.contains(token) {
                    score += 4;
                }
                if content_lower.contains(token) {
                    score += 2;
                }
            }

            if score < MIN_SCORE {
                return None;
            }

            let excerpt_token = tokens
                .iter()
                .find(|token| content_lower.contains(token.as_str()))
                .map(String::as_str)
                .unwrap_or("");

            Some(AssistantRetrievedChunk {
                path: document.path.clone(),
                title: document.title.clone(),
                heading: Some(document.title.clone()),
                excerpt: excerpt_for_match(&document.content, excerpt_token),
                score,
            })
        })
        .collect::<Vec<_>>();

    results.sort_by(|left, right| right.score.cmp(&left.score).then_with(|| left.path.cmp(&right.path)));
    results.truncate(MAX_RESULTS);
    results
}
```

```rust
// src-tauri/src/core/assistant/response.rs
use super::context::AssistantContextBundle;
use super::retrieval::AssistantRetrievedChunk;

fn sources_from_chunks(chunks: &[AssistantRetrievedChunk]) -> Vec<AssistantSource> {
    let mut sources = Vec::new();
    for chunk in chunks {
        if sources.iter().any(|source: &AssistantSource| source.path == chunk.path) {
            continue;
        }

        sources.push(AssistantSource {
            path: chunk.path.clone(),
            title: chunk.title.clone(),
            score: chunk.score,
        });
    }
    sources
}

pub fn build_answer(
    question: &str,
    context: &AssistantContextBundle,
    chunks: Vec<AssistantRetrievedChunk>,
) -> AssistantAnswerResponse {
    if chunks.is_empty() {
        return AssistantAnswerResponse {
            answer: format!(
                "I could not find enough evidence for \"{}\" in the current project docs. Try asking about AGENTS.md rules, docs content, verification, architecture, or module documentation.",
                question.trim()
            ),
            context_summary: format!(
                "Scanned {} documents from {}",
                context.status.indexed_document_count,
                context.status.scope_label
            ),
            sources: Vec::new(),
            retrieved_chunks: Vec::new(),
        };
    }

    let bullets = chunks
        .iter()
        .take(3)
        .map(|chunk| format!("- {}: {}", chunk.path, chunk.excerpt.trim()))
        .collect::<Vec<_>>()
        .join("\n");

    AssistantAnswerResponse {
        answer: format!(
            "I found project documentation related to \"{}\".\n{}\n\nThis prototype is using retrieval plus a deterministic summary, not a cloud model yet.",
            question.trim(),
            bullets
        ),
        context_summary: format!(
            "Scanned {} documents and matched {} excerpts from {}",
            context.status.indexed_document_count,
            chunks.len(),
            context.status.scope_label
        ),
        sources: sources_from_chunks(&chunks),
        retrieved_chunks: chunks,
    }
}
```

Update `context.rs` so `indexed_chunk_count` reflects the number of documents until retrieval exists, then replace it with a chunk count derived from headings and paragraph groups:

```rust
// inside load_context_from_root after documents are loaded
let indexed_chunk_count = documents
    .iter()
    .map(|document| document.content.split("\n\n").filter(|chunk| !chunk.trim().is_empty()).count())
    .sum();

// inside AssistantContextStatus construction
indexed_chunk_count,
```

- [ ] **Step 4: Re-run the retrieval tests and add a focused ask-question test**

Append this test to `response.rs` or `mod.rs` and run it:

```rust
#[cfg(test)]
mod answer_tests {
    use super::super::context::{AssistantContextBundle, AssistantContextDocument, AssistantContextStatus};
    use super::super::retrieval::retrieve_relevant_chunks;
    use super::build_answer;

    #[test]
    fn assistant_answer_includes_sources_for_verify_question() {
        let context = AssistantContextBundle {
            status: AssistantContextStatus {
                project_root: ".".into(),
                scope_label: "AGENTS.md + docs/**/*.md + package.json + src-tauri/Cargo.toml".into(),
                indexed_document_count: 2,
                indexed_chunk_count: 2,
                last_indexed_at: "2026-04-23T00:00:00Z".into(),
                warnings: Vec::new(),
            },
            documents: vec![AssistantContextDocument {
                path: "AGENTS.md".into(),
                title: "Skills Manager System".into(),
                content: "npm run verify\ncargo check --manifest-path src-tauri/Cargo.toml".into(),
            }],
        };

        let chunks = retrieve_relevant_chunks("verify project", &context.documents);
        let response = build_answer("verify project", &context, chunks);

        assert!(response.answer.contains("I found project documentation"));
        assert_eq!(response.sources[0].path, "AGENTS.md");
    }
}
```

Run:

```bash
cargo test --manifest-path src-tauri/Cargo.toml assistant_ -- --nocapture
```

Expected: PASS for context, retrieval, and answer tests.

- [ ] **Step 5: Record a checkpoint commit**

Run:

```bash
git add src-tauri/src/core/assistant/mod.rs src-tauri/src/core/assistant/context.rs src-tauri/src/core/assistant/retrieval.rs src-tauri/src/core/assistant/response.rs
git commit -m "feat: add assistant retrieval and template answers"
```

Expected: a clean checkpoint commit for the assistant core domain.

### Task 3: Wire assistant commands into Tauri

**Files:**
- Create: `src-tauri/src/commands/assistant.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Register assistant commands before the command file exists**

Add the command registration first so `cargo check` proves the missing file boundary:

```rust
// src-tauri/src/commands/mod.rs
pub mod agent_targets;
pub mod agents;
pub mod assistant;
pub mod git;
pub mod projects;
pub mod scenes;
pub mod settings;
pub mod skills;
```

```rust
// inside src-tauri/src/lib.rs generate_handler! list
commands::assistant::get_assistant_context_status,
commands::assistant::ask_project_assistant,
```

- [ ] **Step 2: Run cargo check and confirm the missing command module fails**

Run:

```bash
cargo check --manifest-path src-tauri/Cargo.toml
```

Expected: FAIL with an unresolved module or file error for `commands::assistant`.

- [ ] **Step 3: Create the thin Tauri command layer**

Implement the new commands as thin wrappers over `core::assistant`:

```rust
// src-tauri/src/commands/assistant.rs
use crate::core::assistant::{ask_workspace_question, load_workspace_context, AssistantAnswerResponse, AssistantContextStatus};

#[tauri::command]
pub fn get_assistant_context_status() -> Result<AssistantContextStatus, String> {
    load_workspace_context()
        .map(|bundle| bundle.status)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn ask_project_assistant(question: String) -> Result<AssistantAnswerResponse, String> {
    ask_workspace_question(&question).map_err(|error| error.to_string())
}
```

- [ ] **Step 4: Re-run cargo check and confirm the desktop build compiles**

Run:

```bash
cargo check --manifest-path src-tauri/Cargo.toml
```

Expected: PASS with the new assistant commands registered.

- [ ] **Step 5: Record a checkpoint commit**

Run:

```bash
git add src-tauri/src/commands/mod.rs src-tauri/src/commands/assistant.rs src-tauri/src/lib.rs
git commit -m "feat: expose assistant commands"
```

Expected: a clean checkpoint commit for command wiring only.

### Task 4: Mount the launcher and frontend API wrapper

**Files:**
- Create: `src/lib/assistant.ts`
- Create: `src/components/assistant/ProjectAssistantLauncher.tsx`
- Modify: `src/components/AppShell.tsx`

- [ ] **Step 1: Mount the launcher in AppShell before creating the new component**

Update the shell so the missing import produces a frontend failure first:

```tsx
// src/components/AppShell.tsx
import { ProjectAssistantLauncher } from "./assistant/ProjectAssistantLauncher";

export function AppShell({ children, contentWidthClassName }: AppShellProps) {
  return (
    <div className="min-h-screen bg-slate-950 text-slate-100">
      <header className="border-b border-slate-800 bg-slate-900/90 backdrop-blur">
        {/* keep the existing header markup unchanged */}
      </header>
      <main className={`mx-auto ${widthClassName} px-6 py-8`}>{children}</main>
      <ProjectAssistantLauncher />
      {pendingNavigation ? (
        <UnsavedChangesDialog
          onCancel={cancelNavigation}
          onDiscard={confirmNavigationDiscard}
          onSave={confirmNavigationSave}
          pendingNavigation={pendingNavigation}
        />
      ) : null}
    </div>
  );
}
```

- [ ] **Step 2: Run the frontend build and confirm the missing launcher fails**

Run:

```bash
npm run build
```

Expected: FAIL with `Cannot find module './assistant/ProjectAssistantLauncher'` or an equivalent TypeScript import error.

- [ ] **Step 3: Create the assistant invoke wrapper and launcher shell**

Add the typed invoke wrapper:

```ts
// src/lib/assistant.ts
import { invoke } from "@tauri-apps/api/core";

export interface AssistantContextStatus {
  projectRoot: string;
  scopeLabel: string;
  indexedDocumentCount: number;
  indexedChunkCount: number;
  lastIndexedAt: string;
  warnings: string[];
}

export interface AssistantSource {
  path: string;
  title: string;
  score: number;
}

export interface AssistantRetrievedChunk {
  path: string;
  title: string;
  heading: string | null;
  excerpt: string;
  score: number;
}

export interface AssistantAnswerResponse {
  answer: string;
  contextSummary: string;
  sources: AssistantSource[];
  retrievedChunks: AssistantRetrievedChunk[];
}

export const getAssistantContextStatus = () =>
  invoke<AssistantContextStatus>("get_assistant_context_status");

export const askProjectAssistant = (question: string) =>
  invoke<AssistantAnswerResponse>("ask_project_assistant", { question });
```

Add the launcher with open/close state and status prefetch:

```tsx
// src/components/assistant/ProjectAssistantLauncher.tsx
import { Bot, MessageSquarePlus, X } from "lucide-react";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import {
  getAssistantContextStatus,
  type AssistantContextStatus,
} from "../../lib/assistant";

export function ProjectAssistantLauncher() {
  const { t } = useTranslation();
  const [isOpen, setIsOpen] = useState(false);
  const [status, setStatus] = useState<AssistantContextStatus | null>(null);
  const [statusError, setStatusError] = useState<string | null>(null);
  const [isLoadingStatus, setIsLoadingStatus] = useState(false);

  useEffect(() => {
    if (!isOpen || status || isLoadingStatus) {
      return;
    }

    setIsLoadingStatus(true);
    void getAssistantContextStatus()
      .then((nextStatus) => {
        setStatus(nextStatus);
        setStatusError(null);
      })
      .catch((error) => {
        setStatusError(error instanceof Error ? error.message : String(error));
      })
      .finally(() => setIsLoadingStatus(false));
  }, [isLoadingStatus, isOpen, status]);

  return (
    <>
      <button
        className="fixed bottom-6 right-6 z-40 flex h-14 w-14 items-center justify-center rounded-full border border-sky-400/40 bg-slate-900 text-sky-300 shadow-2xl shadow-sky-950/40 transition hover:border-sky-300 hover:text-sky-100"
        onClick={() => setIsOpen((current) => !current)}
        title={t("assistant.launcherLabel")}
      >
        {isOpen ? <X className="h-6 w-6" /> : <Bot className="h-6 w-6" />}
      </button>

      {!isOpen ? (
        <div className="fixed bottom-24 right-6 z-30 rounded-full bg-slate-900/95 px-3 py-2 text-xs text-slate-300 shadow-lg shadow-slate-950/40">
          <MessageSquarePlus className="mr-2 inline h-4 w-4" />
          {t("assistant.launcherHint")}
        </div>
      ) : null}
    </>
  );
}
```

- [ ] **Step 4: Re-run the frontend build and confirm the launcher compiles**

Run:

```bash
npm run build
```

Expected: PASS, with the launcher mounted and the assistant API wrapper compiling cleanly.

- [ ] **Step 5: Record a checkpoint commit**

Run:

```bash
git add src/components/AppShell.tsx src/components/assistant/ProjectAssistantLauncher.tsx src/lib/assistant.ts
git commit -m "feat: mount assistant launcher"
```

Expected: a clean checkpoint commit for shell integration.

### Task 5: Build the assistant panel and source rail

**Files:**
- Create: `src/components/assistant/ProjectAssistantPanel.tsx`
- Create: `src/components/assistant/ProjectAssistantSourceRail.tsx`
- Modify: `src/components/assistant/ProjectAssistantLauncher.tsx`

- [ ] **Step 1: Make the launcher depend on a missing panel to create a build failure**

Update the launcher before creating the panel components:

```tsx
// src/components/assistant/ProjectAssistantLauncher.tsx
import { ProjectAssistantPanel } from "./ProjectAssistantPanel";

// inside the component return
{isOpen ? (
  <ProjectAssistantPanel
    isLoadingStatus={isLoadingStatus}
    onClose={() => setIsOpen(false)}
    status={status}
    statusError={statusError}
  />
) : null}
```

- [ ] **Step 2: Run the frontend build and confirm the missing panel fails**

Run:

```bash
npm run build
```

Expected: FAIL with `Cannot find module './ProjectAssistantPanel'` or an equivalent TypeScript import error.

- [ ] **Step 3: Create the panel and right context rail**

Add a focused source rail component:

```tsx
// src/components/assistant/ProjectAssistantSourceRail.tsx
import { useTranslation } from "react-i18next";
import type { AssistantContextStatus, AssistantSource } from "../../lib/assistant";

interface ProjectAssistantSourceRailProps {
  status: AssistantContextStatus | null;
  statusError: string | null;
  isLoadingStatus: boolean;
  sources: AssistantSource[];
}

export function ProjectAssistantSourceRail({
  status,
  statusError,
  isLoadingStatus,
  sources,
}: ProjectAssistantSourceRailProps) {
  const { t } = useTranslation();

  return (
    <aside className="rounded-2xl border border-slate-800 bg-slate-900/70 p-4">
      <p className="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400">
        {t("assistant.contextTitle")}
      </p>
      <div className="mt-3 space-y-2 text-sm text-slate-300">
        <p>{isLoadingStatus ? t("assistant.statusLoading") : status?.scopeLabel ?? t("assistant.statusUnavailable")}</p>
        <p>{status ? t("assistant.documentCount", { count: status.indexedDocumentCount }) : null}</p>
        <p>{status ? t("assistant.chunkCount", { count: status.indexedChunkCount }) : null}</p>
        <p>{statusError ? statusError : status?.lastIndexedAt ?? null}</p>
      </div>

      <div className="mt-5">
        <p className="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400">
          {t("assistant.sourcesTitle")}
        </p>
        <ul className="mt-3 space-y-2 text-sm text-slate-300">
          {sources.length === 0 ? <li>{t("assistant.sourcesEmpty")}</li> : null}
          {sources.map((source) => (
            <li key={source.path} className="rounded-xl border border-slate-800 bg-slate-950/60 px-3 py-2">
              <p className="font-medium text-slate-100">{source.title}</p>
              <p className="mt-1 text-xs text-slate-400">{source.path}</p>
            </li>
          ))}
        </ul>
      </div>
    </aside>
  );
}
```

Add the chat panel with local message state:

```tsx
// src/components/assistant/ProjectAssistantPanel.tsx
import { FormEvent, useMemo, useState } from "react";
import { Send, Sparkles, X } from "lucide-react";
import { useTranslation } from "react-i18next";
import {
  askProjectAssistant,
  type AssistantAnswerResponse,
  type AssistantContextStatus,
  type AssistantSource,
} from "../../lib/assistant";
import { ProjectAssistantSourceRail } from "./ProjectAssistantSourceRail";

interface ProjectAssistantPanelProps {
  isLoadingStatus: boolean;
  onClose: () => void;
  status: AssistantContextStatus | null;
  statusError: string | null;
}

interface AssistantMessage {
  id: string;
  role: "assistant" | "user";
  content: string;
}

const INITIAL_MESSAGE: AssistantMessage = {
  id: "assistant-welcome",
  role: "assistant",
  content: "Ask about this project. I will answer from AGENTS.md, docs, package.json, and src-tauri/Cargo.toml.",
};

export function ProjectAssistantPanel({
  isLoadingStatus,
  onClose,
  status,
  statusError,
}: ProjectAssistantPanelProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState("");
  const [isAsking, setIsAsking] = useState(false);
  const [askError, setAskError] = useState<string | null>(null);
  const [messages, setMessages] = useState<AssistantMessage[]>([INITIAL_MESSAGE]);
  const [sources, setSources] = useState<AssistantSource[]>([]);

  const canSubmit = draft.trim().length > 0 && !isAsking;

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const question = draft.trim();
    if (!question) {
      return;
    }

    setDraft("");
    setAskError(null);
    setIsAsking(true);
    setMessages((current) => [...current, { id: crypto.randomUUID(), role: "user", content: question }]);

    try {
      const response: AssistantAnswerResponse = await askProjectAssistant(question);
      setMessages((current) => [
        ...current,
        { id: crypto.randomUUID(), role: "assistant", content: response.answer },
      ]);
      setSources(response.sources);
    } catch (error) {
      setAskError(error instanceof Error ? error.message : String(error));
    } finally {
      setIsAsking(false);
    }
  }

  const panelTitle = useMemo(() => t("assistant.title"), [t]);

  return (
    <section className="fixed bottom-24 right-6 z-50 w-[min(620px,calc(100vw-2rem))] overflow-hidden rounded-[28px] border border-sky-400/30 bg-slate-950/95 shadow-2xl shadow-slate-950/60 backdrop-blur">
      <header className="flex items-center justify-between border-b border-slate-800 px-5 py-4">
        <div>
          <h2 className="text-base font-semibold text-slate-100">{panelTitle}</h2>
          <p className="text-sm text-slate-400">{t("assistant.subtitle")}</p>
        </div>
        <button className="rounded-full p-2 text-slate-400 hover:bg-slate-900 hover:text-slate-100" onClick={onClose}>
          <X className="h-5 w-5" />
        </button>
      </header>

      <div className="grid gap-4 p-5 md:grid-cols-[minmax(0,1fr)_190px]">
        <div className="flex min-h-[420px] flex-col rounded-2xl border border-slate-800 bg-slate-900/60">
          <div className="flex-1 space-y-3 overflow-y-auto px-4 py-4">
            {messages.map((message) => (
              <article
                key={message.id}
                className={message.role === "assistant"
                  ? "mr-10 rounded-2xl bg-slate-900 px-4 py-3 text-sm text-slate-100"
                  : "ml-10 rounded-2xl bg-sky-500/20 px-4 py-3 text-sm text-sky-50"}
              >
                {message.content}
              </article>
            ))}
            {askError ? <p className="text-sm text-rose-300">{askError}</p> : null}
            {isAsking ? (
              <div className="mr-10 flex items-center gap-2 rounded-2xl bg-slate-900 px-4 py-3 text-sm text-slate-300">
                <Sparkles className="h-4 w-4 animate-pulse" />
                {t("assistant.answerLoading")}
              </div>
            ) : null}
          </div>

          <form className="border-t border-slate-800 p-4" onSubmit={handleSubmit}>
            <textarea
              className="min-h-24 w-full resize-none rounded-2xl border border-slate-700 bg-slate-950 px-4 py-3 text-sm text-slate-100 outline-none ring-0 placeholder:text-slate-500 focus:border-sky-400"
              disabled={isAsking}
              onChange={(event) => setDraft(event.target.value)}
              placeholder={t("assistant.inputPlaceholder")}
              value={draft}
            />
            <div className="mt-3 flex items-center justify-between">
              <p className="text-xs text-slate-500">{t("assistant.inputHint")}</p>
              <button
                className="inline-flex items-center gap-2 rounded-full bg-sky-400 px-4 py-2 text-sm font-medium text-slate-950 disabled:cursor-not-allowed disabled:bg-slate-700 disabled:text-slate-400"
                disabled={!canSubmit}
                type="submit"
              >
                <Send className="h-4 w-4" />
                {t("assistant.send")}
              </button>
            </div>
          </form>
        </div>

        <ProjectAssistantSourceRail
          isLoadingStatus={isLoadingStatus}
          sources={sources}
          status={status}
          statusError={statusError}
        />
      </div>
    </section>
  );
}
```

Finish the launcher integration:

```tsx
// src/components/assistant/ProjectAssistantLauncher.tsx
import { ProjectAssistantPanel } from "./ProjectAssistantPanel";

{isOpen ? (
  <ProjectAssistantPanel
    isLoadingStatus={isLoadingStatus}
    onClose={() => setIsOpen(false)}
    status={status}
    statusError={statusError}
  />
) : null}
```

- [ ] **Step 4: Re-run the frontend build and confirm the panel compiles**

Run:

```bash
npm run build
```

Expected: PASS, with the launcher, panel, and right rail compiling together.

- [ ] **Step 5: Record a checkpoint commit**

Run:

```bash
git add src/components/assistant/ProjectAssistantLauncher.tsx src/components/assistant/ProjectAssistantPanel.tsx src/components/assistant/ProjectAssistantSourceRail.tsx
git commit -m "feat: add floating project assistant panel"
```

Expected: a clean checkpoint commit for the assistant frontend surface.

### Task 6: Add i18n and module documentation coverage

**Files:**
- Modify: `src/i18n/en.json`
- Modify: `src/i18n/zh.json`
- Modify: `docs/modules/frontend/components/AppShell.md`
- Create: `docs/modules/frontend/components/assistant/ProjectAssistantLauncher.md`
- Create: `docs/modules/frontend/components/assistant/ProjectAssistantPanel.md`
- Create: `docs/modules/frontend/components/assistant/ProjectAssistantSourceRail.md`
- Create: `docs/modules/frontend/lib/assistant.md`
- Modify: `docs/modules/tauri/lib.md`
- Modify: `docs/modules/tauri/commands/mod.md`
- Create: `docs/modules/tauri/commands/assistant.md`
- Modify: `docs/modules/tauri/core/mod.md`
- Create: `docs/modules/tauri/core/assistant/mod.md`
- Create: `docs/modules/tauri/core/assistant/context.md`
- Create: `docs/modules/tauri/core/assistant/retrieval.md`
- Create: `docs/modules/tauri/core/assistant/response.md`

- [ ] **Step 1: Run the module-doc guard before adding docs**

Run:

```bash
npm run check:module-docs
```

Expected: FAIL because the new assistant source files do not yet have matching docs.

- [ ] **Step 2: Add assistant i18n strings and module docs**

Add these frontend strings:

```json
// src/i18n/en.json
{
  "assistant": {
    "launcherLabel": "Project assistant",
    "launcherHint": "Ask this project",
    "title": "Project assistant",
    "subtitle": "Answers from local project docs and metadata",
    "contextTitle": "Context",
    "statusLoading": "Scanning project docs...",
    "statusUnavailable": "Context is unavailable",
    "sourcesTitle": "Sources",
    "sourcesEmpty": "No sources yet",
    "documentCount": "{{count}} docs indexed",
    "chunkCount": "{{count}} chunks indexed",
    "inputPlaceholder": "Ask about this project...",
    "inputHint": "Enter sends. Shift+Enter adds a new line.",
    "answerLoading": "Searching project docs...",
    "send": "Send"
  }
}
```

```json
// src/i18n/zh.json
{
  "assistant": {
    "launcherLabel": "项目助手",
    "launcherHint": "问问这个项目",
    "title": "项目助手",
    "subtitle": "基于本地项目文档和元信息回答",
    "contextTitle": "上下文",
    "statusLoading": "正在扫描项目文档...",
    "statusUnavailable": "当前无法读取上下文",
    "sourcesTitle": "来源",
    "sourcesEmpty": "暂时没有来源",
    "documentCount": "已索引 {{count}} 个文档",
    "chunkCount": "已索引 {{count}} 个片段",
    "inputPlaceholder": "问这个项目的问题...",
    "inputHint": "Enter 发送，Shift+Enter 换行。",
    "answerLoading": "正在检索项目文档...",
    "send": "发送"
  }
}
```

Use concise module docs with responsibilities and data flow:

```md
# `src/components/assistant/ProjectAssistantLauncher.tsx`

## Responsibility

Renders the fixed floating assistant button, owns open/close state, and preloads assistant context status before the panel is used.

## Data Flow

- Calls `getAssistantContextStatus()` from `src/lib/assistant.ts`
- Passes status and status errors into `ProjectAssistantPanel`
- Mounts from `src/components/AppShell.tsx`
```

```md
# `src-tauri/src/core/assistant/context.rs`

## Responsibility

Discovers the workspace root, filters assistant-allowed files, reads document content, and returns context status plus loaded documents.

## Notes

- Allowed files are `AGENTS.md`, `docs/**/*.md`, `package.json`, and `src-tauri/Cargo.toml`
- Non-readable files become warnings instead of aborting the whole scan
- Chunk count is derived locally for prototype status reporting
```

Mirror this style for each created assistant source file and update the touched existing module docs so they mention the assistant registration points.

- [ ] **Step 3: Re-run the module-doc guard and confirm coverage passes**

Run:

```bash
npm run check:module-docs
```

Expected: PASS, with the new assistant docs matching every new or modified source file.

- [ ] **Step 4: Record a checkpoint commit**

Run:

```bash
git add src/i18n/en.json src/i18n/zh.json docs/modules/frontend/components/AppShell.md docs/modules/frontend/components/assistant docs/modules/frontend/lib/assistant.md docs/modules/tauri/lib.md docs/modules/tauri/commands/mod.md docs/modules/tauri/commands/assistant.md docs/modules/tauri/core/mod.md docs/modules/tauri/core/assistant
git commit -m "docs: add assistant module coverage and i18n"
```

Expected: a clean checkpoint commit for localization and module docs.

### Task 7: Run focused checks and the full verification gate

**Files:**
- No source changes expected; fix only assistant-related failures if verification exposes them.

- [ ] **Step 1: Run focused Rust assistant tests**

Run:

```bash
cargo test --manifest-path src-tauri/Cargo.toml assistant_ -- --nocapture
```

Expected: PASS for all assistant-prefixed tests.

- [ ] **Step 2: Run cargo check and frontend build separately**

Run:

```bash
cargo check --manifest-path src-tauri/Cargo.toml
npm run build
```

Expected: PASS for both commands.

- [ ] **Step 3: Run the architecture and module-doc guards directly**

Run:

```bash
npm run check:architecture
npm run check:module-docs
```

Expected: PASS with no large-file or module-doc coverage regressions.

- [ ] **Step 4: Run the full verify gate**

Run:

```bash
npm run verify
```

Expected: PASS, including module-doc checks, architecture checks, script tests, build, `cargo check`, and `cargo test`.

- [ ] **Step 5: Record the final implementation checkpoint**

Run:

```bash
git status --short
git add src-tauri/src/core/assistant src-tauri/src/commands/assistant.rs src-tauri/src/commands/mod.rs src-tauri/src/core/mod.rs src-tauri/src/lib.rs src/lib/assistant.ts src/components/AppShell.tsx src/components/assistant src/i18n/en.json src/i18n/zh.json docs/modules/frontend/components/AppShell.md docs/modules/frontend/components/assistant docs/modules/frontend/lib/assistant.md docs/modules/tauri/lib.md docs/modules/tauri/commands/mod.md docs/modules/tauri/commands/assistant.md docs/modules/tauri/core/mod.md docs/modules/tauri/core/assistant
git commit -m "feat: add project assistant prototype"
```

Expected: `git status --short` shows only the planned assistant changes before the final commit, and the final commit captures the complete prototype.
