export interface ExperimentRunSnapshot {
  runId: string;
  scenarioId: string;
  effectiveSeed: number;
  tick: number;
  cellCount: number;
  jointCount: number;
  avgEnergy: number;
  warningCount: number;
}

export interface RunComparisonDelta {
  cellDelta: number;
  cellDeltaPercent: number;
  jointDelta: number;
  avgEnergyDelta: number;
  warningDelta: number;
}

export function compareExperimentRuns(
  runA: ExperimentRunSnapshot,
  runB: ExperimentRunSnapshot
): RunComparisonDelta {
  const cellDelta = runB.cellCount - runA.cellCount;
  const cellDeltaPercent = runA.cellCount > 0 ? (cellDelta / runA.cellCount) * 100 : 0;
  const jointDelta = runB.jointCount - runA.jointCount;
  const avgEnergyDelta = runB.avgEnergy - runA.avgEnergy;
  const warningDelta = runB.warningCount - runA.warningCount;

  return {
    cellDelta,
    cellDeltaPercent,
    jointDelta,
    avgEnergyDelta,
    warningDelta
  };
}

export const PRESET_EXPERIMENT_RUNS: ExperimentRunSnapshot[] = [
  {
    runId: 'preset-run-bootstrap',
    scenarioId: 'bootstrap_minimal_viable_world',
    effectiveSeed: 42,
    tick: 100,
    cellCount: 5,
    jointCount: 2,
    avgEnergy: 0.85,
    warningCount: 0
  },
  {
    runId: 'preset-run-diverse-rich',
    scenarioId: 'diverse_rich_world',
    effectiveSeed: 1001,
    tick: 500,
    cellCount: 24,
    jointCount: 18,
    avgEnergy: 0.72,
    warningCount: 1
  },
  {
    runId: 'preset-run-living-ecosystem',
    scenarioId: 'living_ecosystem',
    effectiveSeed: 2026,
    tick: 1000,
    cellCount: 68,
    jointCount: 54,
    avgEnergy: 0.64,
    warningCount: 0
  }
];
