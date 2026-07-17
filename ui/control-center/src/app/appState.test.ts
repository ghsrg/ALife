import { describe, expect, it } from 'vitest';
import {
  canPauseRun,
  canResumeRun,
  canStartRun,
  canStepRun,
  canStopRun,
  createAppStore,
  getMonitorDataState
} from './appState';
import type { WorldFrame } from '../projection/types';
import type { RunStatus, ScenarioListItem, ServerInfo } from '../runner/apiClient';

const connectedInfo: ServerInfo = {
  engineVersion: '0.1.0',
  apiVersion: '1',
  allowRemoteViewer: false
};

const scenarios: ScenarioListItem[] = [
  { id: 'demo_living_world', path: 'demo/demo_living_world.toml' }
];

const runningStatus: RunStatus = {
  processState: 'ready',
  activeRunState: 'running',
  runId: 'run-1',
  committedTick: 5,
  scenarioId: 'demo_living_world',
  scenarioHash: 'hash-1',
  effectiveSeed: 100,
  terminalReason: null
};

function status(activeRunState: RunStatus['activeRunState']): RunStatus {
  return {
    ...runningStatus,
    activeRunState
  };
}

const liveFrame: WorldFrame = {
  schemaVersion: 'WorldFrameProjection/v1',
  source: 'live',
  runId: 'run-1',
  tick: 6,
  world: {
    width: 100,
    height: 80
  },
  resources: [],
  cells: [
    {
      id: 'live-cell-a',
      x: 10,
      y: 20,
      radius: 3,
      energy: 50,
      integrity: 0.9,
      generation: 1,
      roleHint: 'live cell'
    }
  ]
};

describe('createAppStore', () => {
  it('starts from fixture data and supports selection and theme changes', () => {
    const store = createAppStore();

    expect(store.getState().frame.runId).toBe('fixture-ui-1a');
    expect(store.getState().selectedCellId).toBe('cell-a');
    expect(store.getState().theme).toBe('dark');

    store.getState().selectCell('cell-c');
    expect(store.getState().selectedCell?.roleHint).toBe('resource-rich region');

    store.getState().setTheme('light');
    expect(store.getState().theme).toBe('light');
  });

  it('tracks runner endpoint, connection data, scenarios, selected scenario, status, and live frames', () => {
    const store = createAppStore();

    expect(store.getState().runnerEndpoint).toBe('http://127.0.0.1:8080');
    expect(store.getState().connectionState).toBe('disconnected');
    expect(store.getState().serverInfo).toBeNull();
    expect(store.getState().scenarios).toEqual([]);
    expect(store.getState().selectedScenarioId).toBeNull();
    expect(store.getState().runStatus).toBeNull();

    store.getState().setRunnerEndpoint('http://localhost:9090');
    store.getState().setConnected(connectedInfo);
    store.getState().setScenarios(scenarios);
    store.getState().setRunStatus(runningStatus);
    store.getState().setFrame(liveFrame);

    expect(store.getState().runnerEndpoint).toBe('http://localhost:9090');
    expect(store.getState().connectionState).toBe('connected');
    expect(store.getState().serverInfo).toEqual(connectedInfo);
    expect(store.getState().scenarios).toEqual(scenarios);
    expect(store.getState().selectedScenarioId).toBe('demo_living_world');
    expect(store.getState().runStatus).toEqual(runningStatus);
    expect(store.getState().frame.source).toBe('live');
    expect(store.getState().selectedCellId).toBe('live-cell-a');
  });

  it('preserves selected cell across frame updates when present and selects the first available cell otherwise', () => {
    const store = createAppStore(liveFrame);

    store.getState().setFrame({
      ...liveFrame,
      tick: 7,
      cells: [
        {
          ...liveFrame.cells[0],
          energy: 60
        },
        {
          ...liveFrame.cells[0],
          id: 'live-cell-b'
        }
      ]
    });
    expect(store.getState().selectedCellId).toBe('live-cell-a');
    expect(store.getState().selectedCell?.energy).toBe(60);

    store.getState().setFrame({
      ...liveFrame,
      tick: 8,
      cells: [
        {
          ...liveFrame.cells[0],
          id: 'live-cell-c'
        }
      ]
    });
    expect(store.getState().selectedCellId).toBe('live-cell-c');

    store.getState().setFrame({
      ...liveFrame,
      tick: 9,
      cells: []
    });
    expect(store.getState().selectedCellId).toBeNull();
    expect(store.getState().selectedCell).toBeNull();
  });

  it('preserves explicit empty selection across frame updates', () => {
    const store = createAppStore(liveFrame);

    store.getState().selectCell(null);
    store.getState().setFrame({
      ...liveFrame,
      tick: 7,
      cells: [
        {
          ...liveFrame.cells[0],
          energy: 60
        }
      ]
    });

    expect(store.getState().selectedCellId).toBeNull();
    expect(store.getState().selectedCell).toBeNull();
  });

  it('selects the first available cell when the selected cell disappears', () => {
    const store = createAppStore(liveFrame);

    store.getState().setFrame({
      ...liveFrame,
      tick: 8,
      cells: [
        {
          ...liveFrame.cells[0],
          id: 'live-cell-c'
        }
      ]
    });

    expect(store.getState().selectedCellId).toBe('live-cell-c');
  });

  it('keeps current selected scenario only when refreshed scenarios still contain it', () => {
    const store = createAppStore();

    store.getState().setScenarios([
      { id: 'first', path: 'first.toml' },
      { id: 'second', path: 'second.toml' }
    ]);
    expect(store.getState().selectedScenarioId).toBe('first');

    store.getState().setSelectedScenarioId('second');
    store.getState().setScenarios([
      { id: 'second', path: 'second.toml' },
      { id: 'third', path: 'third.toml' }
    ]);
    expect(store.getState().selectedScenarioId).toBe('second');

    store.getState().setScenarios([{ id: 'third', path: 'third.toml' }]);
    expect(store.getState().selectedScenarioId).toBe('third');

    store.getState().setScenarios([]);
    expect(store.getState().selectedScenarioId).toBeNull();
  });

  it('tracks pending command and clears last error when a new command starts', () => {
    const store = createAppStore();

    expect(store.getState().pendingCommand).toBeNull();
    expect(store.getState().lastError).toBeNull();

    store.getState().setError('failed to start');
    store.getState().setPendingCommand('connect');

    expect(store.getState().pendingCommand).toBe('connect');
    expect(store.getState().lastError).toBeNull();

    store.getState().clearPendingCommand();

    expect(store.getState().pendingCommand).toBeNull();
  });

  it('clears last error when connected server info is stored', () => {
    const store = createAppStore();

    store.getState().setError('connection failed');
    store.getState().setConnected(connectedInfo);

    expect(store.getState().connectionState).toBe('connected');
    expect(store.getState().serverInfo).toEqual(connectedInfo);
    expect(store.getState().lastError).toBeNull();
  });
});

describe('run control helpers', () => {
  it('allows starting only when connected with a selected scenario and no active run or terminal run', () => {
    const store = createAppStore();
    const state = store.getState();

    expect(canStartRun(state)).toBe(false);

    const base = {
      ...state,
      connectionState: 'connected' as const,
      selectedScenarioId: 'demo_living_world',
      scenarios,
      pendingCommand: null
    };

    expect(canStartRun({ ...base, runStatus: null })).toBe(true);
    expect(canStartRun({ ...base, runStatus: status('idle') })).toBe(true);
    expect(canStartRun({ ...base, runStatus: status('completed') })).toBe(true);
    expect(canStartRun({ ...base, runStatus: status('failed') })).toBe(true);
    expect(canStartRun({ ...base, runStatus: status('running') })).toBe(false);
    expect(canStartRun({ ...base, pendingCommand: 'start' })).toBe(false);
    expect(canStartRun({ ...base, selectedScenarioId: null })).toBe(false);
    expect(canStartRun({ ...base, selectedScenarioId: 'missing_scenario' })).toBe(false);
    expect(canStartRun({ ...base, scenarios: [] })).toBe(false);
    expect(canStartRun({ ...base, connectionState: 'disconnected' })).toBe(false);
  });

  it('allows pause, resume, step, and stop from valid run states when no command is pending', () => {
    const store = createAppStore();
    const base = {
      ...store.getState(),
      pendingCommand: null
    };

    expect(canPauseRun({ ...base, runStatus: status('running') })).toBe(true);
    expect(canPauseRun({ ...base, runStatus: status('paused') })).toBe(false);
    expect(canResumeRun({ ...base, runStatus: status('paused') })).toBe(true);
    expect(canResumeRun({ ...base, runStatus: status('running') })).toBe(false);
    expect(canStepRun({ ...base, runStatus: status('paused') })).toBe(true);
    expect(canStepRun({ ...base, runStatus: status('running') })).toBe(false);
    expect(canStopRun({ ...base, runStatus: status('running') })).toBe(true);
    expect(canStopRun({ ...base, runStatus: status('paused') })).toBe(true);
    expect(canStopRun({ ...base, runStatus: status('completed') })).toBe(false);
    expect(canStopRun({ ...base, runStatus: status('running'), pendingCommand: 'stop' })).toBe(false);
  });
});

describe('getMonitorDataState', () => {
  it('describes disconnected fixture data as offline fixture fallback', () => {
    const store = createAppStore();

    expect(getMonitorDataState(store.getState())).toBe('fixture-offline');
  });

  it('describes connected idle fixture data as idle fixture fallback', () => {
    const store = createAppStore();
    store.getState().setConnected(connectedInfo);
    store.getState().setRunStatus(status('idle'));

    expect(getMonitorDataState(store.getState())).toBe('fixture-idle');
  });

  it('describes running status without a live frame as waiting for live data', () => {
    const store = createAppStore();
    store.getState().setConnected(connectedInfo);
    store.getState().setRunStatus(runningStatus);

    expect(getMonitorDataState(store.getState())).toBe('live-waiting');
  });

  it('describes live frame data as live and disconnected live data as stale', () => {
    const store = createAppStore();
    store.getState().setConnected(connectedInfo);
    store.getState().setFrame(liveFrame);

    expect(getMonitorDataState(store.getState())).toBe('live');

    store.getState().setConnectionState('disconnected');

    expect(getMonitorDataState(store.getState())).toBe('stale-live');
  });
});
