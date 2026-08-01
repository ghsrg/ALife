import type { BalanceViewModel } from '../app/balanceViewModel';
import { DonutDiagram } from './charts/DonutDiagram';

interface BalanceAnalyticsPanelProps {
  viewModel: BalanceViewModel;
}

export function BalanceAnalyticsPanel({ viewModel }: BalanceAnalyticsPanelProps) {
  if (!viewModel.hasData) {
    return (
      <div className="balance-panel empty" data-testid="balance-analytics-empty">
        <p>No active live frame available for balance analytics.</p>
      </div>
    );
  }

  const { matterCycle, energyFlow, population, warnings } = viewModel;

  const matterSegments = [
    { label: 'Env Organic', value: matterCycle.environmentOrganic, color: '#00c896' },
    { label: 'Env Mineral', value: matterCycle.environmentMineral, color: '#38bdf8' },
    { label: 'Cell Internal', value: matterCycle.cellInternalOrganic + matterCycle.cellInternalMineral, color: '#a855f7' },
    { label: 'Bound Materials', value: matterCycle.cellBoundMaterials, color: '#fbbf24' }
  ];

  return (
    <div className="balance-analytics-grid" data-testid="balance-analytics-panel">
      {/* Matter Cycle Accounting */}
      <section className="analytics-card" aria-label="Matter Cycle Accounting">
        <header className="card-header">
          <h3>Matter Cycle Accounting</h3>
          <span className="unaccounted-badge" title="Unaccounted Matter Difference">
            Unaccounted: {matterCycle.unaccountedDiffLabel}
          </span>
        </header>

        <div style={{ display: 'flex', alignItems: 'center', gap: '16px', marginTop: '10px' }}>
          <DonutDiagram segments={matterSegments} size={85} thickness={12} showLegend={false} />
          <div className="metrics-row" style={{ flex: 1 }}>
            <div className="metric-box">
              <span className="label">Env Organic</span>
              <strong className="value organic">{matterCycle.environmentOrganic.toFixed(2)}</strong>
            </div>
            <div className="metric-box">
              <span className="label">Env Mineral</span>
              <strong className="value mineral">{matterCycle.environmentMineral.toFixed(2)}</strong>
            </div>
            <div className="metric-box">
              <span className="label">Internal Matter</span>
              <strong className="value internal">
                {(matterCycle.cellInternalOrganic + matterCycle.cellInternalMineral).toFixed(2)}
              </strong>
            </div>
            <div className="metric-box">
              <span className="label">Bound Materials</span>
              <strong className="value bound">{matterCycle.cellBoundMaterials.toFixed(2)}</strong>
            </div>
          </div>
        </div>
      </section>


      {/* Energy Flow & Capacity */}
      <section className="analytics-card" aria-label="Energy Utilization">
        <header className="card-header">
          <h3>Energy Utilization</h3>
          <strong className="energy-ratio">{(energyFlow.energyUtilizationRatio * 100).toFixed(1)}%</strong>
        </header>
        <div className="progress-bar-bg">
          <div
            className="progress-bar-fill energy"
            style={{ width: `${Math.min(100, energyFlow.energyUtilizationRatio * 100)}%` }}
          />
        </div>
        <div className="metrics-row compact">
          <span>Stored: {energyFlow.totalCellEnergy.toFixed(1)} EU</span>
          <span>Capacity: {energyFlow.totalSystemCapacity.toFixed(1)} EU</span>
        </div>
      </section>

      {/* Population & Lifecycle Breakdown */}
      <section className="analytics-card" aria-label="Population Lifecycle">
        <header className="card-header">
          <h3>Population Lifecycle</h3>
          <span>Total: {population.total}</span>
        </header>
        <div className="lifecycle-bar">
          <div
            className="segment alive"
            style={{ width: `${(population.alive / (population.total || 1)) * 100}%` }}
            title={`Alive: ${population.alive}`}
          />
          <div
            className="segment stressed"
            style={{ width: `${(population.stressed / (population.total || 1)) * 100}%` }}
            title={`Stressed: ${population.stressed}`}
          />
          <div
            className="segment dormant"
            style={{ width: `${(population.dormant / (population.total || 1)) * 100}%` }}
            title={`Dormant: ${population.dormant}`}
          />
          <div
            className="segment dead"
            style={{ width: `${(population.dead / (population.total || 1)) * 100}%` }}
            title={`Dead: ${population.dead}`}
          />
        </div>
        <div className="lifecycle-legend">
          <span className="legend-item alive">Alive: {population.alive}</span>
          <span className="legend-item stressed">Stressed: {population.stressed}</span>
          <span className="legend-item dormant">Dormant: {population.dormant}</span>
          <span className="legend-item dead">Dead: {population.dead}</span>
        </div>
      </section>

      {/* Engineering Warnings Table */}
      <section className="analytics-card warnings-card" aria-label="Engineering Warnings">
        <header className="card-header">
          <h3>Engineering Warnings ({warnings.length})</h3>
        </header>
        {warnings.length === 0 ? (
          <p className="clean-status">All telemetry systems nominal.</p>
        ) : (
          <ul className="warnings-list">
            {warnings.map((w) => (
              <li key={w.id} className={`warning-item ${w.severity}`}>
                <strong>{w.title}</strong>
                <span>{w.detail}</span>
              </li>
            ))}
          </ul>
        )}
      </section>
    </div>
  );
}
