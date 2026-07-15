import { describe, expect, it } from 'vitest';
import { createAppStore } from './appState';

describe('createAppStore', () => {
  it('starts from fixture data and supports selection and theme changes', () => {
    const store = createAppStore();

    expect(store.getState().frame.runId).toBe('fixture-ui-1a');
    expect(store.getState().selectedCellId).toBe('cell-a');
    expect(store.getState().theme).toBe('dark');

    store.getState().selectCell('cell-c');
    expect(store.getState().selectedCell?.roleHint).toBe('resource-rich region');

    store.getState().setTheme('light');
    expect(store.getState().theme).toBe('light');
  });
});
