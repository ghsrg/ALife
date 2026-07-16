import type { ViewerTruthState } from './viewerTruth';

interface ViewerTruthOverlayProps {
  truthState: ViewerTruthState;
}

export function ViewerTruthOverlay({ truthState }: ViewerTruthOverlayProps) {
  return (
    <aside className="viewer-truth-overlay" aria-label="Viewer projection truth">
      <TruthItem item={truthState.resourceLayer} />
      <TruthItem item={truthState.cellScale} />
    </aside>
  );
}

function TruthItem({ item }: { item: ViewerTruthState[keyof ViewerTruthState] }) {
  return (
    <article className={`viewer-truth-item viewer-truth-${item.state}`}>
      <span>{item.label}</span>
      <strong>{item.value}</strong>
      <small>{item.note}</small>
    </article>
  );
}
