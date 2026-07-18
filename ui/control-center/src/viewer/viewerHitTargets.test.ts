import { describe, expect, it } from 'vitest';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { loadFixtureFrame } from '../projection/fixtureAdapter';
import { buildViewerHitTargets } from './viewerHitTargets';

describe('buildViewerHitTargets', () => {
  it('derives accessible hit targets from frame, viewport and camera', () => {
    const frame = loadFixtureFrame(ui1aFixture);

    const targets = buildViewerHitTargets(frame, 'cell-a', { width: 1200, height: 800 }, { x: 0, y: 0, scale: 1 });

    expect(targets[0]).toMatchObject({
      id: 'cell-a',
      selected: true,
      ariaLabel: 'Select cell-a'
    });
    expect(targets[0].style.width).toMatch(/px$/);
  });
});
