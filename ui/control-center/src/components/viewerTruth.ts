import type { WorldFrame } from '../projection/types';
import { projectCellForRender, type ViewportSize } from '../viewer/renderGeometry';

export type ViewerTruthStateLevel = 'available' | 'missing' | 'physical-scale' | 'presentation-minimum';

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

export function buildViewerTruthState(frame: WorldFrame, viewport: ViewportSize): ViewerTruthState {
  const resourceCellCount = frame.resources.reduce((sum, row) => sum + row.length, 0);
  const renderedCells = frame.cells.map((cell) => projectCellForRender(cell, frame, viewport));
  const enlargedCount = renderedCells.filter((cell) => cell.presentationMinimumApplied).length;

  return {
    resourceLayer: resourceCellCount === 0
      ? {
          state: 'missing',
          label: 'Resources',
          value: 'Missing projection',
          note: 'Runner ALIF v2 does not include resource grid'
        }
      : {
          state: 'available',
          label: 'Resources',
          value: frame.source === 'live' ? 'Live grid' : 'Fixture grid',
          note: `${resourceCellCount} resource cells`
        },
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
