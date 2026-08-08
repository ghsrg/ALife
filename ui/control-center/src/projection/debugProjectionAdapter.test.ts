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
          resource_type_id: 0,
          resource_id: 'amino_acid',
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
      field_layers: [
        {
          field_id: 'temperature',
          width: 2,
          height: 2,
          summary_value: 25,
          cells: [{ x: 1, y: 0, value: 30 }],
          completeness: {
            state: 'bounded',
            missing_fields: [],
            reason: 'Scalar field grid'
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
      resourceTypeId: 0,
      resourceId: 'amino_acid',
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
    expect(normalized.visualWorld.fieldLayers).toEqual([
      {
        fieldId: 'temperature',
        width: 2,
        height: 2,
        summaryValue: 25,
        cells: [{ x: 1, y: 0, value: 30 }],
        completeness: {
          state: 'bounded',
          missingFields: [],
          reason: 'Scalar field grid'
        }
      }
    ]);
  });

  it('preserves all created projection categories without inventing rows', () => {
    const normalized = normalizeDebugProjectionBundle(bundle);

    expectAvailable(normalized);
    expect(normalized.coverage.mechanisms).toEqual([]);
    expect(normalized.warnings.warnings[0].code).toBe('CONFIG_TUNING_RECOMMENDED');
    expect(normalized.classifications.classifications).toEqual([]);
    expect(normalized.balanceFindings.findings).toEqual([]);
  });

  it('attaches optional Monitor Data Panel projection when the bundle provides it', () => {
    const normalized = normalizeDebugProjectionBundle({
      ...bundle,
      monitor: {
        schema_version: 'MonitorDataPanelProjection/v1',
        projection_kind: 'MonitorDataPanelProjection',
        run_id: 'run-1',
        tick: 42,
        source: 'live',
        completeness: { state: 'partial', missing_fields: ['world.energy_flow'], reason: 'partial' },
        payload: {
          world: {
            population_lifecycle: {
              state: 'available',
              source: 'VisualWorldProjection.cells.lifecycleState',
              total: 1,
              alive: 1,
              stressed: 0,
              dormant: 0,
              dead: 0
            },
            resource_cycle: {
              state: 'available',
              source: 'MonitorAccountingProjection.resource',
              total_amount: 4,
              locations: { environment: 4, cells: 0, materials: 0, fragments: 0, explicit_sinks: 0 },
              accounting: {
                explicit_decay_or_sink: 0,
                metabolism_or_cell_uptake: 0,
                material_conversion: 0,
                unclassified_loss: 0
              }
            },
            material_cycle: { state: 'unavailable', source: 'MaterialAccountingProjection', reason: 'missing' },
            energy_flow: { state: 'unavailable', source: 'EnergyAccountingProjection', reason: 'missing' },
            accounting_time: { state: 'unavailable', source: 'UI RRD metric history', reason: 'missing' }
          },
          cells: {
            population_lifecycle: {
              state: 'available',
              source: 'VisualWorldProjection.cells.lifecycleState',
              total: 1,
              alive: 1,
              stressed: 0,
              dormant: 0,
              dead: 0
            },
            observed_primary_roles: { state: 'unavailable', source: 'ClassificationProjection', reason: 'missing' },
            potential_roles: { state: 'unavailable', source: 'ClassificationProjection', reason: 'missing' },
            radius_distribution: { state: 'unavailable', source: 'WorldFrameProjection.cells.radius', reason: 'missing' }
          },
          organisms: {
            behavior_profiles: { state: 'unavailable', source: 'BehaviorProfileProjection', reason: 'missing' },
            size_bins: { state: 'unavailable', source: 'OrganismViewProjection', reason: 'missing' }
          },
          lineages: { state: 'unavailable', source: 'LineageProjection', reason: 'missing' },
          evolution: { state: 'unavailable', source: 'GenomeProjection', reason: 'missing' },
          analytics: { state: 'unavailable', source: 'MetricsProjection', reason: 'missing' }
        }
      }
    });

    expectAvailable(normalized);
    expect(normalized.monitor?.payload.world.resourceCycle).toMatchObject({
      state: 'available',
      totalAmount: 4
    });
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
