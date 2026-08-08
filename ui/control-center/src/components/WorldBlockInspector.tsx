import type { WorldBlockInspection } from '../app/worldBlockInspection';

export function WorldBlockInspector({ inspection }: { inspection: WorldBlockInspection }) {
  return (
    <aside className="side-panel inspector" aria-label="Inspector">
      <h2>Inspector</h2>
      <div className="metric-list">
        <div><span>Selection</span><strong>{`World cell ${inspection.blockX}, ${inspection.blockY}`}</strong></div>
        <div><span>Completeness</span><strong>{inspection.completeness}</strong></div>
        <div>
          <span>Bounds</span>
          <strong>{`${formatNumber(inspection.bounds.x)}, ${formatNumber(inspection.bounds.y)} / ${formatNumber(inspection.bounds.width)}x${formatNumber(inspection.bounds.height)}`}</strong>
        </div>
        <InspectionSection title="Resources" items={inspection.resources} emptyLabel="No resources sampled at this cell" />
        <InspectionSection title="Fields" items={inspection.fields} emptyLabel="No fields sampled at this cell" />
      </div>
    </aside>
  );
}

function InspectionSection({
  title,
  items,
  emptyLabel
}: {
  title: string;
  items: WorldBlockInspection['resources'];
  emptyLabel: string;
}) {
  return (
    <section className="inspector-data-section">
      <h3>{`${title} (${items.length})`}</h3>
      {items.length > 0 ? (
        <div className="inspector-data-grid">
          {items.map((item) => (
            <div key={item.id}>
              <span>{item.label}</span>
              <strong>{formatNumber(item.value)}</strong>
            </div>
          ))}
        </div>
      ) : (
        <p className="empty-state">{emptyLabel}</p>
      )}
    </section>
  );
}

function formatNumber(value: number) {
  return Number.isInteger(value) ? String(value) : value.toFixed(2);
}
