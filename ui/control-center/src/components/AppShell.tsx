import { useEffect, useMemo, useRef, useState } from 'react';
import { createAppStore, getMonitorDataState, type AppStore } from '../app/appState';
import { buildMonitorViewModel, type MonitorViewModel } from '../app/monitorViewModel';
import { createRunnerController } from '../app/runnerController';
import type { CellProjection } from '../projection/types';
import { BottomStatsStrip } from './BottomStatsStrip';
import { ConnectionPanel } from './ConnectionPanel';
import { RunControls } from './RunControls';
import { SelectedEntityFocusCard } from './SelectedEntityFocusCard';
import { WorldViewer, type WorldViewerHandle } from './WorldViewer';
import { buildMonitorStats } from './monitorStats';

export function AppShell() {
  const store = useMemo(() => createAppStore(), []);
  const controller = useMemo(() => createRunnerController({ store }), [store]);
  const viewerRef = useRef<WorldViewerHandle | null>(null);
  const [state, setState] = useState(store.getState());
  const [exportStatus, setExportStatus] = useState<string | null>(null);

  useEffect(() => store.subscribe(setState), [store]);

  useEffect(() => {
    controller.connectRunner();
    return () => {
      controller.disconnect();
    };
  }, [controller]);

  useEffect(() => {
    document.documentElement.dataset.theme = state.theme;
  }, [state.theme]);

  const toggleTheme = () => {
    store.getState().setTheme(state.theme === 'dark' ? 'light' : 'dark');
  };

  const monitorDataState = getMonitorDataState(state);
  const monitorStats = buildMonitorStats(state.frame, monitorDataState);
  const monitorViewModel = buildMonitorViewModel(state);

  const exportScreenshot = () => {
    const png = viewerRef.current?.exportPng();
    setExportStatus(png ? `PNG ready (${png.length} bytes)` : 'PNG export unavailable');
  };

  return (
    <div className="app-shell">
      <header className="top-bar" data-testid="monitor-top-context">
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
          onStart={controller.startRun}
          onPause={controller.pauseRun}
          onResume={controller.resumeRun}
          onStep={controller.stepRun}
          onStop={controller.stopRun}
        />
      </header>

      <main className="monitor-grid">
        <LayerPanel
          state={state}
          monitorDataState={monitorDataState}
          monitorViewModel={monitorViewModel}
          onScenarioChange={(scenarioId) => store.getState().setSelectedScenarioId(scenarioId)}
          onReconnect={controller.connectRunner}
        />
        <section className="viewer-panel" aria-label="Monitor workspace">
          <div className="viewer-toolbar">
            <div>
              <strong>{monitorViewModel.scenarioTitle}</strong>
              <span>{monitorViewModel.subtitle}</span>
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
          <SelectedEntityFocusCard selectedCell={state.selectedCell} />
          <BottomStatsStrip stats={monitorStats} />
          {exportStatus ? <p className="export-status" role="status">{exportStatus}</p> : null}
        </section>
        <Inspector selectedCell={state.selectedCell} />
      </main>
    </div>
  );
}

function LayerPanel({
  state,
  monitorDataState,
  monitorViewModel,
  onScenarioChange,
  onReconnect
}: {
  state: AppStore;
  monitorDataState: ReturnType<typeof getMonitorDataState>;
  monitorViewModel: MonitorViewModel;
  onScenarioChange: (scenarioId: string) => void;
  onReconnect: () => void;
}) {
  const frame = state.frame;

  return (
    <aside className="side-panel" aria-label="Layer controls">
      <h2>Layers</h2>
      <ConnectionPanel
        endpoint={state.runnerEndpoint}
        connectionState={state.connectionState}
        monitorDataState={monitorDataState}
        serverInfo={state.serverInfo}
        scenarios={state.scenarios}
        selectedScenarioId={state.selectedScenarioId}
        lastError={state.lastError}
        onScenarioChange={onScenarioChange}
        onReconnect={onReconnect}
      />
      <label className="layer-option">
        <input type="checkbox" checked readOnly />
        <span>Cells</span>
      </label>
      <label className={monitorViewModel.hasResourceLayer ? 'layer-option' : 'layer-option layer-option-missing'}>
        <input type="checkbox" checked={monitorViewModel.hasResourceLayer} readOnly disabled={!monitorViewModel.hasResourceLayer} />
        <span>
          Composite Resource Concentration
          <small>{monitorViewModel.resourceLayerState}</small>
        </span>
      </label>
      <label className="layer-option muted">
        <input type="checkbox" disabled />
        <span>Joints</span>
      </label>
      <div className="metric-list">
        <div><span>Run</span><strong>{frame.runId}</strong></div>
        <div><span>Cells</span><strong>{frame.cells.length}</strong></div>
        <div><span>World</span><strong>{frame.world.width} x {frame.world.height}</strong></div>
        <div><span>Projection</span><strong>{monitorViewModel.projectionLabel}</strong></div>
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

