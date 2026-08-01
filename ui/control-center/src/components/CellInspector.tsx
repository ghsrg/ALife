import { useState } from 'react';
import type { CellProjection } from '../projection/types';
import { uiText } from '../uiText';

interface CellInspectorProps {
  selectedCell: CellProjection | null;
  selectionNotice?: string | null;
}

export function CellInspector({ selectedCell, selectionNotice }: CellInspectorProps) {
  const [pinnedCell, setPinnedCell] = useState<CellProjection | null>(null);

  return (
    <aside className="side-panel inspector" aria-label={uiText.inspector.title}>
      <h2>{uiText.inspector.title}</h2>
      {selectedCell ? (
        <div className="metric-list">
          <div className="inspector-actions">
            <button type="button" onClick={() => setPinnedCell(selectedCell)} aria-label="Pin selected Cell">
              Pin
            </button>
            <button
              type="button"
              onClick={() => setPinnedCell(null)}
              disabled={pinnedCell === null}
              aria-label="Clear pinned Cell"
            >
              Clear
            </button>
          </div>
          {pinnedCell ? <PinnedCellComparison pinnedCell={pinnedCell} selectedCell={selectedCell} /> : null}
          <div><span>{uiText.inspector.id}</span><strong>{selectedCell.id}</strong></div>
          <div><span>{uiText.inspector.energy}</span><strong>{formatEnergy(selectedCell)}</strong></div>
          <div><span>{uiText.inspector.integrity}</span><strong>{formatRatio(selectedCell.integrity)}</strong></div>
          <div><span>Position</span><strong>{`${selectedCell.x}, ${selectedCell.y}`}</strong></div>
          <div><span>Radius</span><strong>{selectedCell.radius}</strong></div>
          <div><span>{uiText.inspector.generation}</span><strong>{selectedCell.generation}</strong></div>
          <div><span>{uiText.inspector.roleHint}</span><strong>{selectedCell.roleHint}</strong></div>
          <AmountSection
            title="Materials"
            items={selectedCell.materials}
            label={(item) => `Material ${item.materialTypeId}`}
          />
          <AmountSection
            title="Internal resources"
            items={selectedCell.internalResources}
            label={(item) => `Internal resource ${item.resourceTypeId}`}
          />
          <AmountSection
            title="Local external resources"
            items={selectedCell.localExternalResources}
            label={(item) => `Local external resource ${item.resourceTypeId}`}
          />
          <PhenotypeTraitSection
            traits={selectedCell.phenotypeTraits}
            radius={selectedCell.radius}
            energy={selectedCell.energy}
            cellId={selectedCell.id}
          />
        </div>
      ) : (
        <>
          <p className="empty-state">{uiText.inspector.emptyCell}</p>
          {selectionNotice ? <p className="selection-notice">{selectionNotice}</p> : null}
        </>
      )}


    </aside>
  );
}

function formatRatio(value: number) {
  return `${Math.round(value * 100)}%`;
}

function PinnedCellComparison({
  pinnedCell,
  selectedCell
}: {
  pinnedCell: CellProjection;
  selectedCell: CellProjection;
}) {
  return (
    <section className="inspector-comparison" aria-label="Pinned Cell comparison">
      <h3>Compare pinned Cell</h3>
      <div>
        <span>Pinned</span>
        <strong>{pinnedCell.id}</strong>
      </div>
      <div>
        <span>Selected</span>
        <strong>{selectedCell.id}</strong>
      </div>
      <div>
        <span>{`Energy delta ${formatSignedDelta(cellEnergyRaw(selectedCell) - cellEnergyRaw(pinnedCell))}`}</span>
        <strong>{`${formatCompact(cellEnergyRaw(selectedCell))} vs ${formatCompact(cellEnergyRaw(pinnedCell))}`}</strong>
      </div>
      <div>
        <span>Material overlap</span>
        <strong>{materialOverlapLabel(pinnedCell, selectedCell)}</strong>
      </div>
    </section>
  );
}

interface AmountItem {
  amount: number;
}

const INSPECTOR_SECTION_LIMIT = 6;

function AmountSection<T extends AmountItem>({
  title,
  items,
  label
}: {
  title: string;
  items?: T[];
  label: (item: T) => string;
}) {
  if (!items || items.length === 0) {
    return null;
  }

  const visibleItems = items.slice(0, INSPECTOR_SECTION_LIMIT);
  const hiddenCount = items.length - visibleItems.length;

  return (
    <section className="inspector-data-section">
      <h3>{`${title} (${items.length})`}</h3>
      <div className="inspector-data-grid">
        {visibleItems.map((item) => (
          <div key={label(item)}>
            <span>{label(item)}</span>
            <strong>{formatAmount(item.amount)}</strong>
          </div>
        ))}
      </div>
      {hiddenCount > 0 ? <p className="inspector-data-more">{`+${hiddenCount} more`}</p> : null}
    </section>
  );
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

function formatSignedDelta(value: number) {
  if (value > 0) {
    return formatCompact(value);
  }
  return value === 0 ? '0' : `-${formatCompact(Math.abs(value))}`;
}

function cellEnergyRaw(cell: CellProjection) {
  return cell.energyRaw ?? cell.energy;
}

function materialOverlapLabel(pinnedCell: CellProjection, selectedCell: CellProjection) {
  const pinnedMaterialIds = new Set((pinnedCell.materials ?? []).map((material) => material.materialTypeId));
  const selectedMaterialIds = new Set((selectedCell.materials ?? []).map((material) => material.materialTypeId));
  let overlap = 0;

  for (const materialId of selectedMaterialIds) {
    if (pinnedMaterialIds.has(materialId)) {
      overlap += 1;
    }
  }

  return `${overlap} shared`;
}

function PhenotypeTraitSection({
  traits,
  radius,
  energy,
  cellId
}: {
  traits?: import('../projection/types').PhenotypeTraitProjection;
  radius: number;
  energy: number;
  cellId: string;
}) {
  const flagellaCount = traits?.flagellaCount ?? (radius > 8 && energy > 20 ? 2 : radius > 5 ? 1 : 0);
  const spikeCount = traits?.spikeCount ?? (radius > 10 ? 3 : 0);
  const receptorHalo = traits?.receptorHaloIntensity ?? 0.5;
  const lineageHue = traits?.lineageHue ?? ((parseInt(cellId.replace(/\D/g, ''), 10) * 137) % 360 || 180);
  const divisionFlash = traits?.divisionFlashIntensity ?? 0;

  return (
    <div className="phenotype-trait-card" style={{ marginTop: '12px', borderTop: '1px solid rgba(255,255,255,0.1)', paddingTop: '8px' }}>
      <span className="cc-inspector-section-label">GENOME-TO-PHENOTYPE MAPPING</span>
      <div className="cc-inspector-grid" style={{ gap: '6px' }}>
        <div className="cc-inspector-metric">
          <span className="cc-inspector-metric-label">🚩 Motility (Flagella)</span>
          <span className="cc-inspector-metric-value">{flagellaCount > 0 ? `${flagellaCount} tail filaments` : 'None'}</span>
        </div>
        <div className="cc-inspector-metric">
          <span className="cc-inspector-metric-label">⚡ Defense (Spikes)</span>
          <span className="cc-inspector-metric-value">{spikeCount > 0 ? `${spikeCount} radial spikes` : 'Smooth membrane'}</span>
        </div>
        <div className="cc-inspector-metric">
          <span className="cc-inspector-metric-label">📡 Uptake (Receptor Halo)</span>
          <span className="cc-inspector-metric-value">{Math.round(receptorHalo * 100)}% aura intensity</span>
        </div>
        <div className="cc-inspector-metric">
          <span className="cc-inspector-metric-label">🧬 Lineage Color Coat</span>
          <span className="cc-inspector-metric-value" style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
            <span style={{ width: '10px', height: '10px', borderRadius: '50%', background: `hsl(${lineageHue}, 75%, 55%)` }} />
            Hue {lineageHue}°
          </span>
        </div>
        <div className="cc-inspector-metric" style={{ gridColumn: 'span 2' }}>
          <span className="cc-inspector-metric-label">✨ Division Readiness / Flash</span>
          <span className="cc-inspector-metric-value">{divisionFlash > 0 ? `${Math.round(divisionFlash * 100)}% copying flash` : 'Stable'}</span>
        </div>
      </div>
    </div>
  );
}

