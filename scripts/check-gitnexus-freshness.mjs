#!/usr/bin/env node
/**
 * check-gitnexus-freshness.mjs
 *
 * Checks if the GitNexus index is up-to-date with the latest git commit.
 * Reads .gitnexus/meta.json lastCommit and compares with git rev-parse HEAD.
 * Fails (exit 1) if the index is stale.
 *
 * Usage: node scripts/check-gitnexus-freshness.mjs
 */

import { execSync } from "child_process";
import { existsSync, readFileSync } from "fs";
import { resolve } from "path";

const GITNEXUS_DIR = resolve(".gitnexus");
const META_PATH = resolve(".gitnexus/meta.json");
const REPO_ROOT = resolve(".");

function getCurrentHead() {
  try {
    return execSync("git rev-parse HEAD", { cwd: REPO_ROOT, encoding: "utf-8" }).trim();
  } catch {
    console.error("❌ Failed to get current git HEAD. Is this a git repository?");
    process.exit(1);
  }
}

function getMetaCommit() {
  if (!existsSync(META_PATH)) return null;
  try {
    const meta = JSON.parse(readFileSync(META_PATH, "utf-8"));
    return meta.lastCommit || null;
  } catch {
    return null;
  }
}

function getMetaIndexedAt() {
  if (!existsSync(META_PATH)) return null;
  try {
    const meta = JSON.parse(readFileSync(META_PATH, "utf-8"));
    return meta.indexedAt || null;
  } catch {
    return null;
  }
}

function main() {
  if (!existsSync(GITNEXUS_DIR)) {
    console.error("❌ GitNexus index not found at .gitnexus/");
    console.error("   Run:  npm run update:gitnexus");
    process.exit(1);
  }

  const currentHead = getCurrentHead();
  const metaCommit = getMetaCommit();
  const indexedAt = getMetaIndexedAt();

  if (!metaCommit) {
    console.error("❌ GitNexus meta.json missing or corrupt (no lastCommit field)");
    console.error("   Run:  npm run update:gitnexus");
    process.exit(1);
  }

  if (metaCommit !== currentHead) {
    console.error(`❌ GitNexus index is stale`);
    console.error(`   Index commit:  ${metaCommit.slice(0, 8)}`);
    console.error(`   Current HEAD:  ${currentHead.slice(0, 8)}`);
    console.error(`   Last indexed:  ${indexedAt || "unknown"}`);
    console.error("   Run:  npm run update:gitnexus");
    process.exit(1);
  }

  console.log(`✅ GitNexus index is fresh (indexed at ${indexedAt}, commit ${currentHead.slice(0, 8)})`);
  process.exit(0);
}

main();
