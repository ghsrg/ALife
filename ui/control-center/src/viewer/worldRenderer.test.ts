import { describe, expect, it } from 'vitest';
import type { WorldFrame } from '../projection/types';
import type { ViewerCamera } from './viewerNavigation';
import { createWorldRenderPlan } from './worldRenderer';

const frame: WorldFrame = {
  schemaVersion: 'WorldFrameProjection/v1',
  source: 'fixture',
  runId: 'fixture',
  scenarioName: 'fixture',
  tick: 1,
  world: { width: 1200, height: 800 },
  resources: [[{ organic: 0.4, mineral: 0.2, energy: 0.6 }]],
  cells: [
    {
      id: 'cell-a',
      x: 120,
      y: 80,
      radius: 12,
      energy: 0.5,
      integrity: 1,
      generation: 0,
      roleHint: 'alive lifecycle state',
      lifecycle: 1
    }
  ]
};

describe('createWorldRenderPlan', () => {
  it('uses camera transformed geometry for cells', () => {
    const camera: ViewerCamera = { x: -100, y: 40, scale: 2 };
    const plan = createWorldRenderPlan(frame, 'cell-a', { width: 1200, height: 800 }, camera);

    expect(plan.cells).toEqual([
      {
        id: 'cell-a',
        x: 140,
        y: 200,
        radius: 24,
        selected: true
      }
    ]);
  });
});
