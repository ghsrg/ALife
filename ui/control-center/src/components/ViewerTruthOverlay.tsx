import type { ViewerTruthState } from './viewerTruth';

interface ViewerTruthOverlayProps {
  truthState: ViewerTruthState;
  onDismiss?: () => void;
}

export function ViewerTruthOverlay({ truthState, onDismiss }: ViewerTruthOverlayProps) {
  return (
    <aside
      className="viewer-truth-overlay"
      aria-label="Viewer projection truth"
      onClick={(event) => event.stopPropagation()}
    >
      {onDismiss ? (
        <button type="button" className="viewer-truth-close" onClick={onDismiss} aria-label="Dismiss projection notices">
          x
        </button>
      ) : null}
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
