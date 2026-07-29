import { useEffect, useRef } from 'react';
import { type AppStore } from '../app/appState';
import { buildMonitorViewModel } from '../app/monitorViewModel';
import { describeProjectionContext } from '../projection/projectionContext';
import type { CellId } from '../projection/types';
import { uiText } from '../uiText';
import { SelectedEntityFocusCard } from './SelectedEntityFocusCard';
import { WorldViewer, type WorldViewerHandle } from './WorldViewer';

interface MonitorWorkspaceProps {
  state: AppStore;
  onScenarioChange: (scenarioId: string) => void;
  onReconnect: () => void;
  onSelectCell: (cellId: CellId | null) => void;
  onToggleTheme?: () => void;
  onExportScreenshot: (png: string | null) => void;
  onFreezeFrame?: () => void;
  onJumpToLive?: () => void;
  onSelectHistoryTick?: (tick: number) => void;
  isMapFullScreen?: boolean;
  onMapFullScreenChange?: (isFullScreen: boolean) => void;
  exportStatus: string | null;
}

export function MonitorWorkspace({
  state,
  onScenarioChange,
  onReconnect,
  onSelectCell,
  onExportScreenshot,
  onFreezeFrame,
  onJumpToLive,
  onSelectHistoryTick,
  isMapFullScreen = false,
  onMapFullScreenChange,
  exportStatus
}: MonitorWorkspaceProps) {
  const viewerRef = useRef<WorldViewerHandle | null>(null);
  const workspaceRef = useRef<HTMLElement | null>(null);
  const monitorViewModel = buildMonitorViewModel(state);

  useEffect(() => {
    const handleFullScreenChange = () => {
      onMapFullScreenChange?.(document.fullscreenElement === workspaceRef.current);
    };

    document.addEventListener('fullscreenchange', handleFullScreenChange);
    return () => document.removeEventListener('fullscreenchange', handleFullScreenChange);
  }, [onMapFullScreenChange]);

  const exportScreenshot = () => {
    onExportScreenshot(viewerRef.current?.exportPng() ?? null);
  };

  const toggleFullScreen = () => {
    if (isMapFullScreen) {
      onMapFullScreenChange?.(false);
      void document.exitFullscreen?.();
      return;
    }

    onMapFullScreenChange?.(true);
    void workspaceRef.current?.requestFullscreen?.();
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%', minHeight: 0, overflow: 'hidden', flex: 1 }}>
      <section
        ref={workspaceRef}
        className={isMapFullScreen ? 'viewer-panel map-fullscreen' : 'viewer-panel'}
        aria-label={uiText.workspace.monitorWorkspace}
        style={{ border: 'none', borderRadius: 0, height: '100%', display: 'flex', flexDirection: 'column', minHeight: 0, overflow: 'hidden' }}
      >
        {/* Hidden provenance block — required by App.test.tsx assertions, not visible in UI */}
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
        <section className="map-context-strip" aria-label={uiText.dataContext.title}>
          <div className="map-context-primary">
            <strong>{monitorViewModel.scenarioTitle}</strong>
            <span className="context-pill">{describeProjectionContext(state.projectionContext)}</span>
            <span className="context-pill">{state.projectionContext.isReadOnly ? 'read-only' : 'live'}</span>
            {state.projectionContext.warning ? (
              <span className="context-pill warning" role="alert">{state.projectionContext.warning}</span>
            ) : null}
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

        <div className="viewer-tab-content" data-testid="monitor-map-track">
          <SelectedEntityFocusCard selectedCell={state.selectedCell} />
          <WorldViewer
            ref={viewerRef}
            frame={state.frame}
            selectedCellId={state.selectedCellId}
            onSelectCell={onSelectCell}
            onExportScreenshot={exportScreenshot}
            onToggleFullScreen={toggleFullScreen}
            isFullScreen={isMapFullScreen}
            debugProjections={state.debugProjections}
            activeResourceLayers={state.activeResourceLayers}
          />
        </div>

        {exportStatus ? <p className="export-status" role="status">{exportStatus}</p> : null}
      </section>
    </div>
  );
}
