import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

function readIfExists(relativePath) {
  const absolutePath = path.resolve(relativePath);
  return fs.existsSync(absolutePath) ? fs.readFileSync(absolutePath, "utf8") : "";
}

test("AppShell exposes the sources view in the top nav", () => {
  const source = readIfExists("src/components/AppShell.tsx");
  assert.match(source, /"sources"/);
  assert.match(source, /t\(`nav\.\$\{view\}`\)/);
  assert.match(source, /t\(`tooltip\.nav\.\$\{view\}`\)/);
  assert.doesNotMatch(source, /defaultValue/);
});

test('App routes ExternalSourcesView when activeView === "sources"', () => {
  const source = readIfExists("src/App.tsx");
  assert.match(source, /activeView === "sources"/);
  assert.match(source, /<ExternalSourcesView \/>/);
});

test("ExternalSourcesView renders the add form and list surfaces", () => {
  const source = readIfExists("src/views/ExternalSourcesView.tsx");
  assert.match(source, /AddExternalSourceForm/);
  assert.match(source, /ExternalSourceList/);
  assert.match(source, /t\("sources\.title"\)/);
});

test("AddExternalSourceForm catches submit failures and preserves the draft input", () => {
  const source = readIfExists("src/components/external-sources/AddExternalSourceForm.tsx");
  assert.match(source, /onSubmit=\{async \(event\) =>/);
  assert.match(source, /try \{/);
  assert.match(source, /await onSubmit\(trimmed\)/);
  assert.match(source, /setRepoUrl\(""\)/);
  assert.match(source, /catch \{/);
  assert.match(source, /setSubmitFailed\(true\)/);
  assert.doesNotMatch(source, /\.then\(\(\) => setRepoUrl\(""\)\)/);
});

test("Sources surface strings are backed by real i18n keys", () => {
  const en = JSON.parse(readIfExists("src/i18n/en.json"));
  const zh = JSON.parse(readIfExists("src/i18n/zh.json"));
  const helperSource = readIfExists("src/lib/external-sources.ts");
  const cardSource = readIfExists("src/components/external-sources/ExternalSourceCard.tsx");
  const listSource = readIfExists("src/components/external-sources/ExternalSourceList.tsx");
  const importListSource = readIfExists("src/components/external-sources/ExternalImportList.tsx");

  const requiredKeys = [
    "nav.sources",
    "tooltip.nav.sources",
    "sources.title",
    "sources.form.submitFailed",
    "sources.variants.title",
    "sources.imports.update",
    "sources.meta.unknown",
    "sources.warnings.count",
  ];

  for (const key of requiredKeys) {
    assert.ok(en[key], `missing en key: ${key}`);
    assert.ok(zh[key], `missing zh key: ${key}`);
  }

  assert.match(cardSource, /t\("sources\.actions\.fetch"\)/);
  assert.match(cardSource, /t\("sources\.variants\.title"\)/);
  assert.match(cardSource, /shortCommit\(record\.lastFetchedCommit, unknownLabel\)/);
  assert.match(cardSource, /t\("sources\.warnings\.count", \{ count: record\.warnings\.length \}\)/);
  assert.match(importListSource, /t\("sources\.imports\.update"\)/);
  assert.match(listSource, /t\("sources\.emptyTitle"\)/);
  assert.doesNotMatch(helperSource, /No warnings/);
  assert.doesNotMatch(helperSource, /`\$\{warnings\.length\} warnings`/);
});
