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
export const VIEWER_FIT_MARGIN_PX = 24;

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
  const usableWidth = Math.max(1, viewport.width - VIEWER_FIT_MARGIN_PX * 2);
  const usableHeight = Math.max(1, viewport.height - VIEWER_FIT_MARGIN_PX * 2);
  const scale = Number(Math.min(usableWidth / world.width, usableHeight / world.height, MAX_VIEWER_ZOOM).toFixed(3));

  return {
    scale,
    x: Math.round((viewport.width - world.width * scale) / 2),
    y: Math.round((viewport.height - world.height * scale) / 2)
  };
}

export function formatMapScaleLabel(world: Size, viewport: Size, scale: number) {
  if (scale >= 1) {
    return '1:1 cell scale';
  }

  const fittedScale = fitCameraToWorld(world, viewport).scale || scale;
  const fullWorldRatio = 2600;
  const ratio = Math.max(1, Math.round((fittedScale / Math.max(scale, 0.001)) * fullWorldRatio));
  return `1:${ratio}`;
}

function clampZoom(scale: number) {
  return Math.max(MIN_VIEWER_ZOOM, Math.min(MAX_VIEWER_ZOOM, Number(scale.toFixed(3))));
}
