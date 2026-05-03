import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import ts from 'typescript';

async function loadFiltersModule() {
  const sourcePath = path.resolve('src/lib/skills/filters.ts');
  const source = fs.readFileSync(sourcePath, 'utf8');
  const transpiled = ts.transpileModule(source, {
    compilerOptions: {
      module: ts.ModuleKind.ESNext,
      target: ts.ScriptTarget.ES2020,
    },
    fileName: sourcePath,
  });

  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'skills-filters-test-'));
  const tempFile = path.join(tempDir, 'filters.mjs');
  fs.writeFileSync(tempFile, transpiled.outputText, 'utf8');

  return import(pathToFileURL(tempFile).href);
}

test('resolveVisibleSources keeps both lists for all and narrows to one source otherwise', async () => {
  const module = await loadFiltersModule();

  assert.equal(typeof module.resolveVisibleSources, 'function');
  assert.deepEqual(module.resolveVisibleSources('all'), ['custom', 'external']);
  assert.deepEqual(module.resolveVisibleSources('custom'), ['custom']);
  assert.deepEqual(module.resolveVisibleSources('external'), ['external']);
});

test('managed mirror metadata stays additive inside the existing external bucket', async () => {
  const module = await loadFiltersModule();
  const skills = [
    {
      id: 'custom.alpha',
      name: 'Alpha',
      description: '',
      sourceType: 'custom',
      relativePath: 'custom/alpha',
    },
    {
      id: 'external.manual',
      name: 'Manual external',
      description: '',
      sourceType: 'external',
      relativePath: 'external/manual',
      managedSource: null,
    },
    {
      id: 'external.managed',
      name: 'Managed mirror',
      description: '',
      sourceType: 'external',
      relativePath: 'external/managed/github/example/codex/skill',
      managedSource: {
        kind: 'github_import',
        importId: 'import-1',
        repoUrl: 'https://github.com/example/skills',
        pinnedCommit: '0123456789abcdef',
        agentKey: 'codex',
        updateAvailable: true,
        integrity: null,
      },
    },
  ];

  assert.deepEqual(module.buildSourceSummaries(skills), [
    { key: 'all', count: 3 },
    { key: 'custom', count: 1 },
    { key: 'external', count: 2 },
  ]);
  assert.deepEqual(module.groupSkills(skills), {
    custom: [skills[0]],
    external: [skills[1], skills[2]],
  });
});

test('SkillList uses an auto-fit card grid so cards respond to container width', () => {
  const source = fs.readFileSync(path.resolve('src/components/skills/SkillList.tsx'), 'utf8');

  assert.match(source, /repeat\(auto-fit,\s*minmax\(/);
});

test("SkillList caps source windows and scrolls cards with the shared scrollbar skin", () => {
  const source = fs.readFileSync(path.resolve("src/components/skills/SkillList.tsx"), "utf8");

  assert.match(source, /max-h-\[clamp\(24rem,calc\(100vh-10rem\),46rem\)\]/);
  assert.match(source, /flex max-h-\[clamp\(24rem,calc\(100vh-10rem\),46rem\)\] flex-col overflow-hidden/);
  assert.match(source, /skill-markdown-scroll min-h-0 overflow-y-auto pr-1/);
  assert.match(source, /grid gap-3 \[grid-template-columns:repeat\(auto-fit,minmax\(18rem,1fr\)\)\]/);
});

test('SkillList renders managed/manual badges without introducing a third source bucket', () => {
  const source = fs.readFileSync(path.resolve('src/components/skills/SkillList.tsx'), 'utf8');

  assert.match(source, /t\("skills\.badges\.managedGithubMirror"\)/);
  assert.match(source, /t\("skills\.badges\.manualExternal"\)/);
  assert.match(source, /skill\.managedSource/);
  assert.doesNotMatch(source, /managed["']\s*\|/);
  assert.doesNotMatch(source, /sourceType\s*===\s*["']managed["']/);
});

test("SkillDetailPanel surfaces managed source metadata and import warning states", () => {
  const source = fs.readFileSync(path.resolve("src/components/skills/SkillDetailPanel.tsx"), "utf8");

  assert.match(source, /t\("skills\.detail\.managedSource\.title"\)/);
  assert.match(source, /t\("skills\.detail\.managedSource\.repoUrl"\)/);
  assert.match(source, /t\("skills\.detail\.managedSource\.pinnedCommit"\)/);
  assert.match(source, /t\("skills\.warnings\.variantDisappeared"/);
  assert.match(source, /managedImport\?\.upstreamVariantPath\s*\?\?\s*skill\.relativePath/);
});

test("AgentsView mounts an agent-scoped external variant panel", () => {
  const panelSource = fs.readFileSync(
    path.resolve("src/components/agents/AgentExternalVariantPanel.tsx"),
    "utf8",
  );
  const viewSource = fs.readFileSync(path.resolve("src/views/AgentsView.tsx"), "utf8");

  assert.match(panelSource, /t\("externalSources\.agentPanel\.title"\)/);
  assert.match(panelSource, /onImportVariant/);
  assert.match(panelSource, /onUpdateImport/);
  assert.match(viewSource, /AgentExternalVariantPanel/);
  assert.match(viewSource, /externalSources/);
  assert.match(viewSource, /importExternalVariant/);
  assert.match(viewSource, /updateExternalImport/);
});

test("AgentExternalVariantPanel keeps repair busy state independent from update availability", () => {
  const source = fs.readFileSync(
    path.resolve("src/components/agents/AgentExternalVariantPanel.tsx"),
    "utf8",
  );

  assert.match(source, /const isBusyImport = updatingExternalImportId === item\.importId;/);
  assert.match(source, /const \[busyImportAction, setBusyImportAction\] = useState/);
  assert.match(source, /const isUpdatingImport =/);
  assert.match(source, /busyImportAction\.action === "update"/);
  assert.match(source, /const isRepairingImport =/);
  assert.match(source, /busyImportAction\.action === "repair"/);
  assert.match(source, /isRepairingImport\s*\?\s*t\("sources\.imports\.repairing"\)/);
});

test("Managed source i18n keys exist in both locales", () => {
  const en = JSON.parse(fs.readFileSync(path.resolve("src/i18n/en.json"), "utf8"));
  const zh = JSON.parse(fs.readFileSync(path.resolve("src/i18n/zh.json"), "utf8"));
  const requiredKeys = [
    "externalSources.agentPanel.title",
    "externalSources.agentPanel.description",
    "externalSources.agentPanel.empty",
    "externalSources.agentPanel.import",
    "externalSources.agentPanel.imported",
    "externalSources.agentPanel.update",
    "externalSources.agentPanel.updating",
    "externalSources.agentPanel.upToDate",
    "skills.badges.managedGithubMirror",
    "skills.badges.manualExternal",
    "skills.detail.managedSource.title",
    "skills.detail.managedSource.repoUrl",
    "skills.detail.managedSource.pinnedCommit",
    "skills.detail.managedSource.updateAvailable",
    "skills.detail.managedSource.integrityMismatch",
    "skills.warnings.variantDisappeared",
  ];

  for (const key of requiredKeys) {
    assert.ok(en[key], `missing en key: ${key}`);
    assert.ok(zh[key], `missing zh key: ${key}`);
  }
});
