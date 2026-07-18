import type { CellProjection } from '../projection/types';
import { uiText } from '../uiText';

interface CellInspectorProps {
  selectedCell: CellProjection | null;
}

export function CellInspector({ selectedCell }: CellInspectorProps) {
  return (
    <aside className="side-panel inspector" aria-label={uiText.inspector.title}>
      <h2>{uiText.inspector.title}</h2>
      {selectedCell ? (
        <div className="metric-list">
          <div><span>{uiText.inspector.id}</span><strong>{selectedCell.id}</strong></div>
          <div><span>{uiText.inspector.energy}</span><strong>{formatRatio(selectedCell.energy)}</strong></div>
          <div><span>{uiText.inspector.integrity}</span><strong>{formatRatio(selectedCell.integrity)}</strong></div>
          <div><span>{uiText.inspector.generation}</span><strong>{selectedCell.generation}</strong></div>
          <div><span>{uiText.inspector.roleHint}</span><strong>{selectedCell.roleHint}</strong></div>
        </div>
      ) : (
        <p className="empty-state">{uiText.inspector.emptyCell}</p>
      )}
    </aside>
  );
}

function formatRatio(value: number) {
  return `${Math.round(value * 100)}%`;
}
