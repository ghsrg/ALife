import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import type { ScenarioListItem, ServerInfo } from '../runner/apiClient';
import { ConnectionPanel } from './ConnectionPanel';

const serverInfo: ServerInfo = {
  engineVersion: 'core-0.1.0',
  apiVersion: '2',
  allowRemoteViewer: false
};

const scenarios: ScenarioListItem[] = [
  { id: 'demo', path: 'scenarios/demo.toml' },
  { id: 'stress', path: 'scenarios/stress.toml' }
];

describe('ConnectionPanel', () => {
  it('shows endpoint, connected API info, and scenario selection', async () => {
    const onScenarioChange = vi.fn();
    const user = userEvent.setup();

    render(
      <ConnectionPanel
        endpoint="http://127.0.0.1:8080"
        connectionState="connected"
        serverInfo={serverInfo}
        scenarios={scenarios}
        selectedScenarioId="demo"
        lastError={null}
        onScenarioChange={onScenarioChange}
      />
    );

    expect(screen.getByText('Connected')).toBeInTheDocument();
    expect(screen.getByText('http://127.0.0.1:8080')).toBeInTheDocument();
    expect(screen.getByText('API v2')).toBeInTheDocument();
    expect(screen.getByRole('combobox', { name: 'Scenario' })).toHaveValue('demo');

    await user.selectOptions(screen.getByRole('combobox', { name: 'Scenario' }), 'stress');

    expect(onScenarioChange).toHaveBeenCalledWith('stress');
  });

  it('shows disconnected state and last error', () => {
    render(
      <ConnectionPanel
        endpoint="http://127.0.0.1:8080"
        connectionState="disconnected"
        serverInfo={null}
        scenarios={[]}
        selectedScenarioId={null}
        lastError="Runner unavailable"
        onScenarioChange={vi.fn()}
      />
    );

    expect(screen.getByText('Disconnected')).toBeInTheDocument();
    expect(screen.getByText('not connected')).toBeInTheDocument();
    expect(screen.getByRole('combobox', { name: 'Scenario' })).toBeDisabled();
    expect(screen.getByRole('alert')).toHaveTextContent('Runner unavailable');
  });

  it('shows connecting state', () => {
    render(
      <ConnectionPanel
        endpoint="http://127.0.0.1:8080"
        connectionState="connecting"
        serverInfo={null}
        scenarios={[]}
        selectedScenarioId={null}
        lastError={null}
        onScenarioChange={vi.fn()}
      />
    );

    expect(screen.getByText('Connecting')).toBeInTheDocument();
  });
});
