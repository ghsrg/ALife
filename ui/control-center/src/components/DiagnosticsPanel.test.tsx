import { describe, expect, it, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { DiagnosticsPanel } from './DiagnosticsPanel';
import { createAppStore } from '../app/appState';

describe('DiagnosticsPanel', () => {
  it('renders diagnostics information and triggers actions', async () => {
    const store = createAppStore();
    const state = store.getState();
    const user = userEvent.setup();

    const onReconnect = vi.fn();
    const onRefreshProjections = vi.fn();

    render(
      <DiagnosticsPanel
        appState={state}
        monitorDataState="fixture-idle"
        serverInfo={{ engineVersion: '0.1.0', apiVersion: '0.1.0', allowRemoteViewer: false }}
        scenarios={[{ id: 'bootstrap_minimal_viable_world', path: 'bootstrap/minimal.toml' }]}
        endpoint="ws://localhost:3000/ws"
        connectionState="connected"
        onReconnect={onReconnect}
        onRefreshProjections={onRefreshProjections}
      />
    );

    expect(screen.getByText(/System Diagnostics & Recovery/i)).toBeInTheDocument();
    expect(screen.getByText(/Core API: v0.1.0/i)).toBeInTheDocument();
    expect(screen.getByText(/bootstrap_minimal_viable_world/i)).toBeInTheDocument();

    const exportBtn = screen.getByRole('button', { name: /Export Diagnostics \(JSON\)/i });
    expect(exportBtn).toBeInTheDocument();

    const reconnectBtn = screen.getByRole('button', { name: /Soft Reconnect/i });
    await user.click(reconnectBtn);
    expect(onReconnect).toHaveBeenCalled();
  });
});
