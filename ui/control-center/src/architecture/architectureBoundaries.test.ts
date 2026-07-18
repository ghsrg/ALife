import { describe, expect, it } from 'vitest';
import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';

const srcRoot = join(process.cwd(), 'src');

function listFiles(dir: string): string[] {
  return readdirSync(dir).flatMap((entry) => {
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) {
      return listFiles(path);
    }
    return path.endsWith('.ts') || path.endsWith('.tsx') ? [path] : [];
  });
}

function source(path: string) {
  return readFileSync(path, 'utf8');
}

function isTestFile(path: string) {
  return path.endsWith('.test.ts') || path.endsWith('.test.tsx');
}

describe('architecture boundaries', () => {
  it('keeps Runner transport out of presentational components', () => {
    const componentFiles = listFiles(join(srcRoot, 'components'));
    const offenders = componentFiles
      .filter((path) => !isTestFile(path))
      .filter((path) => /import\s+(?!type)[\s\S]*?['"].*runner\//.test(source(path)))
      .map((path) => relative(srcRoot, path));

    expect(offenders).toEqual([]);
  });

  it('keeps Pixi imports behind the renderer boundary', () => {
    const offenders = listFiles(srcRoot)
      .filter((path) => !isTestFile(path))
      .filter((path) => !path.endsWith(join('viewer', 'worldRenderer.ts')))
      .filter((path) => source(path).includes("from 'pixi.js'"))
      .map((path) => relative(srcRoot, path));

    expect(offenders).toEqual([]);
  });

  it('keeps styles.css as an import hub', () => {
    const styles = source(join(srcRoot, 'styles.css'));

    expect(styles).toContain("@import './styles/tokens.css';");
    expect(styles).toContain("@import './styles/layout.css';");
    expect(styles).toContain("@import './styles/components.css';");
  });
});
