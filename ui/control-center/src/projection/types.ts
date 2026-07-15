export type CellId = string;

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
}

export interface WorldFrame {
  schemaVersion: 'WorldFrameProjection/v1';
  runId: string;
  tick: number;
  world: {
    width: number;
    height: number;
  };
  resources: ResourceConcentration[][];
  cells: CellProjection[];
}

export interface UiFixture {
  version: 'ui-1a-fixture/v1';
  scenarioName: string;
  frame: WorldFrame;
}
