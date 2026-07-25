import { describe, expect, it } from 'vitest';
import { compareExperimentRuns, type ExperimentRunSnapshot } from './experimentModel';

describe('experimentModel', () => {
  it('compares two run snapshots and computes accurate deltas', () => {
    const runA: ExperimentRunSnapshot = {
      runId: 'run-a',
      scenarioId: 'scen-1',
      effectiveSeed: 42,
      tick: 100,
      cellCount: 10,
      jointCount: 5,
      avgEnergy: 0.5,
      warningCount: 2
    };

    const runB: ExperimentRunSnapshot = {
      runId: 'run-b',
      scenarioId: 'scen-1',
      effectiveSeed: 43,
      tick: 100,
      cellCount: 15,
      jointCount: 8,
      avgEnergy: 0.7,
      warningCount: 0
    };

    const delta = compareExperimentRuns(runA, runB);

    expect(delta.cellDelta).toBe(5);
    expect(delta.cellDeltaPercent).toBe(50);
    expect(delta.jointDelta).toBe(3);
    expect(delta.avgEnergyDelta).toBeCloseTo(0.2);
    expect(delta.warningDelta).toBe(-2);
  });
});
