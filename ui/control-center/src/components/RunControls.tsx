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
  return (
    <div className="run-controls live-run-controls" aria-label="Live run controls">
      <button
        className="icon-button primary"
        type="button"
        aria-label="Start live run"
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
        Step N
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
  );
}
