import { createStore } from 'zustand/vanilla';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { loadFixtureFrame, selectCell } from '../projection/fixtureAdapter';
import {
  buildProjectionContext,
  buildUnavailableProjectionContext,
  type ProjectionContext
} from '../projection/projectionContext';
import type { CellId, CellProjection, DebugProjectionState, ResourceConcentration, WorldFrame } from '../projection/types';
import type { RunStatus, ScenarioListItem, ServerInfo } from '../runner/apiClient';
import type { RunnerStreamConnectionState as ConnectionState } from '../runner/streamClient';
import { appendRrdSample, type RrdMetricHistory } from './rrdMetricHistory';
import type { AccountingTarget } from './monitorSurfaceModel';
import {
  createCellSelection,
  createNoneSelection,
  createSelectionSet,
  type MonitorSelection
} from './selectionModel';

export type { RunStatus, ScenarioListItem, ServerInfo };
export type { ConnectionState };

export type ThemeMode = 'dark' | 'light';
export type PendingCommand = 'connect' | 'start' | 'pause' | 'resume' | 'step' | 'stop';
export type MonitorDataState =
  | 'fixture-offline'
  | 'fixture-idle'
  | 'live-waiting'
  | 'live'
  | 'stale-live';

export interface VisualEffectsConfig {
  showNebula: boolean;
  showParticles: boolean;
  showFilaments: boolean;
  showPhenotypeTraits: boolean;
  showDivisionFlash: boolean;
  showOrganelles: boolean;
  showOrganismHulls: boolean;
  showJointPulses: boolean;
}

export const DEFAULT_VISUAL_EFFECTS: VisualEffectsConfig = {
  showNebula: true,
  showParticles: true,
  showFilaments: true,
  showPhenotypeTraits: true,
  showDivisionFlash: true,
  showOrganelles: true,
  showOrganismHulls: true,
  showJointPulses: true,
};

export interface AppState {
  frame: WorldFrame;
  latestLiveFrame: WorldFrame | null;
  frameHistory: WorldFrame[];
  projectionContext: ProjectionContext;
  debugProjections: DebugProjectionState;
  selectedCellId: CellId | null;
  selectedCell: CellProjection | null;
  currentSelection: MonitorSelection;
  selectionNotice: string | null;
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
  activeResourceLayers: number[];
  visualEffects: VisualEffectsConfig;
  monitorMetricHistory: Record<string, RrdMetricHistory>;
  monitorAccountingTarget: AccountingTarget;
}

export interface AppActions {
  setFrame: (frame: WorldFrame) => void;
  freezeCurrentFrame: () => void;
  selectHistoryTick: (tick: number) => void;
  jumpToLive: () => void;
  setDebugProjections: (debugProjections: DebugProjectionState) => void;
  selectCell: (cellId: CellId | null) => void;
  selectMonitorTarget: (selection: MonitorSelection) => void;
  clearSelection: (notice?: string | null) => void;
  toggleResourceLayer: (layerIndex: number) => void;
  toggleVisualEffect: (key: keyof VisualEffectsConfig) => void;
  setMonitorAccountingTarget: (target: AccountingTarget) => void;
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

function resolveCellSelectionForFrame(
  frame: WorldFrame,
  currentCellId: CellId | null,
  currentSelection: MonitorSelection,
  selectionCleared: boolean
) {
  if (selectionCleared) {
    return {
      selectedCell: null,
      currentSelection: createNoneSelection(),
      selectionNotice: null
    };
  }

  if (currentSelection.kind === 'cell') {
    const selectedCell = selectCell(frame, currentSelection.cellId);
    if (selectedCell !== null) {
      return {
        selectedCell,
        currentSelection: createCellSelection({
          cellId: selectedCell.id,
          runId: frame.runId,
          tick: frame.tick
        }),
        selectionNotice: null
      };
    }

    return {
      selectedCell: null,
      currentSelection: createNoneSelection(),
      selectionNotice: `Selection target ${currentSelection.cellId} is unavailable`
    };
  }




  if (currentSelection.kind === 'selection-set' && currentSelection.targetKind === 'cell') {
    const liveTargets = currentSelection.targets
      .filter((target) => target.kind === 'cell')
      .map((target) => selectCell(frame, target.cellId))
      .filter((cell): cell is CellProjection => cell !== null)
      .map((cell) => createCellSelection({ cellId: cell.id, runId: frame.runId, tick: frame.tick }));

    if (liveTargets.length === 0) {
      return {
        selectedCell: null,
        currentSelection: createNoneSelection(),
        selectionNotice: 'Selection targets are unavailable'
      };
    }

    if (liveTargets.length === 1) {
      const selectedCell = selectCell(frame, liveTargets[0].cellId);
      return {
        selectedCell,
        currentSelection: liveTargets[0],
        selectionNotice: null
      };
    }

    return {
      selectedCell: null,
      currentSelection: createSelectionSet({ targets: liveTargets, runId: frame.runId, tick: frame.tick }),
      selectionNotice: null
    };
  }

  if (currentSelection.kind === 'world-block') {
    return {
      selectedCell: null,
      currentSelection: {
        ...currentSelection,
        runId: frame.runId,
        tick: frame.tick
      },
      selectionNotice: null
    };
  }

  if (currentCellId !== null) {
    const selectedCell = selectCell(frame, currentCellId);
    if (selectedCell !== null) {
      return {
        selectedCell,
        currentSelection: createCellSelection({
          cellId: selectedCell.id,
          runId: frame.runId,
          tick: frame.tick
        }),
        selectionNotice: null
      };
    }
  }

  return {
    selectedCell: null,
    currentSelection: createNoneSelection(),
    selectionNotice: null
  };


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
    currentSelection: initialCell
      ? createCellSelection({
          cellId: initialCell.id,
          runId: initialFrame.runId,
          tick: initialFrame.tick
        })
      : createNoneSelection(),
    selectionNotice: null,
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
    activeResourceLayers: [0, 1],
    visualEffects: DEFAULT_VISUAL_EFFECTS,
    monitorMetricHistory: {},
    monitorAccountingTarget: 'Energy',
    toggleResourceLayer: (layerIndex) => {
      const current = get().activeResourceLayers;
      const next = current.includes(layerIndex)
        ? current.filter((idx) => idx !== layerIndex)
        : [...current, layerIndex];
      set({ activeResourceLayers: next });
    },
    toggleVisualEffect: (key) =>
      set((state) => ({
        visualEffects: {
          ...state.visualEffects,
          [key]: !state.visualEffects[key]
        }
      })),
    setMonitorAccountingTarget: (monitorAccountingTarget) => set({ monitorAccountingTarget }),
    setFrame: (frame) => {
      const state = get();
      const isLiveFrame = frame.source === 'live';
      const visibleFrame = enrichFrameWithDebugProjection(frame, state.debugProjections);
      const frameHistory = isLiveFrame
        ? appendFrameHistory(state.frameHistory, visibleFrame)
        : state.frameHistory;
      const monitorMetricHistory = appendMonitorMetricHistory(
        state.monitorMetricHistory,
        visibleFrame
      );

      if (state.projectionContext.mode === 'frozen' && isLiveFrame) {
        set({
          latestLiveFrame: visibleFrame,
          frameHistory,
          monitorMetricHistory
        });
        return;
      }

      const selectionState = resolveCellSelectionForFrame(
        visibleFrame,
        state.selectedCellId,
        state.currentSelection,
        state.selectionCleared
      );
      set({
        frame: visibleFrame,
        latestLiveFrame: isLiveFrame ? visibleFrame : state.latestLiveFrame,
        frameHistory,
        monitorMetricHistory,
        projectionContext: buildProjectionContext(visibleFrame, isLiveFrame ? 'live' : 'fixture'),
        selectedCellId: selectionState.selectedCell?.id ?? null,
        selectedCell: selectionState.selectedCell,
        currentSelection: selectionState.currentSelection,
        selectionNotice: selectionState.selectionNotice
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

      const selectionState = resolveCellSelectionForFrame(
        historicalFrame,
        state.selectedCellId,
        state.currentSelection,
        state.selectionCleared
      );
      set({
        frame: historicalFrame,
        projectionContext: buildProjectionContext(historicalFrame, 'frozen'),
        selectedCellId: selectionState.selectedCell?.id ?? null,
        selectedCell: selectionState.selectedCell,
        currentSelection: selectionState.currentSelection,
        selectionNotice: selectionState.selectionNotice
      });
    },
    jumpToLive: () => {
      const state = get();
      if (state.latestLiveFrame === null) {
        return;
      }

      const selectionState = resolveCellSelectionForFrame(
        state.latestLiveFrame,
        state.selectedCellId,
        state.currentSelection,
        state.selectionCleared
      );
      set({
        frame: state.latestLiveFrame,
        projectionContext: buildProjectionContext(state.latestLiveFrame, 'live'),
        selectedCellId: selectionState.selectedCell?.id ?? null,
        selectedCell: selectionState.selectedCell,
        currentSelection: selectionState.currentSelection,
        selectionNotice: selectionState.selectionNotice
      });
    },
    setDebugProjections: (debugProjections) => {
      const state = get();
      const enrichedFrame = enrichFrameWithDebugProjection(state.frame, debugProjections);
      const enrichedLatestLiveFrame =
        state.latestLiveFrame === null
          ? null
          : enrichFrameWithDebugProjection(state.latestLiveFrame, debugProjections);
      const monitorMetricHistory = appendMonitorProjectionMetricHistory(
        state.monitorMetricHistory,
        debugProjections
      );
      const selectionState = resolveCellSelectionForFrame(
        enrichedFrame,
        state.selectedCellId,
        state.currentSelection,
        state.selectionCleared
      );
      set({
        debugProjections,
        frame: enrichedFrame,
        latestLiveFrame: enrichedLatestLiveFrame,
        monitorMetricHistory,
        selectedCellId: selectionState.selectedCell?.id ?? null,
        selectedCell: selectionState.selectedCell,
        currentSelection: selectionState.currentSelection,
        selectionNotice: selectionState.selectionNotice
      });
    },
    selectCell: (cellId) => {
      const selectedCell = selectCell(get().frame, cellId);
      const frame = get().frame;
      set({
        selectedCellId: selectedCell?.id ?? null,
        selectedCell,
        currentSelection: selectedCell
          ? createCellSelection({ cellId: selectedCell.id, runId: frame.runId, tick: frame.tick })
          : createNoneSelection(),
        selectionNotice: cellId !== null && selectedCell === null ? `Selection target ${cellId} is unavailable` : null,
        selectionCleared: cellId === null
      });
    },
    selectMonitorTarget: (selection) => {
      if (selection.kind === 'cell') {
        get().selectCell(selection.cellId);
        return;
      }

      set({
        selectedCellId: null,
        selectedCell: null,
        currentSelection: selection,
        selectionNotice: null,
        selectionCleared: selection.kind === 'none'
      });
    },
    clearSelection: (notice = null) => {
      set({
        selectedCellId: null,
        selectedCell: null,
        currentSelection: createNoneSelection(),
        selectionNotice: notice,
        selectionCleared: true
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

function enrichFrameWithDebugProjection(frame: WorldFrame, debugProjections: DebugProjectionState): WorldFrame {
  if (debugProjections.status !== 'available') {
    return frame;
  }
  if (frame.source !== 'live' || frame.runId !== debugProjections.runId) {
    return frame;
  }

  const resources =
    debugProjections.visualWorld.resourceLayers.length > 0
      ? resourceLayersToGrid(debugProjections.visualWorld.resourceLayers)
      : frame.resources;
  const debugCells = new Map(debugProjections.visualWorld.cells.map((cell) => [cell.id, cell]));
  const cells = frame.cells.map((cell) => {
    const debugCell = debugCells.get(cell.id);
    if (!debugCell) {
      return cell;
    }
    const energyCapacity = Math.max(0, debugCell.energyCapacity);
    const energyRatio = energyCapacity > 0 ? debugCell.energy / energyCapacity : cell.energy;
    return {
      ...cell,
      energy: Math.max(0, Math.min(1, energyRatio)),
      energyRaw: debugCell.energy,
      energyCapacity,
      materials: debugCell.materials,
      internalResources: debugCell.internalResources,
      localExternalResources: debugCell.localExternalResources
    };
  });

  return {
    ...frame,
    resources,
    cells
  };
}

function detectStride(cells: { x: number; y: number }[]): number {
  if (cells.length < 2) return 1;
  let minDx = Infinity;
  const firstY = cells[0].y;
  for (let i = 1; i < cells.length; i++) {
    if (cells[i].y === firstY) {
      const dx = cells[i].x - cells[i - 1].x;
      if (dx > 0 && dx < minDx) minDx = dx;
    }
  }
  return minDx === Infinity ? 1 : minDx;
}

function resourceLayersToGrid(
  layers: Extract<DebugProjectionState, { status: 'available' }>['visualWorld']['resourceLayers']
) {
  const width = Math.max(...layers.map((layer) => layer.width));
  const height = Math.max(...layers.map((layer) => layer.height));
  const rows: ResourceConcentration[][] = Array.from({ length: height }, () =>
    Array.from({ length: width }, () => ({ organic: 0, mineral: 0, energy: 0, layers: {} }))
  );

  layers.forEach((layer) => {
    const stride = detectStride(layer.cells);
    for (const cell of layer.cells) {
      for (let dy = 0; dy < stride; dy++) {
        const ry = cell.y + dy;
        if (ry >= rows.length) continue;
        for (let dx = 0; dx < stride; dx++) {
          const rx = cell.x + dx;
          if (rx >= (rows[ry]?.length ?? 0)) continue;

          const current = rows[ry][rx];
          if (!current.layers) current.layers = {};
          current.layers[layer.layerIndex] = cell.amount;

          const channel = layer.layerIndex % 3;
          if (channel === 0) {
            current.organic = cell.amount;
          } else if (channel === 1) {
            current.mineral = cell.amount;
          } else {
            current.energy = cell.amount;
          }
        }
      }
    }
  });

  return rows;
}

function appendFrameHistory(history: WorldFrame[], frame: WorldFrame) {
  const nextHistory = [...history.filter((historyFrame) => historyFrame.tick !== frame.tick), frame];
  return nextHistory.slice(-FRAME_HISTORY_LIMIT);
}

function appendMonitorMetricHistory(
  history: Record<string, RrdMetricHistory>,
  frame: WorldFrame
): Record<string, RrdMetricHistory> {
  return {
    ...history,
    visibleCellCount: appendRrdSample(history.visibleCellCount ?? [], {
      tick: frame.tick,
      value: frame.cells.length
    })
  };
}

function appendMonitorProjectionMetricHistory(
  history: Record<string, RrdMetricHistory>,
  debugProjections: DebugProjectionState
): Record<string, RrdMetricHistory> {
  if (debugProjections.status !== 'available') {
    return history;
  }

  const resourceCycle = debugProjections.monitor?.payload.world.resourceCycle;
  if (!resourceCycle || resourceCycle.state !== 'available') {
    return history;
  }

  const tick = debugProjections.monitor?.tick ?? debugProjections.tick;
  const samples = {
    'world.resource.environment': resourceCycle.locations.environment,
    'world.resource.cells': resourceCycle.locations.cells,
    'world.resource.materials': resourceCycle.locations.materials,
    'world.resource.fragments': resourceCycle.locations.fragments,
    'world.resource.explicitSinks': resourceCycle.locations.explicitSinks,
    'world.resource.explicitDecayOrSink': resourceCycle.accounting.explicitDecayOrSink,
    'world.resource.metabolismOrCellUptake': resourceCycle.accounting.metabolismOrCellUptake,
    'world.resource.materialConversion': resourceCycle.accounting.materialConversion,
    'world.resource.unclassifiedLoss': resourceCycle.accounting.unclassifiedLoss
  };

  return Object.entries(samples).reduce<Record<string, RrdMetricHistory>>(
    (nextHistory, [metric, value]) => ({
      ...nextHistory,
      [metric]: appendRrdSample(nextHistory[metric] ?? [], { tick, value })
    }),
    { ...history }
  );
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
