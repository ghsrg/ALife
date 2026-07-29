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

    
    expect(screen.getByLabelText('Monitor workspace')).toBeVisible();
    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer', { exact: true })).toHaveAttribute('data-ready', 'true');
    });
    expect(screen.getByLabelText('World stats')).toBeVisible();
  });

  it('exposes the stable monitor map track without workspace tabs', async () => {
    const store = createAppStore();
    const state = store.getState();

    renderApp(
      <MonitorWorkspace
        state={state}
        monitorStats={buildMonitorStats(state.frame, 'fixture-offline')}
        onScenarioChange={vi.fn()}
        onReconnect={vi.fn()}
        onSelectCell={vi.fn()}
        onExportScreenshot={vi.fn()}
        exportStatus={null}
      />
    );

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer', { exact: true })).toHaveAttribute('data-ready', 'true');
    });

    expect(screen.getByTestId('monitor-map-track')).toBeVisible();
    expect(screen.queryByRole('button', { name: 'Analytics' })).not.toBeInTheDocument();
  });

  it('reports screenshot export as unavailable when the viewer cannot provide a PNG', async () => {
    const onExportScreenshot = vi.fn();
    const liveFrame = {
      schemaVersion: 'WorldFrameProjection/v1' as const,
      source: 'live' as const,
      runId: 'run-1',
      tick: 6,
      world: {
        width: 100,
        height: 80
      },
      resources: [],
      cells: []
    };
    const store = createAppStore(liveFrame);
    const state = store.getState();
    mockRenderer.exportPng.mockReturnValueOnce(null);

    renderApp(
      <MonitorWorkspace
        state={state}
        monitorStats={buildMonitorStats(state.frame, 'live')}
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

  it('requests Start full-screen mode from the monitor workspace', async () => {
    const store = createAppStore();
    const state = store.getState();
    const requestFullscreen = vi.fn();

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

    const workspace = screen.getByLabelText('Monitor workspace');
    Object.defineProperty(workspace, 'requestFullscreen', {
      configurable: true,
      value: requestFullscreen
    });

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer', { exact: true })).toHaveAttribute('data-ready', 'true');
    });

    screen.getByRole('button', { name: 'Enter Start full screen' }).click();

    expect(requestFullscreen).toHaveBeenCalledTimes(1);
  });

  it('shows projection context controls and unavailable tick state without substituting the displayed frame', async () => {
    const liveFrame = {
      schemaVersion: 'WorldFrameProjection/v1' as const,
      source: 'live' as const,
      runId: 'run-1',
      tick: 6,
      world: {
        width: 100,
        height: 80
      },
      resources: [],
      cells: []
    };
    const store = createAppStore(liveFrame);
    const onFreezeFrame = vi.fn();
    const onJumpToLive = vi.fn();
    const onSelectHistoryTick = vi.fn();

    store.getState().selectHistoryTick(404);
    const state = store.getState();

    renderApp(
      <MonitorWorkspace
        state={state}
        monitorStats={buildMonitorStats(state.frame, 'live')}
        onScenarioChange={vi.fn()}
        onReconnect={vi.fn()}
        onSelectCell={vi.fn()}
        onToggleTheme={vi.fn()}
        onExportScreenshot={vi.fn()}
        onFreezeFrame={onFreezeFrame}
        onJumpToLive={onJumpToLive}
        onSelectHistoryTick={onSelectHistoryTick}
        exportStatus={null}
      />
    );

    expect(screen.getByLabelText('Data Context')).toHaveTextContent('404');
    expect(screen.getByRole('alert')).toHaveTextContent('Tick is outside bounded client history');
    expect(screen.getByRole('button', { name: 'Freeze current frame' })).toBeVisible();
    expect(screen.getByRole('button', { name: 'Jump to Live' })).toBeVisible();

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer', { exact: true })).toHaveAttribute('data-ready', 'true');
    });

    screen.getByRole('button', { name: 'Freeze current frame' }).click();
    screen.getByRole('button', { name: 'Jump to Live' }).click();
    screen.getByRole('button', { name: 'Inspect Tick 6' }).click();

    expect(onFreezeFrame).toHaveBeenCalledTimes(1);
    expect(onJumpToLive).toHaveBeenCalledTimes(1);
    expect(onSelectHistoryTick).toHaveBeenCalledWith(6);
  });
});
