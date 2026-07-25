import type { AppState } from '../app/appState';
import { extractSpecializationSummary } from '../app/specializationModel';

export interface SpecializationWorkspaceProps {
  state: AppState;
}

export function SpecializationWorkspace({ state }: SpecializationWorkspaceProps) {
  const summary = extractSpecializationSummary(state.frame);

  return (
    <section className="specialization-workspace" aria-label="Specialization Analytics" style={{ color: '#dce6f1' }}>
      <header style={{ marginBottom: '20px', display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
        <div>
          <h2 style={{ margin: 0, fontSize: '18px', fontWeight: 600 }}>Specialization Analytics (AL-007-S18)</h2>
          <p style={{ margin: '4px 0 0 0', fontSize: '13px', color: '#9bb0c1' }}>
            Functional role analytics, classifier confidence, concentration ratio, and provenance metadata.
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
          Observer Heuristics (No Selection Authority)
        </span>
      </header>

      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))', gap: '16px', marginBottom: '24px' }}>
        <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
          <span style={{ fontSize: '12px', color: '#9bb0c1' }}>Total Population</span>
          <div style={{ fontSize: '24px', fontWeight: 700, marginTop: '4px' }}>{summary.totalCells}</div>
        </div>
        <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
          <span style={{ fontSize: '12px', color: '#9bb0c1' }}>Dominant Role</span>
          <div style={{ fontSize: '24px', fontWeight: 700, marginTop: '4px', color: '#58a6ff', textTransform: 'capitalize' }}>
            {summary.dominantRole}
          </div>
        </div>
        <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
          <span style={{ fontSize: '12px', color: '#9bb0c1' }}>Specialization Index (HHI)</span>
          <div style={{ fontSize: '24px', fontWeight: 700, marginTop: '4px', color: '#2ea44f' }}>
            {summary.specializationIndex.toFixed(2)}
          </div>
        </div>
        <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
          <span style={{ fontSize: '12px', color: '#9bb0c1' }}>Classifier Confidence</span>
          <div style={{ fontSize: '24px', fontWeight: 700, marginTop: '4px', color: '#e3b341' }}>
            {(summary.overallConfidence * 100).toFixed(0)}%
          </div>
        </div>
      </div>

      <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
        <h3 style={{ margin: '0 0 12px 0', fontSize: '15px' }}>Functional Role Classifiers & Energy Distribution</h3>
        {summary.roles.length === 0 ? (
          <p style={{ fontSize: '13px', color: '#9bb0c1' }}>No cells present in current frame.</p>
        ) : (
          <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: '13px' }}>
            <thead>
              <tr style={{ borderBottom: '1px solid rgba(255,255,255,0.15)', textAlign: 'left' }}>
                <th style={{ padding: '8px' }}>Role Classification</th>
                <th style={{ padding: '8px' }}>Cell Count</th>
                <th style={{ padding: '8px' }}>Share (%)</th>
                <th style={{ padding: '8px' }}>Average Energy</th>
                <th style={{ padding: '8px' }}>Confidence Score</th>
                <th style={{ padding: '8px' }}>Classifier Provenance</th>
              </tr>
            </thead>
            <tbody>
              {summary.roles.map((group) => (
                <tr key={group.role} style={{ borderBottom: '1px solid rgba(255,255,255,0.05)' }}>
                  <td style={{ padding: '8px', fontWeight: 600, textTransform: 'capitalize' }}>{group.role}</td>
                  <td style={{ padding: '8px' }}>{group.count}</td>
                  <td style={{ padding: '8px' }}>{group.percentage.toFixed(1)}%</td>
                  <td style={{ padding: '8px' }}>{group.avgEnergy.toFixed(2)}</td>
                  <td style={{ padding: '8px' }}>
                    <span style={{ color: group.confidenceScore > 0.8 ? '#2ea44f' : '#e3b341' }}>
                      {(group.confidenceScore * 100).toFixed(0)}%
                    </span>
                  </td>
                  <td style={{ padding: '8px', color: '#9bb0c1' }}>{group.provenance}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </section>
  );
}
