# Open Source Knowledge Base Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first working version of a standalone `project-knowledge-base` repository that auto-generates project summaries, records usage evidence, and verifies knowledge consistency across Windows, macOS, and CI.

**Architecture:** Implement a small Node.js ESM CLI with no runtime dependencies. The repository stores `knowledge.yaml`, a committed normalized snapshot, generated docs, per-host JSONL events, templates, and verification scripts. Local commands read `.local/config.json` and the user's `open-source-project` index; CI commands read only committed files.

**Tech Stack:** Node.js 20+, ESM `.mjs` scripts, JSON/JSONL, a constrained YAML writer/parser for the v1 `knowledge.yaml` shape, Markdown templates, GitHub Actions.

---

## Target Location

Create the new repository as a sibling of the current project:

```text
C:\Users\lt\Desktop\Write\custom-project\project-knowledge-base
```

Do not commit from automation unless the user explicitly asks in the current session.

## File Structure

```text
project-knowledge-base/
├── .github/workflows/verify.yml
├── .gitignore
├── README.md
├── knowledge.yaml
├── package.json
├── snapshots/open-source-project.normalized.json
├── docs/generated/
├── docs/using/
├── docs/abandoned/
├── docs/notes/
├── events/.gitkeep
├── templates/generated.md
├── templates/using.md
├── templates/abandoned.md
├── scripts/sync-index.mjs
├── scripts/record-event.mjs
├── scripts/use-project.mjs
├── scripts/generate-docs.mjs
├── scripts/verify-local.mjs
├── scripts/verify-ci.mjs
├── scripts/report-stale.mjs
├── scripts/lib/config.mjs
├── scripts/lib/event-schema.mjs
├── scripts/lib/events.mjs
├── scripts/lib/files.mjs
├── scripts/lib/index-adapter.mjs
├── scripts/lib/knowledge.mjs
├── scripts/lib/markdown.mjs
├── scripts/lib/normalize-index.mjs
├── scripts/lib/verify-core.mjs
└── tests/fixtures/
    ├── projects.json
    ├── knowledge-valid.yaml
    ├── knowledge-promote.yaml
    └── events-valid.jsonl
```

File responsibilities:

- `scripts/lib/config.mjs`: resolve repo root, load `.local/config.json`, initialize `machine_id`.
- `scripts/lib/index-adapter.mjs`: read Windows JSON and normalized JSON inputs.
- `scripts/lib/normalize-index.mjs`: convert external index entries into snapshot entries keyed by `category/name`.
- `scripts/lib/knowledge.mjs`: parse/write the constrained v1 `knowledge.yaml` shape.
- `scripts/lib/event-schema.mjs`: validate every v1 event kind from the spec table.
- `scripts/lib/events.mjs`: append/read JSONL events and generate UUID-like event IDs.
- `scripts/lib/markdown.mjs`: render generated, using, and abandoned docs from project data and events.
- `scripts/lib/verify-core.mjs`: shared validators used by local and CI entrypoints.

## Task 1: Scaffold Repository

**Files:**
- Create: `C:\Users\lt\Desktop\Write\custom-project\project-knowledge-base\package.json`
- Create: `C:\Users\lt\Desktop\Write\custom-project\project-knowledge-base\.gitignore`
- Create: `C:\Users\lt\Desktop\Write\custom-project\project-knowledge-base\README.md`
- Create: `C:\Users\lt\Desktop\Write\custom-project\project-knowledge-base\knowledge.yaml`
- Create: `C:\Users\lt\Desktop\Write\custom-project\project-knowledge-base\events\.gitkeep`
- Create directories listed in File Structure.

- [ ] **Step 1: Create the repository directory**

Run from `C:\Users\lt\Desktop\Write\custom-project\skills-manager-system`:

```powershell
Test-Path -LiteralPath "C:\Users\lt\Desktop\Write\custom-project"
New-Item -ItemType Directory -Path "C:\Users\lt\Desktop\Write\custom-project\project-knowledge-base"
```

Expected: first command prints `True`; second command creates the new directory or reports it already exists.

- [ ] **Step 2: Create `package.json`**

```json
{
  "name": "project-knowledge-base",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "sync:index": "node scripts/sync-index.mjs",
    "record-event": "node scripts/record-event.mjs",
    "use-project": "node scripts/use-project.mjs",
    "generate": "node scripts/generate-docs.mjs",
    "verify:local": "node scripts/verify-local.mjs",
    "verify:ci": "node scripts/verify-ci.mjs",
    "report:stale": "node scripts/report-stale.mjs",
    "test": "node scripts/verify-ci.mjs"
  },
  "engines": {
    "node": ">=20"
  }
}
```

- [ ] **Step 3: Create `.gitignore`**

```gitignore
.local/
node_modules/
.DS_Store
*.log
```

- [ ] **Step 4: Create initial `knowledge.yaml`**

```yaml
schema_version: 1
source_index:
  adapter: open-source-project
  snapshot: snapshots/open-source-project.normalized.json
  required: true
projects: {}
```

- [ ] **Step 5: Create `README.md`**

```markdown
# Project Knowledge Base

AI-maintained knowledge base for local open-source projects.

## Workflow

1. Configure `.local/config.json` on each machine.
2. Run `npm run sync:index` to normalize the local `open-source-project` index.
3. Run `npm run generate` to create project summaries.
4. Run `npm run verify:local` before reporting setup success for a project.
5. Run `npm run verify:ci` in CI.

Committed files are portable across Windows and macOS. Machine-local paths live only in `.local/config.json`.
```

- [ ] **Step 6: Verify scaffold**

Run:

```powershell
npm run verify:ci
```

Expected before scripts exist: fails with `Missing script: "verify:ci"` if package scripts were not created, or `Cannot find module` until Task 6 creates the verifier. Do not treat this as final failure.

## Task 2: Add Config And File Utilities

**Files:**
- Create: `scripts/lib/files.mjs`
- Create: `scripts/lib/config.mjs`

- [ ] **Step 1: Create `scripts/lib/files.mjs`**

```js
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname } from "node:path";

export async function readJson(filePath) {
  return JSON.parse(await readFile(filePath, "utf8"));
}

export async function writeJson(filePath, value) {
  await mkdir(dirname(filePath), { recursive: true });
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`, "utf8");
}

export async function writeText(filePath, value) {
  await mkdir(dirname(filePath), { recursive: true });
  await writeFile(filePath, value, "utf8");
}
```

- [ ] **Step 2: Create `scripts/lib/config.mjs`**

```js
import { existsSync } from "node:fs";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { randomUUID } from "node:crypto";

export function repoRoot() {
  return resolve(new URL("../..", import.meta.url).pathname);
}

export function localConfigPath(root = repoRoot()) {
  return join(root, ".local", "config.json");
}

export async function loadLocalConfig({ requireConfig = true } = {}) {
  const filePath = localConfigPath();
  if (!existsSync(filePath)) {
    if (!requireConfig) return null;
    throw new Error(`Missing local config: ${filePath}`);
  }

  const config = JSON.parse(await readFile(filePath, "utf8"));
  if (!config.machine_id) {
    config.machine_id = `${config.host ?? "machine"}-${randomUUID().slice(0, 8)}`;
    await mkdir(dirname(filePath), { recursive: true });
    await writeFile(filePath, `${JSON.stringify(config, null, 2)}\n`, "utf8");
  }
  return config;
}
```

- [ ] **Step 3: Smoke test config missing behavior**

Run:

```powershell
node -e "import('./scripts/lib/config.mjs').then(async m => { try { await m.loadLocalConfig(); } catch (error) { console.log(error.message.includes('Missing local config')) } })"
```

Expected: prints `true`.

## Task 3: Normalize External Project Index

**Files:**
- Create: `scripts/lib/index-adapter.mjs`
- Create: `scripts/lib/normalize-index.mjs`
- Create: `scripts/sync-index.mjs`
- Create: `snapshots/open-source-project.normalized.json`
- Create: `tests/fixtures/projects.json`

- [ ] **Step 1: Create fixture `tests/fixtures/projects.json`**

```json
[
  {
    "name": "lean-ctx",
    "category": "AI-tools-agents",
    "description": "Context runtime for AI agents",
    "remote_url": "https://github.com/yvgude/lean-ctx.git",
    "default_branch": "main",
    "path_relative": "AI-tools-agents/lean-ctx",
    "has_readme": true
  }
]
```

- [ ] **Step 2: Create `scripts/lib/index-adapter.mjs`**

```js
import { readJson } from "./files.mjs";

export async function readOpenSourceIndex(indexPath) {
  const value = await readJson(indexPath);
  if (Array.isArray(value)) return value;
  if (Array.isArray(value.projects)) return value.projects;
  throw new Error(`Unsupported index shape: ${indexPath}`);
}
```

- [ ] **Step 3: Create `scripts/lib/normalize-index.mjs`**

```js
export function normalizeProject(project) {
  const name = requiredString(project.name, "name");
  const category = requiredString(project.category, "category");
  return {
    key: `${category}/${name}`,
    name,
    category,
    description: String(project.description ?? ""),
    remote_url: requiredString(project.remote_url, "remote_url"),
    default_branch: String(project.default_branch ?? "main"),
    path_relative: requiredString(project.path_relative, "path_relative"),
    has_readme: Boolean(project.has_readme),
  };
}

export function normalizeIndex(projects, sourceHost = "unknown") {
  return {
    schema_version: 1,
    generated_at: new Date().toISOString(),
    source_host: sourceHost,
    projects: projects.map(normalizeProject).sort((left, right) => left.key.localeCompare(right.key)),
  };
}

function requiredString(value, field) {
  if (typeof value !== "string" || value.trim() === "") throw new Error(`Project is missing ${field}`);
  return value;
}
```

- [ ] **Step 4: Create `scripts/sync-index.mjs`**

```js
import { join } from "node:path";
import { loadLocalConfig, repoRoot } from "./lib/config.mjs";
import { writeJson } from "./lib/files.mjs";
import { readOpenSourceIndex } from "./lib/index-adapter.mjs";
import { normalizeIndex } from "./lib/normalize-index.mjs";

const config = await loadLocalConfig();
const projects = await readOpenSourceIndex(config.openSourceIndex);
const snapshot = normalizeIndex(projects, config.host ?? config.machine_id);
await writeJson(join(repoRoot(), "snapshots", "open-source-project.normalized.json"), snapshot);
console.log(`Normalized ${snapshot.projects.length} project(s)`);
```

- [ ] **Step 5: Create initial empty snapshot**

```json
{
  "schema_version": 1,
  "generated_at": "1970-01-01T00:00:00.000Z",
  "source_host": "bootstrap",
  "projects": []
}
```

- [ ] **Step 6: Test normalization with fixture**

Run:

```powershell
node -e "import('./scripts/lib/index-adapter.mjs').then(async a => { const p = await a.readOpenSourceIndex('tests/fixtures/projects.json'); const n = await import('./scripts/lib/normalize-index.mjs'); console.log(n.normalizeIndex(p, 'test').projects[0].key) })"
```

Expected: prints `AI-tools-agents/lean-ctx`.

## Task 4: Parse And Write Knowledge YAML

**Files:**
- Create: `scripts/lib/knowledge.mjs`
- Create: `tests/fixtures/knowledge-valid.yaml`
- Create: `tests/fixtures/knowledge-promote.yaml`

- [ ] **Step 1: Create fixture `tests/fixtures/knowledge-valid.yaml`**

```yaml
schema_version: 1
source_index:
  adapter: open-source-project
  snapshot: snapshots/open-source-project.normalized.json
  required: true
projects:
  AI-tools-agents/lean-ctx:
    name: lean-ctx
    status: unused
    generated_summary: docs/generated/AI-tools-agents__lean-ctx.md
    automation:
      ai_may_update: true
```

- [ ] **Step 2: Create `scripts/lib/knowledge.mjs`**

```js
import { existsSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { writeText } from "./files.mjs";

export async function readKnowledge(filePath = "knowledge.yaml") {
  if (!existsSync(filePath)) return emptyKnowledge();
  const text = await readFile(filePath, "utf8");
  return parseKnowledge(text);
}

export function emptyKnowledge() {
  return {
    schema_version: 1,
    source_index: { adapter: "open-source-project", snapshot: "snapshots/open-source-project.normalized.json", required: true },
    projects: {},
  };
}

export function parseKnowledge(text) {
  const result = emptyKnowledge();
  const lines = text.split(/\r?\n/);
  let currentProject = null;
  for (const line of lines) {
    const projectMatch = line.match(/^  ([^:\s][^:]*):\s*$/);
    if (projectMatch && !["adapter", "snapshot", "required"].includes(projectMatch[1])) {
      currentProject = projectMatch[1];
      result.projects[currentProject] = {};
      continue;
    }
    if (currentProject) {
      const fieldMatch = line.match(/^    ([a-zA-Z_]+):\s*(.*)$/);
      if (fieldMatch) result.projects[currentProject][fieldMatch[1]] = unquote(fieldMatch[2]);
    }
  }
  return result;
}

export async function writeKnowledge(filePath, knowledge) {
  await writeText(filePath, formatKnowledge(knowledge));
}

export function formatKnowledge(knowledge) {
  const lines = [
    "schema_version: 1",
    "source_index:",
    "  adapter: open-source-project",
    "  snapshot: snapshots/open-source-project.normalized.json",
    "  required: true",
    "projects:",
  ];
  for (const key of Object.keys(knowledge.projects).sort()) {
    const project = knowledge.projects[key];
    lines.push(`  ${key}:`);
    lines.push(`    name: ${quote(project.name)}`);
    lines.push(`    status: ${project.status}`);
    if (project.generated_summary) lines.push(`    generated_summary: ${project.generated_summary}`);
    if (project.doc) lines.push(`    doc: ${project.doc}`);
  }
  return `${lines.join("\n")}\n`;
}

function unquote(value) {
  return value.replace(/^"|"$/g, "");
}

function quote(value) {
  return String(value).includes(":") ? JSON.stringify(value) : String(value);
}
```

- [ ] **Step 3: Test parser**

Run:

```powershell
node -e "import('./scripts/lib/knowledge.mjs').then(async k => { const v = await k.readKnowledge('tests/fixtures/knowledge-valid.yaml'); console.log(v.projects['AI-tools-agents/lean-ctx'].status) })"
```

Expected: prints `unused`.

## Task 5: Validate Event Schemas And Record Events

**Files:**
- Create: `scripts/lib/event-schema.mjs`
- Create: `scripts/lib/events.mjs`
- Create: `scripts/record-event.mjs`
- Create: `tests/fixtures/events-valid.jsonl`

- [ ] **Step 1: Create `scripts/lib/event-schema.mjs`**

```js
const commonFields = ["schema_version", "event_id", "project_key", "project_name", "platform", "machine_id", "host_label", "event_kind", "source_task", "source_type", "observed_at"];

const eventFields = {
  readme_summary: ["summary", "source_files", "confidence"],
  command_success: ["command", "output_summary", "observed_ref", "verification"],
  setup_failure_resolved: ["error_summary", "root_cause", "resolution", "verified_by"],
  setup_failure_open: ["error_summary", "failed_command", "next_action"],
  setup_limitation_accepted: ["limitation", "accepted_reason", "scope"],
  config_file: ["path", "summary"],
  agent_tool_success: ["tool_name", "operation", "output_summary", "observed_ref", "verification"],
  abandonment_decision: ["reason", "confirmed_by_user", "decided_at"],
};

export function validateEvent(event) {
  const errors = [];
  for (const field of commonFields) requireField(event, field, errors);
  const required = eventFields[event.event_kind];
  if (!required) errors.push(`Unsupported event_kind: ${event.event_kind}`);
  for (const field of required ?? []) requireField(event, field, errors);
  if (event.event_kind === "readme_summary" && !["low", "medium", "high"].includes(event.confidence)) errors.push("readme_summary confidence must be low, medium, or high");
  if (event.event_kind === "abandonment_decision" && event.confirmed_by_user !== true) errors.push("abandonment_decision requires confirmed_by_user: true");
  return errors;
}

export function isPromotionEligible(event) {
  if (event.event_kind === "command_success") return event.verification === true;
  if (event.event_kind === "setup_failure_resolved") return true;
  if (event.event_kind === "agent_tool_success") return event.verification === true;
  return false;
}

function requireField(event, field, errors) {
  if (event[field] === undefined || event[field] === null || event[field] === "") errors.push(`Missing ${field}`);
}
```

- [ ] **Step 2: Create `scripts/lib/events.mjs`**

```js
import { appendFile, mkdir, readFile } from "node:fs/promises";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { randomUUID } from "node:crypto";
import { repoRoot } from "./config.mjs";
import { validateEvent } from "./event-schema.mjs";

export function createEventId() {
  return randomUUID();
}

export async function appendEvent(event, fileName) {
  const errors = validateEvent(event);
  if (errors.length) throw new Error(errors.join("; "));
  const filePath = join(repoRoot(), "events", fileName);
  await mkdir(dirname(filePath), { recursive: true });
  await appendFile(filePath, `${JSON.stringify(event)}\n`, "utf8");
}

export async function readEvents(filePath) {
  if (!existsSync(filePath)) return [];
  const text = await readFile(filePath, "utf8");
  return text.split(/\r?\n/).filter(Boolean).map((line) => JSON.parse(line));
}
```

- [ ] **Step 3: Create `scripts/record-event.mjs`**

```js
import { basename } from "node:path";
import { loadLocalConfig } from "./lib/config.mjs";
import { appendEvent, createEventId } from "./lib/events.mjs";

const config = await loadLocalConfig();
const input = JSON.parse(process.argv[2] ?? "{}");
const event = {
  schema_version: 1,
  event_id: createEventId(),
  platform: config.platform,
  machine_id: config.machine_id,
  host_label: config.host,
  observed_at: new Date().toISOString(),
  ...input,
};
await appendEvent(event, `${basename(config.machine_id)}.jsonl`);
console.log(event.event_id);
```

- [ ] **Step 4: Validate event schema**

Run:

```powershell
node -e "import('./scripts/lib/event-schema.mjs').then(m => console.log(m.validateEvent({schema_version:1,event_id:'1',project_key:'AI-tools-agents/lean-ctx',project_name:'lean-ctx',platform:'windows',machine_id:'m',host_label:'h',event_kind:'command_success',source_task:'t',source_type:'command_output',observed_at:'2026-05-06T00:00:00Z',command:'lean-ctx --version',output_summary:'lean-ctx 3.4.7',observed_ref:'lean-ctx 3.4.7',verification:true}).length))"
```

Expected: prints `0`.

## Task 6: Generate Markdown And Materialize State

**Files:**
- Create: `scripts/lib/markdown.mjs`
- Create: `scripts/generate-docs.mjs`
- Create: `templates/generated.md`
- Create: `templates/using.md`
- Create: `templates/abandoned.md`

- [ ] **Step 1: Create `scripts/lib/markdown.mjs`**

```js
export function safeDocName(projectKey) {
  return `${projectKey.replaceAll("/", "__")}.md`;
}

export function renderGeneratedDoc(project) {
  return `# ${project.name}\n\n## What It Appears To Do\n${project.description || "No description was available in the source index."}\n\n## Source Metadata\n- Key: ${project.key}\n- Remote: ${project.remote_url}\n- Category: ${project.category}\n- Default branch: ${project.default_branch}\n\n## Generation Notes\n- Source type: normalized open-source-project index\n- Confidence: medium\n`;
}
```

- [ ] **Step 2: Create `scripts/generate-docs.mjs`**

```js
import { join } from "node:path";
import { readJson, writeText } from "./lib/files.mjs";
import { repoRoot } from "./lib/config.mjs";
import { readKnowledge, writeKnowledge } from "./lib/knowledge.mjs";
import { renderGeneratedDoc, safeDocName } from "./lib/markdown.mjs";

const root = repoRoot();
const snapshot = await readJson(join(root, "snapshots", "open-source-project.normalized.json"));
const knowledge = await readKnowledge(join(root, "knowledge.yaml"));

for (const project of snapshot.projects) {
  const docPath = `docs/generated/${safeDocName(project.key)}`;
  knowledge.projects[project.key] ??= { name: project.name, status: "unused", generated_summary: docPath };
  knowledge.projects[project.key].generated_summary ??= docPath;
  await writeText(join(root, docPath), renderGeneratedDoc(project));
}

await writeKnowledge(join(root, "knowledge.yaml"), knowledge);
console.log(`Generated ${snapshot.projects.length} summary doc(s)`);
```

- [ ] **Step 3: Run generate against empty snapshot**

Run:

```powershell
npm run generate
```

Expected: prints `Generated 0 summary doc(s)` until `sync:index` is run.

## Task 7: Implement Shared Verification

**Files:**
- Create: `scripts/lib/verify-core.mjs`
- Create: `scripts/verify-ci.mjs`
- Create: `scripts/verify-local.mjs`
- Create: `scripts/report-stale.mjs`

- [ ] **Step 1: Create `scripts/lib/verify-core.mjs`**

```js
import { existsSync } from "node:fs";
import { join } from "node:path";
import { readJson } from "./files.mjs";
import { readKnowledge } from "./knowledge.mjs";

export async function verifyCommittedState(root, { requireLocalConfig = false } = {}) {
  const errors = [];
  const snapshotPath = join(root, "snapshots", "open-source-project.normalized.json");
  const knowledgePath = join(root, "knowledge.yaml");
  if (!existsSync(snapshotPath)) errors.push("Missing snapshots/open-source-project.normalized.json");
  if (!existsSync(knowledgePath)) errors.push("Missing knowledge.yaml");
  if (requireLocalConfig && !existsSync(join(root, ".local", "config.json"))) errors.push("Missing .local/config.json");
  if (errors.length) return errors;

  const snapshot = await readJson(snapshotPath);
  const knowledge = await readKnowledge(knowledgePath);
  const snapshotKeys = new Set(snapshot.projects.map((project) => project.key));
  for (const [key, project] of Object.entries(knowledge.projects)) {
    if (!snapshotKeys.has(key)) errors.push(`Knowledge project not found in snapshot: ${key}`);
    if (!["using", "unused", "abandoned"].includes(project.status)) errors.push(`Invalid status for ${key}: ${project.status}`);
    if (!project.generated_summary) errors.push(`Missing generated_summary for ${key}`);
  }
  return errors;
}
```

- [ ] **Step 2: Create `scripts/verify-ci.mjs`**

```js
import { repoRoot } from "./lib/config.mjs";
import { verifyCommittedState } from "./lib/verify-core.mjs";

const errors = await verifyCommittedState(repoRoot(), { requireLocalConfig: false });
if (errors.length) {
  console.error(errors.join("\n"));
  process.exit(1);
}
console.log("verify:ci passed");
```

- [ ] **Step 3: Create `scripts/verify-local.mjs`**

```js
import { repoRoot } from "./lib/config.mjs";
import { verifyCommittedState } from "./lib/verify-core.mjs";

const errors = await verifyCommittedState(repoRoot(), { requireLocalConfig: true });
if (errors.length) {
  console.error(errors.join("\n"));
  process.exit(1);
}
console.log("verify:local passed");
```

- [ ] **Step 4: Create `scripts/report-stale.mjs`**

```js
console.log("No stale report is available in v1 until using projects are recorded.");
```

- [ ] **Step 5: Run CI verification**

Run:

```powershell
npm run verify:ci
```

Expected: prints `verify:ci passed`.

## Task 8: Add Use-Project Wrapper

**Files:**
- Create: `scripts/use-project.mjs`

- [ ] **Step 1: Create `scripts/use-project.mjs`**

```js
import { join } from "node:path";
import { repoRoot } from "./lib/config.mjs";
import { readJson } from "./lib/files.mjs";
import { readKnowledge } from "./lib/knowledge.mjs";

const projectKey = process.argv[2];
if (!projectKey) {
  console.error("Usage: npm run use-project -- <project-key>");
  process.exit(1);
}

const root = repoRoot();
const snapshot = await readJson(join(root, "snapshots", "open-source-project.normalized.json"));
const knowledge = await readKnowledge(join(root, "knowledge.yaml"));
const project = snapshot.projects.find((candidate) => candidate.key === projectKey);
if (!project) {
  console.error(`Unknown project: ${projectKey}`);
  process.exit(1);
}

const entry = knowledge.projects[projectKey];
console.log(`Project: ${project.name}`);
console.log(`Status: ${entry?.status ?? "untracked"}`);
console.log(`Read first: ${entry?.doc ?? entry?.generated_summary ?? "run npm run generate"}`);
console.log("Before reporting setup success, run: npm run record-event -- '<json>' && npm run generate && npm run verify:local -- " + projectKey);
```

- [ ] **Step 2: Smoke test wrapper**

Run after a snapshot with `AI-tools-agents/lean-ctx` exists:

```powershell
npm run use-project -- AI-tools-agents/lean-ctx
```

Expected: prints project status and the doc path to read first.

## Task 9: Add CI Workflow And Final Verification

**Files:**
- Create: `.github/workflows/verify.yml`

- [ ] **Step 1: Create workflow**

```yaml
name: verify

on:
  push:
  pull_request:
  workflow_dispatch:

jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 20
      - run: npm run verify:ci
```

- [ ] **Step 2: Run final local CI-equivalent verification**

Run:

```powershell
npm run verify:ci
```

Expected: prints `verify:ci passed`.

- [ ] **Step 3: Run status check**

Run:

```powershell
git status --short
```

Expected: shows created files. Do not commit unless the user explicitly requests a commit.

## Self-Review Checklist

- Spec coverage: repository boundary, normalized snapshot, local config, lifecycle states, event schemas, generated docs, agent wrapper, local/CI verification, and cross-device sync are covered by tasks.
- Placeholder scan: no task should include `TBD`, `TODO`, `fill in later`, or undefined scripts.
- Type consistency: project keys use `category/name`; events use `event_kind`, `project_key`, `machine_id`, and `source_type`; verification entrypoints are `verify:local` and `verify:ci`.
