import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { createAppStore, type AppState } from '../app/appState';
import type { RunStatus } from '../runner/apiClient';
import { RunControls } from './RunControls';

function runStatus(activeRunState: RunStatus['activeRunState']): RunStatus {
  return {
    processState: 'ready',
    activeRunState,
    runId: activeRunState === 'idle' ? null : 'run-1',
    committedTick: 12,
    scenarioId: 'demo',
    scenarioHash: 'hash-1',
    effectiveSeed: 42,
    terminalReason: null
  };
}

function appState(overrides: Partial<AppState>): AppState {
  return {
    ...createAppStore().getState(),
    connectionState: 'connected',
    scenarios: [{ id: 'demo', path: 'scenarios/demo.toml' }],
    selectedScenarioId: 'demo',
    runStatus: runStatus('idle'),
    ...overrides
  };
}

describe('RunControls', () => {
  it('starts from idle', async () => {
    const onStart = vi.fn();
    const user = userEvent.setup();

    render(
      <RunControls
        state={appState({ runStatus: runStatus('idle') })}
        onStart={onStart}
        onPause={vi.fn()}
        onResume={vi.fn()}
        onStep={vi.fn()}
        onStop={vi.fn()}
      />
    );

    await user.click(screen.getByRole('button', { name: 'Play live run' }));

    expect(onStart).toHaveBeenCalledTimes(1);
    expect(screen.getByRole('button', { name: 'Play live run' })).toHaveTextContent('Play');
    expect(screen.getByRole('button', { name: 'Pause live run' })).toBeDisabled();
  });

  it('pauses and stops from running', async () => {
    const onPause = vi.fn();
    const onStop = vi.fn();
    const user = userEvent.setup();

    render(
      <RunControls
        state={appState({ runStatus: runStatus('running') })}
        onStart={vi.fn()}
        onPause={onPause}
        onResume={vi.fn()}
        onStep={vi.fn()}
        onStop={onStop}
      />
    );

    await user.click(screen.getByRole('button', { name: 'Pause live run' }));
    await user.click(screen.getByRole('button', { name: 'Stop live run' }));

    expect(onPause).toHaveBeenCalledTimes(1);
    expect(onStop).toHaveBeenCalledTimes(1);
    expect(screen.getByRole('button', { name: 'Play live run' })).toBeDisabled();
    expect(screen.getByRole('button', { name: 'Resume live run' })).toBeDisabled();
    expect(screen.getByRole('button', { name: 'Step one committed tick' })).toBeDisabled();
  });

  it('resumes, steps, and stops only while paused', async () => {
    const onResume = vi.fn();
    const onStep = vi.fn();
    const onStop = vi.fn();
    const user = userEvent.setup();

    render(
      <RunControls
        state={appState({ runStatus: runStatus('paused') })}
        onStart={vi.fn()}
        onPause={vi.fn()}
        onResume={onResume}
        onStep={onStep}
        onStop={onStop}
      />
    );

    await user.click(screen.getByRole('button', { name: 'Resume live run' }));
    await user.click(screen.getByRole('button', { name: 'Step one committed tick' }));
    await user.click(screen.getByRole('button', { name: 'Stop live run' }));

    expect(onResume).toHaveBeenCalledTimes(1);
    expect(onStep).toHaveBeenCalledTimes(1);
    expect(onStop).toHaveBeenCalledTimes(1);
    expect(screen.getByRole('button', { name: 'Step one committed tick' })).toHaveTextContent(
      'Step 1'
    );
    expect(screen.getByRole('button', { name: 'Play live run' })).toBeDisabled();
    expect(screen.getByRole('button', { name: 'Pause live run' })).toBeDisabled();
  });
});
