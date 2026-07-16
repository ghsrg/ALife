import { describe, expect, it } from 'vitest';
import type { WorldFrame } from '../projection/types';
import { buildMonitorStats } from './monitorStats';

const frame: WorldFrame = {
  schemaVersion: 'WorldFrameProjection/v1',
  source: 'live',
  runId: 'run-live',
  scenarioName: 'demo_living_world',
  tick: 42,
  world: { width: 1200, height: 800 },
  resources: [],
  cells: [
    { id: '1', x: 10, y: 20, radius: 4, energy: 0.8, integrity: 1, generation: 0, roleHint: 'alive lifecycle state', lifecycle: 1 },
    { id: '2', x: 30, y: 40, radius: 6, energy: 0.2, integrity: 0, generation: 0, roleHint: 'dead lifecycle state', lifecycle: 2 },
    { id: '3', x: 50, y: 60, radius: 8, energy: 0.5, integrity: 1, generation: 0, roleHint: 'lifecycle unknown' }
  ],
  summary: { heat: 2.5, waste: 1.25, projectionSequence: 7, previousTick: 41, generatedAtMs: 1000 }
};

describe('buildMonitorStats', () => {
  it('summarizes only values that exist in the current WorldFrame', () => {
    const stats = buildMonitorStats(frame, 'live');

    expect(stats).toEqual([
      { id: 'cells', label: 'Cells', value: '3', state: 'available' },
      { id: 'alive-dead', label: 'Alive / Dead', value: '1 / 1', state: 'partial', note: '1 unknown' },
      { id: 'cell-energy', label: 'Projected Cell Energy', value: '1.50', state: 'available', note: 'sum of projected cell buffers' },
      { id: 'world', label: 'World', value: '1200 x 800', state: 'available' },
      { id: 'resources', label: 'Resources', value: 'Missing projection', state: 'missing', note: 'Runner ALIF v2 does not include resource grid' }
    ]);
  });

  it('does not invent alive/dead counts when lifecycle is absent', () => {
    const stats = buildMonitorStats({
      ...frame,
      source: 'fixture',
      resources: [[{ organic: 0.1, mineral: 0.2, energy: 0.3 }]],
      cells: frame.cells.map(({ lifecycle: _lifecycle, ...cell }) => cell)
    }, 'fixture-idle');

    expect(stats.find((stat) => stat.id === 'alive-dead')).toEqual({
      id: 'alive-dead',
      label: 'Alive / Dead',
      value: 'Unavailable',
      state: 'missing',
      note: 'lifecycle projection unavailable'
    });
    expect(stats.find((stat) => stat.id === 'resources')).toEqual({
      id: 'resources',
      label: 'Resources',
      value: '1 cells',
      state: 'available',
      note: 'fixture grid'
    });
  });
});
