import { describe, expect, it } from 'vitest';
import type { WorldFrame } from '../projection/types';
import { createWorldRenderPlan } from './worldRenderPlan';

describe('createWorldRenderPlan Phenotype Traits', () => {
  it('maps phenotype traits onto render plan cells', () => {
    const frame: WorldFrame = {
      schemaVersion: 'WorldFrameProjection/v1',
      runId: 'test-run',
      tick: 10,
      world: { width: 100, height: 100 },
      resources: [],
      cells: [
        {
          id: 'cell-1',
          x: 50,
          y: 50,
          radius: 12,
          energy: 80,
          energyCapacity: 100,
          integrity: 1.0,
          generation: 1,
          roleHint: 'Producer',
          phenotypeTraits: {
            flagellaCount: 2,
            spikeCount: 4,
            receptorHaloIntensity: 0.8,
            lineageHue: 120,
            divisionFlashIntensity: 0.7
          }
        }
      ]
    };

    const plan = createWorldRenderPlan(frame, null, { width: 800, height: 600 });
    expect(plan.cells.length).toBe(1);

    const cell = plan.cells[0];
    expect(cell.flagellaCount).toBe(2);
    expect(cell.spikeCount).toBe(4);
    expect(cell.receptorHaloIntensity).toBe(0.8);
    expect(cell.lineageHue).toBe(120);
    expect(cell.divisionFlashIntensity).toBe(0.7);
  });
});
