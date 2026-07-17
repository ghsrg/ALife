import { describe, expect, it } from 'vitest';
import type { CellProjection, WorldFrame } from '../projection/types';
import { projectCellForNavigatedRender, projectCellForRender } from './renderGeometry';

const frame: WorldFrame = {
  schemaVersion: 'WorldFrameProjection/v1',
  source: 'live',
  runId: 'run-live',
  scenarioName: 'demo_living_world',
  tick: 10,
  world: { width: 1200, height: 800 },
  resources: [],
  cells: []
};

function cell(radius: number): CellProjection {
  return {
    id: 'cell-1',
    x: 120,
    y: 80,
    radius,
    energy: 0.5,
    integrity: 1,
    generation: 0,
    roleHint: 'alive lifecycle state',
    lifecycle: 1
  };
}

describe('projectCellForRender', () => {
  it('keeps physical and display radius equal when the cell is large enough on screen', () => {
    const projection = projectCellForRender(cell(24), frame, { width: 1200, height: 800 });

    expect(projection).toEqual({
      id: 'cell-1',
      x: 120,
      y: 80,
      physicalRadiusPx: 24,
      displayRadiusPx: 24,
      interactionRadiusPx: 24,
      presentationMinimumApplied: false
    });
  });

  it('applies a display minimum without changing the physical radius', () => {
    const projection = projectCellForRender(cell(2), frame, { width: 1200, height: 800 });

    expect(projection.physicalRadiusPx).toBe(2);
    expect(projection.displayRadiusPx).toBe(7);
    expect(projection.interactionRadiusPx).toBe(18);
    expect(projection.presentationMinimumApplied).toBe(true);
  });

  it('uses the smaller viewport scale so circles remain proportional when aspect ratio differs', () => {
    const projection = projectCellForRender(cell(12), frame, { width: 600, height: 800 });

    expect(projection.x).toBe(60);
    expect(projection.y).toBe(80);
    expect(projection.physicalRadiusPx).toBe(6);
    expect(projection.displayRadiusPx).toBe(7);
    expect(projection.presentationMinimumApplied).toBe(true);
  });

  it('applies viewer camera transform to position and radius for navigated rendering', () => {
    const projection = projectCellForNavigatedRender(
      cell(12),
      frame,
      { width: 1200, height: 800 },
      { x: -100, y: 40, scale: 2 }
    );

    expect(projection.x).toBe(140);
    expect(projection.y).toBe(200);
    expect(projection.physicalRadiusPx).toBe(24);
    expect(projection.displayRadiusPx).toBe(24);
    expect(projection.presentationMinimumApplied).toBe(false);
  });
});
