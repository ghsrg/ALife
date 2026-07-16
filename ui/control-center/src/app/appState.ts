import { createStore } from 'zustand/vanilla';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { loadFixtureFrame, selectCell } from '../projection/fixtureAdapter';
import type { CellId, CellProjection, WorldFrame } from '../projection/types';
import type { RunStatus, ScenarioListItem, ServerInfo } from '../runner/apiClient';
import type { RunnerStreamConnectionState as ConnectionState } from '../runner/streamClient';

export type ThemeMode = 'dark' | 'light';
export type PendingCommand = 'connect' | 'start' | 'pause' | 'resume' | 'step' | 'stop';
export type MonitorDataState =
  | 'fixture-offline'
  | 'fixture-idle'
  | 'live-waiting'
  | 'live'
  | 'stale-live';

export interface AppState {
  frame: WorldFrame;
  selectedCellId: CellId | null;
  selectedCell: CellProjection | null;
  theme: ThemeMode;
  runnerEndpoint: string;
  connectionState: ConnectionState;
  serverInfo: ServerInfo | null;
  scenarios: ScenarioListItem[];
  selectedScenarioId: string | null;
  runStatus: RunStatus | null;
  pendingCommand: PendingCommand | null;
  lastError: string | null;
}

export interface AppActions {
  setFrame: (frame: WorldFrame) => void;
  selectCell: (cellId: CellId | null) => void;
  setTheme: (theme: ThemeMode) => void;
  setRunnerEndpoint: (endpoint: string) => void;
  setConnectionState: (connectionState: ConnectionState) => void;
  setConnected: (serverInfo: ServerInfo) => void;
  setScenarios: (scenarios: ScenarioListItem[]) => void;
  setSelectedScenarioId: (scenarioId: string | null) => void;
  setRunStatus: (runStatus: RunStatus | null) => void;
  setPendingCommand: (command: PendingCommand) => void;
  clearPendingCommand: () => void;
  setError: (error: string | null) => void;
}

export type AppStore = AppState & AppActions;

function selectInitialCell(frame: WorldFrame) {
  return frame.cells[0] ?? null;
}

function selectCellForFrame(frame: WorldFrame, currentCellId: CellId | null) {
  if (currentCellId !== null) {
    const selectedCell = selectCell(frame, currentCellId);
    if (selectedCell !== null) {
      return selectedCell;
    }
  }

  return selectInitialCell(frame);
}

export function createAppStore(initialFrame = loadFixtureFrame(ui1aFixture)) {
  const initialCell = selectInitialCell(initialFrame);

  return createStore<AppStore>((set, get) => ({
    frame: initialFrame,
    selectedCellId: initialCell?.id ?? null,
    selectedCell: initialCell,
    theme: 'dark',
    runnerEndpoint: 'http://127.0.0.1:8080',
    connectionState: 'disconnected',
    serverInfo: null,
    scenarios: [],
    selectedScenarioId: null,
    runStatus: null,
    pendingCommand: null,
    lastError: null,
    setFrame: (frame) => {
      const selectedCell = selectCellForFrame(frame, get().selectedCellId);
      set({
        frame,
        selectedCellId: selectedCell?.id ?? null,
        selectedCell
      });
    },
    selectCell: (cellId) => {
      const selectedCell = selectCell(get().frame, cellId);
      set({
        selectedCellId: selectedCell?.id ?? null,
        selectedCell
      });
    },
    setTheme: (theme) => set({ theme }),
    setRunnerEndpoint: (runnerEndpoint) => set({ runnerEndpoint }),
    setConnectionState: (connectionState) => set({ connectionState }),
    setConnected: (serverInfo) =>
      set({
        connectionState: 'connected',
        serverInfo,
        lastError: null
      }),
    setScenarios: (scenarios) =>
      set({
        scenarios,
        selectedScenarioId:
          scenarios.find((scenario) => scenario.id === get().selectedScenarioId)?.id ??
          scenarios[0]?.id ??
          null
      }),
    setSelectedScenarioId: (selectedScenarioId) => set({ selectedScenarioId }),
    setRunStatus: (runStatus) => set({ runStatus }),
    setPendingCommand: (pendingCommand) => set({ pendingCommand, lastError: null }),
    clearPendingCommand: () => set({ pendingCommand: null }),
    setError: (lastError) => set({ lastError })
  }));
}

export function getMonitorDataState(
  state: Pick<AppState, 'connectionState' | 'runStatus' | 'frame'>
): MonitorDataState {
  if (state.frame.source === 'live' && state.connectionState === 'disconnected') {
    return 'stale-live';
  }

  if (state.frame.source === 'live') {
    return 'live';
  }

  if (state.connectionState !== 'connected') {
    return 'fixture-offline';
  }

  if (
    state.runStatus?.activeRunState === 'running' ||
    state.runStatus?.activeRunState === 'paused'
  ) {
    return 'live-waiting';
  }

  return 'fixture-idle';
}

function hasNoPendingCommand(state: Pick<AppState, 'pendingCommand'>) {
  return state.pendingCommand === null;
}

export function canStartRun(
  state: Pick<
    AppState,
    'connectionState' | 'scenarios' | 'selectedScenarioId' | 'pendingCommand' | 'runStatus'
  >
) {
  if (
    state.connectionState !== 'connected' ||
    state.selectedScenarioId === null ||
    !state.scenarios.some((scenario) => scenario.id === state.selectedScenarioId) ||
    !hasNoPendingCommand(state)
  ) {
    return false;
  }

  if (state.runStatus === null) {
    return true;
  }

  return ['idle', 'completed', 'failed'].includes(state.runStatus.activeRunState);
}

export function canPauseRun(state: Pick<AppState, 'pendingCommand' | 'runStatus'>) {
  return hasNoPendingCommand(state) && state.runStatus?.activeRunState === 'running';
}

export function canResumeRun(state: Pick<AppState, 'pendingCommand' | 'runStatus'>) {
  return hasNoPendingCommand(state) && state.runStatus?.activeRunState === 'paused';
}

export function canStepRun(state: Pick<AppState, 'pendingCommand' | 'runStatus'>) {
  return hasNoPendingCommand(state) && state.runStatus?.activeRunState === 'paused';
}

export function canStopRun(state: Pick<AppState, 'pendingCommand' | 'runStatus'>) {
  return (
    hasNoPendingCommand(state) &&
    (state.runStatus?.activeRunState === 'running' || state.runStatus?.activeRunState === 'paused')
  );
}
