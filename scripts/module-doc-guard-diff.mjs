import { execFileSync } from 'node:child_process';

import { normalizeRepoPath } from './module-doc-guard-shared.mjs';

export function readGitDiffNameStatus(root, range) {
  const output = execGitText(root, ['diff', '--name-status', '--find-renames', range]);

  if (!output) {
    return [];
  }

  return output.split(/\r?\n/).map(parseDiffLine);
}

export function readEffectiveNameStatus(root, range) {
  return mergeDiffRecords(
    readGitDiffNameStatus(root, range),
    readWorkingTreeNameStatus(root),
  );
}

export function autoDetectRange(root) {
  if (process.env.MODULE_DOC_DIFF_RANGE) {
    return process.env.MODULE_DOC_DIFF_RANGE;
  }

  if (process.env.GITHUB_BASE_REF) {
    return `origin/${process.env.GITHUB_BASE_REF}...HEAD`;
  }

  const candidates = ['origin/main...HEAD', 'origin/master...HEAD', 'HEAD~1..HEAD'];

  for (const candidate of candidates) {
    try {
      execFileSync('git', ['merge-base', '--is-ancestor', candidate.split(/[.]{3}|[.]{2}/)[0], 'HEAD'], {
        cwd: root,
        stdio: 'ignore',
      });
      return candidate;
    } catch {
      try {
        execFileSync('git', ['diff', '--quiet', candidate], {
          cwd: root,
          stdio: 'ignore',
        });
        return candidate;
      } catch (error) {
        if (error.status === 1) {
          return candidate;
        }
      }
    }
  }

  throw new Error('Unable to detect diff range. Pass --range <base>...HEAD.');
}

export function recordPaths(record) {
  return [record.oldPath, record.path].filter(Boolean).map(normalizeRepoPath);
}

export function sourcePathsFromRecord(record) {
  return recordPaths(record);
}

function parseDiffLine(line) {
  const parts = line.split('\t');
  const status = parts[0];

  if (status.startsWith('R') || status.startsWith('C')) {
    return {
      status,
      oldPath: normalizeRepoPath(parts[1]),
      path: normalizeRepoPath(parts[2]),
    };
  }

  return {
    status,
    path: normalizeRepoPath(parts[1]),
  };
}

function readWorkingTreeNameStatus(root) {
  const trackedOutput = execGitText(root, ['diff', '--name-status', '--find-renames', 'HEAD']);
  const untrackedOutput = execGitText(root, ['ls-files', '--others', '--exclude-standard']);
  const trackedRecords = trackedOutput ? trackedOutput.split(/\r?\n/).map(parseDiffLine) : [];
  const untrackedRecords = untrackedOutput
    ? untrackedOutput.split(/\r?\n/).filter(Boolean).map((item) => ({
        status: 'A',
        path: normalizeRepoPath(item),
      }))
    : [];

  return mergeDiffRecords(trackedRecords, untrackedRecords);
}

function mergeDiffRecords(...recordLists) {
  const merged = [];
  const seen = new Set();

  for (const records of recordLists) {
    for (const record of records) {
      const key = [record.status, record.oldPath ?? '', record.path ?? ''].join('\t');
      if (seen.has(key)) {
        continue;
      }

      seen.add(key);
      merged.push(record);
    }
  }

  return merged;
}

function execGitText(root, args) {
  try {
    return execFileSync('git', args, {
      cwd: root,
      encoding: 'utf8',
    }).trim();
  } catch (error) {
    if (error.status === 1 && typeof error.stdout === 'string') {
      return error.stdout.trim();
    }

    if (error.status === 128) {
      return '';
    }

    throw error;
  }
}
