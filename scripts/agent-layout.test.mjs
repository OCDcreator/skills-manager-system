import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

test('Agent global skill sidecar matches the adjacent agent card height', () => {
  const viewSource = fs.readFileSync(path.resolve('src/views/AgentsView.tsx'), 'utf8');
  const listSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentGlobalSkillList.tsx'),
    'utf8',
  );

  assert.match(viewSource, /min-\[1380px\]:relative min-\[1380px\]:overflow-hidden/);
  assert.match(viewSource, /min-\[1380px\]:absolute min-\[1380px\]:inset-0/);
  assert.match(listSource, /h-full min-h-0 .*overflow-hidden.*flex-col/);
  assert.match(listSource, /skill-markdown-scroll.*min-h-0.*flex-1.*overflow-y-auto/);
  assert.doesNotMatch(listSource, /max-h-44/);
});
