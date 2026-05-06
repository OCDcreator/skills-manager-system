import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

function readSource(relativePath) {
  return fs.readFileSync(path.resolve(relativePath), "utf8");
}

test("shared scroll memory hook persists and retries restoration", () => {
  const source = readSource("src/lib/scroll-memory.ts");

  assert.match(source, /useRememberedScrollPosition/);
  assert.match(source, /localStorage/);
  assert.match(source, /requestAnimationFrame/);
  assert.match(source, /scrollTop/);
});

test("skill browser panes opt into remembered scroll positions", () => {
  const listSource = readSource("src/components/skills/SkillList.tsx");
  const detailSource = readSource("src/components/skills/SkillDetailPanel.tsx");

  assert.match(listSource, /useRememberedScrollPosition/);
  assert.match(listSource, /skills:list:/);
  assert.match(detailSource, /useRememberedScrollPosition/);
  assert.match(detailSource, /skills:detail:/);
});

test("agent panels opt into remembered scroll positions", () => {
  const globalListSource = readSource("src/components/agents/AgentGlobalSkillList.tsx");
  const orderModalSource = readSource("src/components/agents/AgentOrderModal.tsx");
  const sceneSelectorSource = readSource("src/components/agents/AgentSceneSelector.tsx");

  assert.match(globalListSource, /useRememberedScrollPosition/);
  assert.match(globalListSource, /agents:global-skills:/);
  assert.match(orderModalSource, /useRememberedScrollPosition/);
  assert.match(orderModalSource, /agents:order-modal/);
  assert.match(sceneSelectorSource, /useRememberedScrollPosition/);
  assert.match(sceneSelectorSource, /agents:scene-selector:/);
});

test("projects, git, and scene subpanels opt into remembered scroll positions", () => {
  const projectSummarySource = readSource("src/components/projects/ProjectAssignmentSummary.tsx");
  const projectEditorSource = readSource("src/components/projects/ProjectAssignmentEditor.tsx");
  const gitFileListSource = readSource("src/components/git/GitFileList.tsx");
  const gitDiffSource = readSource("src/components/git/GitDiffViewer.tsx");
  const sceneSkillChooserSource = readSource("src/components/scenes/SceneSkillChooser.tsx");

  assert.match(projectSummarySource, /useRememberedScrollPosition/);
  assert.match(projectSummarySource, /projects:summary:/);
  assert.match(projectEditorSource, /projects:editor:/);
  assert.match(gitFileListSource, /useRememberedScrollPosition/);
  assert.match(gitFileListSource, /git:file-list/);
  assert.match(gitDiffSource, /useRememberedScrollPosition/);
  assert.match(gitDiffSource, /git:diff:/);
  assert.match(sceneSkillChooserSource, /useRememberedScrollPosition/);
  assert.match(sceneSkillChooserSource, /scenes:card:/);
});
