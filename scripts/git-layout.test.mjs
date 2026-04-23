import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

test('Git file list keeps long paths shrinkable and scrolls internally', () => {
  const source = fs.readFileSync(path.resolve('src/components/git/GitFileList.tsx'), 'utf8');

  assert.match(source, /overflow-y-auto/);
  assert.match(source, /skill-markdown-scroll/);
  assert.match(source, /min-w-0 flex-1 truncate text-slate-200/);
});

test('Git diff viewer caps panel height and keeps diff body scrollable', () => {
  const source = fs.readFileSync(path.resolve('src/components/git/GitDiffViewer.tsx'), 'utf8');

  assert.match(source, /lg:max-h-\[calc\(100vh-16rem\)\]/);
  assert.match(source, /skill-markdown-scroll/);
  assert.match(source, /min-h-0 flex-1 overflow-auto/);
});
