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

test("assistant launcher menu exposes a default-path control", () => {
  const source = fs.readFileSync(
    path.resolve("src/components/assistant/AssistantLauncherMenu.tsx"),
    "utf8",
  );

  assert.match(source, /defaultWorkingDirectory/);
  assert.match(source, /assistant\.defaultPath/);
});

test("assistant panel loads and saves terminal launcher preferences", () => {
  const source = fs.readFileSync(
    path.resolve("src/components/assistant/ProjectAssistantPanel.tsx"),
    "utf8",
  );

  assert.match(source, /getTerminalLauncherPreferences/);
  assert.match(source, /setTerminalWorkingDirectoryPreference/);
});

test("assistant terminal session renders stop and restart controls", () => {
  const source = fs.readFileSync(
    path.resolve("src/components/assistant/AssistantTerminalSession.tsx"),
    "utf8",
  );

  assert.match(source, /stopTerminalSession/);
  assert.match(source, /assistant\.restart/);
  assert.match(source, /new Terminal\(/);
  assert.match(source, /drainTerminalOutput/);
});

test("assistant terminal session does not remount xterm for every session poll", () => {
  const source = fs.readFileSync(
    path.resolve("src/components/assistant/AssistantTerminalSession.tsx"),
    "utf8",
  );

  assert.match(source, /sameSessionSnapshot/);
  assert.doesNotMatch(source, /\},\s*\[props\]\)/);
  assert.match(source, /\},\s*\[launchInput,\s*onSessionChange\]\)/);
});
