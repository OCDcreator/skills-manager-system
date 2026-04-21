import { execFileSync } from 'node:child_process';
import path from 'node:path';
import fs from 'node:fs';

import {
  autoDetectRange,
  readEffectiveNameStatus,
  readGitDiffNameStatus,
  recordPaths,
  sourcePathsFromRecord,
} from './module-doc-guard-diff.mjs';
import {
  isInsideRoot,
  matchesAny,
  normalizeRepoPath,
  toPosix,
  walkFiles,
} from './module-doc-guard-shared.mjs';

export { normalizeRepoPath, toPosix } from './module-doc-guard-shared.mjs';
export { autoDetectRange, readEffectiveNameStatus, readGitDiffNameStatus } from './module-doc-guard-diff.mjs';

export function parseArgs(argv = process.argv.slice(2)) {
  const args = {};

  for (let index = 0; index < argv.length; index += 1) {
    const item = argv[index];
    if (!item.startsWith('--')) {
      continue;
    }

    const [rawKey, inlineValue] = item.slice(2).split('=', 2);
    if (inlineValue !== undefined) {
      args[rawKey] = inlineValue;
      continue;
    }

    const next = argv[index + 1];
    if (next && !next.startsWith('--')) {
      args[rawKey] = next;
      index += 1;
    } else {
      args[rawKey] = true;
    }
  }

  return args;
}

export function repoRoot(cwd = process.cwd()) {
  return execFileSync('git', ['rev-parse', '--show-toplevel'], {
    cwd,
    encoding: 'utf8',
  }).trim();
}

export function loadConfig(root, configPath = 'module-docs.config.json') {
  const resolved = path.isAbsolute(configPath)
    ? configPath
    : path.join(root, configPath);

  if (!fs.existsSync(resolved)) {
    throw new Error(`Missing module docs config: ${toPosix(path.relative(root, resolved))}`);
  }

  const parsed = JSON.parse(fs.readFileSync(resolved, 'utf8'));
  const groups = Array.isArray(parsed.groups) ? parsed.groups : [];

  if (groups.length === 0) {
    throw new Error('module-docs.config.json must define at least one group');
  }

  return {
    version: parsed.version ?? 1,
    groups: groups.map((group, index) => normalizeGroup(group, index)),
  };
}

function normalizeGroup(group, index) {
  const sourceRoot = normalizeRepoPath(group.sourceRoot ?? '');
  const docsRoot = normalizeRepoPath(group.docsRoot ?? '');

  if (!sourceRoot || !docsRoot) {
    throw new Error(`Config group ${index} must define sourceRoot and docsRoot`);
  }

  return {
    name: group.name ?? `group-${index + 1}`,
    sourceRoot,
    docsRoot,
    include: normalizePatterns(group.include?.length ? group.include : ['**/*']),
    exclude: normalizePatterns(group.exclude ?? []),
    docIgnore: normalizePatterns(group.docIgnore ?? []),
    exactMappings: normalizeExactMappings(group.exactMappings ?? {}),
  };
}

function normalizePatterns(patterns) {
  return patterns.map((pattern) => normalizeRepoPath(pattern));
}

function normalizeExactMappings(mappings) {
  return Object.fromEntries(
    Object.entries(mappings).map(([source, doc]) => [
      normalizeRepoPath(source),
      normalizeRepoPath(doc),
    ]),
  );
}

export function collectSourceMappings(root, config) {
  const mappings = [];

  for (const group of config.groups) {
    for (const sourcePath of collectSourceFilesForGroup(root, group)) {
      mappings.push({
        group,
        sourcePath,
        docPath: mapSourceToDoc(group, sourcePath),
      });
    }
  }

  return mappings;
}

export function collectSourceFilesForGroup(root, group) {
  const absoluteRoot = path.join(root, group.sourceRoot);
  if (!fs.existsSync(absoluteRoot)) {
    return [];
  }

  return walkFiles(absoluteRoot)
    .map((absolutePath) => normalizeRepoPath(path.relative(root, absolutePath)))
    .filter((sourcePath) => sourceBelongsToGroup(group, sourcePath))
    .sort();
}

export function collectDocFiles(root, config) {
  const docsRootToGroups = new Map();

  for (const group of config.groups) {
    const groups = docsRootToGroups.get(group.docsRoot) ?? [];
    groups.push(group);
    docsRootToGroups.set(group.docsRoot, groups);
  }

  const docs = [];

  for (const [docsRoot, groups] of docsRootToGroups.entries()) {
    const absoluteRoot = path.join(root, docsRoot);
    if (!fs.existsSync(absoluteRoot)) {
      continue;
    }

    for (const absolutePath of walkFiles(absoluteRoot)) {
      if (!absolutePath.endsWith('.md')) {
        continue;
      }

      const docPath = normalizeRepoPath(path.relative(root, absolutePath));
      const ignored = groups.some((group) => docIgnoredByGroup(group, docPath));

      if (!ignored) {
        docs.push(docPath);
      }
    }
  }

  return [...new Set(docs)].sort();
}

export function sourceBelongsToGroup(group, sourcePath) {
  const normalizedSource = normalizeRepoPath(sourcePath);
  const explicit = Object.prototype.hasOwnProperty.call(group.exactMappings, normalizedSource);

  if (!explicit && !isInsideRoot(normalizedSource, group.sourceRoot)) {
    return false;
  }

  const relative = explicit
    ? normalizeRepoPath(path.posix.relative(group.sourceRoot, normalizedSource))
    : normalizedSource.slice(group.sourceRoot.length + 1);

  return matchesAny(relative, group.include) && !matchesAny(relative, group.exclude);
}

export function findGroupForSource(config, sourcePath) {
  const normalizedSource = normalizeRepoPath(sourcePath);
  return config.groups.find((group) => sourceBelongsToGroup(group, normalizedSource)) ?? null;
}

export function mapSourceToDoc(group, sourcePath) {
  const normalizedSource = normalizeRepoPath(sourcePath);
  const exact = group.exactMappings[normalizedSource];

  if (exact) {
    return exact;
  }

  const relative = normalizeRepoPath(path.posix.relative(group.sourceRoot, normalizedSource));
  const docRelative = relative.replace(/\.[^/.]+$/, '.md');
  return normalizeRepoPath(`${group.docsRoot}/${docRelative}`);
}

export function requiredDocsFromDiff(config, diffRecords) {
  const changedPaths = new Map();
  const requiredDocs = new Map();

  for (const record of diffRecords) {
    for (const diffPath of recordPaths(record)) {
      changedPaths.set(diffPath, record.status);
    }

    for (const sourcePath of sourcePathsFromRecord(record)) {
      const group = findGroupForSource(config, sourcePath);
      if (!group) {
        continue;
      }

      const docPath = mapSourceToDoc(group, sourcePath);
      const existing = requiredDocs.get(docPath);
      requiredDocs.set(docPath, {
        docPath,
        groupName: group.name,
        sourcePaths: [...new Set([...(existing?.sourcePaths ?? []), sourcePath])],
        statuses: [...new Set([...(existing?.statuses ?? []), record.status])],
      });
    }
  }

  return {
    changedPaths,
    requiredDocs: [...requiredDocs.values()].sort((left, right) => left.docPath.localeCompare(right.docPath)),
  };
}

export function aggregateDocsFromRequirements(requirements) {
  const aggregateDocs = new Set();

  for (const requirement of requirements) {
    for (const sourcePath of requirement.sourcePaths) {
      const group = requirement.group;
      if (!group) {
        continue;
      }

      const relative = normalizeRepoPath(path.posix.relative(group.sourceRoot, sourcePath));
      const parent = normalizeRepoPath(path.posix.dirname(relative));
      const base = path.posix.basename(relative);
      const topologyChanged = requirement.statuses.some((status) => /^(A|D|R|C)/.test(status));

      if (topologyChanged) {
        aggregateDocs.add(`${group.docsRoot}/README.md`);
      }

      if (topologyChanged || /^index\.[^.]+$/.test(base)) {
        if (parent && parent !== '.') {
          aggregateDocs.add(`${group.docsRoot}/${parent}/index.md`);
        }
      }
    }
  }

  return [...aggregateDocs].sort();
}

export function printList(title, items, formatter = (item) => `- ${item}`) {
  if (items.length === 0) {
    return;
  }

  console.error(title);
  for (const item of items) {
    console.error(formatter(item));
  }
}

function docIgnoredByGroup(group, docPath) {
  if (!isInsideRoot(docPath, group.docsRoot)) {
    return false;
  }

  const relative = normalizeRepoPath(path.posix.relative(group.docsRoot, docPath));
  return matchesAny(relative, group.docIgnore);
}
