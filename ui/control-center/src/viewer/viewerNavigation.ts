export interface ViewerCamera {
  x: number;
  y: number;
  scale: number;
}

export interface ScreenPoint {
  x: number;
  y: number;
}

export interface PanDelta {
  dx: number;
  dy: number;
}

export interface Size {
  width: number;
  height: number;
}

export const MIN_VIEWER_ZOOM = 0.5;
export const MAX_VIEWER_ZOOM = 24;
export const DEFAULT_VIEWER_CAMERA: ViewerCamera = { x: 0, y: 0, scale: 1 };
export const CELL_DETAIL_DIAMETER_PX = 20;
export const REFERENCE_CELL_RADIUS_WORLD_UNITS = 2;
export const FULL_WORLD_SCALE_DENOMINATOR = 2600;

export function resetCamera(): ViewerCamera {
  return DEFAULT_VIEWER_CAMERA;
}

export function panCamera(camera: ViewerCamera, delta: PanDelta): ViewerCamera {
  return {
    ...camera,
    x: camera.x + delta.dx,
    y: camera.y + delta.dy
  };
}

export function zoomCameraAtPoint(
  camera: ViewerCamera,
  point: ScreenPoint,
  scaleFactor: number
): ViewerCamera {
  const nextScale = clampZoom(camera.scale * scaleFactor);
  const worldX = (point.x - camera.x) / camera.scale;
  const worldY = (point.y - camera.y) / camera.scale;

  return {
    scale: nextScale,
    x: point.x - worldX * nextScale,
    y: point.y - worldY * nextScale
  };
}

export function fitCameraToWorld(world: Size, viewport: Size): ViewerCamera {
  void world;
  void viewport;
  return DEFAULT_VIEWER_CAMERA;
}

export function formatMapScaleLabel(world: Size, viewport: Size, scale: number) {
  const viewportWorldScale = Math.min(viewport.width / world.width, viewport.height / world.height);
  const detailScale =
    CELL_DETAIL_DIAMETER_PX /
    Math.max(0.001, REFERENCE_CELL_RADIUS_WORLD_UNITS * 2 * viewportWorldScale);

  if (scale >= detailScale) {
    return '1:1 cell scale';
  }

  const fittedScale = fitCameraToWorld(world, viewport).scale || scale;
  const scaleSpan = Math.max(0.001, detailScale - fittedScale);
  const distanceFromDetail = Math.max(0, detailScale - Math.max(scale, fittedScale));
  const ratio = Math.max(
    1,
    Math.round(1 + (FULL_WORLD_SCALE_DENOMINATOR - 1) * (distanceFromDetail / scaleSpan))
  );
  return `1:${ratio}`;
}

function clampZoom(scale: number) {
  return Math.max(MIN_VIEWER_ZOOM, Math.min(MAX_VIEWER_ZOOM, Number(scale.toFixed(3))));
}
