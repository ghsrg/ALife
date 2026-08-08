export type CellId = string;

export type FrameSource = 'fixture' | 'live' | 'historical';

export type ProjectionCompleteness = 'full' | 'bounded' | 'partial' | 'stale' | 'unavailable';

export interface ResourceConcentration {
  organic: number;
  mineral: number;
  energy: number;
  layers?: Record<number, number>;
}

export interface PhenotypeTraitProjection {
  flagellaCount: number;
  spikeCount: number;
  receptorHaloIntensity: number;
  lineageHue: number;
  divisionFlashIntensity: number;
}

export interface CellProjection {
  id: CellId;
  x: number;
  y: number;
  radius: number;
  energy: number;
  energyRaw?: number;
  energyCapacity?: number;
  integrity: number;
  generation: number;
  roleHint: string;
  lifecycle?: number;
  materials?: Array<{ materialTypeId: number; amount: number }>;
  internalResources?: Array<{ resourceTypeId: number; amount: number }>;
  localExternalResources?: Array<{ resourceTypeId: number; amount: number }>;
  phenotypeTraits?: PhenotypeTraitProjection;
}

export interface JointProjection {
  id: string;
  sourceCellId: CellId;
  targetCellId: CellId;
  channelType: 'mechanical' | 'resource' | 'signal' | 'heat';
  tension?: number;
  activeSignal?: boolean;
}

export interface OrganismHullProjection {
  id: string;
  cellIds: CellId[];
  hullColorHue: number;
  organicMembraneTension: number;
}

export interface WorldFrame {
  schemaVersion: 'WorldFrameProjection/v1';
  source?: FrameSource;
  runId: string;
  scenarioName?: string;
  tick: number;
  world: {
    width: number;
    height: number;
  };
  resources: ResourceConcentration[][];
  cells: CellProjection[];
  joints?: JointProjection[];
  organismHulls?: OrganismHullProjection[];
  summary?: {
    heat: number;
    waste: number;
    projectionSequence?: number;
    previousTick?: number | null;
    generatedAtMs?: number;
  };
}

export interface UiFixture {
  version: 'ui-1a-fixture/v1';
  scenarioName: string;
  frame: WorldFrame;
}

export type DebugProjectionCompletenessState =
  | 'full'
  | 'bounded'
  | 'sampled'
  | 'partial'
  | 'debug_selected'
  | 'stale'
  | 'unavailable';

export interface DebugProjectionCompleteness {
  state: DebugProjectionCompletenessState;
  missingFields: string[];
  reason: string | null;
}

export interface DebugProjectionSourceMetric {
  fieldId: string;
  sourceOwner: string;
  sourcePath: string;
}

export interface DebugVisualJoint {
  id: number;
  cell1Id: number;
  cell2Id: number;
  restLength: number;
  pulseIntensity: number;
  signalSpeed: number;
}

export interface DebugVisualOrganism {
  id: number;
  cellIds: number[];
  hullColorHue: number;
  organicMembraneTension: number;
}

export interface DebugVisualCell {
  id: CellId;
  x: number;
  y: number;
  radius: number;
  energy: number;
  energyCapacity: number;
  lifecycleState: string;
  materials: Array<{ materialTypeId: number; amount: number }>;
  internalResources: Array<{ resourceTypeId: number; amount: number }>;
  localExternalResources: Array<{ resourceTypeId: number; amount: number }>;
  phenotypeTraits?: PhenotypeTraitProjection;
}

export interface DebugResourceCell {
  x: number;
  y: number;
  amount: number;
}

export interface DebugResourceLayer {
  layerIndex: number;
  resourceTypeId: number;
  resourceId: string;
  width: number;
  height: number;
  totalAmount: number;
  cells: DebugResourceCell[];
  completeness: DebugProjectionCompleteness;
}

export interface DebugField {
  fieldId: string;
  value: number;
  sourceMetric: DebugProjectionSourceMetric;
}

export interface DebugFieldCell {
  x: number;
  y: number;
  value: number;
}

export interface DebugFieldLayer {
  fieldId: string;
  width: number;
  height: number;
  summaryValue: number;
  cells: DebugFieldCell[];
  completeness: DebugProjectionCompleteness;
}

export interface DebugVisualWorldProjection {
  projectionKind: 'VisualWorldProjection';
  completeness: DebugProjectionCompleteness;
  cells: DebugVisualCell[];
  joints?: DebugVisualJoint[];
  organisms?: DebugVisualOrganism[];
  resourceLayers: DebugResourceLayer[];
  fieldLayers?: DebugFieldLayer[];
  fields: DebugField[];
  sourceMetrics: DebugProjectionSourceMetric[];
}

export interface DebugCoverageProjection {
  projectionKind: 'CoverageProjection';
  completeness: DebugProjectionCompleteness;
  mechanisms: Array<{ mechanismId: string; statusId: string; sourceReport: string }>;
}

export interface DebugWarningProjection {
  projectionKind: 'WarningProjection';
  completeness: DebugProjectionCompleteness;
  warnings: Array<{
    code: string;
    disposition: string;
    affectedScope: string;
    sourceReport: string;
    recommendedReruns: string[];
  }>;
}

export interface DebugClassificationProjection {
  projectionKind: 'ClassificationProjection';
  completeness: DebugProjectionCompleteness;
  classifications: unknown[];
}

export interface DebugBalanceFindingProjection {
  projectionKind: 'BalanceFindingProjection';
  completeness: DebugProjectionCompleteness;
  findings: unknown[];
}

export type MonitorDataState = 'available' | 'partial' | 'unavailable';

export interface MonitorUnavailableSection {
  state: 'unavailable';
  source: string;
  reason: string;
}

export interface MonitorPopulationLifecycle {
  state: 'available';
  source: string;
  total: number;
  alive: number;
  stressed: number;
  dormant: number;
  dead: number;
}

export interface MonitorResourceCycle {
  state: 'available';
  source: string;
  totalAmount: number;
  locations: {
    environment: number;
    cells: number;
    materials: number;
    fragments: number;
    explicitSinks: number;
  };
  accounting: {
    explicitDecayOrSink: number;
    metabolismOrCellUptake: number;
    materialConversion: number;
    unclassifiedLoss: number;
  };
}

export interface MonitorMaterialCycle {
  state: 'available';
  source: string;
  totalAmount: number;
  boundary: number;
  transport: number;
  metabolic: number;
  storage: number;
  synthesis: number;
  structural: number;
  repair: number;
  contractile: number;
  sensory: number;
}

export interface MonitorEnergyFlow {
  state: 'available';
  source: string;
  totalEnergy: number;
  energyCapacity: number;
  heat: number;
  waste: number;
  utilizationRate: number;
}

export interface MonitorOrganismBehaviorProfiles {
  state: 'available';
  source: string;
  totalOrganisms: number;
  motile: number;
  sessile: number;
  highEnergy: number;
  generalist: number;
}

export interface MonitorOrganismSizeBins {
  state: 'available';
  source: string;
  singleCell: number;
  small: number;
  medium: number;
  large: number;
}

export interface MonitorLineagesPayload {
  state: 'available';
  source: string;
  activeLineagesCount: number;
  maxGeneration: number;
  dominantHue: number;
  meanSpan: number;
}

export interface MonitorEvolutionPayload {
  state: 'available';
  source: string;
  totalGenerations: number;
  traitDiversityIndex: number;
  mutationEventsEstimate: number;
  activeCarriersCount: number;
}

export interface MonitorAnalyticsPayload {
  state: 'available';
  source: string;
  biomass: number;
  energyDensity: number;
  metabolicEfficiency: number;
  connectivityIndex: number;
}

export interface MonitorProjection {
  projectionKind: 'MonitorDataPanelProjection';
  runId: string;
  tick: number;
  source: string;
  completeness: DebugProjectionCompleteness;
  payload: {
    world: {
      populationLifecycle: MonitorPopulationLifecycle;
      resourceCycle: MonitorResourceCycle | MonitorUnavailableSection;
      materialCycle: MonitorMaterialCycle | MonitorUnavailableSection;
      energyFlow: MonitorEnergyFlow | MonitorUnavailableSection;
      accountingTime: MonitorUnavailableSection;
    };
    cells: {
      populationLifecycle: MonitorPopulationLifecycle;
      observedPrimaryRoles: MonitorUnavailableSection;
      potentialRoles: MonitorUnavailableSection;
      radiusDistribution: MonitorUnavailableSection;
    };
    organisms: {
      behaviorProfiles: MonitorOrganismBehaviorProfiles | MonitorUnavailableSection;
      sizeBins: MonitorOrganismSizeBins | MonitorUnavailableSection;
    };
    lineages: MonitorLineagesPayload | MonitorUnavailableSection;
    evolution: MonitorEvolutionPayload | MonitorUnavailableSection;
    analytics: MonitorAnalyticsPayload | MonitorUnavailableSection;
  };
}

export type DebugProjectionState =
  | {
      status: 'available';
      runId: string;
      tick: number;
      monitor?: MonitorProjection;
      visualWorld: DebugVisualWorldProjection;
      coverage: DebugCoverageProjection;
      warnings: DebugWarningProjection;
      classifications: DebugClassificationProjection;
      balanceFindings: DebugBalanceFindingProjection;
    }
  | {
      status: 'loading';
      runId: string;
      requestedTick: number;
      reason: string;
    }
  | {
      status: 'stale';
      runId: string;
      tick: number;
      reason: string;
    }
  | {
      status: 'unavailable';
      reason: string;
    };
