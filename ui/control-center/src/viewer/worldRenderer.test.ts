import { describe, expect, it } from 'vitest';
import { drawIntegrityArc } from './worldRenderer';

describe('drawIntegrityArc', () => {
  it('moves to the arc start before drawing so Pixi does not connect from a previous path', () => {
    const calls: Array<{ name: string; args: unknown[] }> = [];
    const graphic = {
      moveTo: (...args: unknown[]) => calls.push({ name: 'moveTo', args }),
      arc: (...args: unknown[]) => calls.push({ name: 'arc', args }),
      stroke: (...args: unknown[]) => calls.push({ name: 'stroke', args })
    };

    drawIntegrityArc(graphic, 100, 80, 20, 0.5);

    expect(calls[0]).toEqual({ name: 'moveTo', args: [100, 58] });
    expect(calls[1]).toEqual({ name: 'arc', args: [100, 80, 22, -Math.PI / 2, Math.PI / 2] });
    expect(calls[2]?.name).toBe('stroke');
  });
});
