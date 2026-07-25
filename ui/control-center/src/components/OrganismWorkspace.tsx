import { useState } from 'react';
import type { AppState } from '../app/appState';
import { extractOrganismClusters, type OrganismCluster } from '../app/organismModel';

export interface OrganismWorkspaceProps {
  state: AppState;
  onSelectCell?: (cellId: string) => void;
}

export function OrganismWorkspace({ state, onSelectCell }: OrganismWorkspaceProps) {
  const clusters = extractOrganismClusters(state.frame);
  const [selectedOrganismId, setSelectedOrganismId] = useState<string | null>(
    clusters.length > 0 ? clusters[0].organismId : null
  );

  const selectedCluster = clusters.find((c) => c.organismId === selectedOrganismId) ?? clusters[0];

  return (
    <section className="organism-workspace" aria-label="Organism Observatory" style={{ color: '#dce6f1' }}>
      <header style={{ marginBottom: '20px', display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
        <div>
          <h2 style={{ margin: 0, fontSize: '18px', fontWeight: 600 }}>Organism Observatory (AL-007-S17)</h2>
          <p style={{ margin: '4px 0 0 0', fontSize: '13px', color: '#9bb0c1' }}>
            Observer-only graph clusters, cell-joint topology, and multi-cell organism structures.
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
          Observer-Only Projections (No Physics Authority)
        </span>
      </header>

      <div style={{ display: 'grid', gridTemplateColumns: '320px 1fr', gap: '20px' }}>
        {/* Organisms List */}
        <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
          <h3 style={{ margin: '0 0 12px 0', fontSize: '14px', textTransform: 'uppercase', color: '#9bb0c1' }}>
            Detected Organisms ({clusters.length})
          </h3>
          {clusters.length === 0 ? (
            <p style={{ fontSize: '13px', color: '#9bb0c1' }}>No organisms found in current world frame.</p>
          ) : (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
              {clusters.map((org) => {
                const isSelected = selectedCluster?.organismId === org.organismId;
                return (
                  <button
                    key={org.organismId}
                    type="button"
                    onClick={() => setSelectedOrganismId(org.organismId)}
                    style={{
                      display: 'flex',
                      justifyContent: 'space-between',
                      alignItems: 'center',
                      padding: '10px 12px',
                      borderRadius: '6px',
                      background: isSelected ? 'rgba(56, 139, 253, 0.2)' : '#232931',
                      border: isSelected ? '1px solid #58a6ff' : '1px solid rgba(255,255,255,0.05)',
                      color: '#fff',
                      cursor: 'pointer',
                      textAlign: 'left'
                    }}
                  >
                    <div>
                      <strong style={{ fontSize: '13px', display: 'block' }}>{org.organismId}</strong>
                      <span style={{ fontSize: '11px', color: '#9bb0c1' }}>
                        {org.cellCount > 1 ? `${org.cellCount}-cell Multi-organism` : 'Single Cell'}
                      </span>
                    </div>
                    <span style={{ fontSize: '12px', background: 'rgba(255,255,255,0.1)', padding: '2px 6px', borderRadius: '4px' }}>
                      {org.jointCount} joints
                    </span>
                  </button>
                );
              })}
            </div>
          )}
        </div>

        {/* Selected Organism Detail Panel */}
        {selectedCluster ? (
          <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '16px', borderBottom: '1px solid rgba(255,255,255,0.1)', paddingBottom: '12px' }}>
              <div>
                <h3 style={{ margin: 0, fontSize: '16px' }}>Organism: {selectedCluster.organismId}</h3>
                <span style={{ fontSize: '12px', color: '#9bb0c1' }}>Root Cell: {selectedCluster.rootCellId}</span>
              </div>
              <div style={{ fontSize: '13px' }}>
                Total Energy: <strong>{selectedCluster.totalEnergy.toFixed(2)}</strong>
              </div>
            </div>

            <div style={{ marginBottom: '20px' }}>
              <h4 style={{ margin: '0 0 8px 0', fontSize: '13px', color: '#9bb0c1' }}>Cell Roles Breakdown</h4>
              <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
                {Object.entries(selectedCluster.roleCounts).map(([role, count]) => (
                  <span
                    key={role}
                    style={{
                      fontSize: '12px',
                      background: 'rgba(46, 164, 79, 0.2)',
                      color: '#3fb950',
                      border: '1px solid rgba(46, 164, 79, 0.4)',
                      padding: '4px 10px',
                      borderRadius: '12px'
                    }}
                  >
                    {role}: {count}
                  </span>
                ))}
              </div>
            </div>

            <div>
              <h4 style={{ margin: '0 0 8px 0', fontSize: '13px', color: '#9bb0c1' }}>Member Cells ({selectedCluster.cellIds.length})</h4>
              <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
                {selectedCluster.cellIds.map((cellId) => (
                  <button
                    key={cellId}
                    type="button"
                    onClick={() => onSelectCell?.(cellId)}
                    style={{
                      fontSize: '12px',
                      padding: '4px 8px',
                      borderRadius: '4px',
                      background: '#232931',
                      border: '1px solid rgba(255,255,255,0.1)',
                      color: '#58a6ff',
                      cursor: 'pointer'
                    }}
                  >
                    {cellId}
                  </button>
                ))}
              </div>
            </div>
          </div>
        ) : null}
      </div>
    </section>
  );
}
