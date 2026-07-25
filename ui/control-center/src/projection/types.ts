export type CellId = string;

export type FrameSource = 'fixture' | 'live' | 'historical';

export type ProjectionCompleteness = 'full' | 'bounded' | 'partial' | 'stale' | 'unavailable';

export interface ResourceConcentration {
  organic: number;
  mineral: number;
  energy: number;
  layers?: Record<number, number>;
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
}

export interface JointProjection {
  id: string;
  sourceCellId: CellId;
  targetCellId: CellId;
  channelType: 'mechanical' | 'resource' | 'signal' | 'heat';
  tension?: number;
  activeSignal?: boolean;
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
}

export interface DebugResourceCell {
  x: number;
  y: number;
  amount: number;
}

export interface DebugResourceLayer {
  layerIndex: number;
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

export interface DebugVisualWorldProjection {
  projectionKind: 'VisualWorldProjection';
  completeness: DebugProjectionCompleteness;
  cells: DebugVisualCell[];
  resourceLayers: DebugResourceLayer[];
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

export type DebugProjectionState =
  | {
      status: 'available';
      runId: string;
      tick: number;
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
