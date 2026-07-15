import { describe, expect, it } from 'vitest';
import type { LiveWorldFrameProjection } from '../runner/alifDecoder';
import { liveProjectionToWorldFrame } from './liveAdapter';

const liveFrame: LiveWorldFrameProjection = {
  schemaVersion: 'ALIF/v2',
  committedTick: 12,
  projectionSequence: 3,
  wallClockGeneratedAtMs: 1000,
  previousCommittedTick: 11,
  heat: 2.5,
  waste: 1.25,
  cells: [
    { id: 7, x: 10, y: 20, radius: 4, energy: 0.8, lifecycle: 1 },
    { id: 8, x: 80, y: 40, radius: 6, energy: 0.2, lifecycle: 2 }
  ]
};

describe('liveProjectionToWorldFrame', () => {
  it('maps ALIF live frame into the existing WorldFrame UI model', () => {
    const frame = liveProjectionToWorldFrame(liveFrame, {
      runId: 'run-live',
      scenarioName: 'demo_living_world'
    });

    expect(frame.schemaVersion).toBe('WorldFrameProjection/v1');
    expect(frame.source).toBe('live');
    expect(frame.runId).toBe('run-live');
    expect(frame.tick).toBe(12);
    expect(frame.summary?.heat).toBeCloseTo(2.5);
    expect(frame.summary?.waste).toBeCloseTo(1.25);
    expect(frame.cells).toEqual([
      expect.objectContaining({ id: '7', x: 10, y: 20, radius: 4, energy: 0.8 }),
      expect.objectContaining({ id: '8', x: 80, y: 40, radius: 6, energy: 0.2 })
    ]);
  });

  it('keeps a stable minimum world size when frame has few cells', () => {
    const frame = liveProjectionToWorldFrame({ ...liveFrame, cells: [] }, {
      runId: 'empty',
      scenarioName: 'empty'
    });

    expect(frame.world.width).toBeGreaterThanOrEqual(1200);
    expect(frame.world.height).toBeGreaterThanOrEqual(800);
    expect(frame.cells).toEqual([]);
  });

  it('sanitizes non-finite and out-of-bounds live cell values', () => {
    const frame = liveProjectionToWorldFrame({
      ...liveFrame,
      cells: [
        {
          id: 1,
          x: Number.NaN,
          y: Number.POSITIVE_INFINITY,
          radius: Number.NEGATIVE_INFINITY,
          energy: Number.NaN,
          lifecycle: 1
        },
        {
          id: 2,
          x: Number.POSITIVE_INFINITY,
          y: Number.NaN,
          radius: -5,
          energy: -0.25,
          lifecycle: 1
        },
        { id: 3, x: 15, y: 25, radius: 1, energy: 1.25, lifecycle: 1 }
      ]
    }, {
      runId: 'sanitized',
      scenarioName: 'sanitized'
    });

    expect(frame.cells).toEqual([
      expect.objectContaining({ id: '1', x: 0, y: 0, radius: 2, energy: 0 }),
      expect.objectContaining({ id: '2', x: 0, y: 0, radius: 2, energy: 0 }),
      expect.objectContaining({ id: '3', x: 15, y: 25, radius: 2, energy: 1 })
    ]);
    expect(Number.isFinite(frame.world.width)).toBe(true);
    expect(Number.isFinite(frame.world.height)).toBe(true);
  });
});
