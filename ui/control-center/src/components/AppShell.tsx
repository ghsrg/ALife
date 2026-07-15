import { useEffect, useMemo, useRef, useState } from 'react';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { createAppStore, type AppStore } from '../app/appState';
import type { CellProjection, WorldFrame } from '../projection/types';
import { liveProjectionToWorldFrame } from '../projection/liveAdapter';
import { RunnerApiClient } from '../runner/apiClient';
import { RunnerStreamClient } from '../runner/streamClient';
import type { LiveWorldFrameProjection } from '../runner/alifDecoder';
import { ConnectionPanel } from './ConnectionPanel';
import { RunControls } from './RunControls';
import { WorldViewer, type WorldViewerHandle } from './WorldViewer';

export function AppShell() {
  const store = useMemo(() => createAppStore(), []);
  const viewerRef = useRef<WorldViewerHandle | null>(null);
  const apiClientRef = useRef<RunnerApiClient | null>(null);
  const streamClientRef = useRef<RunnerStreamClient | null>(null);
  const [state, setState] = useState(store.getState());
  const [exportStatus, setExportStatus] = useState<string | null>(null);

  useEffect(() => store.subscribe(setState), [store]);

  useEffect(() => {
    let cancelled = false;
    const endpoint = store.getState().runnerEndpoint;
    const apiClient = new RunnerApiClient(endpoint);
    const streamClient = new RunnerStreamClient(endpoint, {
      onConnectionState: (connectionState) => {
        store.getState().setConnectionState(connectionState);
      },
      onStatus: (runStatus) => {
        store.getState().setRunStatus(runStatus);
      },
      onFrame: (frame) => {
        store.getState().setFrame(toWorldFrame(frame, store.getState()));
      },
      onError: (error) => {
        store.getState().setError(error.message);
      }
    });

    apiClientRef.current = apiClient;
    streamClientRef.current = streamClient;
    store.getState().setPendingCommand('connect');

    Promise.all([
      apiClient.getServerInfo(),
      apiClient.listScenarios(),
      apiClient.getRunStatus()
    ])
      .then(([serverInfo, scenarios, runStatus]) => {
        if (cancelled) {
          return;
        }
        const actions = store.getState();
        actions.setConnected(serverInfo);
        actions.setScenarios(scenarios);
        actions.setRunStatus(runStatus);
        actions.clearPendingCommand();
        streamClient.connect();
      })
      .catch((error: unknown) => {
        if (cancelled) {
          return;
        }
        const actions = store.getState();
        actions.setError(toErrorMessage(error));
        actions.setConnectionState('disconnected');
        actions.clearPendingCommand();
      });

    return () => {
      cancelled = true;
      streamClient.disconnect();
      if (streamClientRef.current === streamClient) {
        streamClientRef.current = null;
      }
      if (apiClientRef.current === apiClient) {
        apiClientRef.current = null;
      }
    };
  }, [store]);

  useEffect(() => {
    document.documentElement.dataset.theme = state.theme;
  }, [state.theme]);

  const toggleTheme = () => {
    store.getState().setTheme(state.theme === 'dark' ? 'light' : 'dark');
  };

  const exportScreenshot = () => {
    const png = viewerRef.current?.exportPng();
    setExportStatus(png ? `PNG ready (${png.length} bytes)` : 'PNG export unavailable');
  };

  const runCommand = async (
    pendingCommand: 'start' | 'pause' | 'resume' | 'step' | 'stop',
    command: (client: RunnerApiClient) => Promise<unknown>
  ) => {
    const apiClient = apiClientRef.current;
    if (apiClient === null) {
      store.getState().setError('Runner API client is not connected');
      return;
    }

    const actions = store.getState();
    actions.setPendingCommand(pendingCommand);
    try {
      await command(apiClient);
      actions.setRunStatus(await apiClient.getRunStatus());
    } catch (error) {
      store.getState().setError(toErrorMessage(error));
    } finally {
      store.getState().clearPendingCommand();
    }
  };

  const startRun = () => {
    const selectedScenarioId = store.getState().selectedScenarioId;
    if (selectedScenarioId === null) {
      store.getState().setError('No scenario selected');
      return;
    }

    void runCommand('start', (apiClient) =>
      apiClient.startRun({
        scenarioId: selectedScenarioId,
        requestId: `ui-${Date.now()}`
      })
    );
  };

  const pauseRun = () => {
    void runCommand('pause', (apiClient) => apiClient.pauseRun());
  };

  const resumeRun = () => {
    void runCommand('resume', (apiClient) => apiClient.resumeRun());
  };

  const stepRun = () => {
    void runCommand('step', (apiClient) => apiClient.stepRun());
  };

  const stopRun = () => {
    void runCommand('stop', (apiClient) => apiClient.stopRun());
  };

  return (
    <div className="app-shell">
      <header className="top-bar">
        <div>
          <p className="eyebrow">ALife Control Center</p>
          <h1>ALife Control Center</h1>
        </div>
        <nav className="mode-tabs" aria-label="Primary views">
          <button type="button" role="tab" aria-selected="true">Monitor</button>
          <button type="button" role="tab" aria-selected="false" disabled>OrganismView</button>
          <button type="button" role="tab" aria-selected="false" disabled>World Editor</button>
        </nav>
        <RunControls
          state={state}
          onStart={startRun}
          onPause={pauseRun}
          onResume={resumeRun}
          onStep={stepRun}
          onStop={stopRun}
        />
      </header>

      <main className="monitor-grid">
        <LayerPanel
          state={state}
          onScenarioChange={(scenarioId) => store.getState().setSelectedScenarioId(scenarioId)}
        />
        <section className="viewer-panel" aria-label="Monitor workspace">
          <div className="viewer-toolbar">
            <div>
              <strong>{state.frame.scenarioName ?? ui1aFixture.scenarioName}</strong>
              <span>{state.frame.source === 'live' ? 'Live' : 'Fixture'} Tick {state.frame.tick}</span>
            </div>
            <button type="button" onClick={toggleTheme} aria-label={`Switch to ${state.theme === 'dark' ? 'light' : 'dark'} theme`}>
              {state.theme === 'dark' ? 'Light' : 'Dark'}
            </button>
            <button type="button" onClick={exportScreenshot} aria-label="Export viewer PNG">
              Export PNG
            </button>
          </div>
          <WorldViewer
            ref={viewerRef}
            frame={state.frame}
            selectedCellId={state.selectedCellId}
            onSelectCell={(cellId) => store.getState().selectCell(cellId)}
          />
          {exportStatus ? <p className="export-status" role="status">{exportStatus}</p> : null}
        </section>
        <Inspector selectedCell={state.selectedCell} />
      </main>
    </div>
  );
}

function LayerPanel({
  state,
  onScenarioChange
}: {
  state: AppStore;
  onScenarioChange: (scenarioId: string) => void;
}) {
  const frame = state.frame;

  return (
    <aside className="side-panel" aria-label="Layer controls">
      <h2>Layers</h2>
      <ConnectionPanel
        endpoint={state.runnerEndpoint}
        connectionState={state.connectionState}
        serverInfo={state.serverInfo}
        scenarios={state.scenarios}
        selectedScenarioId={state.selectedScenarioId}
        lastError={state.lastError}
        onScenarioChange={onScenarioChange}
      />
      <label className="layer-option">
        <input type="checkbox" checked readOnly />
        <span>Cells</span>
      </label>
      <label className="layer-option">
        <input type="checkbox" checked readOnly />
        <span>Composite Resource Concentration</span>
      </label>
      <label className="layer-option muted">
        <input type="checkbox" disabled />
        <span>Joints</span>
      </label>
      <div className="metric-list">
        <div><span>Run</span><strong>{frame.runId}</strong></div>
        <div><span>Cells</span><strong>{frame.cells.length}</strong></div>
        <div><span>World</span><strong>{frame.world.width} x {frame.world.height}</strong></div>
        <div><span>Projection</span><strong>{frame.source === 'live' ? 'live/v1' : 'fixture/v1'}</strong></div>
      </div>
    </aside>
  );
}

function Inspector({ selectedCell }: { selectedCell: CellProjection | null }) {
  return (
    <aside className="side-panel inspector" aria-label="Cell Inspector">
      <h2>Cell Inspector</h2>
      {selectedCell ? (
        <div className="metric-list">
          <div><span>ID</span><strong>{selectedCell.id}</strong></div>
          <div><span>Energy</span><strong>{formatRatio(selectedCell.energy)}</strong></div>
          <div><span>Integrity</span><strong>{formatRatio(selectedCell.integrity)}</strong></div>
          <div><span>Generation</span><strong>{selectedCell.generation}</strong></div>
          <div><span>Role hint</span><strong>{selectedCell.roleHint}</strong></div>
        </div>
      ) : (
        <p className="empty-state">No cell selected.</p>
      )}
    </aside>
  );
}

function formatRatio(value: number) {
  return `${Math.round(value * 100)}%`;
}

function toWorldFrame(
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

function toErrorMessage(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}
