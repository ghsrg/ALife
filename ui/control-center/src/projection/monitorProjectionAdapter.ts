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
        materialCycle: unavailable(value.payload?.world?.material_cycle),
        energyFlow: unavailable(value.payload?.world?.energy_flow),
        accountingTime: unavailable(value.payload?.world?.accounting_time)
      },
      cells: {
        populationLifecycle: populationLifecycle(value.payload?.cells?.population_lifecycle),
        observedPrimaryRoles: unavailable(value.payload?.cells?.observed_primary_roles),
        potentialRoles: unavailable(value.payload?.cells?.potential_roles),
        radiusDistribution: unavailable(value.payload?.cells?.radius_distribution)
      },
      organisms: {
        behaviorProfiles: unavailable(value.payload?.organisms?.behavior_profiles),
        sizeBins: unavailable(value.payload?.organisms?.size_bins)
      },
      lineages: unavailable(value.payload?.lineages),
      evolution: unavailable(value.payload?.evolution),
      analytics: unavailable(value.payload?.analytics)
    }
  };
}
