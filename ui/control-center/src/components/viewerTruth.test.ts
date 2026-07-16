import { describe, expect, it } from 'vitest';
import type { WorldFrame } from '../projection/types';
import { buildViewerTruthState } from './viewerTruth';

const baseFrame: WorldFrame = {
  schemaVersion: 'WorldFrameProjection/v1',
  source: 'live',
  runId: 'run-live',
  scenarioName: 'demo_living_world',
  tick: 20,
  world: { width: 1200, height: 800 },
  resources: [],
  cells: [
    { id: '1', x: 10, y: 20, radius: 2, energy: 0.8, integrity: 1, generation: 0, roleHint: 'alive lifecycle state', lifecycle: 1 },
    { id: '2', x: 40, y: 50, radius: 20, energy: 0.5, integrity: 1, generation: 0, roleHint: 'alive lifecycle state', lifecycle: 1 }
  ]
};

describe('buildViewerTruthState', () => {
  it('marks live resources as missing when the projection has no resource grid', () => {
    const state = buildViewerTruthState(baseFrame, { width: 1200, height: 800 });

    expect(state.resourceLayer).toEqual({
      state: 'missing',
      label: 'Resources',
      value: 'Missing projection',
      note: 'Runner ALIF v2 does not include resource grid'
    });
  });

  it('marks fixture resource grid as available data', () => {
    const state = buildViewerTruthState({
      ...baseFrame,
      source: 'fixture',
      resources: [[{ organic: 0.2, mineral: 0.1, energy: 0.5 }]]
    }, { width: 1200, height: 800 });

    expect(state.resourceLayer).toEqual({
      state: 'available',
      label: 'Resources',
      value: 'Fixture grid',
      note: '1 resource cells'
    });
  });

  it('reports when display radius minimum is applied to any cell', () => {
    const state = buildViewerTruthState(baseFrame, { width: 1200, height: 800 });

    expect(state.cellScale).toEqual({
      state: 'presentation-minimum',
      label: 'Cell size',
      value: 'Display minimum applied',
      note: '1 of 2 cells enlarged for visibility'
    });
  });
});
