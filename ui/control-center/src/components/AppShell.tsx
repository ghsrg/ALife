import { useEffect, useMemo, useRef, useState } from 'react';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { createAppStore } from '../app/appState';
import type { CellProjection, WorldFrame } from '../projection/types';
import { WorldViewer, type WorldViewerHandle } from './WorldViewer';

export function AppShell() {
  const store = useMemo(() => createAppStore(), []);
  const viewerRef = useRef<WorldViewerHandle | null>(null);
  const [state, setState] = useState(store.getState());
  const [exportStatus, setExportStatus] = useState<string | null>(null);

  useEffect(() => store.subscribe(setState), [store]);

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
        <div className="run-controls" aria-label="Fixture run controls">
          <button className="icon-button primary" type="button" aria-label="Play fixture run">Play</button>
          <button className="icon-button" type="button" aria-label="Step fixture run" disabled>Step N</button>
          <button className="icon-button" type="button" aria-label="Pause fixture run">Pause</button>
        </div>
      </header>

      <main className="monitor-grid">
        <LayerPanel frame={state.frame} />
        <section className="viewer-panel" aria-label="Monitor workspace">
          <div className="viewer-toolbar">
            <div>
              <strong>{ui1aFixture.scenarioName}</strong>
              <span>Tick {state.frame.tick}</span>
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

function LayerPanel({ frame }: { frame: WorldFrame }) {
  return (
    <aside className="side-panel" aria-label="Layer controls">
      <h2>Layers</h2>
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
        <div><span>Projection</span><strong>fixture/v1</strong></div>
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
