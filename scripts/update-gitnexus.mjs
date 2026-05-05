#!/usr/bin/env node
/**
 * update-gitnexus.mjs
 *
 * Rebuilds the GitNexus index.
 * On macOS/Linux: runs natively.
 * On Windows: uses WSL (LadybugDB WAL incompatibility).
 */

import { execSync } from "child_process";
import { resolve } from "path";

const REPO_ROOT = resolve(".");
const FORCE = process.argv.includes("--force");
const IS_WINDOWS = process.platform === "win32";

function main() {
  const forceFlag = FORCE ? " --force" : "";

  if (IS_WINDOWS) {
    const DISTRO = process.env.GITNEXUS_WSL_DISTRO || "Ubuntu";
    let wslPath;
    try {
      wslPath = execSync(
        `wsl -d ${DISTRO} -e wslpath -u "${REPO_ROOT}"`,
        { encoding: "utf-8" }
      ).trim();
    } catch {
      console.error(`❌ Failed to convert path to WSL format.`);
      process.exit(1);
    }

    console.log(`🔄 Updating GitNexus index via WSL (${DISTRO})...`);
    execSync(
      `wsl -d ${DISTRO} -e bash -lc "cd '${wslPath.replace(/'/g, "'\"'\"'")}' && npx -y gitnexus@1.6.3 analyze${forceFlag}"`,
      { stdio: "inherit", cwd: REPO_ROOT }
    );
  } else {
    console.log("🔄 Updating GitNexus index...");
    execSync(
      `npx -y gitnexus@1.6.3 analyze${forceFlag}`,
      { stdio: "inherit", cwd: REPO_ROOT }
    );
  }

  console.log("✅ GitNexus index updated successfully");
}

main();
