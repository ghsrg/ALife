import { forwardRef, useEffect, useImperativeHandle, useRef, useState } from 'react';
import type {
  MouseEvent as ReactMouseEvent,
  PointerEvent as ReactPointerEvent,
  WheelEvent as ReactWheelEvent
} from 'react';
import type { CellId, WorldFrame } from '../projection/types';
import { projectCellForNavigatedRender } from '../viewer/renderGeometry';
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
  const hostRef = useRef<HTMLDivElement | null>(null);
  const rendererRef = useRef<WorldRenderer | null>(null);
  const [isReady, setIsReady] = useState(false);
  const [camera, setCamera] = useState<ViewerCamera>(DEFAULT_VIEWER_CAMERA);
  const dragStartRef = useRef<{ pointerId: number; x: number; y: number } | null>(null);
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
  };

  const moveDrag = (pointerId: number, x: number, y: number) => {
    const dragStart = dragStartRef.current;
    if (dragStart === null || dragStart.pointerId !== pointerId) {
      return;
    }
    const dx = x - dragStart.x;
    const dy = y - dragStart.y;
    dragStartRef.current = { pointerId, x, y };
    setCamera((current) => panCamera(current, { dx, dy }));
  };

  const endDrag = (pointerId: number) => {
    if (dragStartRef.current?.pointerId === pointerId) {
      dragStartRef.current = null;
    }
  };

  const handleWheel = (event: ReactWheelEvent<HTMLDivElement>) => {
    event.preventDefault();
    const rect = event.currentTarget.getBoundingClientRect();
    const point = {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top
    };
    setCamera((current) => zoomCameraAtPoint(current, point, event.deltaY < 0 ? 1.12 : 1 / 1.12));
  };

  const handlePointerDown = (event: ReactPointerEvent<HTMLDivElement>) => {
    if (event.button !== 0 && event.button !== undefined) {
      return;
    }
    const pointerId = event.pointerId ?? MOUSE_DRAG_POINTER_ID;
    startDrag(pointerId, event.clientX, event.clientY);
    if (event.currentTarget.hasPointerCapture?.(event.pointerId) === false) {
      event.currentTarget.setPointerCapture(event.pointerId);
    }
  };

  const handlePointerMove = (event: ReactPointerEvent<HTMLDivElement>) => {
    moveDrag(event.pointerId ?? MOUSE_DRAG_POINTER_ID, event.clientX, event.clientY);
  };

  const handlePointerUp = (event: ReactPointerEvent<HTMLDivElement>) => {
    endDrag(event.pointerId ?? MOUSE_DRAG_POINTER_ID);
    if (event.currentTarget.hasPointerCapture?.(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }
  };

  const handleMouseDown = (event: ReactMouseEvent<HTMLDivElement>) => {
    if (event.button !== 0 || dragStartRef.current !== null) {
      return;
    }
    startDrag(MOUSE_DRAG_POINTER_ID, event.clientX, event.clientY);
  };

  const handleMouseMove = (event: ReactMouseEvent<HTMLDivElement>) => {
    moveDrag(MOUSE_DRAG_POINTER_ID, event.clientX, event.clientY);
  };

  const handleMouseUp = () => {
    endDrag(MOUSE_DRAG_POINTER_ID);
  };

  return (
    <div
      className="world-viewer"
      aria-label="World Viewer"
      data-ready={isReady ? 'true' : 'false'}
      onWheel={handleWheel}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerCancel={handlePointerUp}
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      onMouseLeave={handleMouseUp}
    >
      <div ref={hostRef} className="world-canvas-host" />
      <ViewerTruthOverlay truthState={truthState} />
      <div className="viewer-navigation-controls" aria-label="World Viewer navigation">
        <button type="button" onClick={() => zoomAtCenter(1.2)} aria-label="Zoom in World Viewer">+</button>
        <button type="button" onClick={() => zoomAtCenter(1 / 1.2)} aria-label="Zoom out World Viewer">-</button>
        <button type="button" onClick={fitView} aria-label="Fit World Viewer">Fit</button>
        <button type="button" onClick={resetView} aria-label="Reset World Viewer navigation">Reset</button>
        <span aria-label="World Viewer zoom">{Math.round(camera.scale * 100)}%</span>
      </div>
      <div className="world-hit-targets" aria-label="World cell hit targets">
        {frame.cells.map((cell) => {
          const geometry = projectCellForNavigatedRender(cell, frame, viewport, camera);
          const diameter = `${geometry.displayRadiusPx * 2}px`;

          return (
            <button
              key={cell.id}
              type="button"
              className={cell.id === selectedCellId ? 'cell-hotspot selected' : 'cell-hotspot'}
              style={{ left: `${geometry.x}px`, top: `${geometry.y}px`, width: diameter, height: diameter }}
              onClick={() => onSelectCell(cell.id)}
              aria-label={`Select ${cell.id}`}
            />
          );
        })}
      </div>
    </div>
  );
});
