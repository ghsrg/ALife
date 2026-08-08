import { useEffect, useMemo, useRef, useState } from 'react';

import { createAppStore, getMonitorDataState } from '../app/appState';
import { createRunnerController } from '../app/runnerController';
import { MonitorWorkspace } from './MonitorWorkspace';
import { WorldEditorWorkspace } from './WorldEditorWorkspace';
import { RunBar } from './RunBar';
import { GlobalNavigation } from './GlobalNavigation';
import { LevelPanel, type AnalysisLevel } from './LevelPanel';
import { LayerPanel } from './LayerPanel';
import { InspectorPanel } from './InspectorPanel';
import { BottomDataPanel } from './BottomDataPanel';
import { uiText } from '../uiText';
import { DiagnosticsPanel } from './DiagnosticsPanel';
import { ExperimentWorkspace } from './ExperimentWorkspace';
import { EvolutionWorkspace } from './EvolutionWorkspace';
import { OrganismWorkspace } from './OrganismWorkspace';
import { SpecializationWorkspace } from './SpecializationWorkspace';
import { LibraryWorkspace } from './LibraryWorkspace';
import { buildMonitorViewModel } from '../app/monitorViewModel';
import { isSelectionCompatibleWithLevel } from '../app/selectionModel';

export function AppShell() {
  const store = useMemo(() => createAppStore(), []);
  const controller = useMemo(() => createRunnerController({ store }), [store]);
  const [state, setState] = useState(store.getState());
  const [exportStatus, setExportStatus] = useState<string | null>(null);

  const [activeLevel, setActiveLevel] = useState<AnalysisLevel>('world');
  const [activeWorkspace, setActiveWorkspace] = useState<string>('monitor');
  const [isMapFullScreen, setIsMapFullScreen] = useState(false);
  const [isFullScreenDataVisible, setIsFullScreenDataVisible] = useState(false);
  const prevLevelRef = useRef(activeLevel);

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

  useEffect(() => {
    if (prevLevelRef.current !== activeLevel) {
      prevLevelRef.current = activeLevel;
      if (!isSelectionCompatibleWithLevel(state.currentSelection, activeLevel)) {
        store.getState().clearSelection(`Selection cleared: incompatible with ${activeLevel} level`);
      }
    }
  }, [activeLevel, state.currentSelection, store]);


  const toggleTheme = () => {
    store.getState().setTheme(state.theme === 'dark' ? 'light' : 'dark');
  };

  const monitorDataState = getMonitorDataState(state);
  const monitorViewModel = buildMonitorViewModel(state);

  const exportScreenshot = (png: string | null) => {
    setExportStatus(
      png ? `${uiText.controls.startScreenshotReady} (${png.length} bytes)` : uiText.controls.startScreenshotUnavailable
    );
  };

  if (activeWorkspace === 'monitor' && isMapFullScreen) {
    return (
      <div className="app-shell cc-map-fullscreen-shell">
        <div className="cc-map-fullscreen-stage">
          <MonitorWorkspace
            state={state}
            onScenarioChange={(scenarioId) => store.getState().setSelectedScenarioId(scenarioId)}
            onReconnect={controller.connectRunner}
            onSelectCell={(cellId) => store.getState().selectCell(cellId)}
            onSelectTarget={(selection) => store.getState().selectMonitorTarget(selection)}
            activeLevel={activeLevel}
            onExportScreenshot={exportScreenshot}
            onFreezeFrame={() => store.getState().freezeCurrentFrame()}
            onJumpToLive={() => store.getState().jumpToLive()}
            onSelectHistoryTick={(tick) => store.getState().selectHistoryTick(tick)}
            isMapFullScreen={isMapFullScreen}
            onMapFullScreenChange={(next) => {
              setIsMapFullScreen(next);
              if (!next) setIsFullScreenDataVisible(false);
            }}
            exportStatus={exportStatus}
          />
          <button
            type="button"
            className="cc-map-fullscreen-data-toggle"
            onClick={() => setIsFullScreenDataVisible((visible) => !visible)}
            aria-label={isFullScreenDataVisible ? 'Hide Data Panel' : 'Show Data Panel'}
          >
            {isFullScreenDataVisible ? 'Hide Data Panel' : 'Show Data Panel'}
          </button>
          {isFullScreenDataVisible ? (
            <div className="cc-map-fullscreen-data-panel" data-testid="monitor-fullscreen-data-panel">
              <BottomDataPanel state={state} activeLevel={activeLevel} />
            </div>
          ) : null}
        </div>
      </div>
    );
  }

  return (
    <div className="app-shell">
      {/* Row 1: Global Navigation 62px */}
      <GlobalNavigation
        activeWorkspace={activeWorkspace}
        onWorkspaceChange={setActiveWorkspace}
        theme={state.theme}
        onToggleTheme={toggleTheme}
        warningsCount={0}
      />
      
      {/* Row 2: Run & Data Context Bar 82px */}
      <RunBar
        state={state}
        onStart={controller.startRun}
        onPause={controller.pauseRun}
        onResume={controller.resumeRun}
        onStep={controller.stepRun}
        onStop={controller.stopRun}
        onScenarioChange={(scenarioId) => store.getState().setSelectedScenarioId(scenarioId)}
        onReconnect={controller.connectRunner}
      />
      
      {/* Row 3: Workspace area (flex) */}
      <div className="cc-workspace">
        {activeWorkspace === 'monitor' ? (
          <>
            <LevelPanel activeLevel={activeLevel} onLevelChange={setActiveLevel} />
            <LayerPanel
              state={state}
              monitorViewModel={monitorViewModel}
              activeLevel={activeLevel}
            />
            <MonitorWorkspace
              state={state}
              onScenarioChange={(scenarioId) => store.getState().setSelectedScenarioId(scenarioId)}
              onReconnect={controller.connectRunner}
              onSelectCell={(cellId) => store.getState().selectCell(cellId)}
              onSelectTarget={(selection) => store.getState().selectMonitorTarget(selection)}
              activeLevel={activeLevel}
              onExportScreenshot={exportScreenshot}
              onFreezeFrame={() => store.getState().freezeCurrentFrame()}
              onJumpToLive={() => store.getState().jumpToLive()}
              onSelectHistoryTick={(tick) => store.getState().selectHistoryTick(tick)}
              isMapFullScreen={isMapFullScreen}
              onMapFullScreenChange={(next) => {
                setIsMapFullScreen(next);
                if (!next) setIsFullScreenDataVisible(false);
              }}
              exportStatus={exportStatus}
            />
            <InspectorPanel
              frame={state.frame}
              debugProjections={state.debugProjections}
              currentSelection={state.currentSelection}
              selectedCell={state.selectedCell}
              selectionNotice={state.selectionNotice}
              displayedTick={state.frame?.tick || 0}
              onSelectCell={(cellId) => store.getState().selectCell(cellId)}
            />
          </>
        ) : (
          <div className="cc-workspace-full">
            {activeWorkspace === 'organism-view' && (
              <OrganismWorkspace
                state={state}
                onSelectCell={(cellId) => store.getState().selectCell(cellId)}
              />
            )}
            {activeWorkspace === 'world-editor' && (
              <WorldEditorWorkspace
                state={state}
                onSelectScenario={(scenarioId) => store.getState().setSelectedScenarioId(scenarioId)}
                onRelaunchRun={(scenarioId) => {
                  store.getState().setSelectedScenarioId(scenarioId);
                  controller.startRun();
                  setActiveWorkspace('monitor');
                }}
              />
            )}
            {activeWorkspace === 'experiments' && <ExperimentWorkspace state={state} />}
            {activeWorkspace === 'evolution' && <EvolutionWorkspace state={state} />}
            {activeWorkspace === 'specialization' && <SpecializationWorkspace state={state} />}
            {activeWorkspace === 'library' && <LibraryWorkspace state={state} />}
            {activeWorkspace === 'diagnostics' && (
              <DiagnosticsPanel
                appState={state}
                monitorDataState={monitorDataState}
                serverInfo={state.serverInfo}
                scenarios={state.scenarios}
                endpoint="ws://localhost:3000/ws"
                connectionState={state.connectionState}
                onReconnect={controller.connectRunner}
              />
            )}
          </div>
        )}
      </div>
      
      {/* Row 4: adaptive Data Panel reference track */}
      {activeWorkspace === 'monitor' && (
        <div className="cc-data-panel" data-testid="monitor-data-track">
          <BottomDataPanel state={state} activeLevel={activeLevel} />
        </div>
      )}
    </div>
  );
}
