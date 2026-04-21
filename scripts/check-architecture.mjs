import fs from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const SOURCE_EXTENSIONS = new Set([
  '.ts',
  '.tsx',
  '.js',
  '.jsx',
  '.mjs',
  '.cjs',
  '.rs',
]);

const EXCLUDED_DIRECTORIES = new Set([
  '.git',
  '.next',
  '.turbo',
  '.vscode',
  'build',
  'coverage',
  'dist',
  'node_modules',
  'target',
]);

const BUCKET_FILE_NAMES = new Set([
  'common',
  'helper',
  'helpers',
  'misc',
  'shared',
  'util',
  'utils',
]);

const EXPLICIT_BOUNDARY_DIRECTORIES = new Set([
  'scripts',
  'src',
  'src/components',
  'src/context',
  'src/i18n',
  'src/lib',
  'src/views',
  'src-tauri',
  'src-tauri/src',
  'src-tauri/src/commands',
  'src-tauri/src/core',
  'test',
  'tests',
]);

const DOMAIN_BOUNDARY_NAMES = new Set(['agents', 'git', 'projects', 'scenes', 'skills']);

const LINE_LIMITS = {
  command: { warning: 200, error: 300 },
  entry: { warning: 180, error: 260 },
  script: { warning: 380, error: 520 },
  source: { warning: 320, error: 480 },
  view: { warning: 350, error: 480 },
};

function toPosix(relativePath) {
  return relativePath.split(path.sep).join('/');
}

function isSourceFile(fileName) {
  const extension = path.extname(fileName);
  return SOURCE_EXTENSIONS.has(extension) && !fileName.endsWith('.d.ts');
}

function isExcludedDirectory(directoryName) {
  return EXCLUDED_DIRECTORIES.has(directoryName);
}

function isSourceRoot(relativePath) {
  return (
    relativePath.startsWith('scripts/') ||
    relativePath.startsWith('src/') ||
    relativePath.startsWith('src-tauri/src/')
  );
}

function getCategory(relativePath) {
  if (relativePath.startsWith('scripts/')) {
    return 'script';
  }

  if (
    relativePath === 'src/App.tsx' ||
    relativePath === 'src/main.tsx' ||
    relativePath === 'src-tauri/src/lib.rs' ||
    relativePath === 'src-tauri/src/main.rs'
  ) {
    return 'entry';
  }

  if (relativePath.startsWith('src/views/')) {
    return 'view';
  }

  if (relativePath.startsWith('src-tauri/src/commands/')) {
    return 'command';
  }

  return 'source';
}

function countLines(text) {
  if (!text) {
    return 0;
  }

  return text.split(/\r\n|\r|\n/).length;
}

function getLogicalLines(text) {
  return text
    .split(/\r\n|\r|\n/)
    .map((line) => line.trim())
    .filter((line) => line.length > 0)
    .filter((line) => !line.startsWith('//'))
    .filter((line) => !line.startsWith('/*'))
    .filter((line) => !line.startsWith('*'))
    .filter((line) => line !== '*/');
}

function getBaseNameWithoutExtension(relativePath) {
  return path.basename(relativePath, path.extname(relativePath)).toLowerCase();
}

function isStableBoundaryFile(relativePath) {
  return path.posix.basename(relativePath) === 'mod.rs' || relativePath === 'src-tauri/src/main.rs';
}

function isBucketFileName(relativePath) {
  if (!isSourceRoot(relativePath)) {
    return false;
  }

  return BUCKET_FILE_NAMES.has(getBaseNameWithoutExtension(relativePath));
}

function isPurePassthroughModule(logicalLines) {
  if (logicalLines.length === 0 || logicalLines.length > 8) {
    return false;
  }

  return logicalLines.every(
    (line) =>
      /^import\s/.test(line) ||
      /^export\s+\*\s+from\s+/.test(line) ||
      /^export\s+\{.+\}\s+from\s+/.test(line) ||
      /^export\s+type\s+\{.+\}\s+from\s+/.test(line),
  );
}

function isExplicitBoundaryDirectory(relativeDir) {
  if (EXPLICIT_BOUNDARY_DIRECTORIES.has(relativeDir)) {
    return true;
  }

  const segments = relativeDir.split('/').filter(Boolean);
  const lastSegment = segments.at(-1);
  return DOMAIN_BOUNDARY_NAMES.has(lastSegment);
}

function walkDirectory(rootDir, currentDir, files) {
  const entries = fs.readdirSync(currentDir, { withFileTypes: true });

  for (const entry of entries) {
    if (entry.isDirectory()) {
      if (isExcludedDirectory(entry.name)) {
        continue;
      }

      walkDirectory(rootDir, path.join(currentDir, entry.name), files);
      continue;
    }

    if (!entry.isFile() || !isSourceFile(entry.name)) {
      continue;
    }

    const fullPath = path.join(currentDir, entry.name);
    const relativePath = toPosix(path.relative(rootDir, fullPath));

    files.push(relativePath);
  }
}

function collectSourceFiles(rootDir) {
  const files = [];
  walkDirectory(rootDir, rootDir, files);
  return files.sort();
}

function createIssue(severity, code, file, message) {
  return { code, file, message, severity };
}

function analyzeFiles(rootDir) {
  const warnings = [];
  const errors = [];
  const fileInfos = [];

  for (const relativePath of collectSourceFiles(rootDir)) {
    const fullPath = path.join(rootDir, relativePath);
    const content = fs.readFileSync(fullPath, 'utf8');
    const totalLines = countLines(content);
    const logicalLines = getLogicalLines(content);
    const category = getCategory(relativePath);
    const limits = LINE_LIMITS[category];
    const directory = path.posix.dirname(relativePath);

    fileInfos.push({
      category,
      directory,
      logicalLineCount: logicalLines.length,
      relativePath,
      totalLines,
    });

    if (totalLines > limits.error) {
      errors.push(
        createIssue(
          'error',
          'file-too-large',
          relativePath,
          `File has ${totalLines} lines; hard limit for ${category} files is ${limits.error}.`,
        ),
      );
    } else if (totalLines > limits.warning) {
      warnings.push(
        createIssue(
          'warning',
          'file-near-limit',
          relativePath,
          `File has ${totalLines} lines; warning limit for ${category} files is ${limits.warning}.`,
        ),
      );
    }

    if (isBucketFileName(relativePath)) {
      errors.push(
        createIssue(
          'error',
          'bucket-file-name',
          relativePath,
          'Bucket-style file names are forbidden; rename this file to a domain-specific name.',
        ),
      );
    }

    if (isPurePassthroughModule(logicalLines)) {
      errors.push(
        createIssue(
          'error',
          'passthrough-module',
          relativePath,
          'Pure import/export shells are not allowed; merge this file back into its owning module.',
        ),
      );
    } else if (logicalLines.length > 0 && logicalLines.length < 8 && isSourceRoot(relativePath) && !isStableBoundaryFile(relativePath)) {
      warnings.push(
        createIssue(
          'warning',
          'thin-module',
          relativePath,
          'Very small source file; confirm this is a stable module instead of a mechanical split.',
        ),
      );
    }
  }

  return { errors, fileInfos, warnings };
}

function analyzeDirectories(fileInfos) {
  const errors = [];
  const directories = new Map();

  for (const info of fileInfos) {
    if (!isSourceRoot(`${info.directory}/placeholder.ts`)) {
      continue;
    }

    const directory = info.directory;
    if (!directories.has(directory)) {
      directories.set(directory, []);
    }

    directories.get(directory).push(info);
  }

  for (const [directory, entries] of directories.entries()) {
    if (directory === '.' || isExplicitBoundaryDirectory(directory)) {
      continue;
    }

    const hasNestedSourceDirectory = fileInfos.some(
      (candidate) =>
        candidate.directory.startsWith(`${directory}/`) && candidate.directory !== directory,
    );

    const logicalLineTotal = entries.reduce(
      (sum, entry) => sum + entry.logicalLineCount,
      0,
    );

    if (!hasNestedSourceDirectory && entries.length <= 2 && logicalLineTotal < 80) {
      errors.push(
        createIssue(
          'error',
          'shallow-module-directory',
          directory,
          'Directory is too shallow to justify its own level; inline it or expand it into a real module boundary.',
        ),
      );
    }
  }

  return errors;
}

export function runArchitectureCheck(rootDir = process.cwd()) {
  const absoluteRoot = path.resolve(rootDir);
  const { errors, fileInfos, warnings } = analyzeFiles(absoluteRoot);
  const directoryErrors = analyzeDirectories(fileInfos);

  return {
    errors: [...errors, ...directoryErrors],
    filesScanned: fileInfos.length,
    rootDir: absoluteRoot,
    warnings,
  };
}

export function formatArchitectureReport(result) {
  const sections = [
    `Architecture check scanned ${result.filesScanned} source files.`,
  ];

  if (result.warnings.length > 0) {
    sections.push(
      ['Warnings:', ...result.warnings.map((issue) => `- [${issue.code}] ${issue.file}: ${issue.message}`)].join('\n'),
    );
  }

  if (result.errors.length > 0) {
    sections.push(
      ['Errors:', ...result.errors.map((issue) => `- [${issue.code}] ${issue.file}: ${issue.message}`)].join('\n'),
    );
  } else {
    sections.push('No blocking architecture issues found.');
  }

  return sections.join('\n\n');
}

function isDirectExecution() {
  const entryPath = process.argv[1];
  if (!entryPath) {
    return false;
  }

  return import.meta.url === pathToFileURL(path.resolve(entryPath)).href;
}

if (isDirectExecution()) {
  const rootDir = process.argv[2] ?? process.cwd();
  const result = runArchitectureCheck(rootDir);

  console.log(formatArchitectureReport(result));
  process.exitCode = result.errors.length > 0 ? 1 : 0;
}
