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

  it('marks live resources as loading while debug projections are pending', () => {
    const state = (buildViewerTruthState as unknown as Function)(
      baseFrame,
      { width: 1200, height: 800 },
      {
        status: 'loading',
        runId: 'run-live',
        requestedTick: 20,
        reason: 'Waiting for Observer debug projection'
      }
    );

    expect(state.resourceLayer).toEqual({
      state: 'loading',
      label: 'Resources',
      value: 'Loading projection',
      note: 'Waiting for Observer debug projection for Tick 20'
    });
  });

  it('marks stale debug resources without replacing them with missing', () => {
    const state = (buildViewerTruthState as unknown as Function)(
      baseFrame,
      { width: 1200, height: 800 },
      {
        status: 'stale',
        runId: 'run-live',
        tick: 12,
        reason: 'Latest debug projection is behind live Tick 20'
      }
    );

    expect(state.resourceLayer).toEqual({
      state: 'stale',
      label: 'Resources',
      value: 'Stale projection',
      note: 'Latest debug projection is behind live Tick 20'
    });
  });

  it('marks last-known debug resources as stale when the payload is behind the live Tick', () => {
    const state = buildViewerTruthState(
      {
        ...baseFrame,
        resources: [[{ organic: 0.2, mineral: 0.1, energy: 0.5 }]]
      },
      { width: 1200, height: 800 },
      {
        status: 'available',
        runId: 'run-live',
        tick: 12,
        visualWorld: {
          projectionKind: 'VisualWorldProjection',
          completeness: { state: 'bounded', missingFields: [], reason: null },
          cells: [],
          resourceLayers: [],
          fields: [],
          sourceMetrics: []
        },
        coverage: { projectionKind: 'CoverageProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, mechanisms: [] },
        warnings: { projectionKind: 'WarningProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, warnings: [] },
        classifications: { projectionKind: 'ClassificationProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, classifications: [] },
        balanceFindings: { projectionKind: 'BalanceFindingProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, findings: [] }
      }
    );

    expect(state.resourceLayer).toEqual({
      state: 'stale',
      label: 'Resources',
      value: 'Stale projection',
      note: 'Debug projection Tick 12 is behind live Tick 20'
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
