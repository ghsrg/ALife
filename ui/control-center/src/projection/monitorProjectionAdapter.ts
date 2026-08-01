import type {
  DebugProjectionCompleteness,
  MonitorPopulationLifecycle,
  MonitorProjection,
  MonitorResourceCycle,
  MonitorUnavailableSection
} from './types';

interface WireCompleteness {
  state: DebugProjectionCompleteness['state'];
  missing_fields?: string[];
  reason?: string | null;
}

function completeness(value: WireCompleteness): DebugProjectionCompleteness {
  return {
    state: value.state,
    missingFields: value.missing_fields ?? [],
    reason: value.reason ?? null
  };
}

function unavailable(value: any): MonitorUnavailableSection {
  return {
    state: 'unavailable',
    source: String(value?.source ?? 'UnknownProjection'),
    reason: String(value?.reason ?? 'Projection section unavailable')
  };
}

function populationLifecycle(value: any): MonitorPopulationLifecycle {
  return {
    state: 'available',
    source: String(value.source),
    total: Number(value.total ?? 0),
    alive: Number(value.alive ?? 0),
    stressed: Number(value.stressed ?? 0),
    dormant: Number(value.dormant ?? 0),
    dead: Number(value.dead ?? 0)
  };
}

function resourceCycle(value: any): MonitorResourceCycle | MonitorUnavailableSection {
  if (value?.state !== 'available') {
    return unavailable(value);
  }

  return {
    state: 'available',
    source: String(value.source),
    totalAmount: Number(value.total_amount ?? 0),
    locations: {
      environment: Number(value.locations?.environment ?? 0),
      cells: Number(value.locations?.cells ?? 0),
      materials: Number(value.locations?.materials ?? 0),
      fragments: Number(value.locations?.fragments ?? 0),
      explicitSinks: Number(value.locations?.explicit_sinks ?? 0)
    },
    accounting: {
      explicitDecayOrSink: Number(value.accounting?.explicit_decay_or_sink ?? 0),
      metabolismOrCellUptake: Number(value.accounting?.metabolism_or_cell_uptake ?? 0),
      materialConversion: Number(value.accounting?.material_conversion ?? 0),
      unclassifiedLoss: Number(value.accounting?.unclassified_loss ?? 0)
    }
  };
}

function materialCycle(value: any) {
  if (value?.state !== 'available') return unavailable(value);
  return {
    state: 'available' as const,
    source: String(value.source),
    totalAmount: Number(value.total_amount ?? 0),
    boundary: Number(value.boundary ?? 0),
    transport: Number(value.transport ?? 0),
    metabolic: Number(value.metabolic ?? 0),
    storage: Number(value.storage ?? 0),
    synthesis: Number(value.synthesis ?? 0),
    structural: Number(value.structural ?? 0),
    repair: Number(value.repair ?? 0),
    contractile: Number(value.contractile ?? 0),
    sensory: Number(value.sensory ?? 0)
  };
}

function energyFlow(value: any) {
  if (value?.state !== 'available') return unavailable(value);
  return {
    state: 'available' as const,
    source: String(value.source),
    totalEnergy: Number(value.total_energy ?? 0),
    energyCapacity: Number(value.energy_capacity ?? 0),
    heat: Number(value.heat ?? 0),
    waste: Number(value.waste ?? 0),
    utilizationRate: Number(value.utilization_rate ?? 0)
  };
}

function organismBehaviorProfiles(value: any) {
  if (value?.state !== 'available') return unavailable(value);
  return {
    state: 'available' as const,
    source: String(value.source),
    totalOrganisms: Number(value.total_organisms ?? 0),
    motile: Number(value.motile ?? 0),
    sessile: Number(value.sessile ?? 0),
    highEnergy: Number(value.high_energy ?? 0),
    generalist: Number(value.generalist ?? 0)
  };
}

function organismSizeBins(value: any) {
  if (value?.state !== 'available') return unavailable(value);
  return {
    state: 'available' as const,
    source: String(value.source),
    singleCell: Number(value.single_cell ?? 0),
    small: Number(value.small ?? 0),
    medium: Number(value.medium ?? 0),
    large: Number(value.large ?? 0)
  };
}

function lineagesSummary(value: any) {
  if (value?.state !== 'available') return unavailable(value);
  return {
    state: 'available' as const,
    source: String(value.source),
    activeLineagesCount: Number(value.active_lineages_count ?? 0),
    maxGeneration: Number(value.max_generation ?? 0),
    dominantHue: Number(value.dominant_hue ?? 180),
    meanSpan: Number(value.mean_span ?? 1)
  };
}

function evolutionSummary(value: any) {
  if (value?.state !== 'available') return unavailable(value);
  return {
    state: 'available' as const,
    source: String(value.source),
    totalGenerations: Number(value.total_generations ?? 0),
    traitDiversityIndex: Number(value.trait_diversity_index ?? 0),
    mutationEventsEstimate: Number(value.mutation_events_estimate ?? 0),
    activeCarriersCount: Number(value.active_carriers_count ?? 0)
  };
}

function analyticsSummary(value: any) {
  if (value?.state !== 'available') return unavailable(value);
  return {
    state: 'available' as const,
    source: String(value.source),
    biomass: Number(value.biomass ?? 0),
    energyDensity: Number(value.energy_density ?? 0),
    metabolicEfficiency: Number(value.metabolic_efficiency ?? 0),
    connectivityIndex: Number(value.connectivity_index ?? 0)
  };
}

export function normalizeMonitorProjection(value: any): MonitorProjection {
  return {
    projectionKind: 'MonitorDataPanelProjection',
    runId: String(value.run_id),
    tick: Number(value.tick ?? 0),
    source: String(value.source ?? 'live'),
    completeness: completeness(value.completeness),
    payload: {
      world: {
        populationLifecycle: populationLifecycle(value.payload?.world?.population_lifecycle),
        resourceCycle: resourceCycle(value.payload?.world?.resource_cycle),
        materialCycle: materialCycle(value.payload?.world?.material_cycle),
        energyFlow: energyFlow(value.payload?.world?.energy_flow),
        accountingTime: unavailable(value.payload?.world?.accounting_time)
      },
      cells: {
        populationLifecycle: populationLifecycle(value.payload?.cells?.population_lifecycle),
        observedPrimaryRoles: unavailable(value.payload?.cells?.observed_primary_roles),
        potentialRoles: unavailable(value.payload?.cells?.potential_roles),
        radiusDistribution: unavailable(value.payload?.cells?.radius_distribution)
      },
      organisms: {
        behaviorProfiles: organismBehaviorProfiles(value.payload?.organisms?.behavior_profiles),
        sizeBins: organismSizeBins(value.payload?.organisms?.size_bins)
      },
      lineages: lineagesSummary(value.payload?.lineages),
      evolution: evolutionSummary(value.payload?.evolution),
      analytics: analyticsSummary(value.payload?.analytics)
    }
  };
}
