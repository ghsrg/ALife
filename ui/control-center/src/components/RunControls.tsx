import {
  canPauseRun,
  canResumeRun,
  canStartRun,
  canStepRun,
  canStopRun,
  type AppState
} from '../app/appState';

export interface RunControlsProps {
  state: AppState;
  onStart: () => void;
  onPause: () => void;
  onResume: () => void;
  onStep: () => void;
  onStop: () => void;
}

export function RunControls({
  state,
  onStart,
  onPause,
  onResume,
  onStep,
  onStop
}: RunControlsProps) {
  const simRate = formatTicksPerSecond(state.runStatus?.ticksPerSecond);

  return (
    <div className="run-control-stack">
      <div className="run-controls live-run-controls" aria-label="Live run controls">
        <button
          className="icon-button primary"
          type="button"
          aria-label="Play live run"
          disabled={!canStartRun(state)}
          onClick={onStart}
        >
          Play
        </button>
        <button
          className="icon-button"
          type="button"
          aria-label="Pause live run"
          disabled={!canPauseRun(state)}
          onClick={onPause}
        >
          Pause
        </button>
        <button
          className="icon-button"
          type="button"
          aria-label="Resume live run"
          disabled={!canResumeRun(state)}
          onClick={onResume}
        >
          Resume
        </button>
        <button
          className="icon-button"
          type="button"
          aria-label="Step one committed tick"
          disabled={!canStepRun(state)}
          onClick={onStep}
        >
          Step 1
        </button>
        <button
          className="icon-button danger"
          type="button"
          aria-label="Stop live run"
          disabled={!canStopRun(state)}
          onClick={onStop}
        >
          Stop
        </button>
      </div>
      <div className="run-telemetry" aria-label="Run telemetry">
        <span aria-label="Simulation rate">{`Sim rate: ${simRate} ticks/s`}</span>
        <span aria-label="Visualization FPS">Viewer FPS: 20-30 target</span>
      </div>
    </div>
  );
}

function formatTicksPerSecond(value: number | undefined) {
  if (typeof value !== 'number' || !Number.isFinite(value)) {
    return '0.0';
  }

  return value.toFixed(1);
}
