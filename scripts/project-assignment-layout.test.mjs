import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

const identityPanelPath = path.resolve("src/components/projects/ProjectIdentityPanel.tsx");
const editorPath = path.resolve("src/components/projects/ProjectAssignmentEditor.tsx");
const workbenchPath = path.resolve("src/components/projects/ProjectLayerWorkbench.tsx");
const summaryPath = path.resolve("src/components/projects/ProjectAssignmentSummary.tsx");
const projectsViewPath = path.resolve("src/views/ProjectsView.tsx");
const projectCardPath = path.resolve("src/components/projects/ProjectCard.tsx");

test("project assignment layout keeps the save action beside the display name field", () => {
  const source = fs.readFileSync(identityPanelPath, "utf8");

  assert.match(source, /onSave: \(\) => void;/);
  assert.match(source, /saveLabel: string;/);
  assert.match(source, /isSaving: boolean;/);
  assert.match(source, /canSave: boolean;/);
  assert.match(source, /displayName.*saveLabel/s);
  assert.match(source, /disabled=\{!props\.canSave \|\| props\.isSaving\}/);
  assert.match(source, /props\.isSaving \? t\("projects\.summary\.saving"\) : props\.saveLabel/);
  assert.match(source, /min-h-\[3rem\]/);
  assert.doesNotMatch(source, /min-h-\[3\.35rem\]/);
});

test("project assignment editor caps panel height and uses the shared markdown scroll surface", () => {
  const source = fs.readFileSync(editorPath, "utf8");

  assert.match(source, /max-h-\[clamp\([^,]+,calc\(100vh-[^,]+,[^\]]+\)\]/);
  assert.match(source, /skill-markdown-scroll min-h-0 flex-1 space-y-1 overflow-y-auto px-4 py-3/);
  assert.match(source, /overflow-hidden rounded-2xl border border-slate-800 bg-slate-950\/60/);
  assert.match(source, /\[-webkit-line-clamp:2\]/);
  assert.match(source, /min-\[1380px\]:grid-cols-\[minmax\(14rem,4fr\)_minmax\(0,8fr\)\]/);
  assert.match(source, /projects\.editor\.allPaths/);
  assert.match(source, /mb-3 flex flex-wrap items-center gap-2 text-\[11px\] text-slate-400[\s\S]*projects\.editor\.allPaths[\s\S]*projects\.editor\.allSkills/);
  assert.doesNotMatch(source, /mt-2 flex flex-wrap gap-2 text-\[11px\] text-slate-400/);
  assert.match(source, /projects\.editor\.allAgents/);
  assert.match(source, /projectSkillsDirRule/);
  assert.match(source, /skill\.relativePath/);
});

test("project assignment editor edits one selected project agent layer at a time", () => {
  const source = fs.readFileSync(editorPath, "utf8");

  assert.match(source, /selectedAgentKey: string \| null/);
  assert.match(source, /activeAgentDraft/);
  assert.match(source, /projects\.editor\.agentLayerTitle/);
  assert.match(source, /projects\.editor\.projectScenesTitle/);
  assert.match(source, /projects\.editor\.exclusionsTitle/);
  assert.match(source, /onSelectAgent\(agentKey: string\)/);
  assert.match(source, /props\.onSelectAgent\(agent\.key\)/);
  assert.match(source, /onToggleProjectScene\(scene\.id\)/);
  assert.match(source, /onToggle=\{props\.onToggleProjectExclusion\}/);
});

test("project assignment summary stays scrollable but no longer owns the save button", () => {
  const source = fs.readFileSync(summaryPath, "utf8");

  assert.match(source, /max-h-\[clamp\([^,]+,calc\(100vh-[^,]+,[^\]]+\)\]/);
  assert.match(source, /min-\[1380px\]:absolute min-\[1380px\]:inset-0 min-\[1380px\]:max-h-none/);
  assert.match(source, /min-\[1380px\]:max-h-none/);
  assert.match(source, /skill-markdown-scroll flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto px-4 py-4 text-sm/);
  assert.match(source, /props\.agentSummaries\.map/);
  assert.doesNotMatch(source, /saveLabel/);
  assert.doesNotMatch(source, /onSave/);
  assert.doesNotMatch(source, /t\("projects\.summary\.saving"\)/);
});

test("project assignment summary mirrors selected skills and agents before target details", () => {
  const source = fs.readFileSync(summaryPath, "utf8");

  assert.match(source, /agentSummaries: ProjectAgentSummary\[\]/);
  assert.match(source, /summary\.inheritedGlobalSkillIds\.length/);
  assert.match(source, /summary\.projectDirectSkillIds\.length/);
  assert.match(source, /summary\.excludedSkillIds\.length/);
  assert.match(source, /projects\.summary\.agentPreviewTitle/);
  assert.match(source, /projects\.summary\.inheritedGlobal/);
  assert.match(source, /projects\.summary\.selectedAgentsTitle/);
  assert.match(source, /summary\.previewItems\.map/);
  assert.match(source, /item\.isExcludedByProject/);
});

test("projects view forwards save props to the identity panel instead of the summary panel", () => {
  const source = fs.readFileSync(workbenchPath, "utf8");
  const viewSource = fs.readFileSync(projectsViewPath, "utf8");

  assert.match(source, /<ProjectIdentityPanel[\s\S]*canSave=\{!props\.duplicatePath && props\.draft\.projectPath\.trim\(\)\.length > 0\}/);
  assert.match(source, /<ProjectIdentityPanel[\s\S]*isSaving=\{props\.isSaving\}/);
  assert.match(source, /<ProjectIdentityPanel[\s\S]*onSave=\{props\.onSave\}/);
  assert.match(viewSource, /<ProjectLayerWorkbench[\s\S]*onSave=\{\(\) => void handleSaveDraft\(\)\}/);
  assert.match(source, /<ProjectAssignmentSummary[\s\S]*title=/);
  assert.doesNotMatch(source, /<ProjectAssignmentSummary[\s\S]*saveLabel=/);
  assert.doesNotMatch(source, /<ProjectAssignmentSummary[\s\S]*onSave=/);
  assert.doesNotMatch(source, /<ProjectAssignmentSummary[\s\S]*isSaving=/);
});

test("projects view keeps the right summary as a natural sticky inspector", () => {
  const source = fs.readFileSync(projectsViewPath, "utf8");
  const workbenchSource = fs.readFileSync(workbenchPath, "utf8");

  assert.match(source, /scanResult\.skills,/);
  assert.match(workbenchSource, /agentSummaries=\{props\.summary\.agentSummaries\}/);
  assert.match(source, /sortProjectAgentsForEditor/);
  assert.match(source, /skillPathFilter/);
  assert.match(source, /agentStatusFilter/);
  assert.match(workbenchSource, /min-\[1380px\]:sticky min-\[1380px\]:top-8 min-\[1380px\]:relative min-\[1380px\]:self-stretch/);
  assert.match(fs.readFileSync(summaryPath, "utf8"), /min-\[1380px\]:absolute min-\[1380px\]:inset-0/);
});

test("project cards surface apply freshness per project agent", () => {
  const source = fs.readFileSync(projectCardPath, "utf8");
  const enSource = fs.readFileSync(path.resolve("src/i18n/en.json"), "utf8");
  const zhSource = fs.readFileSync(path.resolve("src/i18n/zh.json"), "utf8");

  assert.match(source, /project\.applyStatuses\?\.\[agentKey\]\?\.applyStatus/);
  assert.match(source, /projects\.card\.applyStatus\.\$\{status\}/);
  assert.match(source, /neverApplied/);
  assert.match(source, /unsupported/);
  assert.match(enSource, /projects\.card\.applyStatus\.stale/);
  assert.match(zhSource, /projects\.card\.applyStatus\.neverApplied/);
});
