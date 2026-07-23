import type { DebugProjectionState, WorldFrame } from '../projection/types';
import { projectCellForRender, type ViewportSize } from '../viewer/renderGeometry';

export type ViewerTruthStateLevel =
  | 'available'
  | 'loading'
  | 'stale'
  | 'missing'
  | 'physical-scale'
  | 'presentation-minimum';

export interface ViewerTruthItem {
  state: ViewerTruthStateLevel;
  label: string;
  value: string;
  note: string;
}

export interface ViewerTruthState {
  resourceLayer: ViewerTruthItem;
  cellScale: ViewerTruthItem;
}

export function buildViewerTruthState(
  frame: WorldFrame,
  viewport: ViewportSize,
  debugProjections?: DebugProjectionState
): ViewerTruthState {
  const resourceCellCount = frame.resources.reduce((sum, row) => sum + row.length, 0);
  const renderedCells = frame.cells.map((cell) => projectCellForRender(cell, frame, viewport));
  const enlargedCount = renderedCells.filter((cell) => cell.presentationMinimumApplied).length;

  return {
    resourceLayer: buildResourceLayerTruth(frame, resourceCellCount, debugProjections),
    cellScale: enlargedCount === 0
      ? {
          state: 'physical-scale',
          label: 'Cell size',
          value: 'Physical scale',
          note: 'display radius matches projected radius'
        }
      : {
          state: 'presentation-minimum',
          label: 'Cell size',
          value: 'Display minimum applied',
          note: `${enlargedCount} of ${frame.cells.length} cells enlarged for visibility`
        }
  };
}

function buildResourceLayerTruth(
  frame: WorldFrame,
  resourceCellCount: number,
  debugProjections?: DebugProjectionState
): ViewerTruthItem {
  if (resourceCellCount > 0) {
    if (
      frame.source === 'live' &&
      debugProjections?.status === 'available' &&
      debugProjections.runId === frame.runId &&
      debugProjections.tick < frame.tick
    ) {
      return {
        state: 'stale',
        label: 'Resources',
        value: 'Stale projection',
        note: `Debug projection Tick ${debugProjections.tick} is behind live Tick ${frame.tick}`
      };
    }

    return {
      state: 'available',
      label: 'Resources',
      value: frame.source === 'live' ? 'Live grid' : 'Fixture grid',
      note: `${resourceCellCount} resource cells`
    };
  }

  if (debugProjections?.status === 'loading') {
    return {
      state: 'loading',
      label: 'Resources',
      value: 'Loading projection',
      note: `${debugProjections.reason} for Tick ${debugProjections.requestedTick}`
    };
  }

  if (debugProjections?.status === 'stale') {
    return {
      state: 'stale',
      label: 'Resources',
      value: 'Stale projection',
      note: debugProjections.reason
    };
  }

  return {
    state: 'missing',
    label: 'Resources',
    value: 'Missing projection',
    note: 'Runner ALIF v2 does not include resource grid'
  };
}
