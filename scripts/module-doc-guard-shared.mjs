import fs from 'node:fs';
import path from 'node:path';

export function toPosix(value) {
  return value.replace(/\\/g, '/').replace(/^\.\//, '').replace(/\/+/g, '/');
}

export function normalizeRepoPath(value) {
  return toPosix(value).replace(/^\/+/, '').replace(/\/$/, '');
}

export function walkFiles(root) {
  const result = [];
  const stack = [root];

  while (stack.length > 0) {
    const current = stack.pop();
    const entries = fs.readdirSync(current, { withFileTypes: true });

    for (const entry of entries) {
      const fullPath = path.join(current, entry.name);

      if (entry.isDirectory()) {
        stack.push(fullPath);
      } else if (entry.isFile()) {
        result.push(fullPath);
      }
    }
  }

  return result;
}

export function isInsideRoot(filePath, root) {
  return filePath === root || filePath.startsWith(`${root}/`);
}

export function matchesAny(value, patterns) {
  return patterns.some((pattern) => globToRegExp(pattern).test(value));
}

function globToRegExp(glob) {
  let source = '^';

  for (let index = 0; index < glob.length; index += 1) {
    const char = glob[index];
    const next = glob[index + 1];

    if (char === '*') {
      if (next === '*') {
        const following = glob[index + 2];
        if (following === '/') {
          source += '(?:.*/)?';
          index += 2;
        } else {
          source += '.*';
          index += 1;
        }
      } else {
        source += '[^/]*';
      }
      continue;
    }

    if (char === '?') {
      source += '[^/]';
      continue;
    }

    source += escapeRegex(char);
  }

  source += '$';
  return new RegExp(source);
}

function escapeRegex(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}
