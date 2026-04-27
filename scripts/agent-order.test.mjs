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
