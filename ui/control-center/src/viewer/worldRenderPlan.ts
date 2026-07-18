import type { CellId, WorldFrame } from '../projection/types';
import { projectCellForNavigatedRender } from './renderGeometry';
import { buildCellSemanticDetail, type LifecycleVisualState, type SemanticZoomLevel } from './semanticDetail';
import { DEFAULT_VIEWER_CAMERA, type ViewerCamera } from './viewerNavigation';

export interface RenderPlanCell {
  id: CellId;
  x: number;
  y: number;
  radius: number;
  selected: boolean;
  lifecycleState: LifecycleVisualState;
  energyRatio: number;
  integrityRatio: number;
  semanticLevel: SemanticZoomLevel;
  showMetricRings: boolean;
  label: string;
}

export interface WorldRenderPlan {
  cells: RenderPlanCell[];
  hasResourceField: boolean;
}

export function createWorldRenderPlan(
  frame: WorldFrame,
  selectedCellId: CellId | null,
  viewport: { width: number; height: number },
  camera: ViewerCamera = DEFAULT_VIEWER_CAMERA
): WorldRenderPlan {
  return {
    hasResourceField: frame.resources.length > 0 && (frame.resources[0]?.length ?? 0) > 0,
    cells: frame.cells.map((cell) => {
      const geometry = projectCellForNavigatedRender(cell, frame, viewport, camera);
      const selected = cell.id === selectedCellId;
      const detail = buildCellSemanticDetail(cell, {
        displayRadiusPx: geometry.displayRadiusPx,
        selected
      });

      return {
        id: cell.id,
        x: geometry.x,
        y: geometry.y,
        radius: geometry.displayRadiusPx,
        selected,
        lifecycleState: detail.lifecycleState,
        energyRatio: detail.energyRatio,
        integrityRatio: detail.integrityRatio,
        semanticLevel: detail.level,
        showMetricRings: detail.showMetricRings,
        label: detail.label
      };
    })
  };
}
