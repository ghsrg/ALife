import { createStore } from 'zustand/vanilla';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { loadFixtureFrame, selectCell } from '../projection/fixtureAdapter';
import {
  buildProjectionContext,
  buildUnavailableProjectionContext,
  type ProjectionContext
} from '../projection/projectionContext';
import type { CellId, CellProjection, WorldFrame } from '../projection/types';
import type { DebugProjectionState } from '../projection/types';
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
  latestLiveFrame: WorldFrame | null;
  frameHistory: WorldFrame[];
  projectionContext: ProjectionContext;
  debugProjections: DebugProjectionState;
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
  selectionCleared: boolean;
}

export interface AppActions {
  setFrame: (frame: WorldFrame) => void;
  freezeCurrentFrame: () => void;
  selectHistoryTick: (tick: number) => void;
  jumpToLive: () => void;
  setDebugProjections: (debugProjections: DebugProjectionState) => void;
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

const FRAME_HISTORY_LIMIT = 12;

function selectInitialCell(frame: WorldFrame) {
  return frame.cells[0] ?? null;
}

function selectCellForFrame(frame: WorldFrame, currentCellId: CellId | null, selectionCleared: boolean) {
  if (selectionCleared) {
    return null;
  }

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
  const initialContext = buildProjectionContext(
    initialFrame,
    initialFrame.source === 'live' ? 'live' : 'fixture'
  );

  return createStore<AppStore>((set, get) => ({
    frame: initialFrame,
    latestLiveFrame: initialFrame.source === 'live' ? initialFrame : null,
    frameHistory: initialFrame.source === 'live' ? [initialFrame] : [],
    projectionContext: initialContext,
    debugProjections: {
      status: 'unavailable',
      reason: 'No live Observer debug projection has been loaded'
    },
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
    selectionCleared: false,
    setFrame: (frame) => {
      const state = get();
      const isLiveFrame = frame.source === 'live';
      const frameHistory = isLiveFrame ? appendFrameHistory(state.frameHistory, frame) : state.frameHistory;

      if (state.projectionContext.mode === 'frozen' && isLiveFrame) {
        set({
          latestLiveFrame: frame,
          frameHistory
        });
        return;
      }

      const selectedCell = selectCellForFrame(frame, state.selectedCellId, state.selectionCleared);
      set({
        frame,
        latestLiveFrame: isLiveFrame ? frame : state.latestLiveFrame,
        frameHistory,
        projectionContext: buildProjectionContext(frame, isLiveFrame ? 'live' : 'fixture'),
        selectedCellId: selectedCell?.id ?? null,
        selectedCell
      });
    },
    freezeCurrentFrame: () => {
      const state = get();
      set({
        projectionContext: buildProjectionContext(state.frame, 'frozen')
      });
    },
    selectHistoryTick: (tick) => {
      const state = get();
      const historicalFrame = state.frameHistory.find((frame) => frame.tick === tick) ?? null;

      if (historicalFrame === null) {
        set({
          projectionContext: buildUnavailableProjectionContext({
            runId: state.latestLiveFrame?.runId ?? state.frame.runId,
            tick,
            reason: 'Tick is outside bounded client history'
          })
        });
        return;
      }

      const selectedCell = selectCellForFrame(
        historicalFrame,
        state.selectedCellId,
        state.selectionCleared
      );
      set({
        frame: historicalFrame,
        projectionContext: buildProjectionContext(historicalFrame, 'frozen'),
        selectedCellId: selectedCell?.id ?? null,
        selectedCell
      });
    },
    jumpToLive: () => {
      const state = get();
      if (state.latestLiveFrame === null) {
        return;
      }

      const selectedCell = selectCellForFrame(
        state.latestLiveFrame,
        state.selectedCellId,
        state.selectionCleared
      );
      set({
        frame: state.latestLiveFrame,
        projectionContext: buildProjectionContext(state.latestLiveFrame, 'live'),
        selectedCellId: selectedCell?.id ?? null,
        selectedCell
      });
    },
    setDebugProjections: (debugProjections) => set({ debugProjections }),
    selectCell: (cellId) => {
      const selectedCell = selectCell(get().frame, cellId);
      set({
        selectedCellId: selectedCell?.id ?? null,
        selectedCell,
        selectionCleared: cellId === null
      });
    },
    setTheme: (theme) => set({ theme }),
    setRunnerEndpoint: (runnerEndpoint) => set({ runnerEndpoint }),
    setConnectionState: (connectionState) => {
      const state = get();
      if (
        connectionState === 'disconnected' &&
        state.frame.source === 'live' &&
        state.projectionContext.mode === 'live'
      ) {
        set({
          connectionState,
          projectionContext: buildProjectionContext(state.frame, 'stale')
        });
        return;
      }

      set({ connectionState });
    },
    setConnected: (serverInfo) =>
      set((state) => ({
        connectionState: 'connected',
        serverInfo,
        lastError: null,
        projectionContext:
          state.frame.source === 'live' && state.projectionContext.mode === 'stale'
            ? buildProjectionContext(state.frame, 'live')
            : state.projectionContext
      })),
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

function appendFrameHistory(history: WorldFrame[], frame: WorldFrame) {
  const nextHistory = [...history.filter((historyFrame) => historyFrame.tick !== frame.tick), frame];
  return nextHistory.slice(-FRAME_HISTORY_LIMIT);
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
