import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

test("assistant panel delegates launcher rendering to dedicated components", () => {
  const source = fs.readFileSync(
    path.resolve("src/components/assistant/ProjectAssistantPanel.tsx"),
    "utf8",
  );

  assert.match(source, /AssistantLauncherMenu/);
  assert.doesNotMatch(source, /textarea/);
});

test("assistant launcher menu exposes the four approved CLI keys", () => {
  const source = fs.readFileSync(
    path.resolve("src/components/assistant/AssistantLauncherMenu.tsx"),
    "utf8",
  );

  assert.match(source, /codex/);
  assert.match(source, /opencode/);
  assert.match(source, /claude_code/);
  assert.match(source, /kimi/);
});
