import { describe, expect, it } from 'vitest';
import { extractSpecializationSummary } from './specializationModel';
import type { WorldFrame } from '../projection/types';

describe('specializationModel', () => {
  it('computes role specialization groups, Herfindahl index, and classifier confidence', () => {
    const frame: WorldFrame = {
      schemaVersion: 'WorldFrameProjection/v1',
      runId: 'run-1',
      tick: 100,
      world: { width: 100, height: 100 },
      resources: [],
      cells: [
        { id: 'c1', x: 10, y: 10, radius: 2, energy: 0.8, integrity: 1, generation: 0, roleHint: 'transport' },
        { id: 'c2', x: 12, y: 10, radius: 2, energy: 0.8, integrity: 1, generation: 0, roleHint: 'transport' },
        { id: 'c3', x: 20, y: 20, radius: 2, energy: 0.5, integrity: 1, generation: 0, roleHint: 'feeder' }
      ]
    };

    const summary = extractSpecializationSummary(frame);

    expect(summary.totalCells).toBe(3);
    expect(summary.dominantRole).toBe('transport');
    expect(summary.specializationIndex).toBeGreaterThan(0.5);
    expect(summary.overallConfidence).toBeCloseTo(0.95);
    expect(summary.roles.length).toBe(2);
    expect(summary.roles[0].role).toBe('transport');
    expect(summary.roles[0].count).toBe(2);
  });
});
