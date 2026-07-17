import type { CellProjection } from '../projection/types';

interface CellInspectorProps {
  selectedCell: CellProjection | null;
}

export function CellInspector({ selectedCell }: CellInspectorProps) {
  return (
    <aside className="side-panel inspector" aria-label="Cell Inspector">
      <h2>Cell Inspector</h2>
      {selectedCell ? (
        <div className="metric-list">
          <div><span>ID</span><strong>{selectedCell.id}</strong></div>
          <div><span>Energy</span><strong>{formatRatio(selectedCell.energy)}</strong></div>
          <div><span>Integrity</span><strong>{formatRatio(selectedCell.integrity)}</strong></div>
          <div><span>Generation</span><strong>{selectedCell.generation}</strong></div>
          <div><span>Role hint</span><strong>{selectedCell.roleHint}</strong></div>
        </div>
      ) : (
        <p className="empty-state">No cell selected.</p>
      )}
    </aside>
  );
}

function formatRatio(value: number) {
  return `${Math.round(value * 100)}%`;
}
