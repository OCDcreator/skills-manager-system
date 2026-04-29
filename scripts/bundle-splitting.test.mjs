import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

function read(relativePath) {
  return fs.readFileSync(path.resolve(relativePath), "utf8");
}

test("App lazy-loads non-default top-level views behind Suspense", () => {
  const source = read("src/App.tsx");

  assert.match(source, /lazy\(\(\)\s*=>\s*import\("\.\/views\/AgentsView"\)/);
  assert.match(source, /lazy\(\(\)\s*=>\s*import\("\.\/views\/ExternalSourcesView"\)/);
  assert.match(source, /lazy\(\(\)\s*=>\s*import\("\.\/views\/GitView"\)/);
  assert.match(source, /lazy\(\(\)\s*=>\s*import\("\.\/views\/ProjectsView"\)/);
  assert.match(source, /lazy\(\(\)\s*=>\s*import\("\.\/views\/ScenesView"\)/);
  assert.match(source, /lazy\(\(\)\s*=>\s*import\("\.\/views\/SettingsView"\)/);
  assert.match(source, /<Suspense fallback=/);
  assert.doesNotMatch(source, /import \{ AgentsView \} from "\.\/views\/AgentsView"/);
});

test("SkillsView lazy-loads SkillDetailPanel only after a skill is selected", () => {
  const source = read("src/views/SkillsView.tsx");

  assert.match(source, /lazy\(\(\)\s*=>\s*import\("\.\.\/components\/skills\/SkillDetailPanel"\)/);
  assert.match(source, /selectedSkill \? \(/);
  assert.match(source, /<Suspense fallback=/);
  assert.doesNotMatch(source, /import \{ SkillDetailPanel \} from "\.\.\/components\/skills\/SkillDetailPanel"/);
});

test("ProjectAssistantLauncher lazy-loads the assistant panel when opened", () => {
  const source = read("src/components/assistant/ProjectAssistantLauncher.tsx");

  assert.match(source, /lazy\(\(\)\s*=>\s*import\("\.\/ProjectAssistantPanel"\)/);
  assert.match(source, /isOpen \? \(/);
  assert.match(source, /<Suspense\s+fallback=/);
  assert.doesNotMatch(source, /import \{ ProjectAssistantPanel \} from "\.\/ProjectAssistantPanel"/);
});

test("vite config extracts markdown dependencies into a focused vendor chunk", () => {
  const source = read("vite.config.ts");

  assert.match(source, /manualChunks/);
  assert.match(source, /vendor-markdown/);
  assert.match(source, /highlight\.js/);
  assert.match(source, /marked/);
});

test("markdown renderers use the common highlight.js entry instead of the full bundle", () => {
  const assistantSource = read("src/components/assistant/AssistantMarkdown.tsx");
  const detailSource = read("src/components/skills/SkillDetailPanel.tsx");

  assert.match(assistantSource, /from "highlight\.js\/lib\/common"/);
  assert.match(detailSource, /from "highlight\.js\/lib\/common"/);
  assert.doesNotMatch(assistantSource, /from "highlight\.js"/);
  assert.doesNotMatch(detailSource, /from "highlight\.js"/);
});
