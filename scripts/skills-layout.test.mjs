import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

function read(relativePath) {
  return fs.readFileSync(path.resolve(relativePath), "utf8");
}

test("SkillsView defines wide, compact-drawer, and narrow-stacked detail surfaces", () => {
  const source = read("src/views/SkillsView.tsx");

  assert.match(source, /data-skills-detail-wide/);
  assert.match(source, /data-skills-detail-drawer/);
  assert.match(source, /data-skills-detail-stacked/);
  assert.match(
    source,
    /min-\[1280px\]:grid-cols-\[minmax\(0,1fr\)_clamp\(24rem,30vw,42rem\)\]/,
  );
});

test("SkillList tightens the card grid before the wide breakpoint", () => {
  const source = read("src/components/skills/SkillList.tsx");

  assert.match(source, /minmax\(15rem,1fr\)/);
  assert.match(source, /min-\[1280px\]:auto-rows-\[13\.5rem\]/);
  assert.match(source, /max-\[1279px\]:auto-rows-\[11rem\]/);
});
