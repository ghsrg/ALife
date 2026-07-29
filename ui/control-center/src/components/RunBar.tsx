import { AppState, canPauseRun, canResumeRun, canStartRun, canStepRun, canStopRun } from '../app/appState';
import { uiText } from '../uiText';

export interface RunBarProps {
  state: AppState;
  onStart: () => void;
  onPause: () => void;
  onResume: () => void;
  onStep: () => void;
  onStop: () => void;
}

export function RunBar({ state, onStart, onPause, onResume, onStep, onStop }: RunBarProps) {
  const isRunning = state.runStatus?.activeRunState === 'running';
  const isLive = isRunning;
  const simRate = state.runStatus?.ticksPerSecond || 0;
  const tickCount = state.frame?.tick || 0;
  const seed = 'FIX-10992384';
  const runId = state.frame?.runId || 'LOCAL-001';
  
  const currentScenarioId = state.selectedScenarioId || 'default';
  const scenario = state.scenarios.find(s => s.id === currentScenarioId);
  const scenarioName = scenario ? (scenario as any).title || scenario.id : 'Unknown Scenario';

  return (
    <div className="cc-run-bar">
      {/* 1. DataContext */}
      <div className="cc-run-section" style={{ width: 200 }}>
        <div>
          {isLive ? (
            <span className="cc-live-badge">LIVE</span>
          ) : (
            <span className="cc-paused-badge">PAUSED</span>
          )}
          <span className="cc-run-id">{runId}</span>
        </div>
        <div className="cc-run-scenario">{scenarioName} Context</div>
      </div>
      
      {/* 2. Scenario */}
      <div className="cc-run-section" style={{ width: 200 }}>
        <div className="cc-run-label">SCENARIO</div>
        <div className="cc-scenario-name">{scenarioName}</div>
        <div className="cc-config-hash">CONFIG: {seed.substring(0,8)}</div>
      </div>
      
      {/* 3. RunControls */}
      <div className="cc-run-section" style={{ width: 200 }}>
        <div className="cc-run-label">CONTROL</div>
        <div className="cc-run-controls">
          <button
            className="cc-ctrl-btn"
            onClick={onStop}
            disabled={!canStopRun(state)}
            aria-label="Stop live run"
            title="Stop"
          >◄|</button>
          {!isRunning ? (
            <button
              className="cc-ctrl-btn primary"
              onClick={onStart}
              disabled={!canStartRun(state)}
              aria-label="Play live run"
              title="Play"
            >▶</button>
          ) : (
            <button
              className="cc-ctrl-btn primary"
              onClick={onPause}
              disabled={!canPauseRun(state)}
              aria-label="Pause live run"
              title="Pause"
            >‖</button>
          )}
          <button
            className="cc-ctrl-btn"
            onClick={onResume}
            disabled={!canResumeRun(state)}
            aria-label="Resume live run"
            title="Resume / Jump to Live"
          >►|</button>
          <button
            className="cc-ctrl-btn step"
            onClick={onStep}
            disabled={!canStepRun(state)}
            aria-label="Step one committed tick"
            title="Step 1 Tick"
          >STEP 1 TICK</button>
        </div>
      </div>
      
      {/* 4. SimRate */}
      <div className="cc-run-section" style={{ width: 160 }}>
        <div className="cc-run-label">SIMULATION RATE</div>
        <div className="cc-rate-value" aria-label="Simulation rate">
          {simRate > 9999 ? '∞ unlim' : `${simRate.toFixed(1)} ticks/s`}
        </div>
        <input 
          type="range" 
          className="cc-rate-slider"
          min="0" max="1" step="0.01"
          defaultValue="0.5"
          disabled
          aria-label="Simulation rate control (disabled)"
        />
      </div>
      
      {/* 5. MetricsBar */}
      <div className="cc-run-section" style={{ flex: 1, padding: 0, borderRight: 'none', flexDirection: 'row' }}>
        <div className="cc-metrics-bar">
          <div className="cc-metric">
            <div className="cc-metric-label">TICK</div>
            <div className="cc-metric-value">{tickCount}</div>
            <div className="cc-metric-sub positive">+1/s</div>
          </div>
          <div className="cc-metric">
            <div className="cc-metric-label">VISUAL FPS</div>
            <div className="cc-metric-value" aria-label="Visualization FPS">20-30 target</div>
            <div className="cc-metric-sub">Sync</div>
          </div>
          <div className="cc-metric">
            <div className="cc-metric-label">SEED</div>
            <div className="cc-metric-value">{seed}</div>
            <div className="cc-metric-sub">Active</div>
          </div>
          <div className="cc-metric unavailable">
            <div className="cc-metric-label">POPULATION</div>
            <div className="cc-metric-value">—</div>
            <div className="cc-metric-sub">disabled</div>
          </div>
          <div className="cc-metric unavailable">
            <div className="cc-metric-label">TOTAL ENERGY</div>
            <div className="cc-metric-value">—</div>
            <div className="cc-metric-sub">disabled</div>
          </div>
          <div className="cc-metric unavailable">
            <div className="cc-metric-label">LATENCY</div>
            <div className="cc-metric-value">—</div>
            <div className="cc-metric-sub">disabled</div>
          </div>
        </div>
      </div>
    </div>
  );
}
