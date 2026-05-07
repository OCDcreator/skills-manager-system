import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

test("agent-order helper keeps enabled-first sorting logic", () => {
  const source = fs.readFileSync(path.resolve("src/lib/agent-order.ts"), "utf8");

  assert.match(source, /if \(left\.enabled !== right\.enabled\) \{/);
  assert.match(source, /return left\.enabled \? -1 : 1/);
  assert.match(source, /const leftIndex = indexMap\.get\(left\.key as AgentKey\)/);
  assert.match(source, /if \(leftIndex != null && rightIndex != null\) \{/);
});

test("agent-order helper merges persisted keys with inventory keys", () => {
  const source = fs.readFileSync(path.resolve("src/lib/agent-order.ts"), "utf8");

  assert.match(source, /export function mergeAgentOrderWithInventory/);
  assert.match(source, /const preserved = persistedOrder\.filter/);
  assert.match(source, /const missing = agents\.map\(\(agent\) => agent\.key\)/);
  assert.match(source, /return \[\.\.\.preserved, \.\.\.missing\] as AgentKey\[\]/);
});

test("agent-order helper exposes drag reorder support", () => {
  const source = fs.readFileSync(path.resolve("src/lib/agent-order.ts"), "utf8");

  assert.match(source, /export function applyDraggedAgentOrder/);
  assert.match(source, /const next = currentOrder\.filter\(\(key\) => key !== draggedKey\)/);
  assert.match(source, /next\.splice\(targetIndex, 0, draggedKey\)/);
});

test("agent side-nav helpers support enabled-only filtering and persisted preference wiring", () => {
  const helperSource = fs.readFileSync(
    path.resolve("src/lib/agent-side-nav.ts"),
    "utf8",
  );
  const viewSource = fs.readFileSync(path.resolve("src/views/AgentsView.tsx"), "utf8");
  const modalSource = fs.readFileSync(
    path.resolve("src/components/agents/AgentOrderModal.tsx"),
    "utf8",
  );
  const i18nEn = fs.readFileSync(path.resolve("src/i18n/en.json"), "utf8");
  const i18nZh = fs.readFileSync(path.resolve("src/i18n/zh.json"), "utf8");

  assert.match(helperSource, /skills-manager-system\.agents\.side-nav\.enabled-only/);
  assert.match(helperSource, /export function readAgentSideNavEnabledOnlyPreference/);
  assert.match(helperSource, /export function writeAgentSideNavEnabledOnlyPreference/);
  assert.match(helperSource, /export function applyAgentEnabledOverrides/);
  assert.match(helperSource, /export function resolveAgentSideNavInventory/);
  assert.match(viewSource, /const floatingNavAgents = useMemo/);
  assert.match(viewSource, /readAgentSideNavEnabledOnlyPreference/);
  assert.match(viewSource, /writeAgentSideNavEnabledOnlyPreference/);
  assert.match(viewSource, /resolveAgentSideNavInventory/);
  assert.match(viewSource, /agents=\{floatingNavAgents\}/);
  assert.match(viewSource, /onShowEnabledOnlyInSideNavChange=\{setShowEnabledOnlyInSideNav\}/);
  assert.match(modalSource, /showEnabledOnlyInSideNav: boolean/);
  assert.match(modalSource, /onShowEnabledOnlyInSideNavChange: \(enabled: boolean\) => void/);
  assert.match(modalSource, /enabledByKey/);
  assert.match(modalSource, /enabledByKey\.get\(activeDraggedKey\) !== enabledByKey\.get\(targetKey\)/);
  assert.match(modalSource, /agents\.orderModal\.sideNavEnabledOnly/);
  assert.match(modalSource, /agents\.orderModal\.sideNavEnabledOnlyHint/);
  assert.match(i18nEn, /"agents\.orderModal\.sideNavEnabledOnly"/);
  assert.match(i18nZh, /"agents\.orderModal\.sideNavEnabledOnly"/);
});
