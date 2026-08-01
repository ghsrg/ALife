import { Fragment, forwardRef, useEffect, useImperativeHandle, useRef, useState } from 'react';
import type {
  MouseEvent as ReactMouseEvent,
  PointerEvent as ReactPointerEvent,
  WheelEvent as ReactWheelEvent
} from 'react';
import type { CellId, WorldFrame } from '../projection/types';
import type { DebugProjectionState } from '../projection/types';
import { uiText } from '../uiText';
import { buildViewerHitTargets, type ViewerHitTarget } from '../viewer/viewerHitTargets';
import { useViewerCamera } from '../viewer/useViewerCamera';
import { buildDebugLayerPlan, type DebugLayerMode } from '../viewer/debugLayers';
import { mountWorldRenderer, type WorldRenderer } from '../viewer/worldRenderer';
import { fitCameraToWorld, formatMapScaleLabel } from '../viewer/viewerNavigation';
import { ViewerTruthOverlay } from './ViewerTruthOverlay';
import { buildViewerTruthState } from './viewerTruth';

import {
  createCellSelection,
  createNoneSelection,
  createSelectionSet,
  deriveWorldBlockAtPoint,
  toggleSelectionSetMember,
  type AnalysisLevel,
  type MonitorSelection
} from '../app/selectionModel';
import type { VisualEffectsConfig } from '../app/appState';

const MOUSE_DRAG_POINTER_ID = -1;

interface WorldViewerProps {
  frame: WorldFrame;
  selectedCellId: CellId | null;
  onSelectCell: (cellId: CellId | null) => void;
  activeLevel?: AnalysisLevel;
  currentSelection?: MonitorSelection;
  onSelectTarget?: (selection: MonitorSelection) => void;
  onExportScreenshot?: () => void;
  onToggleFullScreen?: () => void;
  isFullScreen?: boolean;
  debugProjections?: DebugProjectionState;
  activeResourceLayers?: number[];
  visualEffects?: VisualEffectsConfig;
}

export interface WorldViewerHandle {
  exportPng: () => string | null;
}

export const WorldViewer = forwardRef<WorldViewerHandle, WorldViewerProps>(function WorldViewer(
  {
    frame,
    selectedCellId,
    onSelectCell,
    activeLevel = 'cells',
    currentSelection,
    onSelectTarget,
    onExportScreenshot,
    onToggleFullScreen,
    isFullScreen = false,
    debugProjections,
    activeResourceLayers,
    visualEffects
  },
  ref
) {
  const viewerRef = useRef<HTMLDivElement | null>(null);
  const hostRef = useRef<HTMLDivElement | null>(null);
  const rendererRef = useRef<WorldRenderer | null>(null);
  const [isReady, setIsReady] = useState(false);
  const [cameraState, dispatchCamera] = useViewerCamera();
  const { camera } = cameraState;
  const [truthOverlayVisible, setTruthOverlayVisible] = useState(true);
  const [debugLayerMode, setDebugLayerMode] = useState<DebugLayerMode>('exact');
  const [isDebugOverlayCollapsed, setIsDebugOverlayCollapsed] = useState(true);
  const [mapSearchQuery, setMapSearchQuery] = useState('');
  const [viewport, setViewport] = useState(() => ({ width: frame.world.width, height: frame.world.height }));
  const normalizedMapSearchQuery = mapSearchQuery.trim().toLowerCase();
  const truthState = buildViewerTruthState(frame, viewport, debugProjections);
  const debugLayerPlan = debugProjections
    ? buildDebugLayerPlan(debugProjections, {
        mode: debugLayerMode,
        showResourceLayer: true,
        showFieldLayer: true
      })
    : null;

  const resourceRows = frame.resources.length;
  const resourceColumns = Math.max(...frame.resources.map((row) => row.length), 0);

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

  // Fit camera whenever world dimensions change or mount occurs
  useEffect(() => {
    const measured = measureViewport();
    setViewport(measured);
    dispatchCamera({ type: 'fit', world: frame.world, viewport: measured });
  }, [frame.world.width, frame.world.height]);

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

      const updateViewportAndFit = () => {
        const measuredViewport = measureViewport();
        setViewport(measuredViewport);
        renderer.resize(measuredViewport.width, measuredViewport.height);
        dispatchCamera({ type: 'fit', world: frame.world, viewport: measuredViewport });
      };

      // Initial fit and render after mount
      updateViewportAndFit();
      renderer.renderFrame(frame, selectedCellId, camera, activeResourceLayers);
      setIsReady(true);

      // Track container resize continuously if browser supports ResizeObserver
      if (typeof ResizeObserver !== 'undefined') {
        const resizeObserver = new ResizeObserver(() => {
          if (!cancelled) {
            updateViewportAndFit();
          }
        });
        resizeObserver.observe(host);
        (host as any).__resizeObserver = resizeObserver;
      }
    });

    return () => {
      cancelled = true;
      if (hostRef.current && (hostRef.current as any).__resizeObserver) {
        (hostRef.current as any).__resizeObserver.disconnect();
      }
      rendererRef.current?.destroy();
      rendererRef.current = null;
    };
  }, []);

  useEffect(() => {
    if (!rendererRef.current) return;
    if (activeResourceLayers !== undefined) {
      rendererRef.current.renderFrame(frame, selectedCellId, camera, activeResourceLayers);
    } else {
      rendererRef.current.renderFrame(frame, selectedCellId, camera);
    }
  }, [frame, selectedCellId, camera, activeResourceLayers, isReady]);

  const zoomAtCenter = (scaleFactor: number) => {
    const point = { x: frame.world.width / 2, y: frame.world.height / 2 };
    dispatchCamera({ type: 'zoom-at', point, scaleFactor });
  };

  const fitView = () => {
    dispatchCamera({ type: 'fit', world: frame.world, viewport: measureViewport() });
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


  const selectionRectangleRef = useRef<{ startX: number; startY: number; currentX: number; currentY: number } | null>(null);

  const completeSelectionRectangle = (clientX: number, clientY: number) => {
    const activeSelectionRectangle = selectionRectangleRef.current;
    if (activeSelectionRectangle === null) {
      return false;
    }

    const targets = buildViewerHitTargets(frame, selectedCellId, viewport, camera);
    const selectedTargets = targets
      .filter((target) => isHitTargetInsideRectangle(target, {
        startX: activeSelectionRectangle.startX,
        startY: activeSelectionRectangle.startY,
        currentX: clientX,
        currentY: clientY
      }))
      .map((target) => createCellSelection({
        cellId: target.id,
        runId: frame.runId,
        tick: frame.tick
      }));

    if (selectedTargets.length > 0) {
      onSelectTarget?.(createSelectionSet({
        targets: selectedTargets,
        runId: frame.runId,
        tick: frame.tick
      }));
    }

    selectionRectangleRef.current = null;
    return true;
  };

  const handlePointerDown = (event: ReactPointerEvent<HTMLDivElement>) => {
    if (event.button !== 0 && event.button !== undefined) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    const pointerId = event.pointerId ?? MOUSE_DRAG_POINTER_ID;

    if (event.shiftKey && activeLevel === 'cells') {
      selectionRectangleRef.current = {
        startX: event.clientX,
        startY: event.clientY,
        currentX: event.clientX,
        currentY: event.clientY
      };
      return;
    }

    startDrag(pointerId, event.clientX, event.clientY);
    if (event.currentTarget.hasPointerCapture?.(event.pointerId) === false) {
      event.currentTarget.setPointerCapture(event.pointerId);
    }
  };

  const handlePointerMove = (event: ReactPointerEvent<HTMLDivElement>) => {
    if (selectionRectangleRef.current !== null) {
      selectionRectangleRef.current.currentX = event.clientX;
      selectionRectangleRef.current.currentY = event.clientY;
      return;
    }
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
    if (completeSelectionRectangle(event.clientX, event.clientY)) {
      return;
    }
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

    if (event.shiftKey && activeLevel === 'cells') {
      selectionRectangleRef.current = {
        startX: event.clientX,
        startY: event.clientY,
        currentX: event.clientX,
        currentY: event.clientY
      };
      return;
    }

    startDrag(MOUSE_DRAG_POINTER_ID, event.clientX, event.clientY);
  };

  const handleMouseMove = (event: ReactMouseEvent<HTMLDivElement>) => {
    if (selectionRectangleRef.current !== null) {
      selectionRectangleRef.current.currentX = event.clientX;
      selectionRectangleRef.current.currentY = event.clientY;
      return;
    }
    if (cameraState.dragStart !== null) {
      event.preventDefault();
      event.stopPropagation();
    }
    moveDrag(MOUSE_DRAG_POINTER_ID, event.clientX, event.clientY);
  };

  const handleMouseUp = (event: ReactMouseEvent<HTMLDivElement>) => {
    if (completeSelectionRectangle(event.clientX, event.clientY)) {
      return;
    }
    if (cameraState.dragStart !== null) {
      event.preventDefault();
      event.stopPropagation();
    }
    endDrag(MOUSE_DRAG_POINTER_ID);
  };

  const handleCellHotspotClick = (target: ViewerHitTarget, event: ReactMouseEvent) => {
    if (activeLevel === 'world') {
      const blockSelection = deriveWorldBlockAtPoint({
        runId: frame.runId,
        tick: frame.tick,
        world: frame.world,
        resourceRows,
        resourceColumns,
        point: { x: parseFloat(target.style.left), y: parseFloat(target.style.top) }
      });
      onSelectTarget?.(blockSelection);
      return;
    }

    const targetCellSelection = createCellSelection({
      cellId: target.id,
      runId: frame.runId,
      tick: frame.tick
    });

    if (event.shiftKey && onSelectTarget && currentSelection) {
      const nextSelection = toggleSelectionSetMember(currentSelection, targetCellSelection);
      onSelectTarget(nextSelection);
      return;
    }

    onSelectCell(target.id);
    onSelectTarget?.(targetCellSelection);
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

    if (activeLevel === 'world') {
      const rect = event.currentTarget.getBoundingClientRect();
      const point = { x: event.clientX - rect.left, y: event.clientY - rect.top };
      const blockSelection = deriveWorldBlockAtPoint({
        runId: frame.runId,
        tick: frame.tick,
        world: frame.world,
        resourceRows,
        resourceColumns,
        point
      });
      onSelectTarget?.(blockSelection);
      return;
    }

    onSelectCell(null);
    onSelectTarget?.(createNoneSelection());
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
      {(!isReady || debugProjections?.status === 'loading') ? (
        <div className="world-loading-overlay" aria-label="Loading world projection">
          <div className="loading-spinner" aria-hidden="true" />
          <span>{debugProjections?.status === 'loading' ? 'Loading World & Resource Grids...' : 'Initializing World Viewer...'}</span>
        </div>
      ) : null}
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
        {onExportScreenshot ? (
          <button type="button" onClick={onExportScreenshot} aria-label={uiText.controls.exportViewerPng}>
            PNG
          </button>
        ) : null}
        {onToggleFullScreen ? (
          <button
            type="button"
            onClick={onToggleFullScreen}
            aria-label={isFullScreen ? uiText.controls.exitFullScreen : uiText.controls.enterStartFullScreen}
          >
            {isFullScreen ? 'Exit' : 'Full'}
          </button>
        ) : null}
        <span aria-label={uiText.viewer.zoomLabel}>{formatMapScaleLabel(frame.world, viewport, camera.scale)}</span>
      </div>
      {debugLayerPlan ? (
        <aside
          className={isDebugOverlayCollapsed ? 'debug-visualization-mode collapsed' : 'debug-visualization-mode'}
          aria-label="Debug Visualization Mode"
          onMouseDown={(event) => event.stopPropagation()}
          onPointerDown={(event) => event.stopPropagation()}
          onWheel={(event) => event.stopPropagation()}
          onClick={(event) => event.stopPropagation()}
        >
          <div className="debug-mode-header" style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', width: '100%' }}>
            <div style={{ display: 'flex', gap: '6px', alignItems: 'center' }}>
              <strong>Debug</strong>
              <span>{debugLayerPlan.interpolationLabel}</span>
              {debugProjections?.status === 'available' ? (
                <span>{`Tick ${debugProjections.tick}`}</span>
              ) : debugProjections?.status === 'loading' ? (
                <span>{`Tick ${debugProjections.requestedTick}`}</span>
              ) : debugProjections?.status === 'stale' ? (
                <span>{`Tick ${debugProjections.tick}`}</span>
              ) : null}
            </div>
            <button
              type="button"
              className="debug-collapse-btn"
              onClick={() => setIsDebugOverlayCollapsed(!isDebugOverlayCollapsed)}
              aria-label={isDebugOverlayCollapsed ? 'Expand debug overlay' : 'Collapse debug overlay'}
              style={{
                padding: '2px 6px',
                fontSize: '11px',
                borderRadius: '4px',
                background: 'rgba(255, 255, 255, 0.1)',
                border: '1px solid rgba(255, 255, 255, 0.2)',
                color: '#dce6f1',
                cursor: 'pointer'
              }}
            >
              {isDebugOverlayCollapsed ? 'Expand ▲' : 'Minimize ▼'}
            </button>
          </div>

          {!isDebugOverlayCollapsed && (
            <>
              <div className="debug-mode-actions">
            <button
              type="button"
              onClick={() => setDebugLayerMode(debugLayerMode === 'exact' ? 'smooth' : 'exact')}
              aria-label={
                debugLayerMode === 'exact'
                  ? 'Switch debug layers to Smooth interpolation'
                  : 'Switch debug layers to Exact interpolation'
              }
            >
              {debugLayerMode === 'exact' ? 'Smooth' : 'Exact'}
            </button>
            <label>
              <input type="checkbox" checked readOnly />
              Resource / Field
            </label>
            <label>
              <input aria-label="Spatial index overlay unavailable" type="checkbox" disabled />
              Spatial index
            </label>
            <input
              type="search"
              className="debug-mode-search"
              value={mapSearchQuery}
              onChange={(event) => setMapSearchQuery(event.target.value)}
              aria-label="Search cells or resource layers"
              placeholder="Search id/layer"
            />
          </div>
          <div className="debug-mode-body">
            {debugProjections?.status === 'available' ? (
              <>
                <span>VisualWorldProjection</span>
                {normalizedMapSearchQuery ? <span>{`Search match: ${mapSearchQuery}`}</span> : null}
                {normalizedMapSearchQuery ? (
                  <span className="debug-mode-muted">Unsupported: process/contact/history search</span>
                ) : null}
                {debugLayerPlan.totalResourceLayerCount > 0 ? (
                  <span>{`Resource layers ${debugLayerPlan.resources.length} of ${debugLayerPlan.totalResourceLayerCount}`}</span>
                ) : null}
                {debugLayerPlan.resources.filter((layer) => matchesResourceLayerSearch(layer, normalizedMapSearchQuery)).map((layer) => (
                  <span key={`resource-${layer.layerIndex}`} className="debug-resource-legend-row">
                    <span
                      className="debug-resource-swatch"
                      style={{ backgroundColor: layer.colorHex }}
                      aria-hidden="true"
                    />
                    {layer.legendLabel}
                  </span>
                ))}
                {debugLayerPlan.hiddenResourceLayerCount > 0 ? (
                  <span className="debug-mode-muted">{`+${debugLayerPlan.hiddenResourceLayerCount} resource layers hidden`}</span>
                ) : null}
                {debugLayerPlan.fields.map((field) => (
                  <span key={`field-${field.fieldId}`}>{field.sampledValueLabel}</span>
                ))}
                {debugLayerPlan.missingProjectionWarnings.map((warning) => (
                  <span key={warning} className="debug-mode-warning">{warning}</span>
                ))}
                {truthState.resourceLayer.state === 'missing' ? (
                  <span className="debug-mode-warning">Missing live projection</span>
                ) : null}
              </>
            ) : debugLayerPlan.status === 'loading' || debugLayerPlan.status === 'stale' ? (
              <>
                <span>{debugLayerPlan.status === 'loading' ? 'VisualWorldProjection loading' : 'VisualWorldProjection stale'}</span>
                <span className="debug-mode-warning">{debugLayerPlan.reason}</span>
              </>
            ) : (
              <span className="debug-mode-warning">{debugLayerPlan.reason}</span>
            )}
          </div>
            </>
          )}
        </aside>
      ) : null}
      <div className="world-hit-targets" aria-label={uiText.viewer.hitTargetsAriaLabel}>
        {buildViewerHitTargets(frame, selectedCellId, viewport, camera).map((target) => {
          const isSearchMatch = matchesCellSearch(frame, target.id, normalizedMapSearchQuery);
          return (
            <Fragment key={target.id}>
              <button
                type="button"
                className={isSearchMatch ? 'cell-hotspot search-match' : 'cell-hotspot'}
                data-map-affordance={normalizedMapSearchQuery && isSearchMatch ? 'search-match' : undefined}
                data-semantic-level={target.detail.level}
                data-lifecycle-state={target.detail.lifecycleState}
                style={target.style}
                aria-hidden={normalizedMapSearchQuery && !isSearchMatch ? 'true' : undefined}
                onMouseDown={(event) => event.stopPropagation()}
                onPointerDown={(event) => event.stopPropagation()}
                onClick={(event) => {
                  event.stopPropagation();
                  handleCellHotspotClick(target, event);
                }}
                aria-label={target.ariaLabel}
              />
              {target.detail.showLabel ? (
                <span
                  className={target.selected ? 'cell-detail-label selected' : 'cell-detail-label'}
                  data-map-affordance={target.selected ? 'selected-foreground' : undefined}
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

function matchesCellSearch(frame: WorldFrame, cellId: CellId, query: string) {
  if (!query) {
    return true;
  }

  const cell = frame.cells.find((candidate) => candidate.id === cellId);
  if (!cell) {
    return false;
  }

  return [
    cell.id,
    cell.roleHint,
    cell.lifecycle === undefined ? '' : `lifecycle ${cell.lifecycle}`,
    ...((cell.materials ?? []).map((material) => `material ${material.materialTypeId}`)),
    ...((cell.internalResources ?? []).map((resource) => `internal resource ${resource.resourceTypeId}`)),
    ...((cell.localExternalResources ?? []).map((resource) => `local external resource ${resource.resourceTypeId}`))
  ].some((value) => value.toLowerCase().includes(query));
}

function matchesResourceLayerSearch(
  layer: NonNullable<ReturnType<typeof buildDebugLayerPlan>['resources'][number]>,
  query: string
) {
  if (!query) {
    return true;
  }

  return [
    `layer ${layer.layerIndex}`,
    String(layer.layerIndex),
    layer.channelLabel,
    layer.availability,
    layer.legendLabel
  ].some((value) => value.toLowerCase().includes(query));
}

function isHitTargetInsideRectangle(
  target: ViewerHitTarget,
  rect: { startX: number; startY: number; currentX: number; currentY: number }
) {
  const x = parseFloat(target.style.left);
  const y = parseFloat(target.style.top);
  const minX = Math.min(rect.startX, rect.currentX);
  const maxX = Math.max(rect.startX, rect.currentX);
  const minY = Math.min(rect.startY, rect.currentY);
  const maxY = Math.max(rect.startY, rect.currentY);

  return x >= minX && x <= maxX && y >= minY && y <= maxY;
}
