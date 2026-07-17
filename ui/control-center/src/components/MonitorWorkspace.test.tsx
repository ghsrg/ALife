import { describe, expect, it, vi } from 'vitest';
import { screen, waitFor } from '@testing-library/react';
import { renderApp } from '../test/render';
import { createAppStore } from '../app/appState';
import { buildMonitorStats } from './monitorStats';
import { MonitorWorkspace } from './MonitorWorkspace';

vi.mock('../viewer/worldRenderer', () => ({
  mountWorldRenderer: vi.fn(() => Promise.resolve({
    renderFrame: vi.fn(),
    resize: vi.fn(),
    exportPng: vi.fn(() => 'data:image/png;base64,fixture'),
    destroy: vi.fn()
  }))
}));

describe('MonitorWorkspace', () => {
  it('composes layer controls, viewer, focus card, stats and inspector from one state object', async () => {
    const store = createAppStore();
    const state = store.getState();

    renderApp(
      <MonitorWorkspace
        state={state}
        monitorStats={buildMonitorStats(state.frame, 'fixture-offline')}
        onScenarioChange={vi.fn()}
        onReconnect={vi.fn()}
        onSelectCell={vi.fn()}
        onToggleTheme={vi.fn()}
        onExportScreenshot={vi.fn()}
        exportStatus={null}
      />
    );

    expect(screen.getByLabelText('Layer controls')).toBeVisible();
    expect(screen.getByLabelText('Monitor workspace')).toBeVisible();
    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer', { exact: true })).toHaveAttribute('data-ready', 'true');
    });
    expect(screen.getByLabelText('Cell Inspector')).toHaveTextContent('cell-a');
    expect(screen.getByLabelText('World stats')).toBeVisible();
  });
});
