import type { CellId, CellProjection, UiFixture, WorldFrame } from './types';

export function loadFixtureFrame(fixture: UiFixture): WorldFrame {
  return fixture.frame;
}

export function selectCell(frame: WorldFrame, cellId: CellId | null): CellProjection | null {
  if (cellId === null || cellId === undefined) {
    return null;
  }

  return frame.cells.find((cell) => String(cell.id) === String(cellId)) ?? null;
}

