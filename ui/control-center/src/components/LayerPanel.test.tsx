import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { createAppStore } from '../app/appState';
import { buildMonitorViewModel } from '../app/monitorViewModel';
import { type AnalysisLevel } from './LevelPanel';
import { LayerPanel } from './LayerPanel';

function renderLayerPanel(options: { withResourceProjection?: boolean; activeLevel?: AnalysisLevel } = {}) {
  const store = createAppStore();
  store.getState().setConnected({
    engineVersion: 'engine-test',
    apiVersion: 'runner-test',
    allowRemoteViewer: false
  });
  store.getState().setScenarios([{ id: 'living_patchy_world', path: 'config/scenarios/living_patchy_world.toml' }]);
  if (options.withResourceProjection) {
    store.getState().setFrame({ ...store.getState().frame, source: 'live' });
    store.getState().setDebugProjections({
      status: 'available',
      runId: store.getState().frame.runId,
      tick: store.getState().frame.tick,
      visualWorld: {
        projectionKind: 'VisualWorldProjection',
        completeness: { state: 'bounded', missingFields: [], reason: null },
        cells: [],
        resourceLayers: [
          {
            layerIndex: 0,
            resourceTypeId: 0,
            resourceId: 'amino_acid',
            width: 1,
            height: 1,
            totalAmount: 10,
            cells: [{ x: 0, y: 0, amount: 10 }],
            completeness: {
              state: 'bounded',
              missingFields: [],
              reason: 'CommittedSnapshot exposes resource grid cells for this bounded world.'
            }
          },
          {
            layerIndex: 18,
            resourceTypeId: 18,
            resourceId: 'nucleotide_precursor',
            width: 1,
            height: 1,
            totalAmount: 18,
            cells: [{ x: 0, y: 0, amount: 18 }],
            completeness: {
              state: 'bounded',
              missingFields: [],
              reason: 'CommittedSnapshot exposes resource grid cells for this bounded world.'
            }
          }
        ],
        fields: [
          {
            fieldId: 'CommittedSnapshot.heat',
            value: 10,
            sourceMetric: {
              fieldId: 'CommittedSnapshot.heat',
              sourceOwner: 'WorldFrameProjection',
              sourcePath: 'VisualWorldProjection.fields.CommittedSnapshot.heat'
            }
          },
          {
            fieldId: 'CommittedSnapshot.waste',
            value: 0,
            sourceMetric: {
              fieldId: 'CommittedSnapshot.waste',
              sourceOwner: 'WorldFrameProjection',
              sourcePath: 'VisualWorldProjection.fields.CommittedSnapshot.waste'
            }
          }
        ],
        fieldLayers: [
          {
            fieldId: 'temperature',
            width: 1,
            height: 1,
            summaryValue: 21,
            cells: [{ x: 0, y: 0, value: 21 }],
            completeness: { state: 'bounded', missingFields: [], reason: null }
          },
          {
            fieldId: 'light',
            width: 1,
            height: 1,
            summaryValue: 64,
            cells: [{ x: 0, y: 0, value: 64 }],
            completeness: { state: 'bounded', missingFields: [], reason: null }
          }
        ],
        sourceMetrics: []
      },
      coverage: { projectionKind: 'CoverageProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, mechanisms: [] },
      warnings: { projectionKind: 'WarningProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, warnings: [] },
      classifications: { projectionKind: 'ClassificationProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, classifications: [] },
      balanceFindings: { projectionKind: 'BalanceFindingProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, findings: [] }
    });
  }
  const state = store.getState();

  render(
    <LayerPanel
      state={state}
      monitorViewModel={buildMonitorViewModel(state)}
      activeLevel={options.activeLevel ?? 'world'}
    />
  );

  return { store };
}

describe('LayerPanel', () => {
  it('does not render Runner connection controls inside Layers & Filters', () => {
    renderLayerPanel();

    const panel = screen.getByTestId('monitor-layers-track');

    expect(panel).toHaveTextContent('LAYERS & FILTERS');
    expect(panel).not.toHaveTextContent('Runner:');
    expect(panel).not.toHaveTextContent('Reconnect');
    expect(panel).not.toHaveTextContent('http://127.0.0.1:8080');
  });

  it('keeps resource layer toggles as map presentation state only', () => {
    const { store } = renderLayerPanel({ withResourceProjection: true });
    const selectedBefore = store.getState().selectedCellId;
    const tickBefore = store.getState().frame.tick;
    const connectionBefore = store.getState().connectionState;

    screen.getByLabelText('Resource layer amino_acid').click();

    expect(store.getState().activeResourceLayers).not.toContain(0);
    expect(store.getState().selectedCellId).toBe(selectedBefore);
    expect(store.getState().frame.tick).toBe(tickBefore);
    expect(store.getState().connectionState).toBe(connectionBefore);
  });

  it('renders compact source-backed layer rows without verbose provenance as primary text', () => {
    renderLayerPanel({ withResourceProjection: true });

    const panel = screen.getByTestId('monitor-layers-track');
    const fieldRow = screen.getByLabelText('Field layer Temperature');
    const resourceRow = screen.getByLabelText('Resource layer amino_acid');

    expect(fieldRow).toHaveTextContent('Temperature');
    expect(fieldRow).not.toHaveTextContent('CommittedSnapshot.temperature');
    expect(resourceRow).toHaveTextContent('amino_acid');
    expect(screen.getByLabelText('Resource layer nucleotide_precursor')).toBeInTheDocument();
    expect(resourceRow).not.toHaveTextContent('10 total');
    expect(resourceRow).not.toHaveTextContent('CommittedSnapshot exposes resource grid cells');
    expect(panel).not.toHaveTextContent('VisualWorldProjection.fields.CommittedSnapshot.temperature');
  });

  it('renders group switches for Fields, Resources, and Visual Effects', () => {
    const { store } = renderLayerPanel({ withResourceProjection: true });

    const fieldsGroup = screen.getByLabelText('Toggle Fields group');
    const resourcesGroup = screen.getByLabelText('Toggle Resources group');
    const visualEffectsGroup = screen.getByLabelText('Toggle Visual Effects group');

    expect(fieldsGroup).toBeChecked();
    expect(resourcesGroup).toBeChecked();
    expect(visualEffectsGroup).not.toBeChecked();

    resourcesGroup.click();
    expect(store.getState().activeResourceLayers).toEqual([]);

    visualEffectsGroup.click();
    expect(Object.values(store.getState().visualEffects).every((value) => value === true)).toBe(true);
    expect(store.getState().visualEffects.showWorldGrid).toBe(true);

    fieldsGroup.click();
    expect(store.getState().disabledFieldLayers).toEqual(['temperature', 'light']);
  });

  it('renders Fields as configured named scalar layers with color and switches', () => {
    const { store } = renderLayerPanel({ withResourceProjection: true });

    const panel = screen.getByTestId('monitor-layers-track');
    expect(panel).not.toHaveTextContent('Heat');
    expect(panel).not.toHaveTextContent('Waste');

    const temperatureRow = screen.getByLabelText('Field layer Temperature');
    const lightRow = screen.getByLabelText('Field layer Light');
    const temperatureToggle = screen.getByLabelText('Toggle field layer Temperature');

    expect(temperatureRow).toHaveTextContent('Temperature');
    expect(lightRow).toHaveTextContent('Light');
    expect(temperatureRow.querySelector('.cc-field-dot')).toBeInTheDocument();
    expect(temperatureToggle).toBeChecked();

    temperatureToggle.click();

    expect(store.getState().disabledFieldLayers).toContain('temperature');
  });

  it('renders Visual Effects as icon, name, and switch rows without corrupt glyph text', () => {
    renderLayerPanel({ withResourceProjection: true });

    const nebulaRow = screen.getByLabelText('Visual effect Nebula Resource Glow');
    const particlesRow = screen.getByLabelText('Visual effect Stardust Particles');
    const worldGridRow = screen.getByLabelText('Visual effect World Grid');

    expect(nebulaRow).toHaveTextContent('Nebula Resource Glow');
    expect(particlesRow).toHaveTextContent('Stardust Particles');
    expect(worldGridRow).toHaveTextContent('World Grid');
    expect(nebulaRow).not.toHaveTextContent('рџ');
    expect(screen.getByLabelText('Toggle visual effect Nebula Resource Glow')).toBeInTheDocument();
    expect(screen.getByLabelText('Toggle visual effect World Grid')).not.toBeChecked();
  });

  it('hides cell-specific controls outside Cells and Organisms levels', () => {
    renderLayerPanel({ activeLevel: 'world' });

    const panel = screen.getByTestId('monitor-layers-track');

    expect(panel).not.toHaveTextContent('Cell Energy');
    expect(panel).not.toHaveTextContent('Joints');
    expect(panel).not.toHaveTextContent('Trail');
  });

  it('renders canonical source-backed layer groups without tabs or color mode', () => {
    renderLayerPanel({ activeLevel: 'cells' });

    const panel = screen.getByTestId('monitor-layers-track');

    expect(panel).toHaveTextContent('Fields');
    expect(panel).toHaveTextContent('Resources');
    expect(panel).toHaveTextContent('Cell Energy');
    expect(panel).toHaveTextContent('Structure');
    expect(panel).toHaveTextContent('Selection');
    expect(panel).not.toHaveTextContent('SCENE');
    expect(panel).not.toHaveTextContent('DATA');
    expect(panel).not.toHaveTextContent('COLOR MODE');
  });

  it('keeps the dynamic Fields and Resources list as the only local scroll container', () => {
    renderLayerPanel();

    const scroller = screen.getByTestId('layers-dynamic-scroll');
    expect(scroller).toHaveTextContent('Fields');
    expect(scroller).toHaveTextContent('Resources');
  });

  it('does not render fake resource presets when source projections are unavailable', () => {
    renderLayerPanel();

    const panel = screen.getByTestId('monitor-layers-track');
    expect(panel).not.toHaveTextContent('Nutrient / Organic');
    expect(panel).not.toHaveTextContent('Mineral');
    expect(panel).not.toHaveTextContent('Energy Level');
    expect(panel).toHaveTextContent('Resource layers unavailable');
  });

  it('explains when only summary heat and waste fields were received', () => {
    renderLayerPanel();

    const panel = screen.getByTestId('monitor-layers-track');
    expect(panel).toHaveTextContent('No config');
  });
});
