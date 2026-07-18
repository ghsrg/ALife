import { getMonitorDataState, type AppStore } from '../app/appState';
import type { MonitorViewModel } from '../app/monitorViewModel';
import { uiText } from '../uiText';
import { ConnectionPanel } from './ConnectionPanel';

interface LayerPanelProps {
  state: AppStore;
  monitorDataState: ReturnType<typeof getMonitorDataState>;
  monitorViewModel: MonitorViewModel;
  onScenarioChange: (scenarioId: string) => void;
  onReconnect: () => void;
}

export function LayerPanel({
  state,
  monitorDataState,
  monitorViewModel,
  onScenarioChange,
  onReconnect
}: LayerPanelProps) {
  const frame = state.frame;

  return (
    <aside className="side-panel" aria-label={uiText.layers.ariaLabel}>
      <h2>{uiText.layers.title}</h2>
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
        <span>{uiText.layers.cells}</span>
      </label>
      <label className={monitorViewModel.hasResourceLayer ? 'layer-option' : 'layer-option layer-option-missing'}>
        <input type="checkbox" checked={monitorViewModel.hasResourceLayer} readOnly disabled={!monitorViewModel.hasResourceLayer} />
        <span>
          {uiText.layers.resources}
          <small>{monitorViewModel.resourceLayerState}</small>
        </span>
      </label>
      <label className="layer-option muted">
        <input type="checkbox" disabled />
        <span>{uiText.layers.joints}</span>
      </label>
      <div className="metric-list">
        <div><span>{uiText.layers.run}</span><strong>{frame.runId}</strong></div>
        <div><span>{uiText.layers.cells}</span><strong>{frame.cells.length}</strong></div>
        <div><span>{uiText.layers.world}</span><strong>{frame.world.width} x {frame.world.height}</strong></div>
        <div><span>{uiText.layers.projection}</span><strong>{monitorViewModel.projectionLabel}</strong></div>
      </div>
    </aside>
  );
}
