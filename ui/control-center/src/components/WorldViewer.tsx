import { forwardRef, useEffect, useImperativeHandle, useRef, useState } from 'react';
import type { CellId, WorldFrame } from '../projection/types';
import { projectCellForRender } from '../viewer/renderGeometry';
import { mountWorldRenderer, type WorldRenderer } from '../viewer/worldRenderer';
import { ViewerTruthOverlay } from './ViewerTruthOverlay';
import { buildViewerTruthState } from './viewerTruth';

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
      renderer.renderFrame(frame, selectedCellId);
      setIsReady(true);
    });

    return () => {
      cancelled = true;
      rendererRef.current?.destroy();
      rendererRef.current = null;
    };
  }, []);

  useEffect(() => {
    rendererRef.current?.renderFrame(frame, selectedCellId);
  }, [frame, selectedCellId]);

  return (
    <div className="world-viewer" aria-label="World Viewer" data-ready={isReady ? 'true' : 'false'}>
      <div ref={hostRef} className="world-canvas-host" />
      <ViewerTruthOverlay truthState={truthState} />
      <div className="world-hit-targets" aria-label="World cell hit targets">
        {frame.cells.map((cell) => {
          const left = `${(cell.x / frame.world.width) * 100}%`;
          const top = `${(cell.y / frame.world.height) * 100}%`;
          const geometry = projectCellForRender(cell, frame, viewport);
          const diameter = `${geometry.displayRadiusPx * 2}px`;

          return (
            <button
              key={cell.id}
              type="button"
              className={cell.id === selectedCellId ? 'cell-hotspot selected' : 'cell-hotspot'}
              style={{ left, top, width: diameter, height: diameter }}
              onClick={() => onSelectCell(cell.id)}
              aria-label={`Select ${cell.id}`}
            />
          );
        })}
      </div>
    </div>
  );
});
