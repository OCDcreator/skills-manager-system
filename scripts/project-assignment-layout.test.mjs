import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

const identityPanelPath = path.resolve("src/components/projects/ProjectIdentityPanel.tsx");
const editorPath = path.resolve("src/components/projects/ProjectAssignmentEditor.tsx");
const summaryPath = path.resolve("src/components/projects/ProjectAssignmentSummary.tsx");
const projectsViewPath = path.resolve("src/views/ProjectsView.tsx");

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
  assert.match(source, /min-\[1380px\]:grid-cols-\[minmax\(0,7fr\)_minmax\(18rem,5fr\)\]/);
});

test("project assignment summary stays scrollable but no longer owns the save button", () => {
  const source = fs.readFileSync(summaryPath, "utf8");

  assert.match(source, /max-h-\[clamp\([^,]+,calc\(100vh-[^,]+,[^\]]+\)\]/);
  assert.match(source, /skill-markdown-scroll min-h-0 flex-1 space-y-4 overflow-y-auto px-4 py-4 text-sm/);
  assert.doesNotMatch(source, /saveLabel/);
  assert.doesNotMatch(source, /onSave/);
  assert.doesNotMatch(source, /t\("projects\.summary\.saving"\)/);
});

test("project assignment summary mirrors selected skills and agents before target details", () => {
  const source = fs.readFileSync(summaryPath, "utf8");

  assert.match(source, /selectedSkills: Array<\{/);
  assert.match(source, /selectedAgents: Array<\{/);
  assert.match(source, /projects\.summary\.selectedSkillsTitle/);
  assert.match(source, /projects\.summary\.selectedAgentsTitle/);
  assert.match(source, /props\.selectedSkills\.map/);
  assert.match(source, /props\.selectedAgents\.map/);
});

test("projects view forwards save props to the identity panel instead of the summary panel", () => {
  const source = fs.readFileSync(projectsViewPath, "utf8");

  assert.match(source, /<ProjectIdentityPanel[\s\S]*canSave=\{!duplicatePath && draft\.projectPath\.trim\(\)\.length > 0\}/);
  assert.match(source, /<ProjectIdentityPanel[\s\S]*isSaving=\{isSavingDraft\}/);
  assert.match(source, /<ProjectIdentityPanel[\s\S]*onSave=\{\(\) => void handleSaveDraft\(\)\}/);
  assert.match(source, /<ProjectAssignmentSummary[\s\S]*title=/);
  assert.doesNotMatch(source, /<ProjectAssignmentSummary[\s\S]*saveLabel=/);
  assert.doesNotMatch(source, /<ProjectAssignmentSummary[\s\S]*onSave=/);
  assert.doesNotMatch(source, /<ProjectAssignmentSummary[\s\S]*isSaving=/);
});

test("projects view keeps the right summary as a natural sticky inspector", () => {
  const source = fs.readFileSync(projectsViewPath, "utf8");

  assert.match(source, /scanResult\.skills,/);
  assert.match(source, /selectedSkills=\{summary\.selectedSkills\}/);
  assert.match(source, /selectedAgents=\{summary\.selectedAgents\}/);
  assert.match(source, /min-\[1380px\]:sticky min-\[1380px\]:top-8 min-\[1380px\]:self-start/);
  assert.doesNotMatch(source, /min-\[1380px\]:absolute min-\[1380px\]:inset-0/);
});
