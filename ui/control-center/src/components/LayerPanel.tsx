import { type AppStore } from '../app/appState';
import { buildFieldLayerDisplay, buildResourceLayerDisplay } from '../app/layerDisplayModel';
import type { MonitorViewModel } from '../app/monitorViewModel';
import type { DebugField } from '../projection/types';
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
const FIELD_COLORS = ['#22d3ee', '#f97316', '#eab308', '#a855f7', '#38bdf8', '#ef4444'];
const FRAME_SUMMARY_FIELD_IDS = new Set(['heat', 'waste']);

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
  const configuredFields = fields.filter(isConfiguredFieldLayer);
  const resourceLayers = visualWorld?.resourceLayers ?? [];
  const configuredFieldIds = configuredFields.map((field) => field.fieldId);
  const resourceLayerIndices = resourceLayers.map((layer) => layer.layerIndex);
  const fieldsGroupEnabled =
    configuredFieldIds.length > 0 &&
    configuredFieldIds.some((fieldId) => !(state.disabledFieldLayers?.includes(fieldId) ?? false));
  const resourcesGroupEnabled = resourceLayerIndices.some((layerIndex) =>
    state.activeResourceLayers?.includes(layerIndex) ?? false
  );
  const visualEffectsGroupEnabled = Object.values(state.visualEffects ?? {}).some(Boolean);
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
          <LayerGroupHeader
            label="Fields"
            checked={fieldsGroupEnabled}
            disabled={configuredFieldIds.length === 0}
            onChange={() => state.setFieldLayersEnabled?.(configuredFieldIds, !fieldsGroupEnabled)}
          />
          {configuredFields.length > 0 ? (
            configuredFields.map((field, index) => {
              const display = buildFieldLayerDisplay(field);
              const color = FIELD_COLORS[index % FIELD_COLORS.length];
              const isActive = !(state.disabledFieldLayers?.includes(field.fieldId) ?? false);
              return (
                <label
                  key={field.fieldId}
                  className="cc-field-layer cc-layer-row-simple"
                  aria-label={`Field layer ${display.primaryLabel}`}
                  title={display.provenance}
                >
                  <span className="cc-field-dot" style={{ backgroundColor: color }} />
                  <span className="cc-field-name">{display.primaryLabel}</span>
                  <div className="cc-toggle">
                    <input
                      type="checkbox"
                      aria-label={`Toggle field layer ${display.primaryLabel}`}
                      checked={isActive}
                      onChange={() => state.toggleFieldLayer?.(field.fieldId)}
                    />
                    <span className="cc-toggle-slider" />
                  </div>
                </label>
              );
            })
          ) : (
            <UnavailableLayerRow
              label="No config"
              reason={fields.length > 0
                ? 'Only summary heat/waste fields were received; configured field layers are absent from the debug projection.'
                : 'No source-backed configured Field projection loaded'}
            />
          )}
        </section>

        <section className="cc-layers-section">
          <LayerGroupHeader
            label="Resources"
            checked={resourcesGroupEnabled}
            disabled={resourceLayerIndices.length === 0}
            onChange={() => state.setResourceLayersEnabled?.(resourceLayerIndices, !resourcesGroupEnabled)}
          />
          {resourceLayers.length > 0 ? (
            resourceLayers.map((layer) => {
              const isActive = state.activeResourceLayers?.includes(layer.layerIndex) ?? true;
              const color = RESOURCE_COLORS[layer.layerIndex % RESOURCE_COLORS.length];
              const display = buildResourceLayerDisplay(layer);
              return (
                <label
                  key={layer.layerIndex}
                  className="cc-field-layer cc-layer-row-simple"
                  aria-label={`Resource layer ${display.primaryLabel}`}
                  title={display.provenance}
                >
                  <span className="cc-field-dot" style={{ backgroundColor: color }} />
                  <span className="cc-field-name">{display.primaryLabel}</span>
                  <div className="cc-toggle">
                    <input
                      type="checkbox"
                      aria-label={`Toggle resource layer ${display.primaryLabel}`}
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
          <LayerGroupHeader
            label="Visual Effects"
            checked={visualEffectsGroupEnabled}
            onChange={() => state.setVisualEffectsEnabled?.(!visualEffectsGroupEnabled)}
          />
          <VisualEffectRow
            icon="▦"
            label="World Grid"
            checked={state.visualEffects?.showWorldGrid ?? false}
            onChange={() => state.toggleVisualEffect?.('showWorldGrid')}
          />
          <VisualEffectRow
            icon="🌌"
            label="Nebula Resource Glow"
            checked={state.visualEffects?.showNebula ?? false}
            onChange={() => state.toggleVisualEffect?.('showNebula')}
          />
          <VisualEffectRow
            icon="💫"
            label="Stardust Particles"
            checked={state.visualEffects?.showParticles ?? false}
            onChange={() => state.toggleVisualEffect?.('showParticles')}
          />
          <VisualEffectRow
            icon="🕸️"
            label="Resource Filaments"
            checked={state.visualEffects?.showFilaments ?? false}
            onChange={() => state.toggleVisualEffect?.('showFilaments')}
          />
          <VisualEffectRow
            icon="🚩"
            label="Phenotype Traits"
            checked={state.visualEffects?.showPhenotypeTraits ?? false}
            onChange={() => state.toggleVisualEffect?.('showPhenotypeTraits')}
          />
          <VisualEffectRow
            icon="✨"
            label="Division Flash FX"
            checked={state.visualEffects?.showDivisionFlash ?? false}
            onChange={() => state.toggleVisualEffect?.('showDivisionFlash')}
          />
          <VisualEffectRow
            icon="🔬"
            label="Organelle Structure"
            checked={state.visualEffects?.showOrganelles ?? false}
            onChange={() => state.toggleVisualEffect?.('showOrganelles')}
          />
          <VisualEffectRow
            icon="🧬"
            label="Organism Organic Hulls"
            checked={state.visualEffects?.showOrganismHulls ?? false}
            onChange={() => state.toggleVisualEffect?.('showOrganismHulls')}
          />
          <VisualEffectRow
            icon="⚡"
            label="Animated Joint Pulses"
            checked={state.visualEffects?.showJointPulses ?? false}
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

function LayerGroupHeader({
  label,
  checked,
  disabled,
  onChange
}: {
  label: string;
  checked: boolean;
  disabled?: boolean;
  onChange: () => void;
}) {
  return (
    <label className="cc-layer-group-header">
      <span className="cc-layers-section-label">{label}</span>
      <div className="cc-toggle">
        <input
          type="checkbox"
          aria-label={`Toggle ${label} group`}
          checked={checked}
          disabled={disabled}
          onChange={onChange}
        />
        <span className="cc-toggle-slider" />
      </div>
    </label>
  );
}

function isConfiguredFieldLayer(field: DebugField) {
  const canonicalName = field.fieldId.split('.').at(-1)?.toLowerCase() ?? field.fieldId.toLowerCase();
  return !FRAME_SUMMARY_FIELD_IDS.has(canonicalName);
}

function VisualEffectRow({
  icon,
  label,
  checked,
  onChange
}: {
  icon: string;
  label: string;
  checked: boolean;
  onChange: () => void;
}) {
  return (
    <label className="cc-field-layer cc-effect-row" aria-label={`Visual effect ${label}`}>
      <span className="cc-layer-icon" aria-hidden="true">{icon}</span>
      <span className="cc-field-name">{label}</span>
      <div className="cc-toggle">
        <input
          type="checkbox"
          aria-label={`Toggle visual effect ${label}`}
          checked={checked}
          onChange={onChange}
        />
        <span className="cc-toggle-slider" />
      </div>
    </label>
  );
}
