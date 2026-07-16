import { describe, expect, it } from 'vitest';
import {
  DEFAULT_VIEWER_CAMERA,
  fitCameraToWorld,
  panCamera,
  resetCamera,
  zoomCameraAtPoint
} from './viewerNavigation';

describe('viewerNavigation', () => {
  it('zooms around the pointer so the world point under the cursor stays fixed', () => {
    const camera = zoomCameraAtPoint(
      DEFAULT_VIEWER_CAMERA,
      { x: 300, y: 200 },
      2
    );

    expect(camera.scale).toBe(2);
    expect(camera.x).toBe(-300);
    expect(camera.y).toBe(-200);
  });

  it('clamps zoom to the supported range', () => {
    expect(zoomCameraAtPoint(DEFAULT_VIEWER_CAMERA, { x: 0, y: 0 }, 0.01).scale).toBe(0.5);
    expect(zoomCameraAtPoint(DEFAULT_VIEWER_CAMERA, { x: 0, y: 0 }, 20).scale).toBe(6);
  });

  it('pans by screen-space delta without changing zoom', () => {
    expect(panCamera({ x: 10, y: 20, scale: 2 }, { dx: 5, dy: -8 })).toEqual({
      x: 15,
      y: 12,
      scale: 2
    });
  });

  it('fits the world into the viewport with stable margins', () => {
    expect(fitCameraToWorld({ width: 1200, height: 800 }, { width: 600, height: 600 })).toEqual({
      x: 24,
      y: 116,
      scale: 0.46
    });
  });

  it('resets to the default camera', () => {
    expect(resetCamera()).toEqual(DEFAULT_VIEWER_CAMERA);
  });
});
