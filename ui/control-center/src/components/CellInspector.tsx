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
          <div><span>{uiText.inspector.energy}</span><strong>{formatEnergy(selectedCell)}</strong></div>
          <div><span>{uiText.inspector.integrity}</span><strong>{formatRatio(selectedCell.integrity)}</strong></div>
          <div><span>Position</span><strong>{`${selectedCell.x}, ${selectedCell.y}`}</strong></div>
          <div><span>Radius</span><strong>{selectedCell.radius}</strong></div>
          <div><span>{uiText.inspector.generation}</span><strong>{selectedCell.generation}</strong></div>
          <div><span>{uiText.inspector.roleHint}</span><strong>{selectedCell.roleHint}</strong></div>
          {selectedCell.materials?.map((material) => (
            <div key={`material-${material.materialTypeId}`}>
              <span>{`Material ${material.materialTypeId}`}</span>
              <strong>{formatAmount(material.amount)}</strong>
            </div>
          ))}
          {selectedCell.internalResources?.map((resource) => (
            <div key={`internal-resource-${resource.resourceTypeId}`}>
              <span>{`Internal resource ${resource.resourceTypeId}`}</span>
              <strong>{formatAmount(resource.amount)}</strong>
            </div>
          ))}
          {selectedCell.localExternalResources?.map((resource) => (
            <div key={`local-external-resource-${resource.resourceTypeId}`}>
              <span>{`Local external resource ${resource.resourceTypeId}`}</span>
              <strong>{formatAmount(resource.amount)}</strong>
            </div>
          ))}
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

function formatEnergy(cell: CellProjection) {
  if (cell.energyRaw !== undefined && cell.energyCapacity !== undefined) {
    return `${formatCompact(cell.energyRaw)} / ${formatCompact(cell.energyCapacity)} (${formatRatio(cell.energy)})`;
  }
  return formatRatio(cell.energy);
}

function formatAmount(value: number) {
  return value.toFixed(2);
}

function formatCompact(value: number) {
  return Number.isInteger(value) ? String(value) : value.toFixed(2);
}
