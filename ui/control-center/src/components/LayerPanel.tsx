import { type AppStore } from '../app/appState';
import { buildFieldLayerDisplay, buildResourceLayerDisplay } from '../app/layerDisplayModel';
import type { MonitorViewModel } from '../app/monitorViewModel';
import { uiText } from '../uiText';
import { type AnalysisLevel } from './LevelPanel';

interface LayerPanelProps {
  state: AppStore;
  monitorViewModel: MonitorViewModel;
  activeLevel?: AnalysisLevel;
  isCollapsed?: boolean;
  onToggleCollapse?: () => void;
}

const RESOURCE_COLORS = ['#00c896', '#3a86ff', '#ffb703', '#8338ec', '#2dd4bf', '#a78bfa'];

export function LayerPanel({
  state,
  activeLevel = 'world',
  isCollapsed,
  onToggleCollapse
}: LayerPanelProps) {
  if (isCollapsed && onToggleCollapse) {
    return (
      <div className="cc-layers-panel collapsed" data-testid="monitor-layers-track">
        <div className="cc-layers-collapse-strip">
          <button className="cc-layers-strip-btn" onClick={onToggleCollapse} title="Expand Layers">»</button>
        </div>
      </div>
    );
  }

  const visualWorld = state.debugProjections.status === 'available'
    ? state.debugProjections.visualWorld
    : null;
  const fields = visualWorld?.fields ?? [];
  const resourceLayers = visualWorld?.resourceLayers ?? [];
  const showCellSpecificControls = activeLevel === 'cells' || activeLevel === 'organisms';

  return (
    <div className="cc-layers-panel" aria-label={uiText.layers.ariaLabel} data-testid="monitor-layers-track">
      <div className="cc-layers-header">
        <div className="cc-layers-title">LAYERS & FILTERS</div>
        {onToggleCollapse ? (
          <button className="cc-layers-collapse-btn" onClick={onToggleCollapse} title="Collapse Layers">«</button>
        ) : null}
      </div>

      <div className="cc-layers-dynamic-scroll" data-testid="layers-dynamic-scroll">
        <section className="cc-layers-section">
          <span className="cc-layers-section-label">Fields</span>
          {fields.length > 0 ? (
            fields.map((field) => {
              const display = buildFieldLayerDisplay(field);
              return (
                <div
                  key={field.fieldId}
                  className="cc-field-layer"
                  aria-label={`Field layer ${display.primaryLabel}`}
                  title={display.provenance}
                >
                  <span className="cc-field-dot" style={{ backgroundColor: '#22d3ee' }} />
                  <span className="cc-field-name">{display.primaryLabel}</span>
                  <small className="cc-layer-state">{display.secondaryLabel}</small>
                </div>
              );
            })
          ) : (
            <UnavailableLayerRow label="Field layers unavailable" reason="No source-backed spatial Field projection loaded" />
          )}
        </section>

        <section className="cc-layers-section">
          <span className="cc-layers-section-label">Resources</span>
          {resourceLayers.length > 0 ? (
            resourceLayers.map((layer) => {
              const isActive = state.activeResourceLayers?.includes(layer.layerIndex) ?? true;
              const color = RESOURCE_COLORS[layer.layerIndex % RESOURCE_COLORS.length];
              const display = buildResourceLayerDisplay(layer);
              return (
                <label
                  key={layer.layerIndex}
                  className="cc-field-layer"
                  aria-label={`Resource layer ${display.primaryLabel}`}
                  title={display.provenance}
                >
                  <span className="cc-field-dot" style={{ backgroundColor: color }} />
                  <span className="cc-field-name">{display.primaryLabel}</span>
                  <span
                    className="cc-layer-gradient"
                    aria-hidden="true"
                    style={{ background: `linear-gradient(90deg, transparent, ${color})` }}
                  />
                  <small className="cc-layer-state">{display.secondaryLabel}</small>
                  <div className="cc-toggle">
                    <input
                      type="checkbox"
                      checked={isActive}
                      onChange={() => state.toggleResourceLayer?.(layer.layerIndex)}
                    />
                    <span className="cc-toggle-slider" />
                  </div>
                </label>
              );
            })
          ) : (
            <UnavailableLayerRow label="Resource layers unavailable" reason="No source-backed Resource layer projection loaded" />
          )}
        </section>

        <section className="cc-layers-section">
          <span className="cc-layers-section-label">Visual Effects</span>
          <VisualEffectRow
            label="🌌 Nebula Resource Glow"
            checked={state.visualEffects?.showNebula ?? false}
            onChange={() => state.toggleVisualEffect?.('showNebula')}
          />
          <VisualEffectRow
            label="💫 Stardust Particles"
            checked={state.visualEffects?.showParticles ?? false}
            onChange={() => state.toggleVisualEffect?.('showParticles')}
          />
          <VisualEffectRow
            label="🕸️ Resource Filaments"
            checked={state.visualEffects?.showFilaments ?? false}
            onChange={() => state.toggleVisualEffect?.('showFilaments')}
          />
          <VisualEffectRow
            label="🚩 Phenotype Traits"
            checked={state.visualEffects?.showPhenotypeTraits ?? false}
            onChange={() => state.toggleVisualEffect?.('showPhenotypeTraits')}
          />
          <VisualEffectRow
            label="✨ Division Flash FX"
            checked={state.visualEffects?.showDivisionFlash ?? false}
            onChange={() => state.toggleVisualEffect?.('showDivisionFlash')}
          />
          <VisualEffectRow
            label="🔬 Organelle Structure"
            checked={state.visualEffects?.showOrganelles ?? true}
            onChange={() => state.toggleVisualEffect?.('showOrganelles')}
          />
          <VisualEffectRow
            label="🧬 Organism Organic Hulls"
            checked={state.visualEffects?.showOrganismHulls ?? true}
            onChange={() => state.toggleVisualEffect?.('showOrganismHulls')}
          />
          <VisualEffectRow
            label="⚡ Animated Joint Pulses"
            checked={state.visualEffects?.showJointPulses ?? true}
            onChange={() => state.toggleVisualEffect?.('showJointPulses')}
          />
        </section>
      </div>

      {showCellSpecificControls ? (
        <>
          <section className="cc-layers-section">
            <span className="cc-layers-section-label">Cell Energy</span>
            <label className="cc-overlay-item">
              <span className="cc-layer-state">Cells encoding unavailable until source-backed Cell energy projection is present</span>
            </label>
            <label className="cc-overlay-item">
              <span className="cc-layer-state">Heatmap unavailable until aggregate provenance is present</span>
            </label>
          </section>

          <section className="cc-layers-section">
            <span className="cc-layers-section-label">Structure</span>
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
              Joints
            </label>
          </section>

          <section className="cc-layers-section">
            <span className="cc-layers-section-label">Selection</span>
            <label className="cc-overlay-item">
              <span className="cc-layer-state">Trail requires one selected Cell or Organism and retained RRD samples</span>
            </label>
          </section>
        </>
      ) : null}
    </div>
  );
}

function UnavailableLayerRow({ label, reason }: { label: string; reason: string }) {
  return (
    <div className="cc-field-layer layer-option-missing" title={reason}>
      <span className="cc-field-dot" style={{ backgroundColor: '#475569' }} />
      <span className="cc-field-name">{label}</span>
      <small className="cc-layer-state">unavailable</small>
    </div>
  );
}

function VisualEffectRow({ label, checked, onChange }: { label: string; checked: boolean; onChange: () => void }) {
  return (
    <label className="cc-field-layer">
      <span className="cc-field-name">{label}</span>
      <div className="cc-toggle">
        <input type="checkbox" checked={checked} onChange={onChange} />
        <span className="cc-toggle-slider" />
      </div>
    </label>
  );
}
