import { describe, expect, it } from 'vitest';
import { normalizeDebugProjectionBundle, normalizeUnavailableDebugProjection } from './debugProjectionAdapter';
import type { DebugProjectionState } from './types';

function expectAvailable(state: DebugProjectionState): asserts state is Extract<DebugProjectionState, { status: 'available' }> {
  expect(state.status).toBe('available');
}

const bundle = {
  schema_version: 'ControlCenterProjectionBundle/v1',
  projection_kind: 'DebugProjectionBundle',
  source: 'live',
  run_id: 'run-1',
  tick: 42,
  visual_world: {
    schema_version: 'VisualWorldProjection/v1',
    projection_kind: 'VisualWorldProjection',
    source: 'live',
    completeness: {
      state: 'partial',
      missing_fields: ['cells.internal_resources', 'cells.materials'],
      reason: 'CommittedSnapshot lacks per-cell details'
    },
    payload: {
      cells: [
        {
          id: 7,
          x: 3,
          y: 5,
          radius: 1.5,
          energy: 12,
          energy_capacity: 20,
          lifecycle_state: 'Alive',
          materials: [{ material_type_id: 1, amount: 2.5 }],
          internal_resources: [{ resource_type_id: 0, amount: 1.5 }],
          local_external_resources: [{ resource_type_id: 0, amount: 3.5 }]
        }
      ],
      resource_layers: [
        {
          layer_index: 0,
          width: 2,
          height: 2,
          total_amount: 4,
          cells: [
            { x: 0, y: 0, amount: 1 },
            { x: 1, y: 0, amount: 0.5 },
            { x: 0, y: 1, amount: 2 },
            { x: 1, y: 1, amount: 0.5 }
          ],
          completeness: {
            state: 'bounded',
            missing_fields: [],
            reason: 'Totals only'
          }
        }
      ],
      fields: [
        {
          field_id: 'heat',
          value: 2.5,
          source_metric: {
            field_id: 'heat',
            source_owner: 'CoreCommittedSnapshot',
            source_path: 'CommittedSnapshot.heat'
          }
        }
      ],
      source_metrics: [
        {
          field_id: 'cells.energy',
          source_owner: 'CoreCommittedSnapshot',
          source_path: 'CommittedSnapshot.cells[].energy'
        }
      ]
    }
  },
  coverage: {
    projection_kind: 'CoverageProjection',
    completeness: { state: 'bounded', missing_fields: [], reason: 'No rows' },
    payload: { mechanisms: [] }
  },
  warnings: {
    projection_kind: 'WarningProjection',
    completeness: { state: 'bounded', missing_fields: [], reason: 'Gateway exposed' },
    payload: {
      warnings: [
        {
          code: 'CONFIG_TUNING_RECOMMENDED',
          disposition: 'CanonicalObserverWarning',
          affected_scope: 'latest_live_projection',
          source_report: 'observer_live_gateway',
          recommended_reruns: []
        }
      ]
    }
  },
  classifications: {
    projection_kind: 'ClassificationProjection',
    completeness: { state: 'bounded', missing_fields: [], reason: 'No rows' },
    payload: { classifications: [] }
  },
  balance_findings: {
    projection_kind: 'BalanceFindingProjection',
    completeness: { state: 'bounded', missing_fields: [], reason: 'No rows' },
    payload: { findings: [] }
  }
};

describe('normalizeDebugProjectionBundle', () => {
  it('preserves partial visual world completeness and source-backed values', () => {
    const normalized = normalizeDebugProjectionBundle(bundle);

    expectAvailable(normalized);
    expect(normalized.runId).toBe('run-1');
    expect(normalized.tick).toBe(42);
    expect(normalized.visualWorld.completeness.state).toBe('partial');
    expect(normalized.visualWorld.completeness.missingFields).toEqual([
      'cells.internal_resources',
      'cells.materials'
    ]);
    expect(normalized.visualWorld.cells[0]).toMatchObject({
      id: '7',
      x: 3,
      y: 5,
      radius: 1.5,
      energy: 12,
      energyCapacity: 20,
      lifecycleState: 'Alive',
      materials: [{ materialTypeId: 1, amount: 2.5 }],
      internalResources: [{ resourceTypeId: 0, amount: 1.5 }],
      localExternalResources: [{ resourceTypeId: 0, amount: 3.5 }]
    });
    expect(normalized.visualWorld.resourceLayers[0]).toMatchObject({
      layerIndex: 0,
      width: 2,
      height: 2,
      cells: [
        { x: 0, y: 0, amount: 1 },
        { x: 1, y: 0, amount: 0.5 },
        { x: 0, y: 1, amount: 2 },
        { x: 1, y: 1, amount: 0.5 }
      ]
    });
    expect(normalized.visualWorld.fields[0].sourceMetric.sourceOwner).toBe('CoreCommittedSnapshot');
  });

  it('preserves all created projection categories without inventing rows', () => {
    const normalized = normalizeDebugProjectionBundle(bundle);

    expectAvailable(normalized);
    expect(normalized.coverage.mechanisms).toEqual([]);
    expect(normalized.warnings.warnings[0].code).toBe('CONFIG_TUNING_RECOMMENDED');
    expect(normalized.classifications.classifications).toEqual([]);
    expect(normalized.balanceFindings.findings).toEqual([]);
  });
});

describe('normalizeUnavailableDebugProjection', () => {
  it('keeps unavailable projection state explicit', () => {
    expect(
      normalizeUnavailableDebugProjection({
        ok: false,
        category: 'projection_unavailable',
        projection_status: 'unavailable',
        message: 'No active committed snapshot is available'
      })
    ).toEqual({
      status: 'unavailable',
      reason: 'No active committed snapshot is available'
    });
  });
});
