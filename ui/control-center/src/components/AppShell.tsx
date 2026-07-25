import { useEffect, useMemo, useState } from 'react';
import { createAppStore, getMonitorDataState } from '../app/appState';
import { createRunnerController } from '../app/runnerController';
import { MonitorWorkspace } from './MonitorWorkspace';
import { WorldEditorWorkspace } from './WorldEditorWorkspace';
import { RunControls } from './RunControls';
import { buildMonitorStats } from './monitorStats';
import { uiText } from '../uiText';
import { DiagnosticsPanel } from './DiagnosticsPanel';
import { ExperimentWorkspace } from './ExperimentWorkspace';

export function AppShell() {
  const store = useMemo(() => createAppStore(), []);
  const controller = useMemo(() => createRunnerController({ store }), [store]);
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

  const exportScreenshot = (png: string | null) => {
    setExportStatus(
      png ? `${uiText.controls.startScreenshotReady} (${png.length} bytes)` : uiText.controls.startScreenshotUnavailable
    );
  };

  const [activeWorkspace, setActiveWorkspace] = useState<'monitor' | 'world-editor' | 'experiments' | 'diagnostics'>('monitor');

  return (
    <div className="app-shell">
      <header className="top-bar" data-testid="monitor-top-context">
        <div className="top-bar-left">
          <div className="brand-title">
            <p className="eyebrow">{uiText.app.eyebrow}</p>
            <h1>{uiText.app.title}</h1>
          </div>
          <nav className="mode-tabs" aria-label={uiText.app.primaryViews}>
            <button
              type="button"
              role="tab"
              aria-selected={activeWorkspace === 'monitor'}
              onClick={() => setActiveWorkspace('monitor')}
            >
              {uiText.workspace.monitor}
            </button>
            <button
              type="button"
              role="tab"
              aria-selected="false"
              aria-label={uiText.workspace.organismViewUnavailable}
              title={uiText.workspace.unavailableSummary}
              disabled
            >
              {uiText.workspace.organismView}
              <small>{uiText.workspace.unavailable}</small>
            </button>
            <button
              type="button"
              role="tab"
              aria-selected={activeWorkspace === 'world-editor'}
              onClick={() => setActiveWorkspace('world-editor')}
            >
              {uiText.workspace.worldEditor}
            </button>
            <button
              type="button"
              role="tab"
              aria-selected={activeWorkspace === 'experiments'}
              onClick={() => setActiveWorkspace('experiments')}
            >
              Experiments & Comparison
            </button>
            <button
              type="button"
              role="tab"
              aria-selected={activeWorkspace === 'diagnostics'}
              onClick={() => setActiveWorkspace('diagnostics')}
            >
              Diagnostics & Recovery
            </button>
          </nav>
        </div>
        <div className="top-bar-right">
          <RunControls
            state={state}
            onStart={controller.startRun}
            onPause={controller.pauseRun}
            onResume={controller.resumeRun}
            onStep={controller.stepRun}
            onStop={controller.stopRun}
          />
          <div className="top-utility-controls" aria-label="Application settings">
            <button
              type="button"
              onClick={toggleTheme}
              aria-label={state.theme === 'dark' ? uiText.controls.switchToLightTheme : uiText.controls.switchToDarkTheme}
              title={state.theme === 'dark' ? uiText.controls.switchToLightTheme : uiText.controls.switchToDarkTheme}
            >
              {state.theme === 'dark' ? 'Light' : 'Dark'}
            </button>
          </div>
        </div>
      </header>

      {activeWorkspace === 'monitor' ? (
        <MonitorWorkspace
          state={state}
          monitorStats={monitorStats}
          onScenarioChange={(scenarioId) => store.getState().setSelectedScenarioId(scenarioId)}
          onReconnect={controller.connectRunner}
          onSelectCell={(cellId) => store.getState().selectCell(cellId)}
          onExportScreenshot={exportScreenshot}
          onFreezeFrame={() => store.getState().freezeCurrentFrame()}
          onJumpToLive={() => store.getState().jumpToLive()}
          onSelectHistoryTick={(tick) => store.getState().selectHistoryTick(tick)}
          exportStatus={exportStatus}
        />
      ) : activeWorkspace === 'world-editor' ? (
        <div style={{ padding: '24px', maxWidth: '1400px', margin: '0 auto' }}>
          <WorldEditorWorkspace
            state={state}
            onSelectScenario={(scenarioId) => store.getState().setSelectedScenarioId(scenarioId)}
            onRelaunchRun={(scenarioId) => {
              store.getState().setSelectedScenarioId(scenarioId);
              controller.startRun();
              setActiveWorkspace('monitor');
            }}
          />
        </div>
      ) : activeWorkspace === 'experiments' ? (
        <div style={{ padding: '24px', maxWidth: '1400px', margin: '0 auto' }}>
          <ExperimentWorkspace state={state} />
        </div>
      ) : (
        <div style={{ padding: '24px', maxWidth: '1400px', margin: '0 auto' }}>
          <DiagnosticsPanel
            appState={state}
            monitorDataState={monitorDataState}
            serverInfo={state.serverInfo}
            scenarios={state.scenarios}
            endpoint="ws://localhost:3000/ws"
            connectionState={state.connectionState}
            onReconnect={controller.connectRunner}
          />
        </div>
      )}
    </div>
  );
}
