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
        <input
          type="checkbox"
          checked={monitorViewModel.hasResourceLayer && (state.activeResourceLayers?.length ?? 0) > 0}
          readOnly
          disabled={!monitorViewModel.hasResourceLayer}
        />
        <span>
          {uiText.layers.resources}
          <small>{monitorViewModel.resourceLayerState}</small>
        </span>
      </label>

      {/* Field Layers & Dynamic Resource Toggles */}
      {(() => {
        const debugLayers =
          state.debugProjections?.status === 'available'
            ? state.debugProjections.visualWorld.resourceLayers
            : [];
        const colors = ['#2ec4b6', '#3a86ff', '#ffb703', '#8338ec'];

        let layerItems: { index: number; name: string; color: string }[] = [];

        if (debugLayers.length > 0) {
          layerItems = debugLayers.map((l) => ({
            index: l.layerIndex,
            name: `Layer ${l.layerIndex} (${l.totalAmount > 0 ? l.totalAmount.toFixed(1) + ' total' : 'empty'})`,
            color: colors[l.layerIndex % colors.length]
          }));
        } else if (monitorViewModel.hasResourceLayer) {
          layerItems = [
            { index: 0, name: 'Nutrient / Organic', color: '#2ec4b6' },
            { index: 1, name: 'Mineral', color: '#3a86ff' },
            { index: 2, name: 'Energy', color: '#ffb703' }
          ];
        }

        if (layerItems.length === 0) return null;

        return (
          <div className="field-layers-selector" aria-label="Field layers selection">
            <span className="field-layers-title">FIELD LAYERS ({layerItems.length})</span>
            {layerItems.map((layer) => {
              const isActive = state.activeResourceLayers?.includes(layer.index) ?? true;
              return (
                <label key={layer.index} className="resource-layer-item">
                  <input
                    type="checkbox"
                    checked={isActive}
                    disabled={!monitorViewModel.hasResourceLayer}
                    onChange={() => state.toggleResourceLayer?.(layer.index)}
                  />
                  <span className="resource-dot" style={{ backgroundColor: layer.color }} />
                  <span className="resource-name">{layer.name}</span>
                </label>
              );
            })}
          </div>
        );
      })()}

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
