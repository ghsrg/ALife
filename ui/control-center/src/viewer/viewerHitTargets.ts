import type { CellId, WorldFrame } from '../projection/types';
import { projectCellForNavigatedRender, type ViewportSize } from './renderGeometry';
import { buildCellSemanticDetail } from './semanticDetail';
import { DEFAULT_VIEWER_CAMERA, type ViewerCamera } from './viewerNavigation';

export interface ViewerHitTarget {
  id: CellId;
  selected: boolean;
  detail: ReturnType<typeof buildCellSemanticDetail>;
  style: {
    left: string;
    top: string;
    width: string;
    height: string;
  };
  labelStyle: {
    left: string;
    top: string;
  };
  ariaLabel: string;
}

export function buildViewerHitTargets(
  frame: WorldFrame,
  selectedCellId: CellId | null,
  viewport: ViewportSize,
  camera: ViewerCamera = DEFAULT_VIEWER_CAMERA
): ViewerHitTarget[] {
  return frame.cells.map((cell) => {
    const geometry = projectCellForNavigatedRender(cell, frame, viewport, camera);
    const selected = cell.id === selectedCellId;
    const detail = buildCellSemanticDetail(cell, {
      displayRadiusPx: geometry.displayRadiusPx,
      selected
    });
    const diameter = `${geometry.interactionRadiusPx * 2}px`;

    return {
      id: cell.id,
      selected,
      detail,
      style: {
        left: `${geometry.x}px`,
        top: `${geometry.y}px`,
        width: diameter,
        height: diameter
      },
      labelStyle: {
        left: `${geometry.x}px`,
        top: `${geometry.y + geometry.displayRadiusPx + 10}px`
      },
      ariaLabel: `Select ${cell.id}`
    };
  });
}
