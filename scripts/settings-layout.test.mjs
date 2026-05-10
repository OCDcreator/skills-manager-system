import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

test("SettingsView and RepoPathForm stay single-column and overflow-safe", () => {
  const settingsSource = fs.readFileSync(path.resolve("src/views/SettingsView.tsx"), "utf8");
  const formSource = fs.readFileSync(path.resolve("src/components/RepoPathForm.tsx"), "utf8");

  assert.match(settingsSource, /space-y-6 rounded-2xl border border-slate-800 bg-slate-900 p-6/);
  assert.match(formSource, /min-w-0/);
  assert.match(formSource, /break-all|truncate|overflow-x-auto/);
});
