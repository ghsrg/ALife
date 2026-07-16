import { act, screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { App } from './App';
import { liveProjectionToWorldFrame } from './projection/liveAdapter';
import { renderApp } from './test/render';

const mockRunner = vi.hoisted(() => {
  const apiInstance = {
    getServerInfo: vi.fn(),
    listScenarios: vi.fn(),
    getRunStatus: vi.fn(),
    startRun: vi.fn(),
    pauseRun: vi.fn(),
    resumeRun: vi.fn(),
    stepRun: vi.fn(),
    stopRun: vi.fn()
  };
  const streamInstances: Array<{
    baseUrl: string;
    handlers: {
      onConnectionState: (state: 'connecting' | 'connected' | 'disconnected') => void;
      onStatus: (status: RunStatusFixture) => void;
      onFrame: (frame: LiveFrameFixture) => void;
      onError: (error: Error) => void;
    };
    connect: ReturnType<typeof vi.fn>;
    disconnect: ReturnType<typeof vi.fn>;
  }> = [];

  return {
    apiInstance,
    streamInstances,
    RunnerApiClient: vi.fn(() => apiInstance),
    RunnerStreamClient: vi.fn((baseUrl, handlers) => {
      const instance = {
        baseUrl,
        handlers,
        connect: vi.fn(),
        disconnect: vi.fn()
      };
      streamInstances.push(instance);
      return instance;
    })
  };
});

interface RunStatusFixture {
  processState: 'ready';
  activeRunState: 'idle' | 'running' | 'paused' | 'completed' | 'failed';
  runId: string | null;
  committedTick: number;
  scenarioId: string | null;
  scenarioHash: string | null;
  effectiveSeed: number | null;
  terminalReason: string | null;
}

interface LiveFrameFixture {
  schemaVersion: 'ALIF/v2';
  committedTick: number;
  projectionSequence: number;
  wallClockGeneratedAtMs: number;
  previousCommittedTick: number | null;
  heat: number;
  waste: number;
  cells: Array<{
    id: number;
    x: number;
    y: number;
    radius: number;
    energy: number;
    lifecycle: number;
  }>;
}

vi.mock('./runner/apiClient', async (importOriginal) => ({
  ...(await importOriginal<typeof import('./runner/apiClient')>()),
  RunnerApiClient: mockRunner.RunnerApiClient
}));

vi.mock('./runner/streamClient', async (importOriginal) => ({
  ...(await importOriginal<typeof import('./runner/streamClient')>()),
  RunnerStreamClient: mockRunner.RunnerStreamClient
}));

vi.mock('./projection/liveAdapter', async (importOriginal) => {
  const actual = await importOriginal<typeof import('./projection/liveAdapter')>();
  return {
    ...actual,
    liveProjectionToWorldFrame: vi.fn(actual.liveProjectionToWorldFrame)
  };
});

vi.mock('./viewer/worldRenderer', () => ({
  mountWorldRenderer: vi.fn(() => Promise.resolve({
    renderFrame: vi.fn(),
    resize: vi.fn(),
    exportPng: vi.fn(() => 'data:image/png;base64,fixture'),
    destroy: vi.fn()
  }))
}));

const idleStatus: RunStatusFixture = {
  processState: 'ready',
  activeRunState: 'idle',
  runId: null,
  committedTick: 0,
  scenarioId: null,
  scenarioHash: null,
  effectiveSeed: null,
  terminalReason: null
};

const runningStatus: RunStatusFixture = {
  processState: 'ready',
  activeRunState: 'running',
  runId: 'run-live-1',
  committedTick: 7,
  scenarioId: 'demo-scenario',
  scenarioHash: 'sha256:demo',
  effectiveSeed: 42,
  terminalReason: null
};

function setupRunnerMocks() {
  mockRunner.apiInstance.getServerInfo.mockResolvedValue({
    engineVersion: 'engine-test',
    apiVersion: 'runner-test',
    allowRemoteViewer: false
  });
  mockRunner.apiInstance.listScenarios.mockResolvedValue([
    { id: 'demo-scenario', path: 'scenarios/demo.toml' }
  ]);
  mockRunner.apiInstance.getRunStatus.mockResolvedValue(idleStatus);
  mockRunner.apiInstance.startRun.mockResolvedValue({
    ok: true,
    runId: 'run-live-1',
    scenarioHash: 'sha256:demo',
    bootstrapManifest: {},
    effectiveSeed: 42,
    activeRunState: 'running'
  });
  mockRunner.apiInstance.pauseRun.mockResolvedValue({ ok: true, activeRunState: 'paused', committedTick: 7 });
  mockRunner.apiInstance.resumeRun.mockResolvedValue({ ok: true, activeRunState: 'running', committedTick: 7 });
  mockRunner.apiInstance.stepRun.mockResolvedValue({ ok: true, activeRunState: 'paused', committedTick: 8 });
  mockRunner.apiInstance.stopRun.mockResolvedValue({ ok: true, activeRunState: 'completed', committedTick: 8 });
}

function liveFrame({
  tick,
  sequence,
  cellId
}: {
  tick: number;
  sequence: number;
  cellId: number;
}): LiveFrameFixture {
  return {
    schemaVersion: 'ALIF/v2',
    committedTick: tick,
    projectionSequence: sequence,
    wallClockGeneratedAtMs: 1000 + sequence,
    previousCommittedTick: tick > 0 ? tick - 1 : null,
    heat: 0.25,
    waste: 0.5,
    cells: [
      { id: cellId, x: 12, y: 34, radius: 6, energy: 0.8, lifecycle: 1 }
    ]
  };
}

describe('App', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockRunner.streamInstances.length = 0;
    setupRunnerMocks();
  });

  it('renders the Monitor layout with fixture data and selected Cell Inspector', async () => {
    renderApp(<App />);

    expect(screen.getByRole('heading', { name: /alife control center/i })).toBeInTheDocument();
    expect(screen.getByRole('tab', { name: /monitor/i })).toHaveAttribute('aria-selected', 'true');
    expect(screen.getByText('UI-1A Deterministic Fixture')).toBeInTheDocument();
    expect(screen.getByText('Fixture Tick 128')).toBeInTheDocument();
    const inspector = screen.getByLabelText(/cell inspector/i);
    expect(within(inspector).getByRole('heading', { name: /cell inspector/i })).toBeInTheDocument();
    expect(within(inspector).getByText('cell-a')).toBeInTheDocument();
    await waitFor(() => {
      expect(screen.getByLabelText(/world viewer/i)).toHaveAttribute('data-ready', 'true');
    });
  });

  it('exports a viewer PNG from the toolbar', async () => {
    const user = userEvent.setup();
    renderApp(<App />);

    await waitFor(() => {
      expect(screen.getByLabelText(/world viewer/i)).toHaveAttribute('data-ready', 'true');
    });
    await user.click(screen.getByRole('button', { name: /export viewer png/i }));

    expect(screen.getByRole('status')).toHaveTextContent(/png ready/i);
  });

  it('connects to the Runner, lists scenarios, and starts a live run', async () => {
    const user = userEvent.setup();
    mockRunner.apiInstance.getRunStatus
      .mockResolvedValueOnce(idleStatus)
      .mockResolvedValueOnce(runningStatus);

    renderApp(<App />);

    await waitFor(() => {
      expect(mockRunner.RunnerApiClient).toHaveBeenCalledWith('http://127.0.0.1:8080');
      expect(mockRunner.apiInstance.getServerInfo).toHaveBeenCalledTimes(1);
      expect(mockRunner.apiInstance.listScenarios).toHaveBeenCalledTimes(1);
      expect(mockRunner.apiInstance.getRunStatus).toHaveBeenCalledTimes(1);
      expect(mockRunner.streamInstances[0]?.connect).toHaveBeenCalledTimes(1);
    });

    await user.click(screen.getByRole('button', { name: 'Play live run' }));

    expect(mockRunner.apiInstance.startRun).toHaveBeenCalledWith({
      scenarioId: 'demo-scenario',
      requestId: expect.stringMatching(/^ui-\d+$/)
    });
    await waitFor(() => {
      expect(mockRunner.apiInstance.getRunStatus).toHaveBeenCalledTimes(2);
    });
  });

  it('updates the Monitor frame from a stream frame', async () => {
    renderApp(<App />);

    await waitFor(() => {
      expect(mockRunner.streamInstances).toHaveLength(1);
    });

    act(() => {
      mockRunner.streamInstances[0].handlers.onStatus(runningStatus);
      mockRunner.streamInstances[0].handlers.onFrame(liveFrame({ tick: 9, sequence: 3, cellId: 77 }));
    });

    const workspace = screen.getByLabelText(/monitor workspace/i);
    expect(within(workspace).getByText('demo-scenario')).toBeInTheDocument();
    expect(within(workspace).getByText('Live Tick 9')).toBeInTheDocument();
    const inspector = screen.getByLabelText(/cell inspector/i);
    expect(within(inspector).getByText('77')).toBeInTheDocument();
  });

  it('ignores late stream callbacks after unmount', async () => {
    const consoleError = vi.spyOn(console, 'error').mockImplementation(() => undefined);
    const { unmount } = renderApp(<App />);

    await waitFor(() => {
      expect(mockRunner.streamInstances).toHaveLength(1);
    });
    const stream = mockRunner.streamInstances[0];
    vi.mocked(liveProjectionToWorldFrame).mockClear();

    unmount();
    act(() => {
      stream.handlers.onConnectionState('connected');
      stream.handlers.onStatus(runningStatus);
      stream.handlers.onFrame(liveFrame({ tick: 10, sequence: 1, cellId: 88 }));
      stream.handlers.onError(new Error('late stream error'));
    });

    expect(liveProjectionToWorldFrame).not.toHaveBeenCalled();
    expect(consoleError).not.toHaveBeenCalled();
    consoleError.mockRestore();
  });

  it('ignores stale live frames after a newer live frame', async () => {
    renderApp(<App />);

    await waitFor(() => {
      expect(mockRunner.streamInstances).toHaveLength(1);
    });

    act(() => {
      mockRunner.streamInstances[0].handlers.onStatus(runningStatus);
      mockRunner.streamInstances[0].handlers.onFrame(liveFrame({ tick: 12, sequence: 4, cellId: 1204 }));
      mockRunner.streamInstances[0].handlers.onFrame(liveFrame({ tick: 11, sequence: 9, cellId: 1199 }));
      mockRunner.streamInstances[0].handlers.onFrame(liveFrame({ tick: 12, sequence: 4, cellId: 1203 }));
      mockRunner.streamInstances[0].handlers.onFrame(liveFrame({ tick: 12, sequence: 3, cellId: 1202 }));
    });

    const workspace = screen.getByLabelText(/monitor workspace/i);
    expect(within(workspace).getByText('Live Tick 12')).toBeInTheDocument();
    const inspector = screen.getByLabelText(/cell inspector/i);
    expect(within(inspector).getByText('1204')).toBeInTheDocument();
    expect(within(inspector).queryByText('1199')).not.toBeInTheDocument();
    expect(within(inspector).queryByText('1203')).not.toBeInTheDocument();
    expect(within(inspector).queryByText('1202')).not.toBeInTheDocument();
  });

  it('does not start an overlapping command while another command is pending', async () => {
    const user = userEvent.setup();
    let releaseStart: (() => void) | null = null;
    mockRunner.apiInstance.startRun.mockImplementation(
      () => new Promise((resolve) => {
        releaseStart = () => resolve({
          ok: true,
          runId: 'run-live-1',
          scenarioHash: 'sha256:demo',
          bootstrapManifest: {},
          effectiveSeed: 42,
          activeRunState: 'running'
        });
      })
    );

    renderApp(<App />);

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Play live run' })).toBeEnabled();
    });

    await user.click(screen.getByRole('button', { name: 'Play live run' }));
    expect(screen.getByRole('button', { name: 'Play live run' })).toBeDisabled();
    await user.click(screen.getByRole('button', { name: 'Play live run' }));

    expect(mockRunner.apiInstance.startRun).toHaveBeenCalledTimes(1);

    await act(async () => {
      releaseStart?.();
    });
  });

  it('does not start when no selected scenario is available', async () => {
    const user = userEvent.setup();
    mockRunner.apiInstance.listScenarios.mockResolvedValue([]);

    renderApp(<App />);

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Play live run' })).toBeDisabled();
    });
    await user.click(screen.getByRole('button', { name: 'Play live run' }));

    expect(mockRunner.apiInstance.startRun).not.toHaveBeenCalled();
  });
});
