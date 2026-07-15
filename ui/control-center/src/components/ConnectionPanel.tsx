import type { ScenarioListItem, ServerInfo } from '../runner/apiClient';
import type { RunnerStreamConnectionState as ConnectionState } from '../runner/streamClient';

export interface ConnectionPanelProps {
  endpoint: string;
  connectionState: ConnectionState;
  serverInfo: ServerInfo | null;
  scenarios: ScenarioListItem[];
  selectedScenarioId: string | null;
  lastError: string | null;
  onScenarioChange: (scenarioId: string) => void;
}

const connectionLabels: Record<ConnectionState, string> = {
  connected: 'Connected',
  connecting: 'Connecting',
  disconnected: 'Disconnected'
};

export function ConnectionPanel({
  endpoint,
  connectionState,
  serverInfo,
  scenarios,
  selectedScenarioId,
  lastError,
  onScenarioChange
}: ConnectionPanelProps) {
  const hasScenarios = scenarios.length > 0;

  return (
    <section className="connection-panel" aria-label="Runner connection">
      <div className="connection-status">
        <span className={`status-dot status-dot-${connectionState}`} aria-hidden="true" />
        <strong>{connectionLabels[connectionState]}</strong>
      </div>

      <div className="connection-meta">
        <span>{endpoint}</span>
        <span>{serverInfo ? `API v${serverInfo.apiVersion}` : 'not connected'}</span>
      </div>

      <label className="scenario-select">
        <span>Scenario</span>
        <select
          aria-label="Scenario"
          disabled={!hasScenarios}
          value={selectedScenarioId ?? ''}
          onChange={(event) => onScenarioChange(event.currentTarget.value)}
        >
          {hasScenarios ? null : <option value="">No scenarios</option>}
          {scenarios.map((scenario) => (
            <option key={scenario.id} value={scenario.id}>
              {scenario.id}
            </option>
          ))}
        </select>
      </label>

      {lastError ? (
        <p className="connection-error" role="alert">
          {lastError}
        </p>
      ) : null}
    </section>
  );
}
