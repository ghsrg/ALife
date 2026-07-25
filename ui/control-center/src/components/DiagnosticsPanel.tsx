import { useState } from 'react';
import type { AppState, ConnectionState, MonitorDataState, ScenarioListItem, ServerInfo } from '../app/appState';

export interface DiagnosticsPanelProps {
  appState: AppState;
  monitorDataState: MonitorDataState;
  serverInfo: ServerInfo | null;
  scenarios: ScenarioListItem[];
  endpoint: string;
  connectionState: ConnectionState;
  onRefreshProjections?: () => void;
  onReconnect?: () => void;
}

export function DiagnosticsPanel({
  appState,
  monitorDataState,
  serverInfo,
  scenarios,
  endpoint,
  connectionState,
  onRefreshProjections,
  onReconnect
}: DiagnosticsPanelProps) {
  const [downloadNotice, setDownloadNotice] = useState<string | null>(null);

  const exportDiagnosticReport = () => {
    const report = {
      timestamp: new Date().toISOString(),
      client_version: '0.1.0',
      core_engine_version: appState.debugProjections?.status === 'available' ? '0.1.0' : 'unknown',
      server_info: serverInfo,
      endpoint,
      connection_state: connectionState,
      monitor_data_state: monitorDataState,
      run_status: appState.runStatus,
      active_frame: {
        tick: appState.frame.tick,
        cell_count: appState.frame.cells.length,
        joint_count: appState.frame.joints?.length ?? 0,
        world_width: appState.frame.world.width,
        world_height: appState.frame.world.height
      },
      debug_projections: appState.debugProjections,
      last_error: appState.lastError
    };

    const jsonString = JSON.stringify(report, null, 2);
    const blob = new Blob([jsonString], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `alife-diagnostics-${Date.now()}.json`;
    link.click();
    URL.revokeObjectURL(url);
    setDownloadNotice(`Exported diagnostic report (${blob.size} bytes)`);
    setTimeout(() => setDownloadNotice(null), 4000);
  };

  return (
    <section className="diagnostics-panel" aria-label="Diagnostics and Recovery" style={{ padding: '16px', background: 'var(--panel-bg, #1a1e24)', borderRadius: '8px', color: '#dce6f1', marginTop: '12px' }}>
      <header style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '12px' }}>
        <h3 style={{ margin: 0, fontSize: '15px', fontWeight: 600 }}>System Diagnostics & Recovery (AL-007-S14)</h3>
        <div style={{ display: 'flex', gap: '8px' }}>
          {onRefreshProjections ? (
            <button type="button" className="secondary-action" onClick={onRefreshProjections} style={{ padding: '4px 10px', fontSize: '12px' }}>
              Refresh Projections
            </button>
          ) : null}
          {onReconnect ? (
            <button type="button" className="secondary-action" onClick={onReconnect} style={{ padding: '4px 10px', fontSize: '12px' }}>
              Soft Reconnect
            </button>
          ) : null}
          <button type="button" className="primary-action" onClick={exportDiagnosticReport} style={{ padding: '4px 10px', fontSize: '12px' }}>
            Export Diagnostics (JSON)
          </button>
        </div>
      </header>

      {downloadNotice ? (
        <div style={{ padding: '6px 10px', background: 'rgba(46, 164, 79, 0.2)', border: '1px solid #2ea44f', borderRadius: '4px', fontSize: '12px', marginBottom: '12px' }}>
          {downloadNotice}
        </div>
      ) : null}

      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '12px', fontSize: '12px' }}>
        <div style={{ background: 'rgba(255, 255, 255, 0.04)', padding: '8px 12px', borderRadius: '6px' }}>
          <strong>Runtime Versions</strong>
          <div>UI Client: v0.1.0</div>
          <div>Core API: {serverInfo ? `v${serverInfo.apiVersion}` : 'Disconnected'}</div>
          <div>Scenario Hash: {appState.runStatus?.scenarioHash ?? 'None'}</div>
        </div>

        <div style={{ background: 'rgba(255, 255, 255, 0.04)', padding: '8px 12px', borderRadius: '6px' }}>
          <strong>Stream & Projection Cadence</strong>
          <div>WS Transport: {connectionState}</div>
          <div>Data Context: {monitorDataState}</div>
          <div>Grid Stride: {appState.frame.world.width >= 120 && appState.runStatus?.activeRunState === 'running' ? '2 (Sampled 4x)' : '1 (Full 100%)'}</div>
        </div>

        <div style={{ background: 'rgba(255, 255, 255, 0.04)', padding: '8px 12px', borderRadius: '6px' }}>
          <strong>Projection Envelope</strong>
          <div>Status: {appState.debugProjections?.status ?? 'none'}</div>
          <div>Active Tick: {appState.frame.tick}</div>
          <div>Cells / Joints: {appState.frame.cells.length} / {appState.frame.joints?.length ?? 0}</div>
        </div>
      </div>

      <div style={{ marginTop: '14px' }}>
        <strong style={{ fontSize: '13px', display: 'block', marginBottom: '6px' }}>Scenario Suite Fixtures & Multi-Seed Queue Summary</strong>
        <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: '12px' }}>
          <thead>
            <tr style={{ borderBottom: '1px solid rgba(255, 255, 255, 0.1)', textAlign: 'left' }}>
              <th style={{ padding: '4px' }}>Scenario ID</th>
              <th style={{ padding: '4px' }}>Status</th>
              <th style={{ padding: '4px' }}>Type</th>
            </tr>
          </thead>
          <tbody>
            {scenarios.map((scenario) => (
              <tr key={scenario.id} style={{ borderBottom: '1px solid rgba(255, 255, 255, 0.04)' }}>
                <td style={{ padding: '4px' }}>{scenario.id}</td>
                <td style={{ padding: '4px' }}>{scenario.id === appState.runStatus?.scenarioId ? 'Active' : 'Ready'}</td>
                <td style={{ padding: '4px' }}>{scenario.id.includes('rich') ? 'Rich World' : scenario.id.includes('scale') ? 'Scale Benchmark' : 'Bootstrap'}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}
