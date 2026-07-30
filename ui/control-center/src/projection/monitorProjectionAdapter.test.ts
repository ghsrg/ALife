import { describe, expect, it } from 'vitest';
import { normalizeMonitorProjection } from './monitorProjectionAdapter';

describe('normalizeMonitorProjection', () => {
  it('normalizes source-backed world resource accounting buckets', () => {
    const normalized = normalizeMonitorProjection({
      schema_version: 'MonitorDataPanelProjection/v1',
      projection_kind: 'MonitorDataPanelProjection',
      run_id: 'run-monitor',
      tick: 12,
      source: 'live',
      completeness: {
        state: 'partial',
        missing_fields: ['world.energy_flow'],
        reason: 'partial monitor payload'
      },
      payload: {
        world: {
          population_lifecycle: {
            state: 'available',
            source: 'VisualWorldProjection.cells.lifecycleState',
            total: 3,
            alive: 2,
            stressed: 1,
            dormant: 0,
            dead: 0
          },
          resource_cycle: {
            state: 'available',
            source: 'MonitorAccountingProjection.resource',
            total_amount: 90,
            locations: {
              environment: 70,
              cells: 20,
              materials: 0,
              fragments: 0,
              explicit_sinks: 0
            },
            accounting: {
              explicit_decay_or_sink: 3,
              metabolism_or_cell_uptake: 2,
              material_conversion: 1,
              unclassified_loss: 0
            }
          },
          material_cycle: {
            state: 'unavailable',
            source: 'MaterialAccountingProjection',
            reason: 'missing'
          },
          energy_flow: {
            state: 'unavailable',
            source: 'EnergyAccountingProjection',
            reason: 'missing'
          },
          accounting_time: {
            state: 'unavailable',
            source: 'UI RRD metric history',
            reason: 'missing'
          }
        },
        cells: {
          population_lifecycle: {
            state: 'available',
            source: 'VisualWorldProjection.cells.lifecycleState',
            total: 3,
            alive: 2,
            stressed: 1,
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
    });

    expect(normalized).toMatchObject({
      projectionKind: 'MonitorDataPanelProjection',
      runId: 'run-monitor',
      tick: 12,
      completeness: {
        state: 'partial',
        missingFields: ['world.energy_flow'],
        reason: 'partial monitor payload'
      },
      payload: {
        world: {
          resourceCycle: {
            state: 'available',
            source: 'MonitorAccountingProjection.resource',
            totalAmount: 90,
            locations: {
              environment: 70,
              cells: 20,
              explicitSinks: 0
            },
            accounting: {
              explicitDecayOrSink: 3,
              metabolismOrCellUptake: 2,
              materialConversion: 1,
              unclassifiedLoss: 0
            }
          }
        }
      }
    });
  });
});
