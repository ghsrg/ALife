import { describe, expect, it } from 'vitest';
import { createAppStore } from './appState';
import { buildMonitorSurfaceModel } from './monitorSurfaceModel';

function cardTexts(model: ReturnType<typeof buildMonitorSurfaceModel>) {
  return model.cards
    .flatMap((card) => [
      card.title,
      card.subtitle,
      card.reason,
      card.source,
      card.unit,
      ...card.rows.map((row) => `${row.label} ${row.value}`),
      ...card.series.map((series) => `${series.label} ${series.value ?? ''}`)
    ])
    .filter((value): value is string => Boolean(value));
}

describe('buildMonitorSurfaceModel', () => {
  it('marks Energy Flow unavailable when no source-backed accounting projection exists', () => {
    const state = createAppStore().getState();

    const model = buildMonitorSurfaceModel(state);
    const energyFlow = model.cards.find((card) => card.id === 'world-energy-flow');

    expect(model.accountingTarget).toBe('Energy');
    expect(energyFlow).toMatchObject({
      title: 'Energy Flow',
      state: 'unavailable'
    });
    expect(energyFlow?.reason).toContain('No source-backed Energy Flow accounting projection');
  });

  it('does not expose hardcoded fallback values in card text', () => {
    const state = createAppStore().getState();
    const model = buildMonitorSurfaceModel(state);
    const text = cardTexts(model).join(' ');

    expect(text).not.toContain('65.2%');
    expect(text).not.toContain('23.1%');
    expect(text).not.toContain('8.7%');
    expect(text).not.toContain('3.0%');
    expect(text).not.toContain('1270 amu');
  });

  it('does not infer Cell behavior classifications from energy or radius', () => {
    const store = createAppStore();
    store.getState().setDebugProjections({
      status: 'unavailable',
      reason: 'classification projection unavailable'
    });

    const model = buildMonitorSurfaceModel(store.getState(), { activeLevel: 'cells' });

    const roles = model.cards.find((card) => card.id === 'cells-role-distribution');
    expect(roles).toMatchObject({
      title: 'Observed Primary Roles',
      state: 'unavailable'
    });
    expect(cardTexts(model).join(' ')).not.toMatch(/Metabolic|Transport|Structural \(Compact\)/);
  });

  it('selects required card sets for each Monitor level with unavailable missing contracts', () => {
    const state = createAppStore().getState();

    expect(buildMonitorSurfaceModel(state, { activeLevel: 'world' }).cards.map((card) => card.title)).toEqual([
      'Population Lifecycle',
      'Energy Flow',
      'Energy Distribution Over Time'
    ]);
    expect(buildMonitorSurfaceModel(state, { activeLevel: 'cells' }).cards.map((card) => card.title)).toEqual([
      'Population Lifecycle',
      'Observed Primary Roles',
      'Cell Radius Distribution'
    ]);
    expect(buildMonitorSurfaceModel(state, { activeLevel: 'organisms' }).cards.map((card) => card.title)).toEqual([
      'Observed Behavior Profiles',
      'Organism Size Distribution'
    ]);
    expect(buildMonitorSurfaceModel(state, { activeLevel: 'lineages' }).cards.map((card) => card.title)).toContain('Compact Genealogy');
    expect(buildMonitorSurfaceModel(state, { activeLevel: 'evolution' }).cards.map((card) => card.title)).toContain('Genome Provenance');
    expect(buildMonitorSurfaceModel(state, { activeLevel: 'analytics' }).cards.map((card) => card.title)).toEqual(['Selected Metric']);
  });

  it('keeps Energy default unavailable and exposes Resource type options only from source-backed layers', () => {
    const store = createAppStore();
    store.getState().setFrame({ ...store.getState().frame, source: 'live' });
    store.getState().setDebugProjections({
      status: 'available',
      runId: store.getState().frame.runId,
      tick: store.getState().frame.tick,
      visualWorld: {
        projectionKind: 'VisualWorldProjection',
        completeness: { state: 'bounded', missingFields: [], reason: null },
        cells: [],
        resourceLayers: [
          { layerIndex: 2, resourceTypeId: 2, resourceId: 'resource_2', width: 1, height: 1, totalAmount: 7, cells: [{ x: 0, y: 0, amount: 7 }], completeness: { state: 'bounded', missingFields: [], reason: null } }
        ],
        fields: [],
        sourceMetrics: []
      },
      coverage: { projectionKind: 'CoverageProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, mechanisms: [] },
      warnings: { projectionKind: 'WarningProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, warnings: [] },
      classifications: { projectionKind: 'ClassificationProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, classifications: [] },
      balanceFindings: { projectionKind: 'BalanceFindingProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, findings: [] }
    });

    const energyModel = buildMonitorSurfaceModel(store.getState());
    expect(energyModel.accountingTarget).toBe('Energy');
    expect(energyModel.cards.find((card) => card.id === 'world-energy-flow')?.state).toBe('unavailable');

    const resourceModel = buildMonitorSurfaceModel(store.getState(), { accountingTarget: 'Resource' });
    expect(resourceModel.accountingTypeOptions).toEqual(['Layer 2']);
    expect(resourceModel.cards.find((card) => card.id === 'world-resource-cycle')).toMatchObject({
      title: 'Resource Cycle',
      state: 'partial'
    });
    expect(cardTexts(resourceModel).join(' ')).not.toContain('RESOURCE CYCLE (ENERGY & MATTER)');
  });

  it('uses Monitor resource accounting payload and RRD samples for World Resource diagrams', () => {
    const store = createAppStore();
    const baseFrame = { ...store.getState().frame, source: 'live' as const, runId: 'run-monitor', tick: 12 };
    store.getState().setFrame(baseFrame);
    store.getState().setDebugProjections({
      status: 'available',
      runId: 'run-monitor',
      tick: 12,
      monitor: {
        projectionKind: 'MonitorDataPanelProjection',
        runId: 'run-monitor',
        tick: 12,
        source: 'live',
        completeness: { state: 'partial', missingFields: ['world.energy_flow'], reason: 'Energy flow unavailable' },
        payload: {
          world: {
            populationLifecycle: {
              state: 'available',
              source: 'VisualWorldProjection.cells.lifecycleState',
              total: 3,
              alive: 1,
              stressed: 2,
              dormant: 0,
              dead: 0
            },
            resourceCycle: {
              state: 'available',
              source: 'MonitorAccountingProjection.resource',
              totalAmount: 12,
              locations: {
                environment: 8,
                cells: 3,
                materials: 1,
                fragments: 0,
                explicitSinks: 0
              },
              accounting: {
                explicitDecayOrSink: 0.5,
                metabolismOrCellUptake: 2,
                materialConversion: 1,
                unclassifiedLoss: 0
              }
            },
            materialCycle: { state: 'unavailable', source: 'MaterialAccountingProjection', reason: 'missing' },
            energyFlow: { state: 'unavailable', source: 'EnergyAccountingProjection', reason: 'missing' },
            accountingTime: { state: 'unavailable', source: 'UI RRD metric history', reason: 'missing' }
          },
          cells: {
            populationLifecycle: {
              state: 'available',
              source: 'VisualWorldProjection.cells.lifecycleState',
              total: 3,
              alive: 1,
              stressed: 2,
              dormant: 0,
              dead: 0
            },
            observedPrimaryRoles: { state: 'unavailable', source: 'ClassificationProjection', reason: 'missing' },
            potentialRoles: { state: 'unavailable', source: 'ClassificationProjection', reason: 'missing' },
            radiusDistribution: { state: 'unavailable', source: 'WorldFrameProjection.cells.radius', reason: 'missing' }
          },
          organisms: {
            behaviorProfiles: { state: 'unavailable', source: 'BehaviorProfileProjection', reason: 'missing' },
            sizeBins: { state: 'unavailable', source: 'OrganismViewProjection', reason: 'missing' }
          },
          lineages: { state: 'unavailable', source: 'LineageProjection', reason: 'missing' },
          evolution: { state: 'unavailable', source: 'GenomeProjection', reason: 'missing' },
          analytics: { state: 'unavailable', source: 'MetricsProjection', reason: 'missing' }
        }
      },
      visualWorld: {
        projectionKind: 'VisualWorldProjection',
        completeness: { state: 'bounded', missingFields: [], reason: null },
        cells: [],
        resourceLayers: [
          { layerIndex: 9, resourceTypeId: 9, resourceId: 'resource_9', width: 1, height: 1, totalAmount: 99, cells: [{ x: 0, y: 0, amount: 99 }], completeness: { state: 'bounded', missingFields: [], reason: null } }
        ],
        fields: [],
        sourceMetrics: []
      },
      coverage: { projectionKind: 'CoverageProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, mechanisms: [] },
      warnings: { projectionKind: 'WarningProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, warnings: [] },
      classifications: { projectionKind: 'ClassificationProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, classifications: [] },
      balanceFindings: { projectionKind: 'BalanceFindingProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, findings: [] }
    });

    const model = buildMonitorSurfaceModel(store.getState(), { accountingTarget: 'Resource' });

    expect(model.accountingTypeOptions).toEqual([]);
    expect(model.cards.find((card) => card.id === 'world-resource-cycle')).toMatchObject({
      title: 'Resource Cycle',
      state: 'available',
      source: 'MonitorAccountingProjection.resource'
    });
    expect(model.cards.find((card) => card.id === 'world-resource-cycle')?.rows).toContainEqual({
      label: 'Total Amount',
      value: '12'
    });
    expect(model.cards.find((card) => card.id === 'world-resource-cycle')?.series).toContainEqual({
      label: 'Environment',
      value: '8'
    });
    expect(model.cards.find((card) => card.id === 'world-accounting-time')).toMatchObject({
      title: 'Resource Distribution Over Time',
      state: 'available',
      source: 'UI RRD metric history'
    });
  });

  it('uses source-backed lifecycle counts for the Cells level population card', () => {
    const store = createAppStore();
    store.getState().setDebugProjections({
      status: 'available',
      runId: store.getState().frame.runId,
      tick: store.getState().frame.tick,
      visualWorld: {
        projectionKind: 'VisualWorldProjection',
        completeness: { state: 'bounded', missingFields: [], reason: null },
        cells: [
          {
            id: 'cell-a',
            x: 1,
            y: 1,
            radius: 1,
            energy: 1,
            energyCapacity: 2,
            lifecycleState: 'alive',
            materials: [],
            internalResources: [],
            localExternalResources: []
          },
          {
            id: 'cell-b',
            x: 2,
            y: 2,
            radius: 1,
            energy: 1,
            energyCapacity: 2,
            lifecycleState: 'stressed',
            materials: [],
            internalResources: [],
            localExternalResources: []
          },
          {
            id: 'cell-c',
            x: 3,
            y: 3,
            radius: 1,
            energy: 1,
            energyCapacity: 2,
            lifecycleState: 'dead',
            materials: [],
            internalResources: [],
            localExternalResources: []
          }
        ],
        resourceLayers: [],
        fields: [],
        sourceMetrics: []
      },
      coverage: { projectionKind: 'CoverageProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, mechanisms: [] },
      warnings: { projectionKind: 'WarningProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, warnings: [] },
      classifications: { projectionKind: 'ClassificationProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, classifications: [] },
      balanceFindings: { projectionKind: 'BalanceFindingProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, findings: [] }
    });

    const model = buildMonitorSurfaceModel(store.getState(), { activeLevel: 'cells' });
    const lifecycle = model.cards.find((card) => card.id === 'cells-population-lifecycle');

    expect(lifecycle).toMatchObject({
      title: 'Population Lifecycle',
      level: 'cells',
      state: 'available',
      source: 'VisualWorldProjection.cells.lifecycleState'
    });
    expect(lifecycle?.rows).toContainEqual({ label: 'Total', value: '3' });
    expect(lifecycle?.rows).toContainEqual({ label: 'Alive', value: '1' });
    expect(lifecycle?.rows).toContainEqual({ label: 'Stressed', value: '1' });
    expect(lifecycle?.rows).toContainEqual({ label: 'Dead', value: '1' });
  });
});
