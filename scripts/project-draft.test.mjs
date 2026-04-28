import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

const sourcePath = path.resolve("src/lib/project-draft.ts");
const identityPanelPath = path.resolve("src/components/projects/ProjectIdentityPanel.tsx");
const projectsViewPath = path.resolve("src/views/ProjectsView.tsx");

test("project draft helper exports the expected pure helpers", () => {
  const source = fs.readFileSync(sourcePath, "utf8");

  assert.match(source, /export interface ProjectDraft/);
  assert.match(source, /export function suggestProjectDisplayName/);
  assert.match(source, /export function isProjectDraftDirty/);
  assert.match(source, /export function filterProjectSkills/);
  assert.match(source, /export function filterProjectAgents/);
  assert.match(source, /export function applyProjectPathToDraft/);
  assert.match(source, /export function applyProjectDisplayNameToDraft/);
  assert.match(source, /export function buildProjectSummary/);
});

test("project draft helper tracks sourceProjectPath and unsupportedAgentKeys", () => {
  const source = fs.readFileSync(sourcePath, "utf8");

  assert.match(source, /sourceProjectPath: string \| null/);
  assert.match(source, /displayNameManuallyEdited: boolean/);
  assert.match(source, /unsupportedAgentKeys: string\[\]/);
  assert.match(source, /mode: "create" \| "edit"/);
});

test("project creation flow exposes a folder picker for project paths", () => {
  const identityPanelSource = fs.readFileSync(identityPanelPath, "utf8");
  const projectsViewSource = fs.readFileSync(projectsViewPath, "utf8");

  assert.match(identityPanelSource, /onBrowseProjectPath: \(\) => void/);
  assert.match(identityPanelSource, /canBrowseProjectPath: boolean/);
  assert.match(identityPanelSource, /t\("projects\.identity\.browse"\)/);
  assert.match(projectsViewSource, /import \{ open \} from "@tauri-apps\/plugin-dialog"/);
  assert.match(projectsViewSource, /directory: true/);
  assert.match(projectsViewSource, /onBrowseProjectPath=\{\(\) => void handleBrowseProjectPath\(\)\}/);
});

test("project identity panel gives the project path a dedicated hero row", () => {
  const identityPanelSource = fs.readFileSync(identityPanelPath, "utf8");

  assert.match(identityPanelSource, /import \{ FolderSearch \} from "lucide-react"/);
  assert.match(identityPanelSource, /className="space-y-5"/);
  assert.match(identityPanelSource, /className="group flex items-center gap-2 rounded-\[1\.35rem\]/);
  assert.match(identityPanelSource, /className="mt-1 block max-w-xl/);
  assert.match(identityPanelSource, /<FolderSearch className="size-4"/);
  assert.match(identityPanelSource, /className="group rounded-\[1\.2rem\] border border-slate-700\/80 bg-slate-950\/90 p-2/);
});

test("project draft auto-fills display names until the user edits them", () => {
  const helperSource = fs.readFileSync(sourcePath, "utf8");
  const viewSource = fs.readFileSync(projectsViewPath, "utf8");

  assert.match(viewSource, /displayNameManuallyEdited: false/);
  assert.match(
    helperSource,
    /displayName: draft\.displayNameManuallyEdited\s*\?\s*draft\.displayName\s*:\s*suggestProjectDisplayName\(projectPath\)/,
  );
  assert.match(viewSource, /applyProjectPathToDraft/);
  assert.match(viewSource, /applyProjectDisplayNameToDraft/);
  assert.match(helperSource, /displayNameManuallyEdited: true/);
  assert.match(viewSource, /displayNameManuallyEdited: true/);
});
