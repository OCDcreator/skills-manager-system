import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

test('Project assistant launcher hint stays collapsed until the robot is hovered or focused', () => {
  const source = fs.readFileSync(
    path.resolve('src/components/assistant/ProjectAssistantLauncher.tsx'),
    'utf8',
  );

  assert.match(source, /className="fixed bottom-6 right-6 z-40 group"/);
  assert.match(source, /opacity-0/);
  assert.match(source, /group-hover:opacity-100/);
  assert.match(source, /group-focus-within:opacity-100/);
  assert.doesNotMatch(source, /fixed bottom-24 right-6 z-30/);
});
