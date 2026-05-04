import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";
import React from "react";
import { renderToStaticMarkup } from "react-dom/server";
import ts from "typescript";

function readSource(relativePath) {
  return fs.readFileSync(path.resolve(relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(readSource(relativePath));
}

async function loadRenderableAppShell() {
  let tempDir = "";
  const sourcePath = path.resolve("src/components/AppShell.tsx");
  const source = readSource(sourcePath)
    .replace(
      'import { useTranslation } from "react-i18next";',
      'import { useTranslation } from "./mock-i18n.mjs";',
    )
    // Test-only union keeps AppShell independently transpirable; business source stays in AppContext.
    .replace(
      'import { useAppContext, type AppView } from "../context/AppContext";',
      'import { useAppContext } from "./mock-app-context.mjs";\ntype AppView = "skills" | "agents" | "git" | "scenes" | "projects" | "sources" | "settings";',
    )
    .replace(
      'import { ProjectAssistantLauncher } from "./assistant/ProjectAssistantLauncher";',
      'import { ProjectAssistantLauncher } from "./mock-project-assistant-launcher.mjs";',
    )
    .replace(
      'import { UnsavedChangesDialog } from "./UnsavedChangesDialog";',
      'import { UnsavedChangesDialog } from "./mock-unsaved-dialog.mjs";',
    );

  const tempRoot = path.resolve("node_modules/.cache");
  fs.mkdirSync(tempRoot, { recursive: true });
  tempDir = fs.mkdtempSync(path.join(tempRoot, "app-shell-design-test-"));
  fs.writeFileSync(
    path.join(tempDir, "mock-i18n.mjs"),
    'export function useTranslation() { return { t: (key) => key }; }\n',
    "utf8",
  );
  fs.writeFileSync(
    path.join(tempDir, "mock-app-context.mjs"),
    "let context = {};\nexport function setMockContext(next) { context = next; }\nexport function useAppContext() { return context; }\n",
    "utf8",
  );
  fs.writeFileSync(
    path.join(tempDir, "mock-project-assistant-launcher.mjs"),
    'import React from "react";\nexport function ProjectAssistantLauncher() { return React.createElement("div", { "data-assistant-launcher": "true" }); }\n',
    "utf8",
  );
  fs.writeFileSync(
    path.join(tempDir, "mock-unsaved-dialog.mjs"),
    'import React from "react";\nexport function UnsavedChangesDialog({ pendingNavigation }) { return React.createElement("div", { "data-unsaved-dialog": "true" }, pendingNavigation?.targetView ?? "pending"); }\n',
    "utf8",
  );

  const transpiled = ts.transpileModule(source, {
    compilerOptions: {
      jsx: ts.JsxEmit.ReactJSX,
      module: ts.ModuleKind.ESNext,
      target: ts.ScriptTarget.ES2020,
    },
    fileName: sourcePath,
  });
  const modulePath = path.join(tempDir, "AppShell.mjs");
  fs.writeFileSync(modulePath, transpiled.outputText, "utf8");

  const [module, contextModule] = await Promise.all([
    import(pathToFileURL(modulePath).href),
    import(pathToFileURL(path.join(tempDir, "mock-app-context.mjs")).href),
  ]);
  return {
    AppShell: module.AppShell,
    cleanup: () => fs.rmSync(tempDir, { force: true, recursive: true }),
    setMockContext: contextModule.setMockContext,
  };
}

function collectReactElements(node, predicate, matches = []) {
  if (node === null || node === undefined || typeof node === "boolean") {
    return matches;
  }

  if (Array.isArray(node)) {
    for (const child of node) {
      collectReactElements(child, predicate, matches);
    }
    return matches;
  }

  if (typeof node !== "object") {
    return matches;
  }

  if (predicate(node)) {
    matches.push(node);
  }

  collectReactElements(node.props?.children, predicate, matches);
  return matches;
}

function reactText(node) {
  if (node === null || node === undefined || typeof node === "boolean") {
    return "";
  }

  if (Array.isArray(node)) {
    return node.map(reactText).join("");
  }

  if (typeof node === "string" || typeof node === "number") {
    return String(node);
  }

  if (typeof node !== "object") {
    return "";
  }

  return reactText(node.props?.children);
}

test("AppShell renders preserved shell behavior through the DOM contract", async () => {
  const { AppShell, cleanup, setMockContext } = await loadRenderableAppShell();
  try {
    setMockContext({
      activeView: "sources",
      cancelNavigation: () => undefined,
      confirmNavigationDiscard: () => undefined,
      confirmNavigationSave: () => undefined,
      errorMessage: "Repo path is missing",
      pendingNavigation: { targetView: "settings" },
      setActiveView: () => undefined,
    });

    const markup = renderToStaticMarkup(
      React.createElement(AppShell, { contentWidthClassName: "max-w-readable" }, "Shell body"),
    );

    assert.match(markup, /class="app-shell"/);
    assert.match(markup, /data-app-shell-sidebar="true"/);
    assert.match(markup, /data-app-shell-mobile-header="true"/);
    assert.match(markup, /class="[^"]*\bapp-shell__nav-icon\b[^"]*"/);
    assert.match(markup, /aria-current="page"/);
    assert.match(markup, /role="alert"/);
    assert.match(markup, /Repo path is missing/);
    assert.match(markup, /data-assistant-launcher="true"/);
    assert.match(markup, /data-unsaved-dialog="true"/);
    assert.match(markup, /max-w-readable/);
    assert.match(markup, /data-app-shell-main="true"/);
    assert.match(markup, /nav\.sources/);
    assert.match(markup, /aria-label="app\.nav\.desktopLabel"/);
    assert.match(markup, /aria-label="app\.nav\.mobileLabel"/);
    assert.doesNotMatch(markup, /<nav[^>]+aria-label="app\.title"[^>]*>/);
  } finally {
    cleanup();
  }
});

test("AppShell navigation landmark labels are localized", () => {
  const componentSource = readSource("src/components/AppShell.tsx");
  const englishStrings = readJson("src/i18n/en.json");
  const chineseStrings = readJson("src/i18n/zh.json");

  assert.match(componentSource, /t\("app\.nav\.desktopLabel", \{ title: appTitle \}\)/);
  assert.match(componentSource, /t\("app\.nav\.mobileLabel", \{ title: appTitle \}\)/);

  for (const key of ["app.nav.desktopLabel", "app.nav.mobileLabel"]) {
    assert.equal(typeof englishStrings[key], "string", `missing English ${key}`);
    assert.equal(typeof chineseStrings[key], "string", `missing Chinese ${key}`);
    assert.match(englishStrings[key], /\{\{title\}\}/);
    assert.match(chineseStrings[key], /\{\{title\}\}/);
  }

  assert.doesNotMatch(componentSource, /desktop navigation/);
  assert.doesNotMatch(componentSource, /mobile navigation/);
});

test("AppShell dispatches guarded navigation from rendered nav buttons", async () => {
  const { AppShell, cleanup, setMockContext } = await loadRenderableAppShell();
  const requestedViews = [];

  try {
    setMockContext({
      activeView: "skills",
      cancelNavigation: () => undefined,
      confirmNavigationDiscard: () => undefined,
      confirmNavigationSave: () => undefined,
      errorMessage: "",
      pendingNavigation: null,
      setActiveView: (view) => requestedViews.push(view),
    });

    const tree = AppShell({ children: "Shell body" });
    const navButtons = collectReactElements(
      tree,
      (element) => element.type === "button" && element.props?.className === "app-shell__nav-item",
    );
    const labels = navButtons.map(reactText);

    // Seven desktop nav buttons plus seven mobile nav buttons share one nav item contract.
    assert.equal(navButtons.length, 14);
    assert.equal(labels.filter((label) => label === "nav.skills").length, 2);
    assert.equal(labels.filter((label) => label === "nav.sources").length, 2);

    const activeButtons = navButtons.filter((button) => button.props["aria-current"] === "page");
    assert.equal(activeButtons.length, 2);
    assert.deepEqual(activeButtons.map(reactText), ["nav.skills", "nav.skills"]);

    const sourceButton = navButtons.find((button) => reactText(button) === "nav.sources");
    const settingsButton = navButtons.find((button) => reactText(button) === "nav.settings");
    assert.equal(sourceButton.props.title, "tooltip.nav.sources");
    assert.equal(settingsButton.props.title, "tooltip.nav.settings");

    sourceButton.props.onClick();
    settingsButton.props.onClick();
    assert.deepEqual(requestedViews, ["sources", "settings"]);
  } finally {
    cleanup();
  }
});

test("AppShell no longer owns the old top-nav dark slate shell shape", () => {
  const source = readSource("src/components/AppShell.tsx");

  assert.doesNotMatch(source, /bg-slate-950/);
  assert.doesNotMatch(source, /data-app-shell-header/);
  assert.doesNotMatch(source, /<nav className="flex gap-2"/);
  assert.doesNotMatch(source, /NAV_ITEMS: AppView\[\]/);
});

test("AppShell keeps the short brand mark centralized", () => {
  const source = readSource("src/components/AppShell.tsx");

  assert.match(source, /const BRAND_MARK_TEXT = "SM";/);
  assert.equal((source.match(/>SM<\/div>/g) ?? []).length, 0);
});

test("foundation exposes the tinted neutral workbench palette and layout primitives", () => {
  const source = readSource("src/styles/foundation.css");

  const requiredTokens = [
    "--color-workbench-bg",
    "--color-workbench-panel",
    "--color-workbench-panel-strong",
    "--color-workbench-border",
    "--color-workbench-text",
    "--color-workbench-muted",
    "--color-workbench-accent",
  ];

  for (const token of requiredTokens) {
    assert.match(source, new RegExp(`${token}:\\s*oklch\\(`), `missing OKLCH token ${token}`);
  }

  assert.match(source, /font-family:\s*"Aptos"/);
  assert.match(source, /"SF Pro Text"/);
  assert.match(source, /\.app-shell\s*\{/);
  assert.match(source, /\.app-shell__sidebar\s*\{/);
  assert.match(source, /\.app-shell__mobile-header\s*\{/);
  assert.match(source, /\.app-shell__nav-item\s*\{/);
  assert.match(source, /\.app-shell__nav-item:focus-visible\s*\{/);
  assert.match(source, /outline:\s*2px solid var\(--color-workbench-accent\)/);
  assert.match(source, /\.app-shell__content\s*\{/);
  assert.match(source, /@media\s*\(min-width:\s*768px\)/);
  assert.match(source, /align-self:\s*start/);
  assert.match(source, /height:\s*100vh/);
  assert.match(source, /overflow-y:\s*auto/);
  assert.match(source, /scrollbar-width:\s*none/);
  assert.match(source, /\.app-shell__mobile-nav::-webkit-scrollbar\s*\{/);
  assert.match(source, /intentional progressive enhancement before color-mix\(\)/);
  assert.doesNotMatch(source, /letter-spacing:\s*-/);
  assert.doesNotMatch(source, /font-weight:\s*750/);
  assert.doesNotMatch(source, /#020617/);
});
