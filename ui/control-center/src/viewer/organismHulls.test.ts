import { describe, expect, it } from 'vitest';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { createWorldRenderPlan } from './worldRenderPlan';

describe('Organism Organic Hulls & Joint Pulses Render Plan', () => {
  it('creates joint render plan items and organism hulls for multi-cell frames', () => {
    const frame = {
      ...ui1aFixture.frame,
      joints: [
        {
          id: 'j-1',
          sourceCellId: 'cell-a',
          targetCellId: 'cell-b',
          channelType: 'resource' as const,
          activeSignal: true
        }
      ],
      organismHulls: [
        {
          id: 'org-hull-1',
          cellIds: ['cell-a', 'cell-b'],
          hullColorHue: 160,
          organicMembraneTension: 0.8
        }
      ]
    };

    const plan = createWorldRenderPlan(frame, null, { width: 1200, height: 800 });

    expect(plan.joints.length).toBe(1);
    expect(plan.joints[0].sourceCellId).toBe('cell-a');
    expect(plan.joints[0].targetCellId).toBe('cell-b');

    expect(plan.organismHulls.length).toBe(1);
    expect(plan.organismHulls[0].hullColorHue).toBe(160);
    expect(plan.organismHulls[0].points.length).toBe(2);
  });

  it('leaves joints and organismHulls empty when frame has independent unjointed cells', () => {
    const plan = createWorldRenderPlan(ui1aFixture.frame, null, { width: 1200, height: 800 });

    expect(plan.cells.length).toBeGreaterThanOrEqual(2);
    expect(plan.joints.length).toBe(0);
    expect(plan.organismHulls.length).toBe(0);
  });
});
