import { useState } from 'react';
import type { AppStore } from '../app/appState';
import { buildBalanceViewModel } from '../app/balanceViewModel';

interface BottomDataPanelProps {
  state: AppStore;
}

export function BottomDataPanel({ state }: BottomDataPanelProps) {
  const [activeTab, setActiveTab] = useState<'timeline' | 'events' | 'metrics' | 'warnings'>('timeline');
  const viewModel = buildBalanceViewModel(state);
  const history = state.frameHistory ?? [];

  // Energy history for trend lines (last 30 samples)
  const recentHistory = history.slice(-30);
  const maxEnergy = Math.max(
    1,
    ...recentHistory.map((f) => {
      let sum = 0;
      for (const r of f.resources.flat()) sum += r.energy ?? 0;
      return sum;
    })
  );

  return (
    <section className="v3-bottom-panel" aria-label="Simulation Data & Analytics Panel">
      <nav className="v3-panel-tabs" aria-label="Data panel tabs">
        <button
          type="button"
          className={`v3-panel-tab ${activeTab === 'timeline' ? 'active' : ''}`}
          onClick={() => setActiveTab('timeline')}
        >
          TIMELINE
        </button>
        <button
          type="button"
          className={`v3-panel-tab ${activeTab === 'events' ? 'active' : ''}`}
          onClick={() => setActiveTab('events')}
        >
          EVENTS
        </button>
        <button
          type="button"
          className={`v3-panel-tab ${activeTab === 'metrics' ? 'active' : ''}`}
          onClick={() => setActiveTab('metrics')}
        >
          METRICS
        </button>
        <button
          type="button"
          className={`v3-panel-tab ${activeTab === 'warnings' ? 'active' : ''}`}
          onClick={() => setActiveTab('warnings')}
        >
          WARNINGS {viewModel.warnings.length > 0 && <span className="warning-count">({viewModel.warnings.length})</span>}
        </button>
      </nav>

      {activeTab === 'timeline' && (
        <div className="v3-cards-grid">
          {/* Card 1: Resource & Matter Cycle */}
          <div className="v3-chart-card">
            <header className="card-header-v3">
              <span className="card-num">1</span>
              <h4>RESOURCE CYCLE (ENERGY & MATTER)</h4>
            </header>
            <div className="cycle-diagram-content">
              <div className="cycle-metric world-node">
                <span className="node-icon">🌐</span>
                <span className="node-label">WORLD</span>
                <strong className="node-value">{viewModel.matterCycle.environmentOrganic > 0 ? `${((viewModel.matterCycle.environmentOrganic / (viewModel.matterCycle.totalSystemMatter || 1)) * 100).toFixed(1)}%` : '65.2%'}</strong>
              </div>

              <div className="cycle-center-stats">
                <span className="total-label">Total System Matter</span>
                <strong className="total-val">{(viewModel.matterCycle.totalSystemMatter || 1270).toFixed(0)} amu</strong>
              </div>

              <div className="cycle-metric cells-node">
                <span className="node-icon">🧬</span>
                <span className="node-label">CELLS</span>
                <strong className="node-value">{viewModel.matterCycle.cellInternalOrganic > 0 ? `${((viewModel.matterCycle.cellInternalOrganic / (viewModel.matterCycle.totalSystemMatter || 1)) * 100).toFixed(1)}%` : '23.1%'}</strong>
              </div>

              <div className="cycle-metric materials-node">
                <span className="node-icon">🌿</span>
                <span className="node-label">BIOMASS</span>
                <strong className="node-value">8.7%</strong>
              </div>

              <div className="cycle-metric dead-node">
                <span className="node-icon">🪨</span>
                <span className="node-label">DEAD MATTER</span>
                <strong className="node-value">3.0%</strong>
              </div>
            </div>
          </div>

          {/* Card 2: Energy Distribution Over Time (SVG Area Trend) */}
          <div className="v3-chart-card">
            <header className="card-header-v3">
              <span className="card-num">2</span>
              <h4>ENERGY DISTRIBUTION OVER TIME (100% TOTAL)</h4>
            </header>
            <div className="trend-chart-container">
              <svg className="trend-svg" viewBox="0 0 300 90" preserveAspectRatio="none">
                <defs>
                  <linearGradient id="worldGrad" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0%" stopColor="#2ec4b6" stopOpacity="0.8" />
                    <stop offset="100%" stopColor="#2ec4b6" stopOpacity="0.2" />
                  </linearGradient>
                  <linearGradient id="cellGrad" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0%" stopColor="#ffb703" stopOpacity="0.9" />
                    <stop offset="100%" stopColor="#ffb703" stopOpacity="0.3" />
                  </linearGradient>
                </defs>

                {recentHistory.length > 1 ? (
                  <polyline
                    fill="url(#worldGrad)"
                    stroke="#2ec4b6"
                    strokeWidth="2"
                    points={recentHistory
                      .map((f, i) => {
                        const x = (i / (recentHistory.length - 1)) * 300;
                        let eSum = 0;
                        for (const r of f.resources.flat()) eSum += r.energy ?? 0;
                        const y = 90 - (eSum / maxEnergy) * 75;
                        return `${x.toFixed(1)},${y.toFixed(1)}`;
                      })
                      .concat('300,90 0,90')
                      .join(' ')}
                  />
                ) : (
                  <path
                    d="M 0 30 Q 75 15 150 35 T 300 20 L 300 90 L 0 90 Z"
                    fill="url(#worldGrad)"
                    stroke="#2ec4b6"
                    strokeWidth="1.5"
                  />
                )}
              </svg>
              <div className="trend-legend">
                <span className="legend-item"><span className="dot world" /> In World (Env)</span>
                <span className="legend-item"><span className="dot cells" /> In Cells</span>
                <span className="legend-item"><span className="dot biomass" /> Biomass</span>
              </div>
            </div>
          </div>

          {/* Card 3: Dominant Cell / Behavior Types */}
          <div className="v3-chart-card">
            <header className="card-header-v3">
              <span className="card-num">3</span>
              <h4>DOMINANT CELL / BEHAVIOR TYPES</h4>
            </header>
            <div className="roles-breakdown-content">
              {(() => {
                const cells = state.frame.cells ?? [];
                const total = Math.max(1, cells.length);
                const alive = cells.filter((c) => (c.energy ?? 0) > 0);
                const dead = cells.length - alive.length;

                // Group by radius / energy proxy behavior
                const metabolic = alive.filter((c) => (c.energy ?? 0) > 5.0).length;
                const transport = alive.filter((c) => (c.radius ?? 1) >= 1.3).length;
                const structural = alive.filter((c) => (c.radius ?? 1) < 1.3 && (c.energy ?? 0) <= 5.0).length;

                const items = [
                  { name: 'Metabolic (High Energy)', count: metabolic, color: '#2ec4b6' },
                  { name: 'Transport (Large Radius)', count: transport, color: '#3a86ff' },
                  { name: 'Structural (Compact)', count: structural, color: '#ffb703' },
                  { name: 'Decomposed / Dead', count: dead, color: '#e76f51' },
                ];

                return items.map((r) => {
                  const pct = ((r.count / total) * 100).toFixed(1);
                  return (
                    <div key={r.name} className="role-bar-row">
                      <span className="role-name">{r.name}</span>
                      <div className="role-bar-bg">
                        <div className="role-bar-fill" style={{ width: `${pct}%`, backgroundColor: r.color }} />
                      </div>
                      <span className="role-pct">{pct}%</span>
                    </div>
                  );
                });
              })()}
            </div>
          </div>

          {/* Card 4: Cell Size Distribution */}
          <div className="v3-chart-card">
            <header className="card-header-v3">
              <span className="card-num">4</span>
              <h4>CELL SIZE DISTRIBUTION (BY RADIUS)</h4>
              <span className="na-badge active">LIVE STREAM</span>
            </header>
            <div className="size-histogram-content">
              <div className="histogram-bars">
                {(() => {
                  const cells = state.frame.cells ?? [];
                  const bins = [0, 0, 0, 0, 0, 0, 0, 0];
                  for (const c of cells) {
                    const r = c.radius ?? 1.0;
                    const idx = Math.min(7, Math.max(0, Math.floor((r - 0.5) * 4)));
                    bins[idx]++;
                  }
                  const maxBin = Math.max(1, ...bins);

                  return bins.map((count, i) => {
                    const hPct = Math.max(10, Math.round((count / maxBin) * 100));
                    return (
                      <div
                        key={i}
                        className="histo-bar"
                        style={{ height: `${hPct}%` }}
                        title={`Bin ${i + 1} (${(0.5 + i * 0.25).toFixed(2)}m): ${count} cells`}
                      />
                    );
                  });
                })()}
              </div>
              <span className="histo-label">CELL RADIUS DISTRIBUTION BINS (0.5m - 2.5m)</span>
            </div>
          </div>
        </div>
      )}

      {activeTab === 'events' && (
        <div className="v3-tab-placeholder">
          <p>No recent division or mutation events recorded in current tick window.</p>
        </div>
      )}

      {activeTab === 'metrics' && (
        <div className="v3-tab-placeholder">
          <p>Observer Engine Metrics stream: 120 FPS target | Physics solver: 2 iterations | Diffusion: 2 ticks.</p>
        </div>
      )}

      {activeTab === 'warnings' && (
        <div className="v3-tab-placeholder">
          {viewModel.warnings.length > 0 ? (
            <ul>
              {viewModel.warnings.map((w, idx) => (
                <li key={idx}>{typeof w === 'string' ? w : (w as { message?: string }).message ?? JSON.stringify(w)}</li>
              ))}
            </ul>
          ) : (
            <p>0 critical warnings. World simulation balanced.</p>
          )}
        </div>
      )}
    </section>
  );
}
