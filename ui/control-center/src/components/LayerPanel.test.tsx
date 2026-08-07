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
    const fieldRow = screen.getByLabelText('Field layer Heat');
    const resourceRow = screen.getByLabelText('Resource layer amino_acid');

    expect(fieldRow).toHaveTextContent('Heat');
    expect(fieldRow).not.toHaveTextContent('CommittedSnapshot.heat');
    expect(resourceRow).toHaveTextContent('amino_acid');
    expect(screen.getByLabelText('Resource layer nucleotide_precursor')).toBeInTheDocument();
    expect(resourceRow).toHaveTextContent('10 total · bounded');
    expect(resourceRow).not.toHaveTextContent('CommittedSnapshot exposes resource grid cells');
    expect(panel).not.toHaveTextContent('VisualWorldProjection.fields.CommittedSnapshot.heat');
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
});
