import { readFileSync, writeFileSync } from "fs";

const version = process.argv[2]?.replace(/^v/, "");
if (!version) {
  console.error("Usage: node sync-version.mjs <version>");
  process.exit(1);
}

// package.json
const pkg = JSON.parse(readFileSync("package.json", "utf8"));
pkg.version = version;
writeFileSync("package.json", JSON.stringify(pkg, null, 2) + "\n");

// Cargo.toml
let cargo = readFileSync("src-tauri/Cargo.toml", "utf8");
cargo = cargo.replace(/^version = ".*"/m, `version = "${version}"`);
writeFileSync("src-tauri/Cargo.toml", cargo);

// tauri.conf.json
const tauriConf = JSON.parse(readFileSync("src-tauri/tauri.conf.json", "utf8"));
tauriConf.version = version;
writeFileSync("src-tauri/tauri.conf.json", JSON.stringify(tauriConf, null, 2) + "\n");

console.log(`Synced version to ${version}`);
