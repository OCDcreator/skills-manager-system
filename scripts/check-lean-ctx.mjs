#!/usr/bin/env node
/**
 * check-lean-ctx.mjs
 *
 * Verifies lean-ctx is installed and functional.
 * This is a lightweight check — lean-ctx updates itself automatically
 * via shell hooks when you cd between directories.
 *
 * Usage: node scripts/check-lean-ctx.mjs
 */

import { execSync } from "child_process";

function main() {
  try {
    const version = execSync("lean-ctx --version", { encoding: "utf-8" }).trim();
    console.log(`✅ lean-ctx is installed (${version})`);
    console.log("   lean-ctx updates automatically via shell hooks when you change directories.");
    console.log("   If hooks aren't active, run:  . $PROFILE  (PowerShell)");
    process.exit(0);
  } catch {
    console.error("❌ lean-ctx is not installed or not in PATH");
    console.error("   Install:  cargo install lean-ctx");
    console.error("   Then restart your shell or run:  . $PROFILE");
    process.exit(1);
  }
}

main();
