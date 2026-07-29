import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { createAppStore } from '../app/appState';
import { buildMonitorViewModel } from '../app/monitorViewModel';
import { LayerPanel } from './LayerPanel';

function renderLayerPanel() {
  const store = createAppStore();
  store.getState().setConnected({
    engineVersion: 'engine-test',
    apiVersion: 'runner-test',
    allowRemoteViewer: false
  });
  store.getState().setScenarios([{ id: 'living_patchy_world', path: 'config/scenarios/living_patchy_world.toml' }]);
  const state = store.getState();

  render(
    <LayerPanel
      state={state}
      monitorViewModel={buildMonitorViewModel(state)}
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
    const { store } = renderLayerPanel();
    const selectedBefore = store.getState().selectedCellId;
    const tickBefore = store.getState().frame.tick;
    const connectionBefore = store.getState().connectionState;

    screen.getByLabelText('Nutrient / Organic').click();

    expect(store.getState().activeResourceLayers).not.toContain(0);
    expect(store.getState().selectedCellId).toBe(selectedBefore);
    expect(store.getState().frame.tick).toBe(tickBefore);
    expect(store.getState().connectionState).toBe(connectionBefore);
  });
});
