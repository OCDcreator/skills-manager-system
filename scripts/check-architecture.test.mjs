import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

import { runArchitectureCheck } from './check-architecture.mjs';

function createWorkspace(files) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'skills-manager-arch-'));

  for (const [relativePath, contents] of Object.entries(files)) {
    const fullPath = path.join(root, relativePath);
    fs.mkdirSync(path.dirname(fullPath), { recursive: true });
    fs.writeFileSync(fullPath, contents, 'utf8');
  }

  return root;
}

function repeatLine(line, count) {
  return Array.from({ length: count }, () => line).join('\n');
}

test('flags source files that exceed hard line limits', () => {
  const root = createWorkspace({
    'src/views/HugeView.tsx': repeatLine('export const value = 1;', 481),
  });

  const result = runArchitectureCheck(root);

  assert.equal(result.errors.length, 1);
  assert.equal(result.errors[0].code, 'file-too-large');
  assert.equal(result.errors[0].file, 'src/views/HugeView.tsx');
});

test('flags bucket-style file names inside source roots', () => {
  const root = createWorkspace({
    'src/lib/utils.ts': 'export function formatSkillName(name) { return name.trim(); }\n',
  });

  const result = runArchitectureCheck(root);

  assert(result.errors.some((issue) => issue.code === 'bucket-file-name'));
});

test('flags pass-through re-export shells', () => {
  const root = createWorkspace({
    'src/features/skill-card/index.ts': "export { SkillCard } from './SkillCard';\n",
  });

  const result = runArchitectureCheck(root);

  assert(result.errors.some((issue) => issue.code === 'passthrough-module'));
});

test('flags shallow one-file directories without an explicit domain boundary', () => {
  const root = createWorkspace({
    'src/components/skill-card/SkillCard.tsx': [
      'export function SkillCard() {',
      '  return null;',
      '}',
    ].join('\n'),
  });

  const result = runArchitectureCheck(root);

  assert(result.errors.some((issue) => issue.code === 'shallow-module-directory'));
});

test('allows explicit domain modules and ignores i18n payload files', () => {
  const root = createWorkspace({
    'src/lib/git-status.ts': [
      'export function getGitStatusLabel() {',
      "  return 'clean';",
      '}',
    ].join('\n'),
    'src/i18n/en.json': repeatLine('"longKey": "value",', 800),
  });

  const result = runArchitectureCheck(root);

  assert.equal(
    result.errors.filter((issue) => issue.file === 'src/lib/git-status.ts').length,
    0,
  );
  assert.equal(
    result.errors.filter((issue) => issue.file === 'src/i18n/en.json').length,
    0,
  );
});

test('allows thin rust boundary files for mod.rs and tauri main entry', () => {
  const root = createWorkspace({
    'src-tauri/src/core/skills/mod.rs': 'pub mod scan;\n',
    'src-tauri/src/main.rs': [
      '#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]',
      '',
      'fn main() {',
      '    app_lib::run();',
      '}',
    ].join('\n'),
    'src-tauri/src/core/skills/scan.rs': [
      'pub fn scan_skills() -> usize {',
      '  1',
      '}',
    ].join('\n'),
  });

  const result = runArchitectureCheck(root);
  const thinWarnings = result.warnings.filter((issue) => issue.code === 'thin-module');

  assert.equal(
    thinWarnings.filter((issue) => issue.file === 'src-tauri/src/core/skills/mod.rs').length,
    0,
  );
  assert.equal(
    thinWarnings.filter((issue) => issue.file === 'src-tauri/src/main.rs').length,
    0,
  );
});
