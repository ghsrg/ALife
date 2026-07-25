import { useState } from 'react';
import type { AppState } from '../app/appState';
import {
  compareExperimentRuns,
  PRESET_EXPERIMENT_RUNS,
  type ExperimentRunSnapshot
} from '../app/experimentModel';

export interface ExperimentWorkspaceProps {
  state: AppState;
}

export function ExperimentWorkspace({ state }: ExperimentWorkspaceProps) {
  const currentRunSnapshot: ExperimentRunSnapshot = {
    runId: state.frame.runId || 'active-run',
    scenarioId: state.selectedScenarioId || 'current_scenario',
    effectiveSeed: state.runStatus?.effectiveSeed ?? 42,
    tick: state.frame.tick,
    cellCount: state.frame.cells.length,
    jointCount: state.frame.joints?.length ?? 0,
    avgEnergy:
      state.frame.cells.length > 0
        ? state.frame.cells.reduce((acc, c) => acc + c.energy, 0) / state.frame.cells.length
        : 0,
    warningCount: state.debugProjections?.status === 'available' ? state.debugProjections.warnings.warnings.length : 0
  };

  const availableRuns = [currentRunSnapshot, ...PRESET_EXPERIMENT_RUNS];
  const [selectedRunAId, setSelectedRunAId] = useState<string>(availableRuns[0].runId);
  const [selectedRunBId, setSelectedRunBId] = useState<string>(
    availableRuns.length > 1 ? availableRuns[1].runId : availableRuns[0].runId
  );

  const runA = availableRuns.find((r) => r.runId === selectedRunAId) ?? currentRunSnapshot;
  const runB = availableRuns.find((r) => r.runId === selectedRunBId) ?? availableRuns[1] ?? currentRunSnapshot;

  const comparisonDelta = compareExperimentRuns(runA, runB);

  return (
    <section className="experiment-workspace" aria-label="Experiments and Run Comparison" style={{ color: '#dce6f1' }}>
      <header style={{ marginBottom: '20px' }}>
        <h2 style={{ margin: 0, fontSize: '18px', fontWeight: 600 }}>Experiments & Run Comparison (AL-007-S15)</h2>
        <p style={{ margin: '4px 0 0 0', fontSize: '13px', color: '#9bb0c1' }}>
          Compare simulation runs side-by-side by Tick, cell population dynamics, energy efficiency, and warnings.
        </p>
      </header>

      <div style={{ display: 'flex', gap: '16px', marginBottom: '20px' }}>
        <label style={{ display: 'flex', flexDirection: 'column', gap: '4px', fontSize: '13px', flex: 1 }}>
          <strong>Run A (Baseline):</strong>
          <select
            value={selectedRunAId}
            onChange={(e) => setSelectedRunAId(e.target.value)}
            style={{ padding: '6px 10px', borderRadius: '4px', background: '#1a1e24', color: '#fff', border: '1px solid rgba(255,255,255,0.2)' }}
          >
            {availableRuns.map((r) => (
              <option key={r.runId} value={r.runId}>
                {r.runId} ({r.scenarioId}, Seed {r.effectiveSeed})
              </option>
            ))}
          </select>
        </label>

        <label style={{ display: 'flex', flexDirection: 'column', gap: '4px', fontSize: '13px', flex: 1 }}>
          <strong>Run B (Target):</strong>
          <select
            value={selectedRunBId}
            onChange={(e) => setSelectedRunBId(e.target.value)}
            style={{ padding: '6px 10px', borderRadius: '4px', background: '#1a1e24', color: '#fff', border: '1px solid rgba(255,255,255,0.2)' }}
          >
            {availableRuns.map((r) => (
              <option key={r.runId} value={r.runId}>
                {r.runId} ({r.scenarioId}, Seed {r.effectiveSeed})
              </option>
            ))}
          </select>
        </label>
      </div>

      <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
        <h3 style={{ margin: '0 0 12px 0', fontSize: '15px' }}>Side-by-Side Metrics Comparison</h3>
        <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: '13px' }}>
          <thead>
            <tr style={{ borderBottom: '1px solid rgba(255,255,255,0.15)', textAlign: 'left' }}>
              <th style={{ padding: '8px' }}>Metric</th>
              <th style={{ padding: '8px' }}>Run A ({runA.runId})</th>
              <th style={{ padding: '8px' }}>Run B ({runB.runId})</th>
              <th style={{ padding: '8px' }}>Delta (B - A)</th>
            </tr>
          </thead>
          <tbody>
            <tr style={{ borderBottom: '1px solid rgba(255,255,255,0.05)' }}>
              <td style={{ padding: '8px' }}>Scenario ID</td>
              <td style={{ padding: '8px' }}>{runA.scenarioId}</td>
              <td style={{ padding: '8px' }}>{runB.scenarioId}</td>
              <td style={{ padding: '8px' }}>{runA.scenarioId === runB.scenarioId ? 'Identical' : 'Different'}</td>
            </tr>
            <tr style={{ borderBottom: '1px solid rgba(255,255,255,0.05)' }}>
              <td style={{ padding: '8px' }}>Effective Seed</td>
              <td style={{ padding: '8px' }}>{runA.effectiveSeed}</td>
              <td style={{ padding: '8px' }}>{runB.effectiveSeed}</td>
              <td style={{ padding: '8px' }}>{runA.effectiveSeed === runB.effectiveSeed ? 'Same Seed' : 'Multi-seed'}</td>
            </tr>
            <tr style={{ borderBottom: '1px solid rgba(255,255,255,0.05)' }}>
              <td style={{ padding: '8px' }}>Tick Count</td>
              <td style={{ padding: '8px' }}>{runA.tick}</td>
              <td style={{ padding: '8px' }}>{runB.tick}</td>
              <td style={{ padding: '8px' }}>{runB.tick - runA.tick}</td>
            </tr>
            <tr style={{ borderBottom: '1px solid rgba(255,255,255,0.05)' }}>
              <td style={{ padding: '8px' }}>Cell Population</td>
              <td style={{ padding: '8px' }}>{runA.cellCount}</td>
              <td style={{ padding: '8px' }}>{runB.cellCount}</td>
              <td style={{ padding: '8px', color: comparisonDelta.cellDelta >= 0 ? '#2ea44f' : '#f85149' }}>
                {comparisonDelta.cellDelta >= 0 ? `+${comparisonDelta.cellDelta}` : comparisonDelta.cellDelta} ({comparisonDelta.cellDeltaPercent.toFixed(1)}%)
              </td>
            </tr>
            <tr style={{ borderBottom: '1px solid rgba(255,255,255,0.05)' }}>
              <td style={{ padding: '8px' }}>Joint Connections</td>
              <td style={{ padding: '8px' }}>{runA.jointCount}</td>
              <td style={{ padding: '8px' }}>{runB.jointCount}</td>
              <td style={{ padding: '8px' }}>
                {comparisonDelta.jointDelta >= 0 ? `+${comparisonDelta.jointDelta}` : comparisonDelta.jointDelta}
              </td>
            </tr>
            <tr style={{ borderBottom: '1px solid rgba(255,255,255,0.05)' }}>
              <td style={{ padding: '8px' }}>Average Cell Energy</td>
              <td style={{ padding: '8px' }}>{runA.avgEnergy.toFixed(2)}</td>
              <td style={{ padding: '8px' }}>{runB.avgEnergy.toFixed(2)}</td>
              <td style={{ padding: '8px' }}>
                {comparisonDelta.avgEnergyDelta >= 0 ? `+${comparisonDelta.avgEnergyDelta.toFixed(2)}` : comparisonDelta.avgEnergyDelta.toFixed(2)}
              </td>
            </tr>
            <tr style={{ borderBottom: '1px solid rgba(255,255,255,0.05)' }}>
              <td style={{ padding: '8px' }}>Warnings Count</td>
              <td style={{ padding: '8px' }}>{runA.warningCount}</td>
              <td style={{ padding: '8px' }}>{runB.warningCount}</td>
              <td style={{ padding: '8px', color: comparisonDelta.warningDelta <= 0 ? '#2ea44f' : '#f85149' }}>
                {comparisonDelta.warningDelta >= 0 ? `+${comparisonDelta.warningDelta}` : comparisonDelta.warningDelta}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </section>
  );
}
