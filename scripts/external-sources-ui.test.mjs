import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { pathToFileURL } from "node:url";
import ts from "typescript";

function readIfExists(relativePath) {
  const absolutePath = path.resolve(relativePath);
  return fs.existsSync(absolutePath) ? fs.readFileSync(absolutePath, "utf8") : "";
}

async function loadExternalSourcesModule() {
  const sourcePath = path.resolve("src/lib/external-sources.ts");
  const source = fs.readFileSync(sourcePath, "utf8");
  const transpiled = ts.transpileModule(source, {
    compilerOptions: {
      module: ts.ModuleKind.ESNext,
      target: ts.ScriptTarget.ES2020,
    },
    fileName: sourcePath,
  });

  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "external-sources-test-"));
  const tempFile = path.join(tempDir, "external-sources.mjs");
  fs.writeFileSync(tempFile, transpiled.outputText, "utf8");

  return import(pathToFileURL(tempFile).href);
}

test("AppShell exposes the sources view in the workbench nav", () => {
  const source = readIfExists("src/components/AppShell.tsx");
  assert.match(source, /"sources"/);
  assert.match(source, /t\(`nav\.\$\{item\.view\}`\)/);
  assert.match(source, /t\(`tooltip\.nav\.\$\{item\.view\}`\)/);
  assert.match(source, /icon: Boxes/);
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

test("ExternalSourceCard defaults to a collapsed summary with clickable repo access and guidance tags", () => {
  const source = readIfExists("src/components/external-sources/ExternalSourceCard.tsx");

  assert.match(source, /useState\(false\)/);
  assert.match(source, /primaryVariant\?\.description/);
  assert.match(source, /xl:grid-cols-\[minmax\(0,1fr\)_max-content\]/);
  assert.match(source, /min-w-0 space-y-2/);
  assert.match(source, /grid-cols-\[repeat\(auto-fit,minmax\(8rem,1fr\)\)\] gap-2 xl:min-w-max xl:grid-cols-1/);
  assert.match(source, /whitespace-nowrap/);
  assert.match(source, /t\("sources\.actions\.expand"\)/);
  assert.match(source, /t\("sources\.actions\.collapse"\)/);
  assert.match(source, /openUrl\(record\.repoUrl\)/);
  assert.match(source, /t\("sources\.summary\.descriptionFallback"/);
  assert.match(source, /t\("sources\.tags\./);
  assert.match(source, /t\("sources\.variants\.targetLabel"\)/);
  assert.match(source, /t\("sources\.variants\.selectTarget"\)/);
  assert.match(source, /t\("sources\.variants\.importAs"/);
  assert.match(source, /t\("sources\.variants\.childDirectories"\)/);
  assert.match(source, /t\("sources\.variants\.childFiles"\)/);
  assert.match(source, /variant\.childFiles\.length/);
  assert.match(source, /variant\.childFiles\.slice\(0, 5\)/);
  assert.match(source, /variant\.childDirectories\.length/);
  assert.match(source, /variant\.childDirectories\.slice\(0, 5\)/);
  assert.match(source, /variant\.childDirectories\.length > 5/);
  assert.match(source, /skill-markdown-scroll max-h-\[32rem\] overflow-y-auto/);
  assert.match(source, /EXTERNAL_IMPORT_TARGETS\.map/);
  assert.match(source, /variant\.suggestedTargetAgents\[0\]/);
  assert.match(source, /variant\.detectedAgentHint/);
  assert.match(source, /t\(`sources\.variants\.detection\.\$\{variant\.detectionClass\}`\)/);
  assert.match(source, /target="_blank"/);
});

test("AddExternalSourceForm catches submit failures and preserves the draft input", () => {
  const source = readIfExists("src/components/external-sources/AddExternalSourceForm.tsx");
  assert.match(source, /onSubmit=\{async \(event\) =>/);
  assert.match(source, /try \{/);
  assert.match(source, /await onSubmit\(\{/);
  assert.match(source, /branch: branch\.trim\(\) \|\| null/);
  assert.match(source, /subpath: subpath\.trim\(\) \|\| null/);
  assert.match(source, /setRepoUrl\(""\)/);
  assert.match(source, /setBranch\(""\)/);
  assert.match(source, /setSubpath\(""\)/);
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
    "sources.variants.targetLabel",
    "sources.variants.selectTarget",
    "sources.variants.targetRequired",
    "sources.variants.importAs",
    "sources.variants.childDirectories",
    "sources.variants.childFiles",
    "sources.variants.none",
    "sources.variants.rootOnly",
    "sources.imports.update",
    "sources.meta.unknown",
    "sources.meta.subpath",
    "sources.tags.skillRepository",
    "sources.form.branchLabel",
    "sources.form.subpathLabel",
    "sources.warnings.count",
    "sources.actions.expand",
    "sources.actions.collapse",
    "sources.actions.openRepo",
    "sources.variants.detection.exact_supported",
    "sources.variants.detection.alias_supported",
    "sources.variants.detection.generic_discovered",
    "sources.summary.descriptionFallback",
    "sources.summary.supportedVariants",
    "sources.summary.noSupportedVariants",
    "sources.tags.generatedBundle",
    "sources.tags.importedCount",
  ];

  for (const key of requiredKeys) {
    assert.ok(en[key], `missing en key: ${key}`);
    assert.ok(zh[key], `missing zh key: ${key}`);
  }

  assert.match(cardSource, /t\("sources\.actions\.fetch"\)/);
  assert.match(cardSource, /t\("sources\.variants\.title"\)/);
  assert.match(cardSource, /t\("sources\.actions\.openRepo"\)/);
  assert.match(cardSource, /primaryVariant\?\.description/);
  assert.match(cardSource, /t\("sources\.variants\.hint"/);
  assert.match(cardSource, /t\("sources\.variants\.childDirectories"\)/);
  assert.match(cardSource, /t\("sources\.variants\.childFiles"\)/);
  assert.match(cardSource, /shortCommit\(record\.lastFetchedCommit, unknownLabel\)/);
  assert.match(cardSource, /record\.branch \?\? record\.defaultBranch/);
  assert.match(cardSource, /record\.subpath \?\? t\("sources\.meta\.rootSubpath"\)/);
  assert.match(cardSource, /t\("sources\.warnings\.count", \{ count: record\.warnings\.length \}\)/);
  assert.match(importListSource, /t\("sources\.imports\.update"\)/);
  assert.match(importListSource, /skill-markdown-scroll max-h-\[32rem\] overflow-y-auto/);
  assert.match(listSource, /t\("sources\.emptyTitle"\)/);
  assert.doesNotMatch(helperSource, /No warnings/);
  assert.doesNotMatch(helperSource, /`\$\{warnings\.length\} warnings`/);
});

test("Tauri desktop wiring includes opener plugin support for external repo links", () => {
  const pkg = JSON.parse(readIfExists("package.json"));
  const cargo = readIfExists("src-tauri/Cargo.toml");
  const lib = readIfExists("src-tauri/src/lib.rs");
  const capability = readIfExists("src-tauri/capabilities/default.json");

  assert.ok(pkg.dependencies["@tauri-apps/plugin-opener"]);
  assert.match(cargo, /tauri-plugin-opener/);
  assert.match(lib, /plugin\(tauri_plugin_opener::init\(\)\)/);
  assert.match(capability, /"opener:default"/);
});

test("Tauri production config keeps macOS bundle and CSP settings explicit", () => {
  const config = JSON.parse(readIfExists("src-tauri/tauri.conf.json"));

  assert.match(config.app.security.csp, /default-src 'self'/);
  assert.match(config.app.security.csp, /connect-src ipc: http:\/\/ipc\.localhost/);
  assert.equal(config.bundle.macOS.minimumSystemVersion, "11.0");
});

test("scan_skills desktop command keeps managed-source enrichment on the cached scan path", () => {
  const source = readIfExists("src-tauri/src/commands/skills.rs");

  assert.match(source, /scan_repo_skills_cached_with_external_sources/);
  assert.match(source, /load_cached_repo_skills_with_external_sources/);
  assert.doesNotMatch(source, /scan_repo_skills\(Path::new\(&repo_path\)\)/);
});

test("ExternalImportList keeps repair busy state independent from update availability", () => {
  const source = readIfExists("src/components/external-sources/ExternalImportList.tsx");

  assert.match(source, /resolveImportBusyState/);
  assert.match(source, /isRepairingImport\s*\?\s*t\("sources\.imports\.repairing"\)/);
});

test("external source helpers execute busy-state behavior at runtime", async () => {
  const module = await loadExternalSourcesModule();
  const item = {
    importId: "imp-1",
    updateAvailable: true,
  };

  assert.equal(module.shortCommit("abcdef1234567890"), "abcdef12");
  assert.equal(module.shortCommit(null, "unknown"), "unknown");
  assert.equal(module.agentLabel("codex"), "Codex");
  assert.ok(module.EXTERNAL_IMPORT_TARGETS.includes("codex"));
  assert.equal(
    module.warningSummary(
      [{ code: "one", severity: "warning", message: "Only warning" }],
      "2 warnings",
      "none",
    ),
    "Only warning",
  );
  assert.deepEqual(
    module.resolveImportBusyState(item, "imp-1", { action: "repair", importId: "imp-1" }),
    { isBusyImport: true, isUpdatingImport: false, isRepairingImport: true },
  );
});

test("refreshSkills reloads the selected document when the selected skill survives the refresh", () => {
  const source = readIfExists("src/context/AppContext.tsx");

  assert.match(source, /const selectedSkillIdRef = useRef<string \| null>\(null\)/);
  assert.match(source, /selectedSkillIdRef\.current = selectedSkill\?\.id \?\? null/);
  assert.match(source, /const currentSelectedSkillId = selectedSkillIdRef\.current/);
  assert.match(source, /const refreshedSkill = currentSelectedSkillId/);
  assert.match(source, /await api\.getSkillDocument\(refreshedSkill\.relativePath\)/);
  assert.match(source, /setSelectedDocument\(refreshedDocument\)/);
  assert.doesNotMatch(source, /\}, \[repoPath, selectedSkill\?\.id\]\)/);
});
