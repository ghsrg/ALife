import type { AppState } from '../app/appState';
import { extractEvolutionSummary } from '../app/evolutionModel';

export interface EvolutionWorkspaceProps {
  state: AppState;
}

export function EvolutionWorkspace({ state }: EvolutionWorkspaceProps) {
  const summary = extractEvolutionSummary(state.frame);

  return (
    <section className="evolution-workspace" aria-label="Evolution Observatory" style={{ color: '#dce6f1' }}>
      <header style={{ marginBottom: '20px', display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
        <div>
          <h2 style={{ margin: 0, fontSize: '18px', fontWeight: 600 }}>Evolution Observatory (AL-007-S16)</h2>
          <p style={{ margin: '4px 0 0 0', fontSize: '13px', color: '#9bb0c1' }}>
            Observer-only lineage, generation distribution, mutation load, and diversity analytics.
          </p>
        </div>
        <span
          style={{
            fontSize: '11px',
            background: 'rgba(56, 139, 253, 0.15)',
            color: '#58a6ff',
            border: '1px solid rgba(56, 139, 253, 0.4)',
            padding: '4px 8px',
            borderRadius: '4px'
          }}
        >
          Observer-Only (No Selection/Genome Authority)
        </span>
      </header>

      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))', gap: '16px', marginBottom: '24px' }}>
        <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
          <span style={{ fontSize: '12px', color: '#9bb0c1' }}>Total Population</span>
          <div style={{ fontSize: '24px', fontWeight: 700, marginTop: '4px' }}>{summary.totalCells}</div>
        </div>
        <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
          <span style={{ fontSize: '12px', color: '#9bb0c1' }}>Max Generation</span>
          <div style={{ fontSize: '24px', fontWeight: 700, marginTop: '4px', color: '#2ea44f' }}>
            Gen {summary.maxGeneration}
          </div>
        </div>
        <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
          <span style={{ fontSize: '12px', color: '#9bb0c1' }}>Avg Generation</span>
          <div style={{ fontSize: '24px', fontWeight: 700, marginTop: '4px' }}>
            {summary.avgGeneration.toFixed(1)}
          </div>
        </div>
        <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
          <span style={{ fontSize: '12px', color: '#9bb0c1' }}>Diversity Index (Shannon)</span>
          <div style={{ fontSize: '24px', fontWeight: 700, marginTop: '4px', color: '#e3b341' }}>
            {summary.shannonDiversityIndex.toFixed(2)}
          </div>
        </div>
      </div>

      <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
        <h3 style={{ margin: '0 0 12px 0', fontSize: '15px' }}>Lineage & Generation Distribution</h3>
        {summary.generationGroups.length === 0 ? (
          <p style={{ fontSize: '13px', color: '#9bb0c1' }}>No live cells present in current frame.</p>
        ) : (
          <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: '13px' }}>
            <thead>
              <tr style={{ borderBottom: '1px solid rgba(255,255,255,0.15)', textAlign: 'left' }}>
                <th style={{ padding: '8px' }}>Generation</th>
                <th style={{ padding: '8px' }}>Cell Count</th>
                <th style={{ padding: '8px' }}>Share (%)</th>
                <th style={{ padding: '8px' }}>Average Energy</th>
              </tr>
            </thead>
            <tbody>
              {summary.generationGroups.map((group) => (
                <tr key={group.generation} style={{ borderBottom: '1px solid rgba(255,255,255,0.05)' }}>
                  <td style={{ padding: '8px', fontWeight: 600 }}>Gen {group.generation}</td>
                  <td style={{ padding: '8px' }}>{group.count}</td>
                  <td style={{ padding: '8px' }}>{group.percentage.toFixed(1)}%</td>
                  <td style={{ padding: '8px' }}>{group.avgEnergy.toFixed(2)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </section>
  );
}
