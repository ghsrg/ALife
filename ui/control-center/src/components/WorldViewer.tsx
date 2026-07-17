import { Fragment, forwardRef, useEffect, useImperativeHandle, useRef, useState } from 'react';
import type {
  MouseEvent as ReactMouseEvent,
  PointerEvent as ReactPointerEvent,
  WheelEvent as ReactWheelEvent
} from 'react';
import type { CellId, WorldFrame } from '../projection/types';
import { projectCellForNavigatedRender } from '../viewer/renderGeometry';
import { buildCellSemanticDetail } from '../viewer/semanticDetail';
import {
  DEFAULT_VIEWER_CAMERA,
  fitCameraToWorld,
  panCamera,
  resetCamera,
  zoomCameraAtPoint,
  type ViewerCamera
} from '../viewer/viewerNavigation';
import { mountWorldRenderer, type WorldRenderer } from '../viewer/worldRenderer';
import { ViewerTruthOverlay } from './ViewerTruthOverlay';
import { buildViewerTruthState } from './viewerTruth';

const MOUSE_DRAG_POINTER_ID = -1;

interface WorldViewerProps {
  frame: WorldFrame;
  selectedCellId: CellId | null;
  onSelectCell: (cellId: CellId) => void;
}

export interface WorldViewerHandle {
  exportPng: () => string | null;
}

export const WorldViewer = forwardRef<WorldViewerHandle, WorldViewerProps>(function WorldViewer(
  { frame, selectedCellId, onSelectCell },
  ref
) {
  const viewerRef = useRef<HTMLDivElement | null>(null);
  const hostRef = useRef<HTMLDivElement | null>(null);
  const rendererRef = useRef<WorldRenderer | null>(null);
  const [isReady, setIsReady] = useState(false);
  const [camera, setCamera] = useState<ViewerCamera>(DEFAULT_VIEWER_CAMERA);
  const [truthOverlayVisible, setTruthOverlayVisible] = useState(true);
  const dragStartRef = useRef<{ pointerId: number; x: number; y: number } | null>(null);
  const dragMovedRef = useRef(false);
  const viewport = { width: frame.world.width, height: frame.world.height };
  const truthState = buildViewerTruthState(frame, viewport);

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

      rendererRef.current = renderer;
      renderer.renderFrame(frame, selectedCellId, camera);
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

  useEffect(() => {
    const viewer = viewerRef.current;
    if (!viewer) {
      return undefined;
    }

    const handleNativeWheel = (event: WheelEvent) => {
      event.preventDefault();
      event.stopPropagation();
      const target = event.target;
      if (target instanceof Element && target.closest('.viewer-navigation-controls, .viewer-truth-overlay')) {
        return;
      }
      const rect = viewer.getBoundingClientRect();
      const point = {
        x: event.clientX - rect.left,
        y: event.clientY - rect.top
      };
      setCamera((current) => zoomCameraAtPoint(current, point, event.deltaY < 0 ? 1.12 : 1 / 1.12));
    };

    viewer.addEventListener('wheel', handleNativeWheel, { passive: false });

    return () => {
      viewer.removeEventListener('wheel', handleNativeWheel);
    };
  }, []);

  const zoomAtCenter = (scaleFactor: number) => {
    const point = { x: frame.world.width / 2, y: frame.world.height / 2 };
    setCamera((current) => zoomCameraAtPoint(current, point, scaleFactor));
  };

  const fitView = () => {
    setCamera(fitCameraToWorld(frame.world, viewport));
  };

  const resetView = () => {
    setCamera(resetCamera());
  };

  const startDrag = (pointerId: number, x: number, y: number) => {
    dragStartRef.current = { pointerId, x, y };
    dragMovedRef.current = false;
  };

  const moveDrag = (pointerId: number, x: number, y: number) => {
    const dragStart = dragStartRef.current;
    if (dragStart === null || dragStart.pointerId !== pointerId) {
      return;
    }
    const dx = x - dragStart.x;
    const dy = y - dragStart.y;
    if (dx !== 0 || dy !== 0) {
      dragMovedRef.current = true;
    }
    dragStartRef.current = { pointerId, x, y };
    setCamera((current) => panCamera(current, { dx, dy }));
  };

  const endDrag = (pointerId: number) => {
    if (dragStartRef.current?.pointerId === pointerId) {
      dragStartRef.current = null;
    }
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
    if (dragStartRef.current !== null) {
      event.preventDefault();
      event.stopPropagation();
    }
    moveDrag(event.pointerId ?? MOUSE_DRAG_POINTER_ID, event.clientX, event.clientY);
  };

  const handleWheelCapture = (event: ReactWheelEvent<HTMLDivElement>) => {
    event.preventDefault();
    event.stopPropagation();
    const rect = event.currentTarget.getBoundingClientRect();
    const point = {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top
    };
    setCamera((current) => zoomCameraAtPoint(current, point, event.deltaY < 0 ? 1.12 : 1 / 1.12));
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
    if (event.button !== 0 || dragStartRef.current !== null) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    startDrag(MOUSE_DRAG_POINTER_ID, event.clientX, event.clientY);
  };

  const handleMouseMove = (event: ReactMouseEvent<HTMLDivElement>) => {
    if (dragStartRef.current !== null) {
      event.preventDefault();
      event.stopPropagation();
    }
    moveDrag(MOUSE_DRAG_POINTER_ID, event.clientX, event.clientY);
  };

  const handleMouseUp = (event: ReactMouseEvent<HTMLDivElement>) => {
    if (dragStartRef.current !== null) {
      event.preventDefault();
      event.stopPropagation();
    }
    endDrag(MOUSE_DRAG_POINTER_ID);
  };

  const handleViewerClick = (event: ReactMouseEvent<HTMLDivElement>) => {
    if (dragMovedRef.current) {
      dragMovedRef.current = false;
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
  };

  return (
    <div
      ref={viewerRef}
      className="world-viewer"
      aria-label="World Viewer"
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
        aria-label="World Viewer navigation"
        onMouseDown={(event) => event.stopPropagation()}
        onPointerDown={(event) => event.stopPropagation()}
        onWheel={(event) => event.stopPropagation()}
        onClick={(event) => event.stopPropagation()}
      >
        <button type="button" onClick={() => zoomAtCenter(1.2)} aria-label="Zoom in World Viewer">+</button>
        <button type="button" onClick={() => zoomAtCenter(1 / 1.2)} aria-label="Zoom out World Viewer">-</button>
        <button type="button" onClick={fitView} aria-label="Fit World Viewer">Fit</button>
        <button type="button" onClick={resetView} aria-label="Reset World Viewer navigation">Reset</button>
        <span aria-label="World Viewer zoom">{Math.round(camera.scale * 100)}%</span>
      </div>
      <div className="world-hit-targets" aria-label="World cell hit targets">
        {frame.cells.map((cell) => {
          const geometry = projectCellForNavigatedRender(cell, frame, viewport, camera);
          const selected = cell.id === selectedCellId;
          const detail = buildCellSemanticDetail(cell, {
            displayRadiusPx: geometry.displayRadiusPx,
            selected
          });
          const diameter = `${geometry.interactionRadiusPx * 2}px`;

          return (
            <Fragment key={cell.id}>
              <button
                type="button"
                className={selected ? 'cell-hotspot selected' : 'cell-hotspot'}
                data-semantic-level={detail.level}
                data-lifecycle-state={detail.lifecycleState}
                style={{ left: `${geometry.x}px`, top: `${geometry.y}px`, width: diameter, height: diameter }}
                onMouseDown={(event) => event.stopPropagation()}
                onPointerDown={(event) => event.stopPropagation()}
                onClick={(event) => {
                  event.stopPropagation();
                  onSelectCell(cell.id);
                }}
                aria-label={`Select ${cell.id}`}
              />
              {detail.showLabel ? (
                <span
                  className={selected ? 'cell-detail-label selected' : 'cell-detail-label'}
                  style={{ left: `${geometry.x}px`, top: `${geometry.y + geometry.displayRadiusPx + 10}px` }}
                  aria-label={selected ? 'Selected cell detail label' : `Cell detail label ${cell.id}`}
                >
                  {detail.label}
                </span>
              ) : null}
            </Fragment>
          );
        })}
      </div>
    </div>
  );
});
