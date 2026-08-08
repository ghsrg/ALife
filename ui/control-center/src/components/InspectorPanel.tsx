import { buildWorldBlockInspection } from '../app/worldBlockInspection';
import type { MonitorSelection } from '../app/selectionModel';
import type { CellProjection, DebugProjectionState, WorldFrame } from '../projection/types';
import { CellInspector } from './CellInspector';
import { WorldBlockInspector } from './WorldBlockInspector';

export interface InspectorPanelProps {
  frame: WorldFrame;
  debugProjections: DebugProjectionState;
  currentSelection: MonitorSelection;
  selectedCell: CellProjection | null;
  selectionNotice?: string | null;
  displayedTick: number;
  onSelectCell?: (id: string | null) => void;
}

export function InspectorPanel({
  frame,
  debugProjections,
  currentSelection,
  selectedCell,
  selectionNotice,
  displayedTick,
  onSelectCell
}: InspectorPanelProps) {
  void onSelectCell;
  const worldBlockInspection = currentSelection.kind === 'world-block'
    ? buildWorldBlockInspection({ frame, debugProjections, selection: currentSelection })
    : null;

  return (
    <div className="cc-inspector" data-testid="monitor-inspector-track">
      {worldBlockInspection
        ? <WorldBlockInspector inspection={worldBlockInspection} />
        : <CellInspector selectedCell={selectedCell} selectionNotice={selectionNotice} />}

      <div className="cc-inspector-footer">
        <div className="cc-inspector-live-badge">LIVE</div>
        <div className="cc-inspector-tick">{displayedTick}</div>
        <button className="cc-freeze-btn" disabled>FREEZE</button>
      </div>
    </div>
  );
}
