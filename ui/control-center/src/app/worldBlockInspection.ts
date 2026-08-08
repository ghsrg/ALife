import type { DebugFieldLayer, DebugProjectionState, DebugResourceLayer, WorldFrame } from '../projection/types';
import type { WorldBlockSelection } from './selectionModel';

export interface WorldBlockInspectionItem {
  id: string;
  label: string;
  value: number;
}

export interface WorldBlockInspection {
  blockX: number;
  blockY: number;
  bounds: WorldBlockSelection['bounds'];
  completeness: WorldBlockSelection['completeness'];
  resources: WorldBlockInspectionItem[];
  fields: WorldBlockInspectionItem[];
}

export function buildWorldBlockInspection(args: {
  frame: WorldFrame;
  debugProjections: DebugProjectionState;
  selection: WorldBlockSelection;
}): WorldBlockInspection {
  const resourceLayers = args.debugProjections.status === 'available'
    ? args.debugProjections.visualWorld.resourceLayers
    : [];
  const fieldLayers = args.debugProjections.status === 'available'
    ? args.debugProjections.visualWorld.fieldLayers ?? []
    : [];

  return {
    blockX: args.selection.blockX,
    blockY: args.selection.blockY,
    bounds: args.selection.bounds,
    completeness: args.selection.completeness,
    resources: resourceLayers
      .map((layer) => readResourceLayerAt(layer, args.selection.blockX, args.selection.blockY))
      .filter((item): item is WorldBlockInspectionItem => item !== null),
    fields: fieldLayers
      .map((layer) => readFieldLayerAt(layer, args.selection.blockX, args.selection.blockY))
      .filter((item): item is WorldBlockInspectionItem => item !== null)
  };
}

function readResourceLayerAt(
  layer: DebugResourceLayer,
  x: number,
  y: number
): WorldBlockInspectionItem | null {
  const cell = layer.cells.find((candidate) => candidate.x === x && candidate.y === y);
  if (!cell) {
    return null;
  }

  return {
    id: layer.resourceId,
    label: layer.resourceId,
    value: cell.amount
  };
}

function readFieldLayerAt(
  layer: DebugFieldLayer,
  x: number,
  y: number
): WorldBlockInspectionItem | null {
  const cell = layer.cells.find((candidate) => candidate.x === x && candidate.y === y);
  if (!cell) {
    return null;
  }

  return {
    id: layer.fieldId,
    label: titleCase(layer.fieldId),
    value: cell.value
  };
}

function titleCase(value: string) {
  return value
    .split(/[_\s-]+/)
    .filter(Boolean)
    .map((part) => `${part.charAt(0).toUpperCase()}${part.slice(1)}`)
    .join(' ');
}
