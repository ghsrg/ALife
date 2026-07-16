import type { CellProjection } from '../projection/types';

interface SelectedEntityFocusCardProps {
  selectedCell: CellProjection | null;
}

export function SelectedEntityFocusCard({ selectedCell }: SelectedEntityFocusCardProps) {
  if (selectedCell === null) {
    return null;
  }

  return (
    <aside className="selected-focus-card" aria-label="Selected entity focus" data-testid="selected-focus-card">
      <div>
        <span>Selected</span>
        <strong>Cell {selectedCell.id}</strong>
      </div>
      <dl>
        <div>
          <dt>Position</dt>
          <dd>{Math.round(selectedCell.x)}, {Math.round(selectedCell.y)}</dd>
        </div>
        <div>
          <dt>Radius</dt>
          <dd>{Math.round(selectedCell.radius)}</dd>
        </div>
        <div>
          <dt>Energy</dt>
          <dd>{formatRatio(selectedCell.energy)}</dd>
        </div>
        <div>
          <dt>Lifecycle</dt>
          <dd>{formatLifecycle(selectedCell.lifecycle)}</dd>
        </div>
      </dl>
    </aside>
  );
}

function formatRatio(value: number) {
  return `${Math.round(value * 100)}%`;
}

function formatLifecycle(lifecycle: number | undefined) {
  if (lifecycle === 1) {
    return 'alive';
  }
  if (lifecycle === 2) {
    return 'dead';
  }
  return 'Unavailable';
}
