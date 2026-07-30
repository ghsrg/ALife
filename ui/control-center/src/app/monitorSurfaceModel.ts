import type { AppStore } from './appState';
import type { DebugProjectionCompleteness, DebugProjectionState } from '../projection/types';

export type MonitorDataState = 'available' | 'partial' | 'unavailable';
export type AnalysisLevel = 'world' | 'cells' | 'organisms' | 'lineages' | 'evolution' | 'analytics';
export type AccountingTarget = 'Energy' | 'Resource' | 'Material';

export interface MonitorSurfaceRow {
  label: string;
  value: string;
}

export interface MonitorSurfaceSeries {
  label: string;
  value?: string;
  color?: string;
}

export interface MonitorSurfaceCard {
  id: string;
  title: string;
  level: AnalysisLevel;
  state: MonitorDataState;
  source: string;
  completeness: string;
  subtitle?: string;
  reason?: string;
  unit?: string;
  rows: MonitorSurfaceRow[];
  series: MonitorSurfaceSeries[];
}

export interface MonitorSurfaceModel {
  activeLevel: AnalysisLevel;
  accountingTarget: AccountingTarget;
  accountingTypeOptions: string[];
  cards: MonitorSurfaceCard[];
  warnings: string[];
}

export interface MonitorSurfaceModelOptions {
  activeLevel?: AnalysisLevel;
  accountingTarget?: AccountingTarget;
}

export function buildMonitorSurfaceModel(
  state: AppStore,
  options: MonitorSurfaceModelOptions = {}
): MonitorSurfaceModel {
  const activeLevel = options.activeLevel ?? 'world';
  const accountingTarget = options.accountingTarget ?? state.monitorAccountingTarget ?? 'Energy';
  const cards = buildCards(state, activeLevel, accountingTarget);

  return {
    activeLevel,
    accountingTarget,
    accountingTypeOptions: accountingTargetOptions(state.debugProjections, accountingTarget),
    cards,
    warnings: cards
      .filter((card) => card.state !== 'available')
      .map((card) => `${card.title}: ${card.reason ?? 'unavailable'}`)
  };
}

function buildCards(
  state: AppStore,
  activeLevel: AnalysisLevel,
  accountingTarget: AccountingTarget
): MonitorSurfaceCard[] {
  switch (activeLevel) {
    case 'cells':
      return [
        populationLifecycleCard(state, 'cells'),
        unavailableCard({
          id: 'cells-role-distribution',
          title: 'Observed Primary Roles',
          level: 'cells',
          reason: 'No typed ClassificationProjection payload is available for observed Cell roles.',
          source: classificationSource(state.debugProjections)
        }),
        cellRadiusDistributionCard(state)
      ];
    case 'organisms':
      return [
        unavailableCard({
          id: 'organisms-behavior-profiles',
          title: 'Observed Behavior Profiles',
          level: 'organisms',
          reason: 'No source-backed OrganismView or BehaviorProfile projection is available.',
          source: 'OrganismViewProjection / BehaviorProfileProjection'
        }),
        unavailableCard({
          id: 'organisms-size-bins',
          title: 'Organism Size Distribution',
          level: 'organisms',
          reason: 'No source-backed organism membership projection is available.',
          source: 'OrganismViewProjection'
        })
      ];
    case 'lineages':
      return unavailableLevelCards('lineages', [
        ['lineages-current-population', 'Lineage Current Population', 'No source-backed LineageProjection is available.'],
        ['lineages-history', 'Lineage History', 'No source-backed lineage history projection is available.'],
        ['lineages-genealogy', 'Compact Genealogy', 'No source-backed genealogy projection is available.'],
        ['lineages-footprint', 'Spatial Footprint', 'No source-backed lineage carrier footprint is available.']
      ]);
    case 'evolution':
      return unavailableLevelCards('evolution', [
        ['evolution-genome-provenance', 'Genome Provenance', 'No source-backed Genome provenance projection is available.'],
        ['evolution-mutation-history', 'Mutation History', 'No source-backed mutation history projection is available.'],
        ['evolution-diversity', 'Genome Diversity', 'No source-backed diversity projection is available.'],
        ['evolution-carriers', 'Carrier History', 'No source-backed genome carrier history is available.']
      ]);
    case 'analytics':
      return [
        unavailableCard({
          id: 'analytics-selected-metric',
          title: 'Selected Metric',
          level: 'analytics',
          reason: 'No selected source-backed Analytics metric is available.',
          source: 'MetricsProjection'
        })
      ];
    case 'world':
    default:
      return [
        populationLifecycleCard(state, 'world'),
        worldAccountingCard(state, accountingTarget),
        worldAccountingTimeCard(state, accountingTarget)
      ];
  }
}

function populationLifecycleCard(state: AppStore, level: 'world' | 'cells'): MonitorSurfaceCard {
  const availableDebug = state.debugProjections.status === 'available' ? state.debugProjections : null;
  const debugCells = availableDebug?.visualWorld.cells ?? [];
  const id = `${level}-population-lifecycle`;

  if (availableDebug === null || debugCells.length === 0) {
    return unavailableCard({
      id,
      title: 'Population Lifecycle',
      level,
      reason: 'Displayed projection has no source-backed lifecycle state counts.',
      source: 'WorldFrameProjection / VisualWorldProjection'
    });
  }

  const counts = new Map<string, number>();
  for (const cell of debugCells) {
    const key = normalizeLifecycle(cell.lifecycleState);
    counts.set(key, (counts.get(key) ?? 0) + 1);
  }
  const total = debugCells.length;

  return {
    id,
    title: 'Population Lifecycle',
    level,
    state: 'available',
    source: 'VisualWorldProjection.cells.lifecycleState',
    completeness: completenessLabel(availableDebug.visualWorld.completeness),
    unit: 'cells',
    rows: [
      { label: 'Total', value: formatInteger(total) },
      { label: 'Alive', value: formatInteger(counts.get('Alive') ?? 0) },
      { label: 'Stressed', value: formatInteger(counts.get('Stressed') ?? 0) },
      { label: 'Dormant', value: formatInteger(counts.get('Dormant') ?? 0) },
      { label: 'Dead', value: formatInteger(counts.get('Dead') ?? 0) }
    ],
    series: [
      { label: 'Alive', value: formatPercent(counts.get('Alive') ?? 0, total), color: '#4ade80' },
      { label: 'Stressed', value: formatPercent(counts.get('Stressed') ?? 0, total), color: '#fb7185' },
      { label: 'Dormant', value: formatPercent(counts.get('Dormant') ?? 0, total), color: '#fbbf24' },
      { label: 'Dead', value: formatPercent(counts.get('Dead') ?? 0, total), color: '#64748b' }
    ]
  };
}

function worldAccountingCard(state: AppStore, target: AccountingTarget): MonitorSurfaceCard {
  if (target === 'Energy') {
    return unavailableCard({
      id: 'world-energy-flow',
      title: 'Energy Flow',
      level: 'world',
      reason: 'No source-backed Energy Flow accounting projection is available. UI must not estimate it from resource energy values.',
      source: 'EnergyAccountingProjection'
    });
  }

  const resourceCycle =
    state.debugProjections.status === 'available'
      ? state.debugProjections.monitor?.payload.world.resourceCycle
      : null;

  if (target === 'Resource' && resourceCycle?.state === 'available') {
    return {
      id: 'world-resource-cycle',
      title: 'Resource Cycle',
      level: 'world',
      state: 'available',
      source: resourceCycle.source,
      completeness: completenessLabel(
        state.debugProjections.status === 'available'
          ? state.debugProjections.monitor?.completeness ?? null
          : null
      ),
      unit: 'resource amount',
      rows: [
        { label: 'Total Amount', value: formatNumber(resourceCycle.totalAmount) },
        { label: 'Explicit Decay / Sink', value: formatNumber(resourceCycle.accounting.explicitDecayOrSink) },
        { label: 'Metabolism / Cell Uptake', value: formatNumber(resourceCycle.accounting.metabolismOrCellUptake) },
        { label: 'Material Conversion', value: formatNumber(resourceCycle.accounting.materialConversion) },
        { label: 'Unclassified Loss', value: formatNumber(resourceCycle.accounting.unclassifiedLoss) }
      ],
      series: [
        { label: 'Environment', value: formatNumber(resourceCycle.locations.environment) },
        { label: 'Cells', value: formatNumber(resourceCycle.locations.cells) },
        { label: 'Materials', value: formatNumber(resourceCycle.locations.materials) },
        { label: 'Fragments', value: formatNumber(resourceCycle.locations.fragments) },
        { label: 'Explicit Sinks', value: formatNumber(resourceCycle.locations.explicitSinks) }
      ]
    };
  }

  const layers =
    state.debugProjections.status === 'available'
      ? state.debugProjections.visualWorld.resourceLayers
      : [];

  if (target === 'Resource' && layers.length > 0) {
    const total = layers.reduce((sum, layer) => sum + layer.totalAmount, 0);
    return {
      id: 'world-resource-cycle',
      title: 'Resource Cycle',
      level: 'world',
      state: 'partial',
      source: 'VisualWorldProjection.resourceLayers',
      completeness: completenessLabel(state.debugProjections.status === 'available' ? state.debugProjections.visualWorld.completeness : null),
      unit: 'resource amount',
      reason: 'Resource location accounting is limited to available visual resource layers.',
      rows: [
        { label: 'Total Resource Layers', value: formatInteger(layers.length) },
        { label: 'Total Amount', value: formatNumber(total) }
      ],
      series: layers.map((layer) => ({
        label: `Layer ${layer.layerIndex}`,
        value: formatNumber(layer.totalAmount)
      }))
    };
  }

  return unavailableCard({
    id: target === 'Resource' ? 'world-resource-cycle' : 'world-material-cycle',
    title: target === 'Resource' ? 'Resource Cycle' : 'Material Cycle',
    level: 'world',
    reason:
      target === 'Resource'
        ? 'No source-backed Resource registry/type projection is available.'
        : 'No source-backed Material accounting projection is available.',
    source: target === 'Resource' ? 'ResourceProjection' : 'MaterialAccountingProjection'
  });
}

function worldAccountingTimeCard(state: AppStore, target: AccountingTarget): MonitorSurfaceCard {
  const sourceBackedResourceMetrics = [
    'world.resource.environment',
    'world.resource.cells',
    'world.resource.materials',
    'world.resource.fragments',
    'world.resource.explicitSinks'
  ];
  const hasResourceHistory = sourceBackedResourceMetrics.some(
    (metric) => (state.monitorMetricHistory[metric]?.length ?? 0) > 0
  );

  if (target === 'Resource' && hasResourceHistory) {
    return {
      id: 'world-accounting-time',
      title: 'Resource Distribution Over Time',
      level: 'world',
      state: 'available',
      source: 'UI RRD metric history',
      completeness: 'bounded',
      unit: 'resource amount',
      rows: sourceBackedResourceMetrics.map((metric) => {
        const latest = state.monitorMetricHistory[metric]?.at(-1);
        return {
          label: metric.replace('world.resource.', ''),
          value: latest ? formatNumber(latest.value) : '0'
        };
      }),
      series: sourceBackedResourceMetrics.map((metric) => {
        const latest = state.monitorMetricHistory[metric]?.at(-1);
        return {
          label: metric.replace('world.resource.', ''),
          value: latest ? formatNumber(latest.value) : '0'
        };
      })
    };
  }

  return unavailableCard({
    id: 'world-accounting-time',
    title: `${target} Distribution Over Time`,
    level: 'world',
    reason: 'No UI RRD metric history has been populated from a source-backed accounting projection.',
    source: 'UI RRD metric history',
    unit: '100% distribution'
  });
}

function cellRadiusDistributionCard(state: AppStore): MonitorSurfaceCard {
  const cells = state.frame.cells;
  if (cells.length === 0) {
    return unavailableCard({
      id: 'cells-radius-distribution',
      title: 'Cell Radius Distribution',
      level: 'cells',
      reason: 'Displayed projection contains no Cells.',
      source: 'WorldFrameProjection.cells.radius'
    });
  }

  const radii = cells.map((cell) => cell.radius);
  const min = Math.min(...radii);
  const max = Math.max(...radii);

  return {
    id: 'cells-radius-distribution',
    title: 'Cell Radius Distribution',
    level: 'cells',
    state: 'available',
    source: 'WorldFrameProjection.cells.radius',
    completeness: 'bounded',
    unit: 'radius',
    rows: [
      { label: 'Cells', value: formatInteger(cells.length) },
      { label: 'Min Radius', value: formatNumber(min) },
      { label: 'Max Radius', value: formatNumber(max) }
    ],
    series: []
  };
}

function unavailableLevelCards(
  level: AnalysisLevel,
  rows: Array<[id: string, title: string, reason: string]>
): MonitorSurfaceCard[] {
  return rows.map(([id, title, reason]) =>
    unavailableCard({
      id,
      title,
      level,
      reason,
      source: 'Observer projection'
    })
  );
}

function unavailableCard(args: {
  id: string;
  title: string;
  level: AnalysisLevel;
  reason: string;
  source: string;
  unit?: string;
}): MonitorSurfaceCard {
  return {
    id: args.id,
    title: args.title,
    level: args.level,
    state: 'unavailable',
    source: args.source,
    completeness: 'unavailable',
    reason: args.reason,
    unit: args.unit,
    rows: [],
    series: []
  };
}

function accountingTargetOptions(debugProjections: DebugProjectionState, target: AccountingTarget): string[] {
  if (target !== 'Resource' || debugProjections.status !== 'available') {
    return [];
  }

  if (debugProjections.monitor?.payload.world.resourceCycle.state === 'available') {
    return [];
  }

  return debugProjections.visualWorld.resourceLayers.map((layer) => `Layer ${layer.layerIndex}`);
}

function classificationSource(debugProjections: DebugProjectionState): string {
  if (debugProjections.status !== 'available') {
    return 'ClassificationProjection';
  }

  return `ClassificationProjection (${completenessLabel(debugProjections.classifications.completeness)})`;
}

function completenessLabel(completeness: DebugProjectionCompleteness | null): string {
  return completeness?.state ?? 'unavailable';
}

function normalizeLifecycle(value: string): string {
  const lower = value.toLowerCase();
  if (lower.includes('stress')) return 'Stressed';
  if (lower.includes('dormant')) return 'Dormant';
  if (lower.includes('dead')) return 'Dead';
  return 'Alive';
}

function formatInteger(value: number): string {
  return new Intl.NumberFormat('en-US', { maximumFractionDigits: 0 }).format(value);
}

function formatNumber(value: number): string {
  return new Intl.NumberFormat('en-US', { maximumFractionDigits: 2 }).format(value);
}

function formatPercent(value: number, total: number): string {
  if (total <= 0) return '0.0%';
  return `${((value / total) * 100).toFixed(1)}%`;
}
