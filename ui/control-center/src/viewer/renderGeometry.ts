import type { CellId, CellProjection, WorldFrame } from '../projection/types';
import type { ViewerCamera } from './viewerNavigation';

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
  interactionRadiusPx: number;
  presentationMinimumApplied: boolean;
}

export const MIN_CELL_DISPLAY_RADIUS_PX = 7;
export const MIN_CELL_INTERACTION_RADIUS_PX = 18;

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
  const interactionRadiusPx = Math.max(MIN_CELL_INTERACTION_RADIUS_PX, displayRadiusPx);

  return {
    id: cell.id,
    x: cell.x * scaleX,
    y: cell.y * scaleY,
    physicalRadiusPx,
    displayRadiusPx,
    interactionRadiusPx,
    presentationMinimumApplied: displayRadiusPx !== physicalRadiusPx
  };
}

export function projectCellForNavigatedRender(
  cell: CellProjection,
  frame: Pick<WorldFrame, 'world'>,
  viewport: ViewportSize,
  camera: ViewerCamera
): RenderedCellGeometry {
  const base = projectCellForRender(cell, frame, viewport);
  const physicalRadiusPx = base.physicalRadiusPx * camera.scale;
  const displayRadiusPx = Math.max(MIN_CELL_DISPLAY_RADIUS_PX, physicalRadiusPx);
  const interactionRadiusPx = Math.max(MIN_CELL_INTERACTION_RADIUS_PX, displayRadiusPx);

  return {
    id: base.id,
    x: base.x * camera.scale + camera.x,
    y: base.y * camera.scale + camera.y,
    physicalRadiusPx,
    displayRadiusPx,
    interactionRadiusPx,
    presentationMinimumApplied: displayRadiusPx !== physicalRadiusPx
  };
}
