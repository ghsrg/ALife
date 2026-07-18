import { describe, expect, it } from 'vitest';
import type { WorldFrame } from '../projection/types';
import type { ViewerCamera } from './viewerNavigation';
import { createWorldRenderPlan } from './worldRenderPlan';

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
        selected: true,
        lifecycleState: 'alive',
        energyRatio: 0.5,
        integrityRatio: 1,
        semanticLevel: 'structure',
        showMetricRings: true,
        label: 'cell-a · E50 · I100'
      }
    ]);
    expect(plan.hasResourceField).toBe(true);
  });

  it('keeps live missing resources explicit in the render plan', () => {
    const plan = createWorldRenderPlan({ ...frame, source: 'live', resources: [] }, null, {
      width: 1200,
      height: 800
    });

    expect(plan.hasResourceField).toBe(false);
    expect(plan.cells[0].semanticLevel).toBe('entity');
    expect(plan.cells[0].showMetricRings).toBe(false);
  });

  it('does not expand unselected high-zoom cells with external metric rings', () => {
    const plan = createWorldRenderPlan(
      {
        ...frame,
        cells: [
          {
            ...frame.cells[0],
            id: 'cell-a'
          },
          {
            ...frame.cells[0],
            id: 'cell-b',
            x: 124
          }
        ]
      },
      'cell-a',
      { width: 1200, height: 800 },
      { x: 0, y: 0, scale: 12 }
    );

    expect(plan.cells[0].showMetricRings).toBe(true);
    expect(plan.cells[1].semanticLevel).toBe('internal-detail');
    expect(plan.cells[1].showMetricRings).toBe(false);
  });
});
