import { describe, expect, it } from 'vitest';
import { extractEvolutionSummary } from './evolutionModel';
import type { WorldFrame } from '../projection/types';

describe('evolutionModel', () => {
  it('extracts evolution metrics and Shannon diversity from cell generations', () => {
    const frame: WorldFrame = {
      schemaVersion: 'WorldFrameProjection/v1',
      runId: 'run-1',
      tick: 50,
      world: { width: 100, height: 100 },
      resources: [],
      cells: [
        { id: 'c1', x: 10, y: 10, radius: 2, energy: 0.8, integrity: 1, generation: 0, roleHint: 'transport' },
        { id: 'c2', x: 20, y: 20, radius: 2, energy: 0.6, integrity: 1, generation: 1, roleHint: 'feeder' },
        { id: 'c3', x: 30, y: 30, radius: 2, energy: 0.4, integrity: 1, generation: 1, roleHint: 'feeder' },
        { id: 'c4', x: 40, y: 40, radius: 2, energy: 0.9, integrity: 1, generation: 2, roleHint: 'transport' }
      ]
    };

    const summary = extractEvolutionSummary(frame);

    expect(summary.totalCells).toBe(4);
    expect(summary.minGeneration).toBe(0);
    expect(summary.maxGeneration).toBe(2);
    expect(summary.avgGeneration).toBe(1);
    expect(summary.shannonDiversityIndex).toBeGreaterThan(0);
    expect(summary.generationGroups.length).toBe(3);
    expect(summary.generationGroups[1].count).toBe(2); // Gen 1 has 2 cells
  });
});
