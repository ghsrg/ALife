import { useReducer } from 'react';
import {
  DEFAULT_VIEWER_CAMERA,
  fitCameraToWorld,
  panCamera,
  resetCamera,
  zoomCameraAtPoint,
  type ScreenPoint,
  type Size,
  type ViewerCamera
} from './viewerNavigation';

export interface ViewerCameraState {
  camera: ViewerCamera;
  dragStart: { pointerId: number; x: number; y: number } | null;
  dragMoved: boolean;
}

export type ViewerCameraAction =
  | { type: 'zoom-at'; point: ScreenPoint; scaleFactor: number }
  | { type: 'fit'; world: Size; viewport: Size }
  | { type: 'reset' }
  | { type: 'drag-start'; pointerId: number; point: ScreenPoint }
  | { type: 'drag-move'; pointerId: number; point: ScreenPoint }
  | { type: 'drag-end'; pointerId: number }
  | { type: 'clear-drag-moved' };

export function createViewerCameraState(camera = DEFAULT_VIEWER_CAMERA): ViewerCameraState {
  return {
    camera,
    dragStart: null,
    dragMoved: false
  };
}

export function viewerCameraReducer(
  state: ViewerCameraState,
  action: ViewerCameraAction
): ViewerCameraState {
  switch (action.type) {
    case 'zoom-at':
      return {
        ...state,
        camera: zoomCameraAtPoint(state.camera, action.point, action.scaleFactor)
      };
    case 'fit':
      return {
        ...state,
        camera: fitCameraToWorld(action.world, action.viewport)
      };
    case 'reset':
      return {
        ...state,
        camera: resetCamera()
      };
    case 'drag-start':
      return {
        ...state,
        dragStart: { pointerId: action.pointerId, x: action.point.x, y: action.point.y },
        dragMoved: false
      };
    case 'drag-move': {
      if (state.dragStart === null || state.dragStart.pointerId !== action.pointerId) {
        return state;
      }

      const dx = action.point.x - state.dragStart.x;
      const dy = action.point.y - state.dragStart.y;

      return {
        ...state,
        camera: dx === 0 && dy === 0 ? state.camera : panCamera(state.camera, { dx, dy }),
        dragStart: { pointerId: action.pointerId, x: action.point.x, y: action.point.y },
        dragMoved: state.dragMoved || dx !== 0 || dy !== 0
      };
    }
    case 'drag-end':
      if (state.dragStart?.pointerId !== action.pointerId) {
        return state;
      }

      return {
        ...state,
        dragStart: null
      };
    case 'clear-drag-moved':
      return {
        ...state,
        dragMoved: false
      };
  }
}

export function useViewerCamera(initialCamera = DEFAULT_VIEWER_CAMERA) {
  return useReducer(viewerCameraReducer, initialCamera, createViewerCameraState);
}
