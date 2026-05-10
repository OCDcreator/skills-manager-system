import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

test("scene skill chooser uses a dense grid with the shared scrollbar skin", () => {
  const source = fs.readFileSync(
    path.resolve("src/components/scenes/SceneSkillChooser.tsx"),
    "utf8",
  );

  assert.match(source, /skill-markdown-scroll grid max-h-\[18rem\]/);
  assert.match(source, /grid-cols-\[repeat\(auto-fill,minmax\(15rem,1fr\)\)\]/);
  assert.match(source, /gap-x-3 gap-y-1 overflow-y-auto px-3 py-2 pr-4/);
  assert.match(source, /flex h-8 min-w-0 items-center gap-2 rounded-md px-2/);
  assert.match(source, /min-w-0 flex-1 truncate/);
  assert.doesNotMatch(source, /max-h-48 space-y-2 overflow-y-auto pr-1/);
});

test("scene skill drag stores a real dataTransfer payload for drops", () => {
  const source = fs.readFileSync(
    path.resolve("src/components/scenes/SceneSkillChooser.tsx"),
    "utf8",
  );

  assert.match(source, /type DragEvent/);
  assert.match(source, /event\.dataTransfer\.effectAllowed = "move"/);
  assert.match(source, /event\.dataTransfer\.setData\("text\/plain", skillId\)/);
  assert.match(source, /event\.dataTransfer\.getData\("text\/plain"\) \|\| draggedSkillId/);
  assert.match(source, /event\.dataTransfer\.dropEffect = "move"/);
});

test("scene skill chooser exposes search, status, source, and external group filters", () => {
  const source = fs.readFileSync(
    path.resolve("src/components/scenes/SceneSkillChooser.tsx"),
    "utf8",
  );
  const sceneCardSource = fs.readFileSync(
    path.resolve("src/components/scenes/SceneCard.tsx"),
    "utf8",
  );
  const filterSource = fs.readFileSync(
    path.resolve("src/lib/scene-skill-filters.ts"),
    "utf8",
  );

  assert.match(sceneCardSource, /<SceneSkillChooser/);
  assert.match(source, /const \[search, setSearch\] = useState\(""\)/);
  assert.match(filterSource, /SceneSkillStatusFilter = "all" \| "enabled" \| "disabled"/);
  assert.match(filterSource, /SceneSkillSourceFilter = "all" \| "custom" \| "external"/);
  assert.match(source, /externalGroupFilter/);
  assert.match(source, /buildExternalGroupSummaries/);
  assert.match(filterSource, /skill\.managedSource/);
  assert.match(filterSource, /skill\.relativePath\.split/);
});

test("scene external group menu floats above its filter button", () => {
  const source = fs.readFileSync(
    path.resolve("src/components/scenes/SceneSkillChooser.tsx"),
    "utf8",
  );

  assert.match(source, /relative overflow-visible rounded-lg/);
  assert.match(source, /relative inline-flex/);
  assert.match(source, /absolute bottom-full left-0 z-50 mb-2/);
  assert.doesNotMatch(source, /absolute left-0 top-full/);
});

test("ScenesView stacks the create form before inputs become cramped", () => {
  const viewSource = fs.readFileSync(path.resolve("src/views/ScenesView.tsx"), "utf8");
  const cardSource = fs.readFileSync(path.resolve("src/components/scenes/SceneCard.tsx"), "utf8");

  assert.match(
    viewSource,
    /grid gap-3 rounded-2xl border border-slate-800 bg-slate-900 p-4 min-\[900px\]:grid-cols-\[minmax\(0,1fr\)_minmax\(0,1fr\)_auto\]/,
  );
  assert.match(cardSource, /data-scene-active-config/);
});
