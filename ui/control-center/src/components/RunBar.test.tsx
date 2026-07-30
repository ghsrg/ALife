import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { createAppStore, type AppStore } from '../app/appState';
import type { RunStatus } from '../runner/apiClient';
import { RunBar } from './RunBar';

function runningStatus(): RunStatus {
  return {
    processState: 'ready',
    activeRunState: 'running',
    runId: 'run-1',
    committedTick: 42,
    scenarioId: 'living_patchy_world',
    scenarioHash: 'hash-1',
    effectiveSeed: 10992384,
    terminalReason: null,
    ticksPerSecond: 30
  };
}

function appState(overrides: Partial<AppStore> = {}): AppStore {
  return {
    ...createAppStore().getState(),
    connectionState: 'connected',
    scenarios: [{ id: 'living_patchy_world', path: 'config/scenarios/living_patchy_world.toml' }],
    selectedScenarioId: 'living_patchy_world',
    runStatus: runningStatus(),
    ...overrides
  };
}

describe('RunBar', () => {
  it('renders Stop as an unambiguous square command', async () => {
    const onStop = vi.fn();
    const user = userEvent.setup();

    render(
      <RunBar
        state={appState()}
        onStart={vi.fn()}
        onPause={vi.fn()}
        onResume={vi.fn()}
        onStep={vi.fn()}
        onStop={onStop}
      />
    );

    const stop = screen.getByRole('button', { name: 'Stop live run' });

    expect(stop).toHaveTextContent('\u25a0');
    expect(stop).not.toHaveTextContent('\u25c4');
    expect(stop).not.toHaveTextContent('\u25c0');
    await user.click(stop);
    expect(onStop).toHaveBeenCalledTimes(1);
  });

  it('renders Runner connection controls in the Run/Data Context surface', () => {
    render(
      <RunBar
        state={appState({
          serverInfo: { engineVersion: 'engine-test', apiVersion: 'runner-test', allowRemoteViewer: false }
        })}
        onStart={vi.fn()}
        onPause={vi.fn()}
        onResume={vi.fn()}
        onStep={vi.fn()}
        onStop={vi.fn()}
      />
    );

    const runTrack = screen.getByTestId('monitor-run-track');

    expect(runTrack).toHaveTextContent('Runner: Connected');
    expect(runTrack).toHaveTextContent('http://127.0.0.1:8080');
    expect(runTrack).toHaveTextContent('API runner-test');
    expect(runTrack).toHaveTextContent('Data:');
    expect(screen.getByRole('button', { name: 'Reconnect to Runner' })).toBeInTheDocument();
    expect(screen.queryByRole('combobox', { name: 'Scenario' })).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: /scenario/i })).toHaveTextContent('living_patchy_world');
  });

  it('uses Frame Age instead of Latency in the monitor metrics', () => {
    render(
      <RunBar
        state={appState()}
        onStart={vi.fn()}
        onPause={vi.fn()}
        onResume={vi.fn()}
        onStep={vi.fn()}
        onStop={vi.fn()}
      />
    );

    const runTrack = screen.getByTestId('monitor-run-track');

    expect(runTrack).toHaveTextContent('FRAME AGE');
    expect(runTrack).not.toHaveTextContent('LATENCY');
  });
});
