import { describe, expect, it } from 'vitest';
import type { DebugProjectionState } from '../projection/types';
import { buildDebugLayerPlan } from './debugLayers';

const availableDebugProjections: DebugProjectionState = {
  status: 'available',
  runId: 'run-1',
  tick: 12,
  visualWorld: {
    projectionKind: 'VisualWorldProjection',
    completeness: {
      state: 'partial',
      missingFields: ['cells.materials'],
      reason: 'CommittedSnapshot lacks per-cell materials'
    },
    cells: [],
    resourceLayers: [
      {
        layerIndex: 0,
        width: 2,
        height: 2,
        totalAmount: 4,
        cells: [
          { x: 0, y: 0, amount: 1 },
          { x: 1, y: 0, amount: 1 },
          { x: 0, y: 1, amount: 1 },
          { x: 1, y: 1, amount: 1 }
        ],
        completeness: {
          state: 'bounded',
          missingFields: [],
          reason: 'Totals only'
        }
      }
    ],
    fields: [
      {
        fieldId: 'heat',
        value: 2.5,
        sourceMetric: {
          fieldId: 'heat',
          sourceOwner: 'CoreCommittedSnapshot',
          sourcePath: 'CommittedSnapshot.heat'
        }
      }
    ],
    sourceMetrics: []
  },
  coverage: {
    projectionKind: 'CoverageProjection',
    completeness: { state: 'bounded', missingFields: [], reason: 'No rows' },
    mechanisms: []
  },
  warnings: {
    projectionKind: 'WarningProjection',
    completeness: { state: 'bounded', missingFields: [], reason: 'No rows' },
    warnings: []
  },
  classifications: {
    projectionKind: 'ClassificationProjection',
    completeness: { state: 'bounded', missingFields: [], reason: 'No rows' },
    classifications: []
  },
  balanceFindings: {
    projectionKind: 'BalanceFindingProjection',
    completeness: { state: 'bounded', missingFields: [], reason: 'No rows' },
    findings: []
  }
};

describe('buildDebugLayerPlan', () => {
  it('keeps projection loading explicit until Observer debug layers arrive', () => {
    const plan = buildDebugLayerPlan(
      {
        status: 'loading',
        runId: 'run-1',
        requestedTick: 44,
        reason: 'Waiting for Observer debug projection'
      } as unknown as DebugProjectionState,
      {
        mode: 'exact',
        showResourceLayer: true,
        showFieldLayer: true
      }
    );

    expect(plan).toMatchObject({
      status: 'loading',
      reason: 'Waiting for Observer debug projection',
      resources: [],
      fields: []
    });
  });

  it('plans exact resource and field layers with source-backed legend entries', () => {
    const plan = buildDebugLayerPlan(availableDebugProjections, {
      mode: 'exact',
      showResourceLayer: true,
      showFieldLayer: true
    });

    expect(plan.status).toBe('available');
    expect(plan.interpolationLabel).toBe('Exact');
    expect(plan.resources[0]).toMatchObject({
      layerIndex: 0,
      totalAmount: 4,
      availability: 'bounded',
      channelLabel: 'green channel',
      colorHex: '#27b582',
      legendLabel: 'Layer 0 green channel total 4'
    });
    expect(plan.fields[0]).toMatchObject({
      fieldId: 'heat',
      value: 2.5,
      sourceOwner: 'CoreCommittedSnapshot',
      legendLabel: 'heat 2.5'
    });
    expect(plan.missingProjectionWarnings).toEqual(['cells.materials']);
  });

  it('marks smooth mode as interpolation without changing sampled values', () => {
    const plan = buildDebugLayerPlan(availableDebugProjections, {
      mode: 'smooth',
      showResourceLayer: true,
      showFieldLayer: true
    });

    expect(plan.interpolationLabel).toBe('Smooth interpolated');
    expect(plan.fields[0].value).toBe(2.5);
    expect(plan.fields[0].sampledValueLabel).toBe('sampled heat: 2.5');
  });

  it('keeps unavailable projections explicit and produces no drawable layers', () => {
    const plan = buildDebugLayerPlan(
      {
        status: 'unavailable',
        reason: 'No active committed snapshot is available'
      },
      {
        mode: 'exact',
        showResourceLayer: true,
        showFieldLayer: true
      }
    );

    expect(plan).toEqual({
      status: 'unavailable',
      reason: 'No active committed snapshot is available',
      interpolationLabel: 'Exact',
      resources: [],
      fields: [],
      totalResourceLayerCount: 0,
      hiddenResourceLayerCount: 0,
      missingProjectionWarnings: []
    });
  });

  it('bounds large resource legends while preserving total layer count', () => {
    const resourceLayers = Array.from({ length: 27 }, (_, layerIndex) => ({
      layerIndex,
      width: 1,
      height: 1,
      totalAmount: layerIndex + 1,
      cells: [{ x: 0, y: 0, amount: layerIndex + 1 }],
      completeness: {
        state: 'bounded' as const,
        missingFields: [],
        reason: null
      }
    }));
    const plan = buildDebugLayerPlan(
      {
        ...availableDebugProjections,
        visualWorld: {
          ...availableDebugProjections.visualWorld,
          resourceLayers
        }
      },
      {
        mode: 'exact',
        showResourceLayer: true,
        showFieldLayer: true
      }
    );

    expect(plan.resources).toHaveLength(8);
    expect(plan.totalResourceLayerCount).toBe(27);
    expect(plan.hiddenResourceLayerCount).toBe(19);
  });
});
