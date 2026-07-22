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
      <div className="selected-focus-bars" aria-label="Selected cell projection bars">
        <MetricBar label="Energy" value={selectedCell.energy} ariaLabel="Selected cell energy" />
        <MetricBar label="Integrity" value={selectedCell.integrity} ariaLabel="Selected cell integrity" />
      </div>
    </aside>
  );
}

function MetricBar({ label, value, ariaLabel }: { label: string; value: number; ariaLabel: string }) {
  const percent = Math.round(clampRatio(value) * 100);

  return (
    <div className="selected-focus-bar">
      <span>{label}</span>
      <div
        role="meter"
        aria-label={ariaLabel}
        aria-valuemin={0}
        aria-valuemax={100}
        aria-valuenow={percent}
      >
        <i style={{ width: `${percent}%` }} />
      </div>
    </div>
  );
}

function clampRatio(value: number) {
  return Math.max(0, Math.min(1, value));
}

function formatRatio(value: number) {
  return `${Math.round(value * 100)}%`;
}

function formatLifecycle(lifecycle: number | undefined) {
  if (lifecycle === 0) {
    return 'alive';
  }
  if (lifecycle === 1) {
    return 'stressed';
  }
  if (lifecycle === 2) {
    return 'dormant';
  }
  if (lifecycle === 3) {
    return 'dead';
  }
  return 'Unavailable';
}
