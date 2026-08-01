import type { CellId, ProjectionCompleteness } from '../projection/types';

export type AnalysisLevel = 'world' | 'cells' | 'organisms' | 'lineages' | 'evolution' | 'analytics';

export interface SelectionContext {
  runId: string;
  tick: number;
}

export interface WorldBlockBounds {
  x: number;
  y: number;
  width: number;
  height: number;
}

export type NoneSelection = { kind: 'none' };
export type CellSelection = { kind: 'cell'; cellId: CellId } & SelectionContext;
export type WorldBlockSelection = {
      kind: 'world-block';
      blockX: number;
      blockY: number;
      bounds: WorldBlockBounds;
      completeness: ProjectionCompleteness;
    } & SelectionContext;
export type ConcreteMonitorSelection = CellSelection | WorldBlockSelection;
export type SelectionSet = {
  kind: 'selection-set';
  targetKind: ConcreteMonitorSelection['kind'];
  targets: ConcreteMonitorSelection[];
} & SelectionContext;
export type MonitorSelection = NoneSelection | ConcreteMonitorSelection | SelectionSet;

export function createNoneSelection(): NoneSelection {
  return { kind: 'none' };
}

export function createCellSelection(args: SelectionContext & { cellId: CellId }): CellSelection {
  return {
    kind: 'cell',
    cellId: args.cellId,
    runId: args.runId,
    tick: args.tick
  };
}

export function createWorldBlockSelection(
  args: SelectionContext & {
    blockX: number;
    blockY: number;
    bounds: WorldBlockBounds;
    completeness: ProjectionCompleteness;
  }
): WorldBlockSelection {
  return {
    kind: 'world-block',
    blockX: args.blockX,
    blockY: args.blockY,
    runId: args.runId,
    tick: args.tick,
    bounds: args.bounds,
    completeness: args.completeness
  };
}

export function createSelectionSet(
  args: SelectionContext & {
    targets: ConcreteMonitorSelection[];
  }
): SelectionSet {
  const targetKind = args.targets[0]?.kind ?? 'cell';
  return {
    kind: 'selection-set',
    targetKind,
    targets: args.targets,
    runId: args.runId,
    tick: args.tick
  };
}

export function deriveWorldBlockAtPoint(args: SelectionContext & {
  world: { width: number; height: number };
  resourceRows: number;
  resourceColumns: number;
  point: { x: number; y: number };
}): MonitorSelection {
  const rows = args.resourceRows > 0 ? args.resourceRows : 1;
  const columns = args.resourceColumns > 0 ? args.resourceColumns : 1;
  const blockWidth = args.world.width / columns;
  const blockHeight = args.world.height / rows;
  const blockX = Math.min(columns - 1, Math.max(0, Math.floor(args.point.x / blockWidth)));
  const blockY = Math.min(rows - 1, Math.max(0, Math.floor(args.point.y / blockHeight)));

  return createWorldBlockSelection({
    blockX,
    blockY,
    runId: args.runId,
    tick: args.tick,
    bounds: {
      x: blockX * blockWidth,
      y: blockY * blockHeight,
      width: blockWidth,
      height: blockHeight
    },
    completeness: args.resourceRows > 0 && args.resourceColumns > 0 ? 'bounded' : 'unavailable'
  });
}

export function toggleSelectionSetMember(
  currentSelection: MonitorSelection,
  targetSelection: ConcreteMonitorSelection
): MonitorSelection {
  if (currentSelection.kind === 'none') {
    return targetSelection;
  }

  if (currentSelection.kind === 'selection-set') {
    if (currentSelection.targetKind !== targetSelection.kind) {
      return createSelectionSet({
        runId: targetSelection.runId,
        tick: targetSelection.tick,
        targets: [targetSelection]
      });
    }

    const targetIndex = currentSelection.targets.findIndex((candidate) => isSameSelectionTarget(candidate, targetSelection));
    if (targetIndex >= 0) {
      const nextTargets = currentSelection.targets.filter((_, index) => index !== targetIndex);
      if (nextTargets.length === 0) {
        return createNoneSelection();
      }
      if (nextTargets.length === 1) {
        return nextTargets[0];
      }
      return createSelectionSet({
        runId: targetSelection.runId,
        tick: targetSelection.tick,
        targets: nextTargets
      });
    }

    return createSelectionSet({
      runId: targetSelection.runId,
      tick: targetSelection.tick,
      targets: [...currentSelection.targets, targetSelection]
    });
  }

  if (currentSelection.kind !== targetSelection.kind) {
    return createSelectionSet({
      runId: targetSelection.runId,
      tick: targetSelection.tick,
      targets: [targetSelection]
    });
  }

  if (isSameSelectionTarget(currentSelection, targetSelection)) {
    return createNoneSelection();
  }

  return createSelectionSet({
    runId: targetSelection.runId,
    tick: targetSelection.tick,
    targets: [currentSelection, targetSelection]
  });
}

export function isSelectionCompatibleWithLevel(selection: MonitorSelection, level: AnalysisLevel) {
  if (selection.kind === 'none') {
    return true;
  }

  if (selection.kind === 'selection-set') {
    return isSelectionKindCompatibleWithLevel(selection.targetKind, level);
  }

  return isSelectionKindCompatibleWithLevel(selection.kind, level);
}

function isSelectionKindCompatibleWithLevel(
  selectionKind: ConcreteMonitorSelection['kind'],
  level: AnalysisLevel
) {
  if (selectionKind === 'cell') {
    return level === 'cells';
  }



  if (selectionKind === 'world-block') {
    return level === 'world';
  }

  return false;
}

function isSameSelectionTarget(left: ConcreteMonitorSelection, right: ConcreteMonitorSelection) {
  if (left.kind !== right.kind) {
    return false;
  }

  if (left.kind === 'cell' && right.kind === 'cell') {
    return left.cellId === right.cellId;
  }

  if (left.kind === 'world-block' && right.kind === 'world-block') {
    return left.blockX === right.blockX && left.blockY === right.blockY;
  }

  return false;
}
