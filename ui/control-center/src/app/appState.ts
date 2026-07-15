import { createStore } from 'zustand/vanilla';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { loadFixtureFrame, selectCell } from '../projection/fixtureAdapter';
import type { CellId, CellProjection, WorldFrame } from '../projection/types';

export type ThemeMode = 'dark' | 'light';

export interface AppState {
  frame: WorldFrame;
  selectedCellId: CellId | null;
  selectedCell: CellProjection | null;
  theme: ThemeMode;
}

export interface AppActions {
  selectCell: (cellId: CellId | null) => void;
  setTheme: (theme: ThemeMode) => void;
}

export type AppStore = AppState & AppActions;

export function createAppStore(initialFrame = loadFixtureFrame(ui1aFixture)) {
  return createStore<AppStore>((set, get) => ({
    frame: initialFrame,
    selectedCellId: initialFrame.cells[0]?.id ?? null,
    selectedCell: initialFrame.cells[0] ?? null,
    theme: 'dark',
    selectCell: (cellId) => {
      const selectedCell = selectCell(get().frame, cellId);
      set({
        selectedCellId: selectedCell?.id ?? null,
        selectedCell
      });
    },
    setTheme: (theme) => set({ theme })
  }));
}
