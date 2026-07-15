import { describe, expect, it } from 'vitest';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { loadFixtureFrame, selectCell } from './fixtureAdapter';

describe('fixtureAdapter', () => {
  it('loads the fixture frame without mutating projection data', () => {
    const frame = loadFixtureFrame(ui1aFixture);

    expect(frame).toBe(ui1aFixture.frame);
    expect(frame.schemaVersion).toBe('WorldFrameProjection/v1');
  });

  it('selects a cell by id from the loaded frame', () => {
    const frame = loadFixtureFrame(ui1aFixture);

    expect(selectCell(frame, 'cell-b')?.roleHint).toBe('boundary contact');
    expect(selectCell(frame, 'missing-cell')).toBeNull();
    expect(selectCell(frame, null)).toBeNull();
  });
});
