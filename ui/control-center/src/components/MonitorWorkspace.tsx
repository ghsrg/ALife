import { useEffect, useRef, useState } from 'react';
import { getMonitorDataState, type AppStore } from '../app/appState';
import { buildBalanceViewModel } from '../app/balanceViewModel';
import { buildMonitorViewModel } from '../app/monitorViewModel';
import { describeProjectionContext } from '../projection/projectionContext';
import type { CellId } from '../projection/types';
import { uiText } from '../uiText';
import { BalanceAnalyticsPanel } from './BalanceAnalyticsPanel';
import { BottomDataPanel } from './BottomDataPanel';
import { BottomStatsStrip } from './BottomStatsStrip';
import { CellInspector } from './CellInspector';
import { LayerPanel } from './LayerPanel';
import { RawDataGridPanel } from './RawDataGridPanel';
import { SelectedEntityFocusCard } from './SelectedEntityFocusCard';
import { WorldViewer, type WorldViewerHandle } from './WorldViewer';
import type { MonitorStat } from './monitorStats';

interface MonitorWorkspaceProps {
  state: AppStore;
  monitorStats: MonitorStat[];
  onScenarioChange: (scenarioId: string) => void;
  onReconnect: () => void;
  onSelectCell: (cellId: CellId | null) => void;
  onToggleTheme?: () => void;
  onExportScreenshot: (png: string | null) => void;
  onFreezeFrame?: () => void;
  onJumpToLive?: () => void;
  onSelectHistoryTick?: (tick: number) => void;
  exportStatus: string | null;
}

export function MonitorWorkspace({
  state,
  monitorStats,
  onScenarioChange,
  onReconnect,
  onSelectCell,
  onExportScreenshot,
  onFreezeFrame,
  onJumpToLive,
  onSelectHistoryTick,
  exportStatus
}: MonitorWorkspaceProps) {
  const viewerRef = useRef<WorldViewerHandle | null>(null);
  const workspaceRef = useRef<HTMLElement | null>(null);
  const [isFullScreen, setIsFullScreen] = useState(false);
  const [activeTab, setActiveTab] = useState<'viewer' | 'analytics' | 'rawdata'>('viewer');
  const monitorDataState = getMonitorDataState(state);
  const monitorViewModel = buildMonitorViewModel(state);

  useEffect(() => {
    const handleFullScreenChange = () => {
      setIsFullScreen(document.fullscreenElement === workspaceRef.current);
    };

    document.addEventListener('fullscreenchange', handleFullScreenChange);
    return () => document.removeEventListener('fullscreenchange', handleFullScreenChange);
  }, []);

  const exportScreenshot = () => {
    onExportScreenshot(viewerRef.current?.exportPng() ?? null);
  };

  const toggleFullScreen = () => {
    if (document.fullscreenElement === workspaceRef.current) {
      void document.exitFullscreen?.();
      return;
    }

    void workspaceRef.current?.requestFullscreen?.();
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
      <section
        ref={workspaceRef}
        className="viewer-panel"
        aria-label={uiText.workspace.monitorWorkspace}
        data-fullscreen={isFullScreen ? 'true' : 'false'}
      >
        <section className="map-context-strip" aria-label={uiText.dataContext.title}>
          <div className="map-context-primary">
            <strong>{monitorViewModel.scenarioTitle}</strong>
            <span className="context-pill">{describeProjectionContext(state.projectionContext)}</span>
            <span className="context-pill">{state.projectionContext.isReadOnly ? 'read-only' : 'live'}</span>
            {state.projectionContext.warning ? (
              <span className="context-pill warning" role="alert">{state.projectionContext.warning}</span>
            ) : null}
          </div>
          <div className="start-demo-provenance compact" aria-label="Start demo provenance" style={{ display: 'none' }}>
            <strong>{uiText.demo.startDemo}</strong>
            <span>
              {monitorViewModel.startDemo.projectionSource === 'live'
                ? uiText.demo.liveProjectionSource
                : uiText.demo.fixtureProjectionSource}
            </span>
            <span>{`${uiText.demo.runnerDataPrefix}: ${monitorViewModel.startDemo.runnerDataLabel}`}</span>
            <span>{monitorViewModel.startDemo.unavailableFieldsLabel}</span>
          </div>
          <div className="data-context-actions">
            <button type="button" onClick={onFreezeFrame} aria-label={uiText.dataContext.freezeFrame}>
              Freeze
            </button>
            <button
              type="button"
              onClick={onJumpToLive}
              aria-label={uiText.dataContext.jumpToLive}
              disabled={state.latestLiveFrame === null}
            >
              Live
            </button>
          </div>
          <div className="history-strip" aria-label={uiText.dataContext.boundedHistory}>
            {[...state.frameHistory, state.frame]
              .filter((frame, index, frames) => frames.findIndex((item) => item.tick === frame.tick) === index)
              .slice(-5)
              .map((frame) => (
                <button
                  type="button"
                  key={`${frame.runId}-${frame.tick}`}
                  onClick={() => onSelectHistoryTick?.(frame.tick)}
                  aria-label={`${uiText.dataContext.inspectTick} ${frame.tick}`}
                >
                  {frame.tick}
                </button>
              ))}
          </div>
        </section>

        <nav className="workspace-tab-nav" aria-label="Workspace View Mode">
          <button
            type="button"
            className={`tab-btn ${activeTab === 'viewer' ? 'active' : ''}`}
            onClick={() => setActiveTab('viewer')}
          >
            Map Viewer
          </button>
          <button
            type="button"
            className={`tab-btn ${activeTab === 'analytics' ? 'active' : ''}`}
            onClick={() => setActiveTab('analytics')}
          >
            Analytics
          </button>
          <button
            type="button"
            className={`tab-btn ${activeTab === 'rawdata' ? 'active' : ''}`}
            onClick={() => setActiveTab('rawdata')}
          >
            Raw Data
          </button>
        </nav>

        {activeTab === 'viewer' && (
          <div className="viewer-tab-content">
            <SelectedEntityFocusCard selectedCell={state.selectedCell} />
            <WorldViewer
              ref={viewerRef}
              frame={state.frame}
              selectedCellId={state.selectedCellId}
              onSelectCell={onSelectCell}
              onExportScreenshot={exportScreenshot}
              onToggleFullScreen={toggleFullScreen}
              isFullScreen={isFullScreen}
              debugProjections={state.debugProjections}
              activeResourceLayers={state.activeResourceLayers}
            />
            <BottomDataPanel state={state} />
            <BottomStatsStrip stats={monitorStats} />
          </div>
        )}
        {activeTab === 'analytics' && (
          <div className="viewer-tab-content">
            <BalanceAnalyticsPanel viewModel={buildBalanceViewModel(state)} />
          </div>
        )}
        {activeTab === 'rawdata' && (
          <div className="viewer-tab-content">
            <RawDataGridPanel frame={state.latestLiveFrame ?? state.frame} onSelectCell={(id) => onSelectCell(id)} />
          </div>
        )}

        {exportStatus ? <p className="export-status" role="status">{exportStatus}</p> : null}
      </section>
      <CellInspector selectedCell={state.selectedCell} />
    </main>
  );
}
