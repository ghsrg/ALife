import { describe, expect, it } from 'vitest';
import type { WorldFrame } from './types';
import {
  buildProjectionContext,
  buildUnavailableProjectionContext,
  describeProjectionContext
} from './projectionContext';

const baseFrame: WorldFrame = {
  schemaVersion: 'WorldFrameProjection/v1',
  source: 'live',
  runId: 'run-1',
  tick: 42,
  world: {
    width: 100,
    height: 80
  },
  resources: [],
  cells: []
};

describe('projection context', () => {
  it('describes live frames with version, source, completeness, run, and tick', () => {
    const context = buildProjectionContext(baseFrame, 'live');

    expect(context).toEqual({
      mode: 'live',
      schemaVersion: 'WorldFrameProjection/v1',
      source: 'live',
      completeness: 'partial',
      runId: 'run-1',
      tick: 42,
      isLive: true,
      isReadOnly: false,
      warning: 'Missing live resource projection'
    });
    expect(describeProjectionContext(context)).toBe(
      'Live Tick 42 - live - partial - WorldFrameProjection/v1'
    );
  });

  it('marks frozen frames as historical read-only views without changing their tick', () => {
    const context = buildProjectionContext(baseFrame, 'frozen');

    expect(context.mode).toBe('frozen');
    expect(context.tick).toBe(42);
    expect(context.isLive).toBe(false);
    expect(context.isReadOnly).toBe(true);
    expect(describeProjectionContext(context)).toContain('Frozen Tick 42');
  });

  it('represents unavailable ticks without substituting a nearby frame', () => {
    const context = buildUnavailableProjectionContext({
      runId: 'run-1',
      tick: 39,
      reason: 'Tick is outside bounded client history'
    });

    expect(context).toEqual({
      mode: 'unavailable',
      schemaVersion: 'WorldFrameProjection/v1',
      source: 'historical',
      completeness: 'unavailable',
      runId: 'run-1',
      tick: 39,
      isLive: false,
      isReadOnly: true,
      warning: 'Tick is outside bounded client history'
    });
    expect(describeProjectionContext(context)).toBe(
      'Unavailable Tick 39 - historical - unavailable - Tick is outside bounded client history'
    );
  });
});
