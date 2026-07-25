import { describe, expect, it } from 'vitest';
import { validatePlacementCommand, generateResearchReport } from './libraryModel';
import { createAppStore } from './appState';

describe('libraryModel', () => {
  it('validates placement coordinates against world boundaries', () => {
    expect(validatePlacementCommand(10, 20, 100, 100).valid).toBe(true);
    expect(validatePlacementCommand(-5, 20, 100, 100).valid).toBe(false);
    expect(validatePlacementCommand(150, 20, 100, 100).valid).toBe(false);
  });

  it('generates reproducible Markdown research report with metadata', () => {
    const store = createAppStore();
    const state = store.getState();
    const report = generateResearchReport(state);

    expect(report).toContain('# ALife Research Experiment Report');
    expect(report).toContain('Committed Tick:');
    expect(report).toContain('Total Active Cells:');
  });
});
