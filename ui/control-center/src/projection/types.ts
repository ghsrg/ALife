export type CellId = string;

export type FrameSource = 'fixture' | 'live' | 'historical';

export type ProjectionCompleteness = 'full' | 'bounded' | 'partial' | 'stale' | 'unavailable';

export interface ResourceConcentration {
  organic: number;
  mineral: number;
  energy: number;
}

export interface CellProjection {
  id: CellId;
  x: number;
  y: number;
  radius: number;
  energy: number;
  integrity: number;
  generation: number;
  roleHint: string;
  lifecycle?: number;
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
