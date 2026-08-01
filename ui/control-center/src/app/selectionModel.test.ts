import { describe, expect, it } from 'vitest';
import {
  createCellSelection,
  deriveWorldBlockAtPoint,
  createNoneSelection,
  createSelectionSet,
  createWorldBlockSelection,
  isSelectionCompatibleWithLevel,
  toggleSelectionSetMember
} from './selectionModel';

describe('selectionModel', () => {
  it('creates typed none selection', () => {
    expect(createNoneSelection()).toEqual({ kind: 'none' });
  });

  it('creates typed Cell selection with run and displayed Tick context', () => {
    expect(createCellSelection({ cellId: 'cell-a', runId: 'run-1', tick: 42 })).toEqual({
      kind: 'cell',
      cellId: 'cell-a',
      runId: 'run-1',
      tick: 42
    });
  });

  it('creates typed World block selection with bounds and grid coordinates', () => {
    expect(
      createWorldBlockSelection({
        blockX: 2,
        blockY: 3,
        runId: 'run-1',
        tick: 42,
        bounds: { x: 20, y: 30, width: 10, height: 10 },
        completeness: 'bounded'
      })
    ).toEqual({
      kind: 'world-block',
      blockX: 2,
      blockY: 3,
      runId: 'run-1',
      tick: 42,
      bounds: { x: 20, y: 30, width: 10, height: 10 },
      completeness: 'bounded'
    });
  });

  it('creates compatible selection sets without losing target context', () => {
    const first = createCellSelection({ cellId: 'cell-a', runId: 'run-1', tick: 42 });
    const second = createCellSelection({ cellId: 'cell-b', runId: 'run-1', tick: 42 });

    expect(createSelectionSet({ targets: [first, second], runId: 'run-1', tick: 42 })).toEqual({
      kind: 'selection-set',
      targetKind: 'cell',
      targets: [first, second],
      runId: 'run-1',
      tick: 42
    });
  });

  it('derives a World block from resource grid geometry and screen-to-world point', () => {
    expect(
      deriveWorldBlockAtPoint({
        world: { width: 200, height: 100 },
        resourceRows: 2,
        resourceColumns: 2,
        point: { x: 150, y: 75 },
        runId: 'run-1',
        tick: 42
      })
    ).toEqual({
      kind: 'world-block',
      blockX: 1,
      blockY: 1,
      runId: 'run-1',
      tick: 42,
      bounds: { x: 100, y: 50, width: 100, height: 50 },
      completeness: 'bounded'
    });
  });

  it('falls back to one full-world block when no resource grid exists', () => {
    expect(
      deriveWorldBlockAtPoint({
        world: { width: 200, height: 100 },
        resourceRows: 0,
        resourceColumns: 0,
        point: { x: 150, y: 75 },
        runId: 'run-1',
        tick: 42
      })
    ).toMatchObject({
      kind: 'world-block',
      blockX: 0,
      blockY: 0,
      bounds: { x: 0, y: 0, width: 200, height: 100 },
      completeness: 'unavailable'
    });
  });

  it('toggles compatible Cell targets into a selection set', () => {
    const first = createCellSelection({ cellId: 'cell-a', runId: 'run-1', tick: 42 });
    const second = createCellSelection({ cellId: 'cell-b', runId: 'run-1', tick: 42 });

    expect(toggleSelectionSetMember(first, second)).toEqual({
      kind: 'selection-set',
      targetKind: 'cell',
      targets: [first, second],
      runId: 'run-1',
      tick: 42
    });
  });

  it('removes an existing target when toggled inside a compatible selection set', () => {
    const first = createCellSelection({ cellId: 'cell-a', runId: 'run-1', tick: 42 });
    const second = createCellSelection({ cellId: 'cell-b', runId: 'run-1', tick: 42 });
    const set = createSelectionSet({ targets: [first, second], runId: 'run-1', tick: 42 });

    expect(toggleSelectionSetMember(set, second)).toEqual(first);
  });

  it('marks selections as compatible only with levels that can inspect them', () => {
    expect(isSelectionCompatibleWithLevel(
      createCellSelection({ cellId: 'cell-a', runId: 'run-1', tick: 42 }),
      'cells'
    )).toBe(true);
    expect(isSelectionCompatibleWithLevel(
      createCellSelection({ cellId: 'cell-a', runId: 'run-1', tick: 42 }),
      'world'
    )).toBe(false);
    expect(isSelectionCompatibleWithLevel(
      createWorldBlockSelection({
        blockX: 2,
        blockY: 3,
        runId: 'run-1',
        tick: 42,
        bounds: { x: 20, y: 30, width: 10, height: 10 },
        completeness: 'bounded'
      }),
      'world'
    )).toBe(true);
  });
});
