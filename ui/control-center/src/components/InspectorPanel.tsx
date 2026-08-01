import { CellProjection } from '../projection/types';
import { CellInspector } from './CellInspector';

export interface InspectorPanelProps {
  selectedCell: CellProjection | null;
  selectionNotice?: string | null;
  displayedTick: number;
  onSelectCell?: (id: string | null) => void;
}

export function InspectorPanel({ selectedCell, selectionNotice, displayedTick, onSelectCell }: InspectorPanelProps) {
  return (
    <div className="cc-inspector" data-testid="monitor-inspector-track">
      <CellInspector selectedCell={selectedCell} selectionNotice={selectionNotice} />

      <div className="cc-inspector-footer">
        <div className="cc-inspector-live-badge">LIVE</div>
        <div className="cc-inspector-tick">{displayedTick}</div>
        <button className="cc-freeze-btn" disabled>FREEZE</button>
      </div>
    </div>
  );
}
