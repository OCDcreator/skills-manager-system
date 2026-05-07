import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

const identityPanelPath = path.resolve("src/components/projects/ProjectIdentityPanel.tsx");
const editorPath = path.resolve("src/components/projects/ProjectAssignmentEditor.tsx");
const filterToolbarPath = path.resolve("src/components/projects/ProjectSkillFilterToolbar.tsx");
const workbenchPath = path.resolve("src/components/projects/ProjectLayerWorkbench.tsx");
const summaryPath = path.resolve("src/components/projects/ProjectAssignmentSummary.tsx");
const targetSkillListPath = path.resolve("src/components/projects/ProjectTargetSkillList.tsx");
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
  const toolbarSource = fs.readFileSync(filterToolbarPath, "utf8");

  assert.match(source, /max-h-\[clamp\([^,]+,calc\(100vh-[^,]+,[^\]]+\)\]/);
  assert.match(source, /skill-markdown-scroll grid min-h-0 flex-1 grid-cols-\[repeat\(auto-fill,minmax\(15rem,1fr\)\)\]/);
  assert.match(source, /content-start gap-x-3 gap-y-1 overflow-y-auto px-3 py-2 pr-4/);
  assert.match(source, /flex h-10 min-w-0 items-center gap-2 rounded-md border px-2 text-xs/);
  assert.match(source, /props\.variant === "primary" \? "overflow-visible" : "overflow-hidden"/);
  assert.doesNotMatch(source, /\[-webkit-line-clamp:2\]/);
  assert.match(source, /skill-markdown-scroll flex gap-2 overflow-x-auto px-3 py-3/);
  assert.match(source, /min-\[1120px\]:grid-cols-\[minmax\(0,1\.45fr\)_minmax\(18rem,0\.75fr\)\]/);
  assert.match(source, /variant="primary"/);
  assert.match(source, /variant="secondary"/);
  assert.match(toolbarSource, /projects\.editor\.allPaths/);
  assert.match(toolbarSource, /mt-3 flex flex-wrap items-center gap-2 text-\[11px\] text-slate-400[\s\S]*projects\.editor\.allPaths[\s\S]*projects\.editor\.allSkills/);
  assert.doesNotMatch(source, /mt-2 flex flex-wrap gap-2 text-\[11px\] text-slate-400/);
  assert.match(source, /projects\.editor\.allAgents/);
  assert.match(source, /projects\.editor\.searchAgents/);
  assert.match(toolbarSource, /projects\.editor\.searchSkills/);
  assert.match(source, /projectSkillsDirRule/);
  assert.match(source, /skill\.relativePath/);
});

test("project assignment editor edits one selected project agent layer at a time", () => {
  const source = fs.readFileSync(editorPath, "utf8");

  assert.match(source, /selectedAgentKey: string \| null/);
  assert.match(source, /activeAgentDraft/);
  assert.match(source, /projects\.editor\.agentLayerTitle/);
  assert.match(source, /projects\.editor\.projectScenesTitle/);
  assert.doesNotMatch(source, /projects\.editor\.exclusionsTitle/);
  assert.match(source, /onSelectAgent\(agentKey: string\)/);
  assert.match(source, /onSelect=\{props\.onSelectAgent\}/);
  assert.match(source, /props\.onSelect\(props\.agent\.key\)/);
  assert.match(source, /onToggleProjectScene\(scene\.id\)/);
  assert.doesNotMatch(source, /onToggleProjectExclusion/);
  assert.doesNotMatch(source, /excludedSkillIds/);
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
  assert.doesNotMatch(source, /summary\.excludedSkillIds\.length/);
  assert.match(source, /projects\.summary\.agentPreviewTitle/);
  assert.match(source, /projects\.summary\.inheritedGlobal/);
  assert.match(source, /projects\.summary\.selectedAgentsTitle/);
  assert.match(source, /summary\.previewItems\.map/);
  assert.doesNotMatch(source, /item\.isExcludedByProject/);
});

test("project assignment summary shows collapsible existing project target skills with scoped management", () => {
  const source = fs.readFileSync(summaryPath, "utf8");
  const targetSkillListSource = fs.readFileSync(targetSkillListPath, "utf8");
  const summarySource = fs.readFileSync(path.resolve("src/lib/project-summary.ts"), "utf8");
  const projectsApiSource = fs.readFileSync(path.resolve("src/lib/projects.ts"), "utf8");
  const workbenchSource = fs.readFileSync(workbenchPath, "utf8");
  const enSource = fs.readFileSync(path.resolve("src/i18n/en.json"), "utf8");
  const zhSource = fs.readFileSync(path.resolve("src/i18n/zh.json"), "utf8");

  assert.match(projectsApiSource, /targetSkillEntries: AgentTargetSkillEntry\[\]/);
  assert.match(projectsApiSource, /targetSkillScanError: string \| null/);
  assert.match(summarySource, /targetSkillEntries: AgentTargetSkillEntry\[\]/);
  assert.match(source, /summary\.targetSkillEntries/);
  assert.match(source, /ProjectTargetSkillList/);
  assert.match(targetSkillListSource, /projects\.summary\.existingTargetSkills/);
  assert.match(targetSkillListSource, /props\.entries\.length/);
  assert.match(targetSkillListSource, /entry\.managed/);
  assert.match(targetSkillListSource, /entry\.hasSkillDocument/);
  assert.match(targetSkillListSource, /projects\.summary\.managed/);
  assert.match(targetSkillListSource, /projects\.summary\.unmanaged/);
  assert.match(targetSkillListSource, /projects\.summary\.noSkillDocument/);
  assert.match(source, /expandedTargetSkillGroups/);
  assert.match(targetSkillListSource, /aria-expanded=\{props\.isExpanded\}/);
  assert.match(targetSkillListSource, /max-h-64/);
  assert.match(source, /onDeleteTargetSkill/);
  assert.match(targetSkillListSource, /projects\.summary\.cancelSelection/);
  assert.match(targetSkillListSource, /projects\.summary\.deleteTargetSkill/);
  assert.match(workbenchSource, /onDeleteTargetSkill=\{props\.onDeleteTargetSkill\}/);
  assert.match(enSource, /projects\.summary\.existingTargetSkills/);
  assert.match(zhSource, /projects\.summary\.existingTargetSkills/);
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

test("project skill external filter uses the scene-style group popover", () => {
  const toolbarSource = fs.readFileSync(filterToolbarPath, "utf8");
  const viewSource = fs.readFileSync(projectsViewPath, "utf8");
  const filtersSource = fs.readFileSync(path.resolve("src/lib/project-filters.ts"), "utf8");
  const editorSource = fs.readFileSync(editorPath, "utf8");

  assert.match(viewSource, /buildExternalGroupSummaries/);
  assert.match(viewSource, /externalGroupFilter/);
  assert.match(viewSource, /setExternalGroupFilter\("all"\)/);
  assert.match(filtersSource, /getExternalGroupKey/);
  assert.match(toolbarSource, /isExternalGroupOpen/);
  assert.match(toolbarSource, /selectedExternalGroup/);
  assert.match(toolbarSource, /absolute bottom-full left-0 z-50 mb-2/);
  assert.match(toolbarSource, /externalGroupSummaries\.map/);
  assert.match(editorSource, /props\.variant === "primary"[\s\S]*overflow-visible/);
  assert.doesNotMatch(viewSource, /includeExternalSubpaths/);
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
