import type { MonitorDataState } from '../app/appState';
import type { WorldFrame } from '../projection/types';

export type MonitorStatState = 'available' | 'partial' | 'missing';

export interface MonitorStat {
  id: 'cells' | 'alive-dead' | 'cell-energy' | 'world' | 'resources';
  label: string;
  value: string;
  state: MonitorStatState;
  note?: string;
}

export function buildMonitorStats(frame: WorldFrame, _dataState: MonitorDataState): MonitorStat[] {
  const lifecycleValues = frame.cells
    .map((cell) => cell.lifecycle)
    .filter((value): value is number => typeof value === 'number');
  const alive = lifecycleValues.filter((value) => value === 1).length;
  const dead = lifecycleValues.filter((value) => value === 2).length;
  const unknown = frame.cells.length - lifecycleValues.length;
  const energy = frame.cells.reduce((sum, cell) => sum + cell.energy, 0);
  const resourceCells = frame.resources.reduce((sum, row) => sum + row.length, 0);

  return [
    { id: 'cells', label: 'Cells', value: String(frame.cells.length), state: 'available' },
    lifecycleValues.length === 0
      ? {
          id: 'alive-dead',
          label: 'Alive / Dead',
          value: 'Unavailable',
          state: 'missing',
          note: 'lifecycle projection unavailable'
        }
      : {
          id: 'alive-dead',
          label: 'Alive / Dead',
          value: `${alive} / ${dead}`,
          state: unknown > 0 ? 'partial' : 'available',
          ...(unknown > 0 ? { note: `${unknown} unknown` } : {})
        },
    {
      id: 'cell-energy',
      label: 'Projected Cell Energy',
      value: energy.toFixed(2),
      state: 'available',
      note: 'sum of projected cell buffers'
    },
    {
      id: 'world',
      label: 'World',
      value: `${frame.world.width} x ${frame.world.height}`,
      state: 'available'
    },
    resourceCells === 0
      ? {
          id: 'resources',
          label: 'Resources',
          value: 'Missing projection',
          state: 'missing',
          note: 'Runner ALIF v2 does not include resource grid'
        }
      : {
          id: 'resources',
          label: 'Resources',
          value: `${resourceCells} cells`,
          state: 'available',
          note: frame.source === 'live' ? 'live grid' : 'fixture grid'
        }
  ];
}
