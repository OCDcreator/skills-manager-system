import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

test('Agent global skill sidecar matches the adjacent agent card height', () => {
  const sectionSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentTargetsSection.tsx'),
    'utf8',
  );
  const listSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentGlobalSkillList.tsx'),
    'utf8',
  );

  assert.match(sectionSource, /min-\[1380px\]:relative min-\[1380px\]:overflow-hidden/);
  assert.match(sectionSource, /min-\[1380px\]:absolute min-\[1380px\]:inset-0/);
  assert.match(listSource, /h-full min-h-0 .*overflow-hidden.*flex-col/);
  assert.match(listSource, /skill-markdown-scroll.*min-h-0.*flex-1.*overflow-y-auto/);
  assert.doesNotMatch(listSource, /max-h-44/);
});

test('Agent floating nav only reacts inside the icon hit area', () => {
  const navSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentFloatingNav.tsx'),
    'utf8',
  );

  assert.doesNotMatch(navSource, /<div className="pointer-events-auto relative">/);
  assert.match(navSource, /className="pointer-events-none relative z-10 h-14"/);
  assert.match(navSource, /className="pointer-events-auto relative ml-auto block h-14 w-14 text-right/);
  assert.doesNotMatch(navSource, /className="relative ml-auto block h-14 w-14 text-right/);
  assert.doesNotMatch(navSource, /className="relative block h-14 w-full text-right/);
});

test('Agent floating nav exposes an order button above the jump-top control', () => {
  const navSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentFloatingNav.tsx'),
    'utf8',
  );

  assert.match(navSource, /kind: "action"/);
  assert.match(navSource, /actionKey: "open-order-modal"/);
  assert.match(navSource, /onOpenOrderModal/);
});

test('Agent order modal caps its height and keeps the reorder list scrollable', () => {
  const source = fs.readFileSync(
    path.resolve('src/components/agents/AgentOrderModal.tsx'),
    'utf8',
  );

  assert.match(source, /max-h-\[calc\(100vh-2rem\)\]/);
  assert.match(source, /flex max-h-\[calc\(100vh-2rem\)\] w-full max-w-xl flex-col overflow-hidden/);
  assert.match(source, /skill-markdown-scroll min-h-0 flex-1 space-y-5 overflow-y-auto/);
});

test('Agent floating nav measures app chrome bounds instead of using raw viewport centering', () => {
  const shellSource = fs.readFileSync(
    path.resolve('src/components/AppShell.tsx'),
    'utf8',
  );
  const launcherSource = fs.readFileSync(
    path.resolve('src/components/assistant/ProjectAssistantLauncher.tsx'),
    'utf8',
  );
  const navSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentFloatingNav.tsx'),
    'utf8',
  );

  assert.match(shellSource, /data-app-shell-header/);
  assert.match(shellSource, /data-app-shell-main/);
  assert.match(launcherSource, /data-project-assistant-launcher/);
  assert.match(navSource, /data-app-shell-header/);
  assert.match(navSource, /data-project-assistant-launcher/);
  assert.match(navSource, /style=\{\{\s*top:\s*topOffsetPx,\s*bottom:\s*bottomOffsetPx\s*\}\}/);
  assert.match(navSource, /className="relative h-full overflow-y-auto py-1 pr-1"/);
  assert.doesNotMatch(navSource, /fixed right-3 top-1\/2/);
  assert.doesNotMatch(navSource, /fixed right-3 top-1\/2 z-50 .* -translate-y-1\/2/);
  assert.doesNotMatch(navSource, /max-h-\[calc\(100vh-5rem\)\]/);
});
