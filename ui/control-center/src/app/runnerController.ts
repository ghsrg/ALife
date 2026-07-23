import type { StoreApi } from 'zustand/vanilla';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import type { DebugProjectionState } from '../projection/types';
import { liveProjectionToWorldFrame } from '../projection/liveAdapter';
import type { LiveWorldFrameProjection } from '../runner/alifDecoder';
import { RunnerApiClient, type RunStatus } from '../runner/apiClient';
import { RunnerStreamClient, type RunnerStreamHandlers } from '../runner/streamClient';
import type { AppStore, PendingCommand } from './appState';

export function createRequestId(now: () => number = Date.now) {
  return `ui-${now()}`;
}

export function shouldApplyRunStatus(runStatus: RunStatus | null, state: Pick<AppStore, 'runStatus'>) {
  if (runStatus === null || state.runStatus === null) {
    return true;
  }

  return !(
    runStatus.runId !== null &&
    runStatus.runId === state.runStatus.runId &&
    runStatus.committedTick < state.runStatus.committedTick
  );
}

export function shouldApplyLiveFrame(
  frame: LiveWorldFrameProjection,
  state: Pick<AppStore, 'frame' | 'runStatus'>
) {
  if (state.frame.source !== 'live') {
    return true;
  }

  const activeRunId = state.runStatus?.runId ?? state.frame.runId;
  if (state.frame.runId !== activeRunId) {
    return true;
  }

  if (frame.committedTick < state.frame.tick) {
    return false;
  }

  const currentSequence = state.frame.summary?.projectionSequence;
  return !(
    frame.committedTick === state.frame.tick &&
    currentSequence !== undefined &&
    frame.projectionSequence <= currentSequence
  );
}

export function toWorldFrame(
  frame: LiveWorldFrameProjection,
  state: Pick<AppStore, 'runStatus' | 'selectedScenarioId' | 'frame'>
) {
  return liveProjectionToWorldFrame(frame, {
    runId: state.runStatus?.runId ?? state.frame.runId,
    scenarioName:
      state.runStatus?.scenarioId ??
      state.selectedScenarioId ??
      state.frame.scenarioName ??
      ui1aFixture.scenarioName
  });
}

export function toErrorMessage(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}

export function shouldApplyDebugProjections(
  debugProjections: DebugProjectionState,
  requestedFrameTick: number,
  state: Pick<AppStore, 'frame'>
) {
  if (state.frame.source !== 'live') {
    return true;
  }

  if (debugProjections.status === 'unavailable') {
    return state.frame.tick === requestedFrameTick;
  }

  if (debugProjections.status === 'loading' || debugProjections.status === 'stale') {
    return state.frame.tick === requestedFrameTick;
  }

  return debugProjections.tick >= state.frame.tick;
}

interface RunnerApiPort {
  getServerInfo: RunnerApiClient['getServerInfo'];
  listScenarios: RunnerApiClient['listScenarios'];
  getRunStatus: RunnerApiClient['getRunStatus'];
  getLatestDebugProjections: RunnerApiClient['getLatestDebugProjections'];
  startRun: RunnerApiClient['startRun'];
  pauseRun: RunnerApiClient['pauseRun'];
  resumeRun: RunnerApiClient['resumeRun'];
  stepRun: RunnerApiClient['stepRun'];
  stopRun: RunnerApiClient['stopRun'];
}

interface RunnerStreamPort {
  connect: () => void;
  disconnect: () => void;
}

export interface RunnerControllerDependencies {
  store: StoreApi<AppStore>;
  createApiClient?: (endpoint: string) => RunnerApiPort;
  createStreamClient?: (endpoint: string, handlers: RunnerStreamHandlers) => RunnerStreamPort;
  now?: () => number;
}

export interface RunnerController {
  connectRunner: () => void;
  disconnect: () => void;
  startRun: () => void;
  pauseRun: () => void;
  resumeRun: () => void;
  stepRun: () => void;
  stopRun: () => void;
}

export function createRunnerController({
  store,
  createApiClient = (endpoint) => new RunnerApiClient(endpoint),
  createStreamClient = (endpoint, handlers) => new RunnerStreamClient(endpoint, handlers),
  now = Date.now
}: RunnerControllerDependencies): RunnerController {
  let apiClient: RunnerApiPort | null = null;
  let streamClient: RunnerStreamPort | null = null;
  let commandSequence = 0;

  const connectRunner = () => {
    const endpoint = store.getState().runnerEndpoint;
    const nextApiClient = createApiClient(endpoint);
    let nextStreamClient: RunnerStreamPort;
    const isActive = () => streamClient === nextStreamClient;

    nextStreamClient = createStreamClient(endpoint, {
      onConnectionState: (connectionState) => {
        if (!isActive()) {
          return;
        }
        store.getState().setConnectionState(connectionState);
      },
      onStatus: (runStatus) => {
        if (!isActive() || !shouldApplyRunStatus(runStatus, store.getState())) {
          return;
        }
        store.getState().setRunStatus(runStatus);
      },
      onFrame: (frame) => {
        if (!isActive()) {
          return;
        }
        const currentState = store.getState();
        if (!shouldApplyLiveFrame(frame, currentState)) {
          return;
        }
        const requestedFrameTick = frame.committedTick;
        const liveWorldFrame = toWorldFrame(frame, currentState);
        currentState.setFrame(liveWorldFrame);
        if (shouldSetDebugProjectionLoading(liveWorldFrame.runId, store.getState())) {
          store.getState().setDebugProjections({
            status: 'loading',
            runId: liveWorldFrame.runId,
            requestedTick: requestedFrameTick,
            reason: 'Waiting for Observer debug projection'
          });
        }
        void nextApiClient
          .getLatestDebugProjections()
          .then((debugProjections) => {
            if (!isActive()) {
              return;
            }
            const state = store.getState();
            if (debugProjections.status === 'available' && debugProjections.tick < state.frame.tick) {
              if (
                state.frame.tick === requestedFrameTick &&
                shouldApplyLaggedAvailableDebugProjection(debugProjections, state)
              ) {
                store.getState().setDebugProjections(debugProjections);
              }
              return;
            }
            if (shouldApplyDebugProjections(debugProjections, requestedFrameTick, state)) {
              store.getState().setDebugProjections(debugProjections);
            }
          })
          .catch((error: unknown) => {
            const unavailableDebugProjections: DebugProjectionState = {
              status: 'unavailable',
              reason: toErrorMessage(error)
            };
            if (
              isActive() &&
              shouldApplyDebugProjections(unavailableDebugProjections, requestedFrameTick, store.getState())
            ) {
              store.getState().setDebugProjections(unavailableDebugProjections);
            }
          });
      },
      onError: (error) => {
        if (!isActive()) {
          return;
        }
        store.getState().setError(error.message);
      }
    });

    streamClient?.disconnect();
    apiClient = nextApiClient;
    streamClient = nextStreamClient;
    store.getState().setPendingCommand('connect');

    Promise.all([
      nextApiClient.getServerInfo(),
      nextApiClient.listScenarios(),
      nextApiClient.getRunStatus()
    ])
      .then(([serverInfo, scenarios, runStatus]) => {
        if (!isActive()) {
          return;
        }
        const actions = store.getState();
        actions.setConnected(serverInfo);
        actions.setScenarios(scenarios);
        actions.setRunStatus(runStatus);
        actions.clearPendingCommand();
        nextStreamClient.connect();
      })
      .catch((error: unknown) => {
        if (!isActive()) {
          return;
        }
        const actions = store.getState();
        actions.setError(toErrorMessage(error));
        actions.setConnectionState('disconnected');
        actions.clearPendingCommand();
      });
  };

  const runCommand = async (
    pendingCommand: Exclude<PendingCommand, 'connect'>,
    command: (client: RunnerApiPort) => Promise<unknown>
  ) => {
    if (apiClient === null) {
      store.getState().setError('Runner API client is not connected');
      return;
    }

    if (store.getState().pendingCommand !== null) {
      return;
    }

    const currentCommandSequence = commandSequence + 1;
    commandSequence = currentCommandSequence;
    store.getState().setPendingCommand(pendingCommand);
    try {
      await command(apiClient);
      const runStatus = await apiClient.getRunStatus();
      if (commandSequence === currentCommandSequence) {
        store.getState().setRunStatus(runStatus);
      }
    } catch (error) {
      if (commandSequence === currentCommandSequence) {
        store.getState().setError(toErrorMessage(error));
      }
    } finally {
      if (commandSequence === currentCommandSequence) {
        store.getState().clearPendingCommand();
      }
    }
  };

  return {
    connectRunner,
    disconnect: () => {
      streamClient?.disconnect();
      streamClient = null;
      apiClient = null;
    },
    startRun: () => {
      const selectedScenarioId = store.getState().selectedScenarioId;
      if (selectedScenarioId === null) {
        store.getState().setError('No scenario selected');
        return;
      }

      void runCommand('start', (client) =>
        client.startRun({
          scenarioId: selectedScenarioId,
          requestId: createRequestId(now)
        })
      );
    },
    pauseRun: () => {
      void runCommand('pause', (client) => client.pauseRun());
    },
    resumeRun: () => {
      void runCommand('resume', (client) => client.resumeRun());
    },
    stepRun: () => {
      void runCommand('step', (client) => client.stepRun());
    },
    stopRun: () => {
      void runCommand('stop', (client) => client.stopRun());
    }
  };
}

export function shouldSetDebugProjectionLoading(
  runId: string,
  state: Pick<AppStore, 'debugProjections'>
) {
  return !(state.debugProjections.status === 'available' && state.debugProjections.runId === runId);
}

function shouldApplyLaggedAvailableDebugProjection(
  debugProjections: Extract<DebugProjectionState, { status: 'available' }>,
  state: Pick<AppStore, 'debugProjections'>
) {
  if (state.debugProjections.status !== 'available') {
    return true;
  }

  if (state.debugProjections.runId !== debugProjections.runId) {
    return true;
  }

  return debugProjections.tick >= state.debugProjections.tick;
}
