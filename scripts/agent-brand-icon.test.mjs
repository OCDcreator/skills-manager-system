import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

test("agent brand icon maps Kimi Code CLI to the local webp asset", () => {
  const source = fs.readFileSync(
    path.resolve("src/components/agents/AgentBrandIcon.tsx"),
    "utf8",
  );

  assert.match(source, /kimi:\s*\{/);
  assert.match(source, /kimi-logo\.webp/);
});
