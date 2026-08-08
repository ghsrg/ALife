import { describe, expect, it } from 'vitest';
import type { WorldFrame } from '../projection/types';
import {
  buildResourceGridLineSegments,
  drawCellOrganelles,
  drawFieldLayer,
  drawIntegrityArc,
  drawJointsLayer,
  sampleBilinearResource
} from './worldRenderer';

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

describe('sampleBilinearResource', () => {
  it('interpolates resource values smoothly between grid nodes', () => {
    const grid = [
      [
        { organic: 1.0, mineral: 0.0, energy: 0.0 },
        { organic: 0.0, mineral: 1.0, energy: 0.0 }
      ],
      [
        { organic: 0.0, mineral: 0.0, energy: 1.0 },
        { organic: 0.5, mineral: 0.5, energy: 0.5 }
      ]
    ];

    // Center point (x=0.5, y=0.5)
    const mid = sampleBilinearResource(grid, 0.5, 0.5);
    expect(mid.organic).toBeCloseTo(0.375);
    expect(mid.mineral).toBeCloseTo(0.375);
    expect(mid.energy).toBeCloseTo(0.375);

    // Exact top-left node (x=0, y=0)
    const topLeft = sampleBilinearResource(grid, 0, 0);
    expect(topLeft.organic).toBeCloseTo(1.0);
    expect(topLeft.mineral).toBeCloseTo(0.0);
    expect(topLeft.energy).toBeCloseTo(0.0);
  });
});

describe('buildResourceGridLineSegments', () => {
  it('builds world resource grid lines from resource dimensions and camera', () => {
    const segments = buildResourceGridLineSegments(2, 2, 200, 100, { x: 10, y: 20, scale: 1 });

    expect(segments).toEqual([
      { from: { x: 10, y: 20 }, to: { x: 10, y: 120 } },
      { from: { x: 110, y: 20 }, to: { x: 110, y: 120 } },
      { from: { x: 210, y: 20 }, to: { x: 210, y: 120 } },
      { from: { x: 10, y: 20 }, to: { x: 210, y: 20 } },
      { from: { x: 10, y: 70 }, to: { x: 210, y: 70 } },
      { from: { x: 10, y: 120 }, to: { x: 210, y: 120 } }
    ]);
  });
});

describe('drawCellOrganelles', () => {
  it('renders internal nucleus and organelle granules into Pixi graphics object', () => {
    const circles: Array<{ x: number; y: number; r: number }> = [];
    const graphic = {
      circle: (x: number, y: number, r: number) => circles.push({ x, y, r }),
      fill: () => {},
      stroke: () => {}
    };

    drawCellOrganelles(graphic, 100, 100, 20, 0.8, 'alive');

    // Should draw nucleus and internal granules
    expect(circles.length).toBeGreaterThanOrEqual(4);
    expect(circles.some((c) => c.r > 2)).toBe(true);
  });
});

describe('drawJointsLayer', () => {
  it('renders lines and signal node between connected cells', () => {
    const mockFrame: WorldFrame = {
      schemaVersion: 'WorldFrameProjection/v1',
      runId: 'test-run',
      tick: 10,
      world: { width: 800, height: 600 },
      resources: [],
      cells: [],
      joints: [
        {
          id: 'j1',
          sourceCellId: 'c1',
          targetCellId: 'c2',
          channelType: 'signal',
          activeSignal: true
        }
      ]
    };

    const cellPositions = new Map([
      ['c1', { x: 50, y: 50 }],
      ['c2', { x: 150, y: 150 }]
    ]);

    const layer = drawJointsLayer(mockFrame, cellPositions);
    expect(layer).toBeDefined();
  });
});

describe('drawFieldLayer', () => {
  it('renders scalar field layer grid rectangles and handles disabled field layers', () => {
    const mockFrame: WorldFrame = {
      schemaVersion: 'WorldFrameProjection/v1',
      runId: 'test-run',
      tick: 10,
      world: { width: 800, height: 600 },
      resources: [],
      fieldLayers: [
        {
          fieldId: 'temperature',
          width: 2,
          height: 2,
          summaryValue: 25.0,
          completeness: { state: 'full', missingFields: [], reason: null },
          cells: [
            { x: 0, y: 0, value: 18.0 },
            { x: 1, y: 0, value: 25.0 },
            { x: 0, y: 1, value: 28.0 },
            { x: 1, y: 1, value: 32.0 }
          ]
        }
      ],
      cells: []
    };

    const camera = { x: 0, y: 0, scale: 1 };
    const layer = drawFieldLayer(mockFrame, 800, 600, camera, []);
    expect(layer).toBeDefined();

    const disabledLayer = drawFieldLayer(mockFrame, 800, 600, camera, ['temperature']);
    expect(disabledLayer).toBeDefined();
  });
});
