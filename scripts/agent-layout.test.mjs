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

test('AgentsView switches from floating rail padding to compact tab focus below 1280px', () => {
  const viewSource = fs.readFileSync(path.resolve('src/views/AgentsView.tsx'), 'utf8');
  const navSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentFloatingNav.tsx'),
    'utf8',
  );
  const sectionSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentTargetsSection.tsx'),
    'utf8',
  );

  assert.match(viewSource, /min-\[1280px\]:pr-12/);
  assert.match(viewSource, /AgentCompactTabs/);
  assert.match(navSource, /max-\[1279px\]:hidden/);
  assert.match(sectionSource, /activeAgentKey/);
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

  assert.match(source, /max-h-\[min\(44rem,calc\(100vh-2rem\)\)\]/);
  assert.match(source, /flex max-h-\[min\(44rem,calc\(100vh-2rem\)\)\] w-full max-w-\[58rem\] flex-col overflow-hidden/);
  assert.match(source, /skill-markdown-scroll min-h-0 flex-1 space-y-6 overflow-y-auto px-6 py-5/);
  assert.match(source, /border-t border-slate-800\/90 bg-slate-950\/70 px-6 py-4/);
});

test('Agent order modal surfaces a lightweight dirty state in the header and footer', () => {
  const source = fs.readFileSync(
    path.resolve('src/components/agents/AgentOrderModal.tsx'),
    'utf8',
  );
  const i18nEn = fs.readFileSync(path.resolve('src/i18n/en.json'), 'utf8');
  const i18nZh = fs.readFileSync(path.resolve('src/i18n/zh.json'), 'utf8');

  assert.match(source, /const isDirty =/);
  assert.match(source, /agents\.orderModal\.dirtyBadge/);
  assert.match(source, /agents\.orderModal\.dirtyHint/);
  assert.match(source, /agents\.orderModal\.cleanHint/);
  assert.match(source, /disabled=\{isSaving \|\| !isDirty\}/);
  assert.match(i18nEn, /"agents\.orderModal\.dirtyBadge"/);
  assert.match(i18nZh, /"agents\.orderModal\.dirtyHint"/);
});

test('Agent order modal uses pointer drag handles instead of native draggable rows', () => {
  const source = fs.readFileSync(
    path.resolve('src/components/agents/AgentOrderModal.tsx'),
    'utf8',
  );

  assert.match(source, /data-agent-order-key=\{agent\.key\}/);
  assert.match(source, /onPointerDown=\{\(event\) =>/);
  assert.match(source, /onPointerMove=\{handlePointerDragMove\}/);
  assert.match(source, /onPointerUp=\{handlePointerDragEnd\}/);
  assert.match(source, /setPointerCapture\(event\.pointerId\)/);
  assert.match(source, /document\.elementFromPoint\(event\.clientX, event\.clientY\)/);
  assert.doesNotMatch(source, /\sdraggable\s/);
});

test('Agent selection summary can deselect direct skills from the sync set', () => {
  const summarySource = fs.readFileSync(
    path.resolve('src/components/agents/AgentSelectionSummary.tsx'),
    'utf8',
  );
  const i18nEn = fs.readFileSync(path.resolve('src/i18n/en.json'), 'utf8');
  const i18nZh = fs.readFileSync(path.resolve('src/i18n/zh.json'), 'utf8');

  assert.match(summarySource, /removeId/);
  assert.match(summarySource, /item\.isDirect/);
  assert.match(summarySource, /selectedSkillIds: removeId\(draft\.selectedSkillIds, item\.skill\.id\)/);
  assert.match(summarySource, /agents\.card\.deselectDirect/);
  assert.match(i18nEn, /"agents\.card\.deselectDirect"/);
  assert.match(i18nZh, /"agents\.card\.deselectDirect"/);
});

test('Agent selection summary exposes batch exclude and deselect controls', () => {
  const summarySource = fs.readFileSync(
    path.resolve('src/components/agents/AgentSelectionSummary.tsx'),
    'utf8',
  );
  const i18nEn = fs.readFileSync(path.resolve('src/i18n/en.json'), 'utf8');
  const i18nZh = fs.readFileSync(path.resolve('src/i18n/zh.json'), 'utf8');

  assert.match(summarySource, /selectedPreviewSkillIds/);
  assert.match(summarySource, /togglePreviewSkillSelection/);
  assert.match(summarySource, /batchExcludeSelected/);
  assert.match(summarySource, /batchDeselectDirectSelected/);
  assert.match(summarySource, /agents\.card\.batchExclude/);
  assert.match(summarySource, /agents\.card\.batchDeselectDirect/);
  assert.match(i18nEn, /"agents\.card\.batchExclude"/);
  assert.match(i18nZh, /"agents\.card\.batchDeselectDirect"/);
});

test('Agent selection summary turns status pills into selectable filters', () => {
  const summarySource = fs.readFileSync(
    path.resolve('src/components/agents/AgentSelectionSummary.tsx'),
    'utf8',
  );

  assert.match(summarySource, /activeFilter/);
  assert.match(summarySource, /preview\.items\.filter\(\(item\) =>/);
  assert.match(summarySource, /item\.needsSync/);
  assert.match(summarySource, /item\.isSynced/);
  assert.match(summarySource, /item\.isGloballyDisabled/);
  assert.match(summarySource, /item\.isExcluded/);
  assert.match(summarySource, /type="button"/);
});

test('Agent skill selector keeps the selected-only toggle beside the search field', () => {
  const selectorSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentSkillSelector.tsx'),
    'utf8',
  );
  const i18nEn = fs.readFileSync(path.resolve('src/i18n/en.json'), 'utf8');
  const i18nZh = fs.readFileSync(path.resolve('src/i18n/zh.json'), 'utf8');

  assert.match(selectorSource, /showSelectedOnly/);
  assert.match(selectorSource, /className="flex items-center gap-2"/);
  assert.match(selectorSource, /placeholder=\{t\("agents\.card\.searchSkills"\)\}/);
  assert.match(selectorSource, /t\("agents\.card\.selectedOnly"\)/);
  assert.match(selectorSource, /selected\.has\(skill\.id\)/);
  assert.match(i18nEn, /"agents\.card\.selectedOnly"/);
  assert.match(i18nZh, /"agents\.card\.selectedOnly"/);
});

test('Agent selection summary distinguishes synced skills from pending sync items', () => {
  const summarySource = fs.readFileSync(
    path.resolve('src/components/agents/AgentSelectionSummary.tsx'),
    'utf8',
  );
  const helperSource = fs.readFileSync(path.resolve('src/lib/agent-selection.ts'), 'utf8');
  const sectionSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentTargetsSection.tsx'),
    'utf8',
  );
  const i18nEn = fs.readFileSync(path.resolve('src/i18n/en.json'), 'utf8');
  const i18nZh = fs.readFileSync(path.resolve('src/i18n/zh.json'), 'utf8');

  assert.match(helperSource, /targetSkillEntries: AgentTargetSkillEntry\[\]/);
  assert.match(helperSource, /isSynced:/);
  assert.match(helperSource, /needsSync:/);
  assert.match(helperSource, /syncedCount:/);
  assert.match(sectionSource, /agent\.targetSkillEntries/);
  assert.match(summarySource, /agents\.card\.syncedCount/);
  assert.match(summarySource, /agents\.card\.synced/);
  assert.match(i18nEn, /"agents\.card\.syncedCount"/);
  assert.match(i18nZh, /"agents\.card\.synced"/);
});

test('Agent skill selector exposes path-based filter pills for skill directories', () => {
  const selectorSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentSkillSelector.tsx'),
    'utf8',
  );
  const filterSource = fs.readFileSync(path.resolve('src/lib/skills/filters.ts'), 'utf8');
  const i18nEn = fs.readFileSync(path.resolve('src/i18n/en.json'), 'utf8');
  const i18nZh = fs.readFileSync(path.resolve('src/i18n/zh.json'), 'utf8');

  assert.match(selectorSource, /pathFilter/);
  assert.match(selectorSource, /buildSkillPathSummaries/);
  assert.match(selectorSource, /matchesSkillPathFilter/);
  assert.match(filterSource, /export function buildSkillPathSummaries/);
  assert.match(filterSource, /export function matchesSkillPathFilter/);
  assert.match(i18nEn, /"agents\.card\.allPaths"/);
  assert.match(i18nZh, /"agents\.card\.allPaths"/);
});

test('Agent skill selector reuses external subgroup filtering when the external path is active', () => {
  const selectorSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentSkillSelector.tsx'),
    'utf8',
  );
  const i18nEn = fs.readFileSync(path.resolve('src/i18n/en.json'), 'utf8');
  const i18nZh = fs.readFileSync(path.resolve('src/i18n/zh.json'), 'utf8');

  assert.match(selectorSource, /buildExternalGroupSummaries/);
  assert.match(selectorSource, /externalGroupFilter/);
  assert.match(selectorSource, /isExternalGroupOpen/);
  assert.match(selectorSource, /pathFilter === "external"/);
  assert.match(selectorSource, /externalGroupSummaries\.map/);
  assert.match(selectorSource, /agents\.card\.externalGroups\.all/);
  assert.match(selectorSource, /agents\.card\.externalGroups\.empty/);
  assert.match(i18nEn, /"agents\.card\.externalGroups\.all"/);
  assert.match(i18nEn, /"agents\.card\.externalGroups\.empty"/);
  assert.match(i18nZh, /"agents\.card\.externalGroups\.all"/);
  assert.match(i18nZh, /"agents\.card\.externalGroups\.empty"/);
});

test('Agent global skill list exposes batch target actions in selection mode', () => {
  const listSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentGlobalSkillList.tsx'),
    'utf8',
  );
  const sectionSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentTargetsSection.tsx'),
    'utf8',
  );
  const hookSource = fs.readFileSync(path.resolve('src/lib/agent-target-actions.ts'), 'utf8');
  const i18nEn = fs.readFileSync(path.resolve('src/i18n/en.json'), 'utf8');
  const i18nZh = fs.readFileSync(path.resolve('src/i18n/zh.json'), 'utf8');

  assert.match(listSource, /isSelectionMode/);
  assert.match(listSource, /selectedEntryNames/);
  assert.match(listSource, /agents\.globalSkills\.multiSelect/);
  assert.match(listSource, /agents\.globalSkills\.batchDelete/);
  assert.match(listSource, /agents\.globalSkills\.batchTakeOver/);
  assert.match(listSource, /agents\.globalSkills\.batchImportDelete/);
  assert.match(sectionSource, /batchDeleteTargetSkills/);
  assert.match(hookSource, /batchDeleteTargetSkills/);
  assert.match(hookSource, /executeBatchTargetAction/);
  assert.match(i18nEn, /"agents\.globalSkills\.batchDelete"/);
  assert.match(i18nZh, /"agents\.globalSkills\.batchImportDelete"/);
});

test('Agent global skill list lets the managed and unmanaged pills filter the visible entries', () => {
  const listSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentGlobalSkillList.tsx'),
    'utf8',
  );

  assert.match(listSource, /activeFilter/);
  assert.match(listSource, /agent\.targetSkillEntries\.filter\(\(entry\) =>/);
  assert.match(listSource, /entry\.managed/);
  assert.match(listSource, /managedCount/);
  assert.match(listSource, /unmanagedCount/);
  assert.match(listSource, /toggleFilter\("managed"\)/);
  assert.match(listSource, /toggleFilter\("unmanaged"\)/);
});

test('Agent global skill list renders symlink target metadata when available', () => {
  const listSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentGlobalSkillList.tsx'),
    'utf8',
  );
  const i18nEn = fs.readFileSync(path.resolve('src/i18n/en.json'), 'utf8');
  const i18nZh = fs.readFileSync(path.resolve('src/i18n/zh.json'), 'utf8');

  assert.match(listSource, /entry\.entryKind === "symlink"/);
  assert.match(listSource, /entry\.symlinkTargetPath/);
  assert.match(listSource, /secondaryLabel !== entry\.displayName/);
  assert.match(listSource, /-webkit-line-clamp:2/);
  assert.match(listSource, /agents\.globalSkills\.symlinkTarget/);
  assert.match(i18nEn, /"agents\.globalSkills\.symlinkTarget"/);
  assert.match(i18nZh, /"agents\.globalSkills\.symlinkTarget"/);
});

test('Agent floating nav stays centered in the viewport and caps its own scroll height', () => {
  const navSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentFloatingNav.tsx'),
    'utf8',
  );

  assert.match(navSource, /className="pointer-events-none fixed right-3 top-1\/2 z-50/);
  assert.match(navSource, /-translate-y-1\/2 bg-transparent/);
  assert.match(
    navSource,
    /className="agent-floating-nav-scroll relative max-h-\[calc\(100vh-2rem\)\] overflow-y-auto py-1 pr-1"/,
  );
});

test('Agent floating nav hides its own scrollbar while remaining scrollable', () => {
  const navSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentFloatingNav.tsx'),
    'utf8',
  );
  const styleEntrySource = fs.readFileSync(
    path.resolve('src/styles.css'),
    'utf8',
  );
  const agentStyleSource = fs.readFileSync(
    path.resolve('src/styles/agents.css'),
    'utf8',
  );

  assert.match(navSource, /agent-floating-nav-scroll/);
  assert.match(styleEntrySource, /@import "\.\/styles\/agents\.css";/);
  assert.match(agentStyleSource, /\.agent-floating-nav-scroll\s*\{/);
  assert.match(agentStyleSource, /scrollbar-width:\s*none/);
  assert.match(agentStyleSource, /\.agent-floating-nav-scroll::\-webkit-scrollbar\s*\{/);
  assert.match(agentStyleSource, /display:\s*none/);
});

test('Agent unsaved actions stay inside the summary card instead of inserting a top-level banner', () => {
  const viewSource = fs.readFileSync(path.resolve('src/views/AgentsView.tsx'), 'utf8');
  const summarySource = fs.readFileSync(
    path.resolve('src/components/agents/AgentSyncSummary.tsx'),
    'utf8',
  );

  assert.match(viewSource, /dirtyAgentCount=\{dirtyAgentKeys\.length\}/);
  assert.doesNotMatch(viewSource, /t\("agents\.unsavedBanner"/);
  assert.match(summarySource, /t\("agents\.unsavedBanner"/);
  assert.match(summarySource, /min-h-\[/);
  assert.match(summarySource, /onDiscardChanges/);
  assert.match(summarySource, /onSaveChanges/);
});

test('Agent selector cards keep an independent header layout inside a capped three-panel row', () => {
  const targetCardSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentTargetCard.tsx'),
    'utf8',
  );
  const selectorSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentSkillSelector.tsx'),
    'utf8',
  );
  const sceneSource = fs.readFileSync(
    path.resolve('src/components/agents/AgentSceneSelector.tsx'),
    'utf8',
  );
  const summarySource = fs.readFileSync(
    path.resolve('src/components/agents/AgentSelectionSummary.tsx'),
    'utf8',
  );
  assert.match(
    targetCardSource,
    /className=\{`rounded-2xl border p-5 xl:flex xl:min-h-0 xl:flex-col/,
  );
  assert.match(
    targetCardSource,
    /className="mt-4 grid gap-4 xl:max-h-\[clamp\(20rem,calc\(100vh-38rem\),30rem\)\] xl:min-h-0 xl:grid-cols-3"/,
  );
  assert.match(selectorSource, /className="flex h-full min-h-0 flex-col rounded-xl border border-slate-800 bg-slate-950\/50 p-3"/);
  assert.match(sceneSource, /className="flex h-full min-h-0 flex-col rounded-xl border border-slate-800 bg-slate-950\/50 p-3"/);
  assert.match(summarySource, /className="flex h-full min-h-0 flex-col rounded-xl border border-slate-800 bg-slate-950\/50 p-3"/);
  assert.match(summarySource, /className="flex flex-wrap items-center gap-2 text-\[11px\] text-slate-400"/);
  assert.match(
    selectorSource,
    /className="skill-markdown-scroll mt-2 max-h-56 min-h-0 flex-1 space-y-1 overflow-y-auto pr-1 xl:max-h-none"/,
  );
  assert.match(
    sceneSource,
    /className="skill-markdown-scroll mt-2 max-h-40 min-h-0 flex-1 space-y-1 overflow-y-auto pr-1 xl:max-h-none"/,
  );
  assert.match(
    summarySource,
    /className="skill-markdown-scroll mt-2 max-h-64 min-h-0 flex-1 space-y-1 overflow-y-auto pr-1 xl:max-h-none"/,
  );
  assert.doesNotMatch(targetCardSource, /useAgentPanelHeaderHeight/);
  assert.doesNotMatch(targetCardSource, /xl:overflow-hidden/);
  assert.doesNotMatch(selectorSource, /headerStyle/);
  assert.doesNotMatch(sceneSource, /headerStyle/);
  assert.doesNotMatch(summarySource, /headerStyle/);
  assert.doesNotMatch(summarySource, /clamp\(18rem,calc\(100vh-20rem\),40rem\)/);
  assert.doesNotMatch(summarySource, /overflow-x-auto px-1 pb-1/);
});
