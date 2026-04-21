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

test('SkillList uses an auto-fit card grid so cards respond to container width', () => {
  const source = fs.readFileSync(path.resolve('src/components/skills/SkillList.tsx'), 'utf8');

  assert.match(source, /repeat\(auto-fit,\s*minmax\(/);
});
