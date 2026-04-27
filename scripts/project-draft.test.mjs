import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

const sourcePath = path.resolve("src/lib/project-draft.ts");

test("project draft helper exports the expected pure helpers", () => {
  const source = fs.readFileSync(sourcePath, "utf8");

  assert.match(source, /export interface ProjectDraft/);
  assert.match(source, /export function suggestProjectDisplayName/);
  assert.match(source, /export function isProjectDraftDirty/);
  assert.match(source, /export function filterProjectSkills/);
  assert.match(source, /export function filterProjectAgents/);
  assert.match(source, /export function buildProjectSummary/);
});

test("project draft helper tracks sourceProjectPath and unsupportedAgentKeys", () => {
  const source = fs.readFileSync(sourcePath, "utf8");

  assert.match(source, /sourceProjectPath: string \| null/);
  assert.match(source, /unsupportedAgentKeys: string\[\]/);
  assert.match(source, /mode: "create" \| "edit"/);
});
