import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

test("Projects route opts into the wider shell", () => {
  const source = fs.readFileSync(path.resolve("src/App.tsx"), "utf8");
  assert.match(source, /activeView === "projects"/);
  assert.match(source, /max-w-\[min\(96vw,1800px\)\]/);
});

test("Projects view uses section-level apply-all instead of card-level apply", () => {
  const viewSource = fs.readFileSync(path.resolve("src/views/ProjectsView.tsx"), "utf8");
  const cardSource = fs.readFileSync(path.resolve("src/components/projects/ProjectCard.tsx"), "utf8");

  assert.match(viewSource, /applyProjectAssignments/);
  assert.match(viewSource, /projects\.saved\.applyAll/);
  assert.doesNotMatch(cardSource, /projects\.card\.apply/);
  assert.doesNotMatch(cardSource, /onApply\s*=/);
});
