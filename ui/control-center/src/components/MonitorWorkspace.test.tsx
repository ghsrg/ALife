import { beforeEach, describe, expect, it, vi } from 'vitest';
import { screen, waitFor } from '@testing-library/react';
import { renderApp } from '../test/render';
import { createAppStore } from '../app/appState';
import { buildMonitorStats } from './monitorStats';
import { MonitorWorkspace } from './MonitorWorkspace';

const mockRenderer = vi.hoisted(() => ({
  exportPng: vi.fn<() => string | null>(() => 'data:image/png;base64,fixture')
}));

vi.mock('../viewer/worldRenderer', () => ({
  mountWorldRenderer: vi.fn(() => Promise.resolve({
    renderFrame: vi.fn(),
    resize: vi.fn(),
    exportPng: mockRenderer.exportPng,
    destroy: vi.fn()
  }))
}));

describe('MonitorWorkspace', () => {
  beforeEach(() => {
    mockRenderer.exportPng.mockReset();
    mockRenderer.exportPng.mockReturnValue('data:image/png;base64,fixture');
  });

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

  it('reports screenshot export as unavailable when the viewer cannot provide a PNG', async () => {
    const onExportScreenshot = vi.fn();
    const store = createAppStore();
    const state = store.getState();
    mockRenderer.exportPng.mockReturnValueOnce(null);

    renderApp(
      <MonitorWorkspace
        state={state}
        monitorStats={buildMonitorStats(state.frame, 'fixture-offline')}
        onScenarioChange={vi.fn()}
        onReconnect={vi.fn()}
        onSelectCell={vi.fn()}
        onToggleTheme={vi.fn()}
        onExportScreenshot={onExportScreenshot}
        exportStatus={null}
      />
    );

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer', { exact: true })).toHaveAttribute('data-ready', 'true');
    });

    screen.getByRole('button', { name: /export viewer png/i }).click();

    expect(onExportScreenshot).toHaveBeenCalledWith(null);
  });
});
