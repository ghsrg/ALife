import type { CellId, CellProjection, WorldFrame } from '../projection/types';

export interface ViewportSize {
  width: number;
  height: number;
}

export interface RenderedCellGeometry {
  id: CellId;
  x: number;
  y: number;
  physicalRadiusPx: number;
  displayRadiusPx: number;
  presentationMinimumApplied: boolean;
}

export const MIN_CELL_DISPLAY_RADIUS_PX = 7;

export function projectCellForRender(
  cell: CellProjection,
  frame: Pick<WorldFrame, 'world'>,
  viewport: ViewportSize
): RenderedCellGeometry {
  const scaleX = viewport.width / frame.world.width;
  const scaleY = viewport.height / frame.world.height;
  const radiusScale = Math.min(scaleX, scaleY);
  const physicalRadiusPx = cell.radius * radiusScale;
  const displayRadiusPx = Math.max(MIN_CELL_DISPLAY_RADIUS_PX, physicalRadiusPx);

  return {
    id: cell.id,
    x: cell.x * scaleX,
    y: cell.y * scaleY,
    physicalRadiusPx,
    displayRadiusPx,
    presentationMinimumApplied: displayRadiusPx !== physicalRadiusPx
  };
}
