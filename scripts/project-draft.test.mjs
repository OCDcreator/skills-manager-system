import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

const sourcePath = path.resolve("src/lib/project-draft.ts");
const summaryPath = path.resolve("src/lib/project-summary.ts");
const identityPanelPath = path.resolve("src/components/projects/ProjectIdentityPanel.tsx");
const projectsViewPath = path.resolve("src/views/ProjectsView.tsx");

test("project draft helper exports the expected pure helpers", () => {
  const source = fs.readFileSync(sourcePath, "utf8");
  const summarySource = fs.readFileSync(summaryPath, "utf8");

  assert.match(source, /export interface ProjectDraft/);
  assert.match(source, /export function suggestProjectDisplayName/);
  assert.match(source, /export function isProjectDraftDirty/);
  assert.match(source, /export function filterProjectSkills/);
  assert.match(source, /export function filterProjectAgents/);
  assert.match(source, /export function sortProjectAgentsForEditor/);
  assert.match(source, /export function applyProjectPathToDraft/);
  assert.match(source, /export function applyProjectDisplayNameToDraft/);
  assert.match(summarySource, /export function buildProjectSummary/);
  assert.match(source, /projectDraftFromAssignment/);
  assert.match(source, /projectDraftToAgentAssignments/);
  assert.match(summarySource, /agentSummaries/);
});

test("project draft helper tracks sourceProjectPath and unsupportedAgentKeys", () => {
  const source = fs.readFileSync(sourcePath, "utf8");

  assert.match(source, /sourceProjectPath: string \| null/);
  assert.match(source, /displayNameManuallyEdited: boolean/);
  assert.match(source, /agents: Record<string, ProjectAgentDraft>/);
  assert.match(source, /unsupportedAgentKeys: string\[\]/);
  assert.match(source, /mode: "create" \| "edit"/);
});

test("project draft stores selected scenes per agent", () => {
  const source = fs.readFileSync(sourcePath, "utf8");

  assert.match(source, /export interface ProjectAgentDraft/);
  assert.match(source, /selectedSceneIds: string\[\]/);
  assert.match(source, /export function ensureProjectAgentDraft/);
  assert.match(source, /export function toggleProjectAgentScene/);
  assert.match(source, /agents: \{\s*\.\.\.ensured\.agents,\s*\[agentKey\]: \{/s);
});

test("project summary marks inherited global skills and local exclusions", () => {
  const source = fs.readFileSync(summaryPath, "utf8");

  assert.match(source, /inheritedGlobalSkillIds/);
  assert.match(source, /projectDirectSkillIds/);
  assert.match(source, /projectSceneNames/);
  assert.match(source, /isExcludedByProject/);
  assert.match(source, /resolveInheritedGlobalSkillIds/);
  assert.match(source, /new Set\(agent\.excludedSkillIds\)/);
  assert.match(source, /const excluded = new Set\(agentDraft\.excludedSkillIds\)/);
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

test("project edit flow exposes a cancel action that resets the draft", () => {
  const identityPanelSource = fs.readFileSync(identityPanelPath, "utf8");
  const projectsViewSource = fs.readFileSync(projectsViewPath, "utf8");

  assert.match(identityPanelSource, /onCancelEdit: \(\) => void/);
  assert.match(identityPanelSource, /props\.mode === "edit" \?/);
  assert.match(identityPanelSource, /t\("projects\.identity\.cancelEdit"\)/);
  assert.match(projectsViewSource, /const handleCancelEdit = \(\) => \{/);
  assert.match(projectsViewSource, /setDraft\(emptyDraft\)/);
  assert.match(projectsViewSource, /onCancelEdit=\{handleCancelEdit\}/);
});

test("project identity panel gives the project path a dedicated hero row", () => {
  const identityPanelSource = fs.readFileSync(identityPanelPath, "utf8");

  assert.match(identityPanelSource, /import \{ FolderSearch \} from "lucide-react"/);
  assert.match(identityPanelSource, /className="space-y-5"/);
  assert.match(identityPanelSource, /className="group flex h-12 items-center gap-2 rounded-\[1\.2rem\]/);
  assert.match(identityPanelSource, /className="mt-1 flex flex-col gap-3/);
  assert.match(identityPanelSource, /className="block min-w-0 flex-1 space-y-3 min-\[720px\]:max-w-xl/);
  assert.match(identityPanelSource, /<FolderSearch className="size-4"/);
  assert.match(identityPanelSource, /className="group relative h-12 rounded-\[1\.1rem\] border border-slate-700\/80 bg-slate-950\/90 p-1/);
});

test("project identity panel moves helper chips inline until a display name is typed", () => {
  const identityPanelSource = fs.readFileSync(identityPanelPath, "utf8");

  assert.match(identityPanelSource, /const showDisplayInlineHint = !props\.displayName\.trim\(\)/);
  assert.match(identityPanelSource, /showDisplayInlineHint \? \(/);
  assert.match(identityPanelSource, /justify-start/);
  assert.match(identityPanelSource, /t\("projects\.identity\.suggested"\)/);
  assert.match(identityPanelSource, /readOnlyPath \? \(/);
  assert.doesNotMatch(identityPanelSource, /justify-end/);
  assert.doesNotMatch(identityPanelSource, /rounded-full border border-slate-700 px-3 py-1">\s*\{t\("projects\.identity\.suggested"\)/);
  assert.doesNotMatch(identityPanelSource, /rounded-full border border-slate-700 px-3 py-1">\s*\{t\("projects\.identity\.readOnlyPath"\)/);
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
  assert.match(helperSource, /displayNameManuallyEdited: true/);
});

test("project draft filters skills by path and agents by enabled state", () => {
  const helperSource = fs.readFileSync(sourcePath, "utf8");

  assert.match(helperSource, /matchesSkillPathFilter/);
  assert.match(helperSource, /skill\.relativePath/);
  assert.match(helperSource, /projectSkillsDirRule/);
  assert.match(helperSource, /statusFilter === "enabled"/);
  assert.match(helperSource, /statusFilter === "disabled"/);
});

test("project draft can sort selected agents ahead of unselected ones while editing", () => {
  const helperSource = fs.readFileSync(sourcePath, "utf8");

  assert.match(helperSource, /selectedAgentKeys/);
  assert.match(helperSource, /selected\.has\(left\.key\)/);
  assert.match(helperSource, /prioritizeSelected/);
});
