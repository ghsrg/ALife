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
        observedPrimaryRolesCard(state),
        cellRadiusDistributionCard(state)
      ];
    case 'organisms':
      return [
        organismBehaviorProfilesCard(state),
        organismSizeBinsCard(state)
      ];
    case 'lineages':
      return [
        lineagePopulationCard(state),
        lineageHistoryCard(state),
        lineageGenealogyCard(state),
        lineageFootprintCard(state)
      ];
    case 'evolution':
      return [
        evolutionGenomeCard(state),
        evolutionMutationCard(state),
        evolutionDiversityCard(state),
        evolutionCarriersCard(state)
      ];
    case 'analytics':
      return [
        analyticsSelectedMetricCard(state)
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
  const monitorPayload =
    state.debugProjections.status === 'available'
      ? state.debugProjections.monitor?.payload
      : null;

  if (target === 'Energy') {
    const energyFlow = monitorPayload?.world.energyFlow;
    if (energyFlow?.state === 'available') {
      return {
        id: 'world-energy-flow',
        title: 'Energy Flow',
        level: 'world',
        state: 'available',
        source: energyFlow.source,
        completeness: 'bounded',
        unit: 'energy units',
        rows: [
          { label: 'Total Internal Energy', value: formatNumber(energyFlow.totalEnergy) },
          { label: 'Energy Capacity', value: formatNumber(energyFlow.energyCapacity) },
          { label: 'Environment Heat', value: formatNumber(energyFlow.heat) },
          { label: 'Waste Energy', value: formatNumber(energyFlow.waste) },
          { label: 'Utilization Rate', value: formatPercent(energyFlow.utilizationRate * 100, 100) }
        ],
        series: [
          { label: 'Internal Energy', value: formatNumber(energyFlow.totalEnergy), color: '#38bdf8' },
          { label: 'Capacity', value: formatNumber(energyFlow.energyCapacity), color: '#64748b' },
          { label: 'Heat', value: formatNumber(energyFlow.heat), color: '#ef4444' },
          { label: 'Waste', value: formatNumber(energyFlow.waste), color: '#fbbf24' }
        ]
      };
    }
  }

  if (target === 'Material') {
    const materialCycle = monitorPayload?.world.materialCycle;
    if (materialCycle?.state === 'available') {
      return {
        id: 'world-material-cycle',
        title: 'Material Cycle',
        level: 'world',
        state: 'available',
        source: materialCycle.source,
        completeness: 'bounded',
        unit: 'material units',
        rows: [
          { label: 'Total Materials', value: formatNumber(materialCycle.totalAmount) },
          { label: 'Boundary Material', value: formatNumber(materialCycle.boundary) },
          { label: 'Transport Material', value: formatNumber(materialCycle.transport) },
          { label: 'Metabolic Material', value: formatNumber(materialCycle.metabolic) },
          { label: 'Storage Material', value: formatNumber(materialCycle.storage) },
          { label: 'Synthesis Material', value: formatNumber(materialCycle.synthesis) }
        ],
        series: [
          { label: 'Boundary', value: formatNumber(materialCycle.boundary), color: '#38bdf8' },
          { label: 'Transport', value: formatNumber(materialCycle.transport), color: '#a855f7' },
          { label: 'Metabolic', value: formatNumber(materialCycle.metabolic), color: '#00c896' },
          { label: 'Storage', value: formatNumber(materialCycle.storage), color: '#eab308' },
          { label: 'Synthesis', value: formatNumber(materialCycle.synthesis), color: '#ef4444' }
        ]
      };
    }
  }

  const resourceCycle = monitorPayload?.world.resourceCycle;

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
    id: target === 'Resource' ? 'world-resource-cycle' : target === 'Energy' ? 'world-energy-flow' : 'world-material-cycle',
    title: target === 'Resource' ? 'Resource Cycle' : target === 'Energy' ? 'Energy Flow' : 'Material Cycle',
    level: 'world',
    reason: `No source-backed ${target === 'Energy' ? 'Energy Flow' : target} accounting projection is available.`,
    source: `${target}AccountingProjection`
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

function observedPrimaryRolesCard(state: AppStore): MonitorSurfaceCard {
  const debugProjections = state.debugProjections;
  if (!debugProjections || debugProjections.status !== 'available') {
    return unavailableCard({
      id: 'cells-role-distribution',
      title: 'Observed Primary Roles',
      level: 'cells',
      reason: 'No typed ClassificationProjection payload is available for observed Cell roles.',
      source: classificationSource(debugProjections)
    });
  }

  const classifications = debugProjections.classifications;
  if (
    !classifications ||
    classifications.completeness.state === 'unavailable' ||
    classifications.classifications.length === 0
  ) {
    return unavailableCard({
      id: 'cells-role-distribution',
      title: 'Observed Primary Roles',
      level: 'cells',
      reason: 'No typed ClassificationProjection payload is available for observed Cell roles.',
      source: classificationSource(debugProjections)
    });
  }

  const roleCounts: Record<string, number> = {};
  for (const item of classifications.classifications as Array<{ primary_label?: string; role?: string }>) {
    const rawRole = item.primary_label ?? item.role ?? 'Unclassified';
    const role = rawRole.charAt(0).toUpperCase() + rawRole.slice(1);
    roleCounts[role] = (roleCounts[role] ?? 0) + 1;
  }

  const rows = Object.entries(roleCounts).map(([label, count]) => ({
    label,
    value: formatInteger(count)
  }));

  return {
    id: 'cells-role-distribution',
    title: 'Observed Primary Roles',
    level: 'cells',
    state: 'available',
    source: classificationSource(state.debugProjections),
    completeness: classifications.completeness.state,
    unit: 'roles distribution',
    rows,
    series: rows
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
    subtitle: args.reason,
    reason: args.reason,
    unit: args.unit,
    rows: [],
    series: []
  };
}

function organismBehaviorProfilesCard(state: AppStore): MonitorSurfaceCard {
  const monitorPayload = state.debugProjections.status === 'available' ? state.debugProjections.monitor?.payload : null;
  const bp = monitorPayload?.organisms.behaviorProfiles;

  if (bp?.state === 'available') {
    return {
      id: 'organisms-behavior-profiles',
      title: 'Observed Behavior Profiles',
      level: 'organisms',
      state: 'available',
      source: bp.source,
      completeness: 'bounded',
      unit: 'organisms',
      rows: [
        { label: 'Total Organisms', value: formatInteger(bp.totalOrganisms) },
        { label: 'Motile Archetypes', value: formatInteger(bp.motile) },
        { label: 'Sessile Archetypes', value: formatInteger(bp.sessile) },
        { label: 'High Energy Organisms', value: formatInteger(bp.highEnergy) },
        { label: 'Generalist Organisms', value: formatInteger(bp.generalist) }
      ],
      series: [
        { label: 'Motile', value: formatInteger(bp.motile), color: '#38bdf8' },
        { label: 'Sessile', value: formatInteger(bp.sessile), color: '#64748b' },
        { label: 'High Energy', value: formatInteger(bp.highEnergy), color: '#eab308' },
        { label: 'Generalist', value: formatInteger(bp.generalist), color: '#00c896' }
      ]
    };
  }

  return unavailableCard({
    id: 'organisms-behavior-profiles',
    title: 'Observed Behavior Profiles',
    level: 'organisms',
    reason: 'No source-backed OrganismView or BehaviorProfile projection is available.',
    source: 'BehaviorProfileProjection'
  });
}

function organismSizeBinsCard(state: AppStore): MonitorSurfaceCard {
  const monitorPayload = state.debugProjections.status === 'available' ? state.debugProjections.monitor?.payload : null;
  const sb = monitorPayload?.organisms.sizeBins;

  if (sb?.state === 'available') {
    return {
      id: 'organisms-size-bins',
      title: 'Organism Size Distribution',
      level: 'organisms',
      state: 'available',
      source: sb.source,
      completeness: 'bounded',
      unit: 'size bins',
      rows: [
        { label: 'Single Cell (1)', value: formatInteger(sb.singleCell) },
        { label: 'Small (2-3 cells)', value: formatInteger(sb.small) },
        { label: 'Medium (4-7 cells)', value: formatInteger(sb.medium) },
        { label: 'Large (8+ cells)', value: formatInteger(sb.large) }
      ],
      series: [
        { label: 'Single', value: formatInteger(sb.singleCell), color: '#00c896' },
        { label: 'Small', value: formatInteger(sb.small), color: '#38bdf8' },
        { label: 'Medium', value: formatInteger(sb.medium), color: '#a855f7' },
        { label: 'Large', value: formatInteger(sb.large), color: '#ef4444' }
      ]
    };
  }

  return unavailableCard({
    id: 'organisms-size-bins',
    title: 'Organism Size Distribution',
    level: 'organisms',
    reason: 'No source-backed organism membership projection is available.',
    source: 'OrganismViewProjection'
  });
}

function lineagePopulationCard(state: AppStore): MonitorSurfaceCard {
  const monitorPayload = state.debugProjections.status === 'available' ? state.debugProjections.monitor?.payload : null;
  const lin = monitorPayload?.lineages;

  if (lin?.state === 'available') {
    return {
      id: 'lineages-current-population',
      title: 'Lineage Current Population',
      level: 'lineages',
      state: 'available',
      source: lin.source,
      completeness: 'bounded',
      unit: 'lineages',
      rows: [
        { label: 'Active Lineages', value: formatInteger(lin.activeLineagesCount) },
        { label: 'Max Generation', value: formatInteger(lin.maxGeneration) },
        { label: 'Dominant Lineage Hue', value: `${lin.dominantHue}°` },
        { label: 'Mean Lineage Span', value: formatNumber(lin.meanSpan) }
      ],
      series: [
        { label: 'Active Lineages', value: formatInteger(lin.activeLineagesCount), color: '#38bdf8' },
        { label: 'Mean Span', value: formatNumber(lin.meanSpan), color: '#00c896' }
      ]
    };
  }

  return unavailableCard({
    id: 'lineages-current-population',
    title: 'Lineage Current Population',
    level: 'lineages',
    reason: 'No source-backed LineageProjection is available.',
    source: 'LineageProjection'
  });
}

function lineageHistoryCard(state: AppStore): MonitorSurfaceCard {
  const monitorPayload = state.debugProjections.status === 'available' ? state.debugProjections.monitor?.payload : null;
  const lin = monitorPayload?.lineages;

  if (lin?.state === 'available') {
    return {
      id: 'lineages-history',
      title: 'Lineage History',
      level: 'lineages',
      state: 'available',
      source: lin.source,
      completeness: 'bounded',
      unit: 'lineages over time',
      rows: [
        { label: 'Active Lineages', value: formatInteger(lin.activeLineagesCount) },
        { label: 'Recorded Generations', value: formatInteger(lin.maxGeneration) }
      ],
      series: [
        { label: 'Lineages Count', value: formatInteger(lin.activeLineagesCount), color: '#a855f7' }
      ]
    };
  }

  return unavailableCard({
    id: 'lineages-history',
    title: 'Lineage History',
    level: 'lineages',
    reason: 'No source-backed lineage history projection is available.',
    source: 'LineageProjection'
  });
}

function lineageGenealogyCard(state: AppStore): MonitorSurfaceCard {
  const monitorPayload = state.debugProjections.status === 'available' ? state.debugProjections.monitor?.payload : null;
  const lin = monitorPayload?.lineages;

  if (lin?.state === 'available') {
    return {
      id: 'lineages-genealogy',
      title: 'Compact Genealogy',
      level: 'lineages',
      state: 'available',
      source: lin.source,
      completeness: 'bounded',
      unit: 'tree nodes',
      rows: [
        { label: 'Root Lineages', value: formatInteger(lin.activeLineagesCount) },
        { label: 'Tree Depth', value: formatInteger(lin.maxGeneration) }
      ],
      series: [
        { label: 'Roots', value: formatInteger(lin.activeLineagesCount), color: '#00c896' }
      ]
    };
  }

  return unavailableCard({
    id: 'lineages-genealogy',
    title: 'Compact Genealogy',
    level: 'lineages',
    reason: 'No source-backed genealogy projection is available.',
    source: 'LineageProjection'
  });
}

function lineageFootprintCard(state: AppStore): MonitorSurfaceCard {
  const monitorPayload = state.debugProjections.status === 'available' ? state.debugProjections.monitor?.payload : null;
  const lin = monitorPayload?.lineages;

  if (lin?.state === 'available') {
    return {
      id: 'lineages-footprint',
      title: 'Spatial Footprint',
      level: 'lineages',
      state: 'available',
      source: lin.source,
      completeness: 'bounded',
      unit: 'spatial area',
      rows: [
        { label: 'Active Clusters', value: formatInteger(lin.activeLineagesCount) },
        { label: 'Mean Lineage Span', value: formatNumber(lin.meanSpan) }
      ],
      series: [
        { label: 'Span', value: formatNumber(lin.meanSpan), color: '#fbbf24' }
      ]
    };
  }

  return unavailableCard({
    id: 'lineages-footprint',
    title: 'Spatial Footprint',
    level: 'lineages',
    reason: 'No source-backed lineage carrier footprint is available.',
    source: 'LineageProjection'
  });
}

function evolutionGenomeCard(state: AppStore): MonitorSurfaceCard {
  const monitorPayload = state.debugProjections.status === 'available' ? state.debugProjections.monitor?.payload : null;
  const evo = monitorPayload?.evolution;

  if (evo?.state === 'available') {
    return {
      id: 'evolution-genome-provenance',
      title: 'Genome Provenance',
      level: 'evolution',
      state: 'available',
      source: evo.source,
      completeness: 'bounded',
      unit: 'generations',
      rows: [
        { label: 'Total Generations', value: formatInteger(evo.totalGenerations) },
        { label: 'Active Carriers', value: formatInteger(evo.activeCarriersCount) }
      ],
      series: [
        { label: 'Generations', value: formatInteger(evo.totalGenerations), color: '#38bdf8' }
      ]
    };
  }

  return unavailableCard({
    id: 'evolution-genome-provenance',
    title: 'Genome Provenance',
    level: 'evolution',
    reason: 'No source-backed Genome provenance projection is available.',
    source: 'GenomeProjection'
  });
}

function evolutionMutationCard(state: AppStore): MonitorSurfaceCard {
  const monitorPayload = state.debugProjections.status === 'available' ? state.debugProjections.monitor?.payload : null;
  const evo = monitorPayload?.evolution;

  if (evo?.state === 'available') {
    return {
      id: 'evolution-mutation-history',
      title: 'Mutation History',
      level: 'evolution',
      state: 'available',
      source: evo.source,
      completeness: 'bounded',
      unit: 'mutations',
      rows: [
        { label: 'Estimated Mutation Events', value: formatInteger(evo.mutationEventsEstimate) },
        { label: 'Trait Diversity Index', value: formatNumber(evo.traitDiversityIndex) }
      ],
      series: [
        { label: 'Mutations', value: formatInteger(evo.mutationEventsEstimate), color: '#ef4444' }
      ]
    };
  }

  return unavailableCard({
    id: 'evolution-mutation-history',
    title: 'Mutation History',
    level: 'evolution',
    reason: 'No source-backed mutation history projection is available.',
    source: 'GenomeProjection'
  });
}

function evolutionDiversityCard(state: AppStore): MonitorSurfaceCard {
  const monitorPayload = state.debugProjections.status === 'available' ? state.debugProjections.monitor?.payload : null;
  const evo = monitorPayload?.evolution;

  if (evo?.state === 'available') {
    return {
      id: 'evolution-diversity',
      title: 'Genome Diversity',
      level: 'evolution',
      state: 'available',
      source: evo.source,
      completeness: 'bounded',
      unit: 'diversity index',
      rows: [
        { label: 'Diversity Index', value: formatNumber(evo.traitDiversityIndex) },
        { label: 'Active Carriers', value: formatInteger(evo.activeCarriersCount) }
      ],
      series: [
        { label: 'Diversity', value: formatNumber(evo.traitDiversityIndex), color: '#00c896' }
      ]
    };
  }

  return unavailableCard({
    id: 'evolution-diversity',
    title: 'Genome Diversity',
    level: 'evolution',
    reason: 'No source-backed diversity projection is available.',
    source: 'GenomeProjection'
  });
}

function evolutionCarriersCard(state: AppStore): MonitorSurfaceCard {
  const monitorPayload = state.debugProjections.status === 'available' ? state.debugProjections.monitor?.payload : null;
  const evo = monitorPayload?.evolution;

  if (evo?.state === 'available') {
    return {
      id: 'evolution-carriers',
      title: 'Carrier History',
      level: 'evolution',
      state: 'available',
      source: evo.source,
      completeness: 'bounded',
      unit: 'carriers',
      rows: [
        { label: 'Active Carriers', value: formatInteger(evo.activeCarriersCount) },
        { label: 'Total Generations', value: formatInteger(evo.totalGenerations) }
      ],
      series: [
        { label: 'Carriers', value: formatInteger(evo.activeCarriersCount), color: '#eab308' }
      ]
    };
  }

  return unavailableCard({
    id: 'evolution-carriers',
    title: 'Carrier History',
    level: 'evolution',
    reason: 'No source-backed genome carrier history is available.',
    source: 'GenomeProjection'
  });
}

function analyticsSelectedMetricCard(state: AppStore): MonitorSurfaceCard {
  const monitorPayload = state.debugProjections.status === 'available' ? state.debugProjections.monitor?.payload : null;
  const an = monitorPayload?.analytics;

  if (an?.state === 'available') {
    return {
      id: 'analytics-selected-metric',
      title: 'Selected Metric',
      level: 'analytics',
      state: 'available',
      source: an.source,
      completeness: 'bounded',
      unit: 'system metric',
      rows: [
        { label: 'Total Biomass', value: formatNumber(an.biomass) },
        { label: 'Energy Density', value: formatNumber(an.energyDensity) },
        { label: 'Metabolic Efficiency', value: formatPercent(an.metabolicEfficiency * 100, 100) },
        { label: 'Connectivity Index', value: formatNumber(an.connectivityIndex) }
      ],
      series: [
        { label: 'Biomass', value: formatNumber(an.biomass), color: '#00c896' },
        { label: 'Energy Density', value: formatNumber(an.energyDensity), color: '#38bdf8' },
        { label: 'Efficiency', value: formatPercent(an.metabolicEfficiency * 100, 100), color: '#fbbf24' },
        { label: 'Connectivity', value: formatNumber(an.connectivityIndex), color: '#a855f7' }
      ]
    };
  }

  return unavailableCard({
    id: 'analytics-selected-metric',
    title: 'Selected Metric',
    level: 'analytics',
    reason: 'No selected source-backed Analytics metric is available.',
    source: 'MetricsProjection'
  });
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
