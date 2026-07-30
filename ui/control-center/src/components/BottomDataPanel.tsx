import type { AppStore } from '../app/appState';
import {
  buildMonitorSurfaceModel,
  type AccountingTarget,
  type AnalysisLevel,
  type MonitorSurfaceCard
} from '../app/monitorSurfaceModel';

interface BottomDataPanelProps {
  state: AppStore;
  activeLevel?: AnalysisLevel;
}

export function BottomDataPanel({ state, activeLevel = 'world' }: BottomDataPanelProps) {
  const viewModel = buildMonitorSurfaceModel(state, { activeLevel });

  return (
    <section className="v3-bottom-panel" aria-label="Simulation Data & Analytics Panel">
      <div className="v3-panel-header" aria-label="Data panel context">
        <span className="v3-panel-kicker">DATA PANEL</span>
        <strong>{levelTitle(viewModel.activeLevel)}</strong>
        {viewModel.activeLevel === 'world' ? (
          <div className="monitor-accounting-selector" aria-label="World accounting target">
            {(['Energy', 'Resource', 'Material'] satisfies AccountingTarget[]).map((target) => (
              <button
                key={target}
                type="button"
                className={viewModel.accountingTarget === target ? 'active' : ''}
                onClick={() => state.setMonitorAccountingTarget(target)}
              >
                {target}
              </button>
            ))}
            {viewModel.accountingTarget !== 'Energy' ? (
              <span className="monitor-accounting-type">
                {viewModel.accountingTypeOptions.length > 0
                  ? `Type: ${viewModel.accountingTypeOptions[0]}`
                  : 'Type unavailable'}
              </span>
            ) : null}
          </div>
        ) : null}
        {viewModel.warnings.length > 0 ? (
          <span className="warning-count">{viewModel.warnings.length} warnings</span>
        ) : null}
      </div>

      <div className="v3-cards-grid">
        {viewModel.cards.map((card, index) => (
          <SurfaceCard key={card.id} card={card} index={index} />
        ))}
      </div>
    </section>
  );
}

function SurfaceCard({ card, index }: { card: MonitorSurfaceCard; index: number }) {
  return (
    <div className={`v3-chart-card monitor-surface-card is-${card.state}`} data-card-id={card.id}>
      <header className="card-header-v3">
        <span className="card-num">{index + 1}</span>
        <h4>{card.title}</h4>
        {isPopulationLifecycleCard(card) ? (
          <span className="monitor-lifecycle-total">Total: {rowValue(card, 'Total') ?? '0'}</span>
        ) : null}
        <span className={`na-badge ${card.state === 'available' ? 'active' : ''}`}>
          {card.state === 'available' ? 'SOURCE' : card.state.toUpperCase()}
        </span>
      </header>

      <div className="monitor-card-body">
        {card.state === 'available' || card.rows.length > 0 || card.series.length > 0 ? (
          <>
            {card.subtitle ? <p className="monitor-card-subtitle">{card.subtitle}</p> : null}

            {card.rows.length > 0 && !isPopulationLifecycleCard(card) ? (
              <div className="monitor-card-rows">
                {card.rows.map((row) => (
                  <div key={row.label} className="monitor-card-row">
                    <span>{row.label}</span>
                    <strong>{row.value}</strong>
                  </div>
                ))}
              </div>
            ) : null}

            {isPopulationLifecycleCard(card) && card.series.length > 0 ? <LifecycleDistribution card={card} /> : null}

            {card.series.length > 0 && !isPopulationLifecycleCard(card) ? (
              <div className="trend-legend">
                {card.series.map((series) => (
                  <span key={series.label} className={`legend-item ${seriesClassName(series.label)}`}>
                    <span className="dot world" style={series.color ? { background: series.color } : undefined} />
                    {series.label}
                    {series.value ? <strong>{series.value}</strong> : null}
                  </span>
                ))}
              </div>
            ) : null}
          </>
        ) : (
          <div className="monitor-card-placeholder">
            <strong>Unavailable</strong>
            <span title={card.reason}>{shortReason(card.reason)}</span>
            <small title={card.source}>{card.source}</small>
          </div>
        )}
      </div>

      <div className="monitor-card-provenance compact" aria-label={`${card.title} provenance`}>
        <span className="monitor-card-provenance-chip" title={card.source}>
          src: {card.source}
        </span>
        <span className="monitor-card-provenance-chip" title={card.completeness}>
          {card.completeness}
        </span>
        {card.unit ? (
          <span className="monitor-card-provenance-chip" title={card.unit}>
            {card.unit}
          </span>
        ) : null}
      </div>
    </div>
  );
}

function LifecycleDistribution({ card }: { card: MonitorSurfaceCard }) {
  return (
    <>
      <div className="lifecycle-bar" aria-label="Population lifecycle distribution">
        {card.series.map((series) => (
          <span
            key={series.label}
            className={`segment ${seriesClassName(series.label)}`}
            style={{ width: series.value ?? '0.0%' }}
            title={`${series.label}: ${series.value ?? '0.0%'}`}
          />
        ))}
      </div>
      <div className="lifecycle-legend compact" aria-label="Population lifecycle counts">
        {card.series.map((series) => (
          <span key={series.label} className={`legend-item ${seriesClassName(series.label)}`}>
            {series.label}: {rowValue(card, series.label) ?? '0'}
          </span>
        ))}
      </div>
    </>
  );
}

function isPopulationLifecycleCard(card: MonitorSurfaceCard) {
  return card.id.endsWith('-population-lifecycle');
}

function seriesClassName(label: string) {
  return label.toLowerCase().replace(/[^a-z0-9]+/g, '-');
}

function rowValue(card: MonitorSurfaceCard, label: string) {
  return card.rows.find((row) => row.label === label)?.value;
}

function shortReason(reason?: string) {
  if (!reason) return 'Source projection is not available.';
  return reason.length > 92 ? `${reason.slice(0, 89)}...` : reason;
}

function levelTitle(level: AnalysisLevel) {
  switch (level) {
    case 'cells':
      return 'Cells Analytics';
    case 'organisms':
      return 'Organisms Analytics';
    case 'lineages':
      return 'Lineages Analytics';
    case 'evolution':
      return 'Evolution Analytics';
    case 'analytics':
      return 'Metric Analytics';
    case 'world':
    default:
      return 'World Analytics';
  }
}
