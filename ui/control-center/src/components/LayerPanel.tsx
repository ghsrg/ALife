import { type AppStore } from '../app/appState';
import type { MonitorViewModel } from '../app/monitorViewModel';
import { uiText } from '../uiText';

interface LayerPanelProps {
  state: AppStore;
  monitorViewModel: MonitorViewModel;
  isCollapsed?: boolean;
  onToggleCollapse?: () => void;
}

export function LayerPanel({
  state,
  monitorViewModel,
  isCollapsed,
  onToggleCollapse
}: LayerPanelProps) {
  const frame = state.frame;

  if (isCollapsed && onToggleCollapse) {
    return (
      <div className="cc-layers-panel collapsed" data-testid="monitor-layers-track">
        <div className="cc-layers-collapse-strip">
          <button className="cc-layers-strip-btn" onClick={onToggleCollapse} title="Expand Layers">»</button>
        </div>
      </div>
    );
  }

  return (
    <div className="cc-layers-panel" aria-label={uiText.layers.ariaLabel} data-testid="monitor-layers-track">
      <div className="cc-layers-header">
        <div className="cc-layers-title">LAYERS & FILTERS</div>
        {onToggleCollapse ? (
          <button className="cc-layers-collapse-btn" onClick={onToggleCollapse} title="Collapse Layers">«</button>
        ) : null}
      </div>

      <div className="cc-layers-tabs">
        <button className="cc-layers-tab active">SCENE</button>
        <button className="cc-layers-tab">DATA</button>
      </div>

      <div className="cc-layers-section">
        <span className="cc-layers-section-label">COLOR MODE</span>
        <select className="cc-color-mode-select">
          <option>Organism ID (Ancestry)</option>
          <option>Energy Level</option>
          <option>Age</option>
        </select>
      </div>

      <div className="cc-layers-section">
        <span className="cc-layers-section-label">FIELD LAYERS</span>
        <label className={`cc-resource-layer-row ${monitorViewModel.hasResourceLayer ? '' : 'layer-option-missing'}`}>
          <input
            type="checkbox"
            checked={monitorViewModel.hasResourceLayer}
            readOnly
            disabled={!monitorViewModel.hasResourceLayer}
          />
          <span>
            {uiText.layers.resources}
            {monitorViewModel.resourceLayerState ? (
              <small className="cc-layer-state">{monitorViewModel.resourceLayerState}</small>
            ) : null}
          </span>
        </label>
        {(() => {
          const debugLayers =
            state.debugProjections?.status === 'available'
              ? state.debugProjections.visualWorld.resourceLayers
              : [];
          const colors = ['#00c896', '#3a86ff', '#ffb703', '#8338ec'];

          let layerItems: { index: number; name: string; color: string }[] = [];

          if (debugLayers.length > 0) {
            layerItems = debugLayers.map((l) => ({
              index: l.layerIndex,
              name: `Layer ${l.layerIndex} (${l.totalAmount > 0 ? l.totalAmount.toFixed(1) + ' total' : 'empty'})`,
              color: colors[l.layerIndex % colors.length]
            }));
          } else if (monitorViewModel.hasResourceLayer) {
            layerItems = [
              { index: 0, name: 'Nutrient / Organic', color: '#00c896' },
              { index: 1, name: 'Mineral', color: '#3a86ff' },
              { index: 2, name: 'Energy', color: '#ffb703' }
            ];
          }

          if (layerItems.length === 0) return null;

          return layerItems.map((layer) => {
            const isActive = state.activeResourceLayers?.includes(layer.index) ?? true;
            return (
              <label key={layer.index} className="cc-field-layer">
                <span className="cc-field-dot" style={{ backgroundColor: layer.color }} />
                <span className="cc-field-name">{layer.name}</span>
                <div className="cc-toggle">
                  <input
                    type="checkbox"
                    checked={isActive}
                    disabled={!monitorViewModel.hasResourceLayer}
                    onChange={() => state.toggleResourceLayer?.(layer.index)}
                  />
                  <span className="cc-toggle-slider" />
                </div>
              </label>
            );
          });
        })()}
      </div>

      <div className="cc-layers-section">
        <span className="cc-layers-section-label">OVERLAYS</span>
        <label className="cc-overlay-item">
          <div className="cc-toggle">
            <input type="checkbox" defaultChecked />
            <span className="cc-toggle-slider" />
          </div>
          Cell outlines
        </label>
        <label className="cc-overlay-item">
          <div className="cc-toggle">
            <input type="checkbox" />
            <span className="cc-toggle-slider" />
          </div>
          Organism view outline
        </label>
        <label className="cc-overlay-item">
          <div className="cc-toggle">
            <input type="checkbox" defaultChecked />
            <span className="cc-toggle-slider" />
          </div>
          Dead matter
        </label>
      </div>

      <div style={{ flex: 1 }} />

      <div className="cc-layers-section" style={{ borderTop: '1px solid rgba(255,255,255,0.06)', paddingBottom: '16px' }}>
        <span className="cc-layers-section-label" style={{ marginTop: '8px' }}>RENDERING</span>
        <div className="cc-rendering-row">
          <span className="cc-rendering-label">Semantic zoom</span>
          <span className="cc-rendering-val">Overview</span>
        </div>
        <div className="cc-rendering-row">
          <span className="cc-rendering-label">Quality</span>
          <div className="cc-quality-btns">
            <button className="cc-quality-btn active">AUTO</button>
            <button className="cc-quality-btn">HIGH</button>
          </div>
        </div>
      </div>
    </div>
  );
}
