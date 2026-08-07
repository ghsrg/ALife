import { describe, expect, it } from 'vitest';
import { uiText } from './uiText';

describe('uiText', () => {
  it('contains critical Monitor labels', () => {
    expect(uiText.app.title).toBe('ALife Control Center');
    expect(uiText.workspace.monitor).toBe('Monitor');
    expect(uiText.viewer.ariaLabel).toBe('World Viewer');
    expect(uiText.inspector.title).toBe('Inspector');
    expect(uiText.inspector.emptyCell).toBe('No cell selected.');
  });

  it('keeps canonical English technical terms stable', () => {
    expect(uiText.layers.cells).toBe('Cells');
    expect(uiText.layers.resources).toBe('Composite Resource Concentration');
    expect(uiText.controls.exportPng).toBe('Export PNG');
  });
});
