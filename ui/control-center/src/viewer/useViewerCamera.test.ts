import { describe, expect, it } from 'vitest';
import { createViewerCameraState, viewerCameraReducer } from './useViewerCamera';

describe('viewerCameraReducer', () => {
  it('zooms around a viewer point', () => {
    const state = createViewerCameraState();

    expect(viewerCameraReducer(state, {
      type: 'zoom-at',
      point: { x: 300, y: 200 },
      scaleFactor: 2
    }).camera).toEqual({ x: -300, y: -200, scale: 2 });
  });

  it('tracks drag movement without selecting on the same click', () => {
    let state = createViewerCameraState();
    state = viewerCameraReducer(state, { type: 'drag-start', pointerId: 1, point: { x: 10, y: 10 } });
    state = viewerCameraReducer(state, { type: 'drag-move', pointerId: 1, point: { x: 15, y: 4 } });

    expect(state.camera).toEqual({ x: 5, y: -6, scale: 1 });
    expect(state.dragMoved).toBe(true);
  });
});
