import { describe, expect, it } from 'vitest';
import { ui1aFixture } from './ui1aFixture';

describe('ui1aFixture', () => {
  it('contains a deterministic WorldFrameProjection v1 frame', () => {
    expect(ui1aFixture.version).toBe('ui-1a-fixture/v1');
    expect(ui1aFixture.frame.schemaVersion).toBe('WorldFrameProjection/v1');
    expect(ui1aFixture.frame.runId).toBe('fixture-ui-1a');
    expect(ui1aFixture.frame.tick).toBe(128);
    expect(ui1aFixture.frame.cells).toHaveLength(3);
    expect(ui1aFixture.frame.resources).toHaveLength(3);
  });

  it('keeps fixture cells inside world bounds', () => {
    for (const cell of ui1aFixture.frame.cells) {
      expect(cell.x).toBeGreaterThanOrEqual(0);
      expect(cell.x).toBeLessThanOrEqual(ui1aFixture.frame.world.width);
      expect(cell.y).toBeGreaterThanOrEqual(0);
      expect(cell.y).toBeLessThanOrEqual(ui1aFixture.frame.world.height);
    }
  });
});
