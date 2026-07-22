import { Fragment, forwardRef, useEffect, useImperativeHandle, useRef, useState } from 'react';
import type {
  MouseEvent as ReactMouseEvent,
  PointerEvent as ReactPointerEvent,
  WheelEvent as ReactWheelEvent
} from 'react';
import type { CellId, WorldFrame } from '../projection/types';
import { uiText } from '../uiText';
import { buildViewerHitTargets } from '../viewer/viewerHitTargets';
import { useViewerCamera } from '../viewer/useViewerCamera';
import { mountWorldRenderer, type WorldRenderer } from '../viewer/worldRenderer';
import { fitCameraToWorld, formatMapScaleLabel } from '../viewer/viewerNavigation';
import { ViewerTruthOverlay } from './ViewerTruthOverlay';
import { buildViewerTruthState } from './viewerTruth';

const MOUSE_DRAG_POINTER_ID = -1;

interface WorldViewerProps {
  frame: WorldFrame;
  selectedCellId: CellId | null;
  onSelectCell: (cellId: CellId | null) => void;
  onExportScreenshot?: () => void;
  onToggleFullScreen?: () => void;
  isFullScreen?: boolean;
}

export interface WorldViewerHandle {
  exportPng: () => string | null;
}

export const WorldViewer = forwardRef<WorldViewerHandle, WorldViewerProps>(function WorldViewer(
  { frame, selectedCellId, onSelectCell, onExportScreenshot, onToggleFullScreen, isFullScreen = false },
  ref
) {
  const viewerRef = useRef<HTMLDivElement | null>(null);
  const hostRef = useRef<HTMLDivElement | null>(null);
  const rendererRef = useRef<WorldRenderer | null>(null);
  const [isReady, setIsReady] = useState(false);
  const [cameraState, dispatchCamera] = useViewerCamera();
  const { camera } = cameraState;
  const [truthOverlayVisible, setTruthOverlayVisible] = useState(true);
  const [viewport, setViewport] = useState(() => ({ width: frame.world.width, height: frame.world.height }));
  const truthState = buildViewerTruthState(frame, viewport);

  const measureViewport = () => {
    const host = hostRef.current;
    if (!host) {
      return { width: frame.world.width, height: frame.world.height };
    }

    return {
      width: host.clientWidth || frame.world.width,
      height: host.clientHeight || frame.world.height
    };
  };

  useImperativeHandle(ref, () => ({
    exportPng: () => rendererRef.current?.exportPng() ?? null
  }), []);

  useEffect(() => {
    const host = hostRef.current;
    let cancelled = false;

    if (!host) {
      return undefined;
    }

    Promise.resolve(mountWorldRenderer(host)).then((renderer) => {
      if (cancelled) {
        renderer.destroy();
        return;
      }

      const measuredViewport = measureViewport();
      const fittedCamera = fitCameraToWorld(frame.world, measuredViewport);
      setViewport(measuredViewport);
      rendererRef.current = renderer;
      dispatchCamera({ type: 'fit', world: frame.world, viewport: measuredViewport });
      renderer.renderFrame(frame, selectedCellId, fittedCamera);
      setIsReady(true);
    });

    return () => {
      cancelled = true;
      rendererRef.current?.destroy();
      rendererRef.current = null;
    };
  }, []);

  useEffect(() => {
    rendererRef.current?.renderFrame(frame, selectedCellId, camera);
  }, [frame, selectedCellId, camera]);

  const zoomAtCenter = (scaleFactor: number) => {
    const point = { x: frame.world.width / 2, y: frame.world.height / 2 };
    dispatchCamera({ type: 'zoom-at', point, scaleFactor });
  };

  const fitView = () => {
    dispatchCamera({ type: 'fit', world: frame.world, viewport });
  };

  const resetView = () => {
    dispatchCamera({ type: 'reset' });
  };

  const startDrag = (pointerId: number, x: number, y: number) => {
    dispatchCamera({ type: 'drag-start', pointerId, point: { x, y } });
  };

  const moveDrag = (pointerId: number, x: number, y: number) => {
    dispatchCamera({ type: 'drag-move', pointerId, point: { x, y } });
  };

  const endDrag = (pointerId: number) => {
    dispatchCamera({ type: 'drag-end', pointerId });
  };

  const handlePointerDown = (event: ReactPointerEvent<HTMLDivElement>) => {
    if (event.button !== 0 && event.button !== undefined) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    const pointerId = event.pointerId ?? MOUSE_DRAG_POINTER_ID;
    startDrag(pointerId, event.clientX, event.clientY);
    if (event.currentTarget.hasPointerCapture?.(event.pointerId) === false) {
      event.currentTarget.setPointerCapture(event.pointerId);
    }
  };

  const handlePointerMove = (event: ReactPointerEvent<HTMLDivElement>) => {
    if (cameraState.dragStart !== null) {
      event.preventDefault();
      event.stopPropagation();
    }
    moveDrag(event.pointerId ?? MOUSE_DRAG_POINTER_ID, event.clientX, event.clientY);
  };

  const handleWheelCapture = (event: ReactWheelEvent<HTMLDivElement>) => {
    event.preventDefault();
    event.stopPropagation();
    const target = event.target;
    if (target instanceof Element && target.closest('.viewer-navigation-controls, .viewer-truth-overlay')) {
      return;
    }
    const rect = event.currentTarget.getBoundingClientRect();
    const point = {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top
    };
    dispatchCamera({ type: 'zoom-at', point, scaleFactor: event.deltaY < 0 ? 1.12 : 1 / 1.12 });
  };

  const handlePointerUp = (event: ReactPointerEvent<HTMLDivElement>) => {
    event.preventDefault();
    event.stopPropagation();
    endDrag(event.pointerId ?? MOUSE_DRAG_POINTER_ID);
    if (event.currentTarget.hasPointerCapture?.(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }
  };

  const handleMouseDown = (event: ReactMouseEvent<HTMLDivElement>) => {
    if (event.button !== 0 || cameraState.dragStart !== null) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    startDrag(MOUSE_DRAG_POINTER_ID, event.clientX, event.clientY);
  };

  const handleMouseMove = (event: ReactMouseEvent<HTMLDivElement>) => {
    if (cameraState.dragStart !== null) {
      event.preventDefault();
      event.stopPropagation();
    }
    moveDrag(MOUSE_DRAG_POINTER_ID, event.clientX, event.clientY);
  };

  const handleMouseUp = (event: ReactMouseEvent<HTMLDivElement>) => {
    if (cameraState.dragStart !== null) {
      event.preventDefault();
      event.stopPropagation();
    }
    endDrag(MOUSE_DRAG_POINTER_ID);
  };

  const handleViewerClick = (event: ReactMouseEvent<HTMLDivElement>) => {
    if (cameraState.dragMoved) {
      dispatchCamera({ type: 'clear-drag-moved' });
      return;
    }

    const target = event.target;
    if (!(target instanceof Element)) {
      return;
    }

    if (target.closest('button, .viewer-navigation-controls, .viewer-truth-overlay, .cell-hotspot')) {
      return;
    }

    setTruthOverlayVisible(false);
    onSelectCell(null);
  };

  return (
    <div
      ref={viewerRef}
      className="world-viewer"
      aria-label={uiText.viewer.ariaLabel}
      data-ready={isReady ? 'true' : 'false'}
      onWheelCapture={handleWheelCapture}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerCancel={handlePointerUp}
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      onMouseLeave={handleMouseUp}
      onClick={handleViewerClick}
    >
      <div ref={hostRef} className="world-canvas-host" />
      {truthOverlayVisible ? <ViewerTruthOverlay truthState={truthState} onDismiss={() => setTruthOverlayVisible(false)} /> : null}
      <div
        className="viewer-navigation-controls"
        aria-label={uiText.viewer.navigationAriaLabel}
        onMouseDown={(event) => event.stopPropagation()}
        onPointerDown={(event) => event.stopPropagation()}
        onWheel={(event) => event.stopPropagation()}
        onClick={(event) => event.stopPropagation()}
      >
        <button type="button" onClick={() => zoomAtCenter(1.2)} aria-label={uiText.viewer.zoomIn}>+</button>
        <button type="button" onClick={() => zoomAtCenter(1 / 1.2)} aria-label={uiText.viewer.zoomOut}>-</button>
        <button type="button" onClick={fitView} aria-label={uiText.viewer.fit}>{uiText.viewer.fitButton}</button>
        <button type="button" onClick={resetView} aria-label={uiText.viewer.reset}>{uiText.viewer.resetButton}</button>
        {onExportScreenshot ? (
          <button type="button" onClick={onExportScreenshot} aria-label={uiText.controls.exportViewerPng}>
            PNG
          </button>
        ) : null}
        {onToggleFullScreen ? (
          <button type="button" onClick={onToggleFullScreen} aria-label={uiText.controls.enterStartFullScreen}>
            {isFullScreen ? 'Exit' : 'Full'}
          </button>
        ) : null}
        <span aria-label={uiText.viewer.zoomLabel}>{formatMapScaleLabel(frame.world, viewport, camera.scale)}</span>
      </div>
      <div className="world-hit-targets" aria-label={uiText.viewer.hitTargetsAriaLabel}>
        {buildViewerHitTargets(frame, selectedCellId, viewport, camera).map((target) => {
          return (
            <Fragment key={target.id}>
              <button
                type="button"
                className="cell-hotspot"
                data-semantic-level={target.detail.level}
                data-lifecycle-state={target.detail.lifecycleState}
                style={target.style}
                onMouseDown={(event) => event.stopPropagation()}
                onPointerDown={(event) => event.stopPropagation()}
                onClick={(event) => {
                  event.stopPropagation();
                  onSelectCell(target.id);
                }}
                aria-label={target.ariaLabel}
              />
              {target.detail.showLabel ? (
                <span
                  className={target.selected ? 'cell-detail-label selected' : 'cell-detail-label'}
                  style={target.labelStyle}
                  aria-label={target.selected ? 'Selected cell detail label' : `Cell detail label ${target.id}`}
                >
                  {target.detail.label}
                </span>
              ) : null}
            </Fragment>
          );
        })}
      </div>
    </div>
  );
});
