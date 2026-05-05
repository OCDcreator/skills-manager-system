#!/usr/bin/env node
/**
 * update-gitnexus.mjs
 *
 * Rebuilds the GitNexus index using WSL (required because LadybugDB
 * WAL is incompatible with Windows native filesystem).
 *
 * Supports GITNEXUS_WSL_DISTRO env var (default: Ubuntu).
 *
 * Usage: node scripts/update-gitnexus.mjs [--force]
 */

import { execSync } from "child_process";
import { resolve } from "path";

const REPO_ROOT = resolve(".");
const FORCE = process.argv.includes("--force");
const DISTRO = process.env.GITNEXUS_WSL_DISTRO || "Ubuntu";

function main() {
  const forceFlag = FORCE ? " --force" : "";

  // Convert Windows path to WSL path safely
  let wslPath;
  try {
    wslPath = execSync(
      `wsl -d ${DISTRO} -e wslpath -u "${REPO_ROOT}"`,
      { encoding: "utf-8" }
    ).trim();
  } catch {
    console.error(`❌ Failed to convert path to WSL format. Is WSL ${DISTRO} installed?`);
    process.exit(1);
  }

  console.log(`🔄 Updating GitNexus index via WSL (${DISTRO})...`);
  console.log(`   WSL path: ${wslPath}`);

  try {
    execSync(
      `wsl -d ${DISTRO} -e bash -lc "cd ${escapeShellArg(wslPath)} && npx -y gitnexus@1.6.3 analyze${forceFlag}"`,
      { stdio: "inherit", cwd: REPO_ROOT }
    );
    console.log("✅ GitNexus index updated successfully");
  } catch (err) {
    console.error("❌ Failed to update GitNexus index");
    console.error("   Make sure WSL Ubuntu is installed and gitnexus is available in WSL");
    process.exit(1);
  }
}

/**
 * Escape a string for safe use in bash single-quoted strings.
 */
function escapeShellArg(arg) {
  return "'" + arg.replace(/'/g, "'\"'\"'") + "'";
}

main();
