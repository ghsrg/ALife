import { describe, expect, it } from 'vitest';
import { buildFieldLayerDisplay, buildResourceLayerDisplay } from './layerDisplayModel';

describe('layerDisplayModel', () => {
  it('keeps source-backed field provenance out of the primary label', () => {
    const display = buildFieldLayerDisplay({
      fieldId: 'CommittedSnapshot.heat',
      value: 12,
      sourceMetric: {
        fieldId: 'CommittedSnapshot.heat',
        sourceOwner: 'WorldFrameProjection',
        sourcePath: 'VisualWorldProjection.fields.CommittedSnapshot.heat'
      }
    });

    expect(display.primaryLabel).toBe('Heat');
    expect(display.primaryLabel).not.toContain('CommittedSnapshot');
    expect(display.provenance).toContain('VisualWorldProjection.fields.CommittedSnapshot.heat');
  });

  it('summarizes resource rows without exposing verbose completeness as primary text', () => {
    const display = buildResourceLayerDisplay({
      layerIndex: 2,
      width: 1,
      height: 1,
      totalAmount: 5004.31,
      cells: [{ x: 0, y: 0, amount: 5004.31 }],
      completeness: {
        state: 'bounded',
        missingFields: [],
        reason: 'CommittedSnapshot exposes resource grid cells for this bounded world.'
      }
    });

    expect(display.primaryLabel).toBe('Resource Layer 2');
    expect(display.secondaryLabel).toBe('5,004.31 total · bounded');
    expect(display.secondaryLabel).not.toContain('CommittedSnapshot');
    expect(display.provenance).toContain('CommittedSnapshot exposes resource grid cells');
  });
});
