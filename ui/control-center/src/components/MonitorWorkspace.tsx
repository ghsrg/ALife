import { useEffect, useRef, useState } from 'react';
import { getMonitorDataState, type AppStore } from '../app/appState';
import { buildMonitorViewModel } from '../app/monitorViewModel';
import { describeProjectionContext } from '../projection/projectionContext';
import type { CellId } from '../projection/types';
import { uiText } from '../uiText';
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
            <span>{monitorViewModel.subtitle}</span>
          </div>
          <div className="start-demo-provenance compact" aria-label="Start demo provenance">
            <strong>{uiText.demo.startDemo}</strong>
            <span>
              {monitorViewModel.startDemo.projectionSource === 'live'
                ? uiText.demo.liveProjectionSource
                : uiText.demo.fixtureProjectionSource}
            </span>
            <span>{`${uiText.demo.runnerDataPrefix}: ${monitorViewModel.startDemo.runnerDataLabel}`}</span>
            <span>{monitorViewModel.startDemo.unavailableFieldsLabel}</span>
          </div>
          <div className="data-context-summary">
            <span>{uiText.dataContext.title}</span>
            <strong>{describeProjectionContext(state.projectionContext)}</strong>
          </div>
          <div className="data-context-run">
            <span>Run {state.projectionContext.runId}</span>
            <span>{state.projectionContext.isReadOnly ? 'read-only' : 'live writable controls'}</span>
          </div>
          {state.projectionContext.warning ? (
            <p role="alert">{state.projectionContext.warning}</p>
          ) : null}
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
              .slice(-8)
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
        <WorldViewer
          ref={viewerRef}
          frame={state.frame}
          selectedCellId={state.selectedCellId}
          onSelectCell={onSelectCell}
          onExportScreenshot={exportScreenshot}
          onToggleFullScreen={toggleFullScreen}
          isFullScreen={isFullScreen}
        />
        <SelectedEntityFocusCard selectedCell={state.selectedCell} />
        <BottomStatsStrip stats={monitorStats} />
        {exportStatus ? <p className="export-status" role="status">{exportStatus}</p> : null}
      </section>
      <CellInspector selectedCell={state.selectedCell} />
    </main>
  );
}
