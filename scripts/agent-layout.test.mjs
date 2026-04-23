import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

test('Agent global skill list expands its scroll area to the sidecar height', () => {
  const source = fs.readFileSync(
    path.resolve('src/components/agents/AgentGlobalSkillList.tsx'),
    'utf8',
  );

  assert.match(source, /h-full min-h-0 .*flex-col/);
  assert.match(source, /skill-markdown-scroll.*min-h-0.*flex-1.*overflow-y-auto/);
  assert.doesNotMatch(source, /max-h-44/);
});
