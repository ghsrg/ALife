import { useRef } from 'react';
import { getMonitorDataState, type AppStore } from '../app/appState';
import { buildMonitorViewModel } from '../app/monitorViewModel';
import type { CellId } from '../projection/types';
import { BottomStatsStrip } from './BottomStatsStrip';
import { CellInspector } from './CellInspector';
import { LayerPanel } from './LayerPanel';
import { SelectedEntityFocusCard } from './SelectedEntityFocusCard';
import { WorldViewer, type WorldViewerHandle } from './WorldViewer';
import type { MonitorStat } from './monitorStats';

interface MonitorWorkspaceProps {
  state: AppStore;
  monitorStats: MonitorStat[];
  onScenarioChange: (scenarioId: string) => void;
  onReconnect: () => void;
  onSelectCell: (cellId: CellId | null) => void;
  onToggleTheme: () => void;
  onExportScreenshot: (png: string | null) => void;
  exportStatus: string | null;
}

export function MonitorWorkspace({
  state,
  monitorStats,
  onScenarioChange,
  onReconnect,
  onSelectCell,
  onToggleTheme,
  onExportScreenshot,
  exportStatus
}: MonitorWorkspaceProps) {
  const viewerRef = useRef<WorldViewerHandle | null>(null);
  const monitorDataState = getMonitorDataState(state);
  const monitorViewModel = buildMonitorViewModel(state);

  const exportScreenshot = () => {
    onExportScreenshot(viewerRef.current?.exportPng() ?? null);
  };

  return (
    <main className="monitor-grid">
      <LayerPanel
        state={state}
        monitorDataState={monitorDataState}
        monitorViewModel={monitorViewModel}
        onScenarioChange={onScenarioChange}
        onReconnect={onReconnect}
      />
      <section className="viewer-panel" aria-label="Monitor workspace">
        <div className="viewer-toolbar">
          <div>
            <strong>{monitorViewModel.scenarioTitle}</strong>
            <span>{monitorViewModel.subtitle}</span>
          </div>
          <button type="button" onClick={onToggleTheme} aria-label={`Switch to ${state.theme === 'dark' ? 'light' : 'dark'} theme`}>
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
          onSelectCell={onSelectCell}
        />
        <SelectedEntityFocusCard selectedCell={state.selectedCell} />
        <BottomStatsStrip stats={monitorStats} />
        {exportStatus ? <p className="export-status" role="status">{exportStatus}</p> : null}
      </section>
      <CellInspector selectedCell={state.selectedCell} />
    </main>
  );
}
