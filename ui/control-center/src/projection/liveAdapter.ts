import type { LiveWorldFrameProjection } from '../runner/alifDecoder';
import type { WorldFrame } from './types';

interface LiveFrameContext {
  runId: string;
  scenarioName: string;
}

export function liveProjectionToWorldFrame(
  projection: LiveWorldFrameProjection,
  context: LiveFrameContext
): WorldFrame {
  let maxX = 1200;
  let maxY = 800;
  const cells: WorldFrame['cells'] = [];

  for (const cell of projection.cells) {
    const x = nonNegativeFiniteOrZero(cell.x);
    const y = nonNegativeFiniteOrZero(cell.y);
    const radius = sanitizeRadius(cell.radius);

    maxX = Math.max(maxX, x + radius * 2);
    maxY = Math.max(maxY, y + radius * 2);

    cells.push({
      id: String(cell.id),
      x,
      y,
      radius,
      energy: clamp01(finiteOrZero(cell.energy)),
      integrity: lifecycleToIntegrity(cell.lifecycle),
      generation: 0,
      roleHint: lifecycleLabel(cell.lifecycle),
      lifecycle: cell.lifecycle
    });
  }

  return {
    schemaVersion: 'WorldFrameProjection/v1',
    source: 'live',
    runId: context.runId,
    scenarioName: context.scenarioName,
    tick: projection.committedTick,
    world: {
      width: Math.ceil(maxX),
      height: Math.ceil(maxY)
    },
    resources: [],
    cells,
    summary: {
      heat: projection.heat,
      waste: projection.waste,
      projectionSequence: projection.projectionSequence,
      previousTick: projection.previousCommittedTick,
      generatedAtMs: projection.wallClockGeneratedAtMs
    }
  };
}

function finiteOrZero(value: number) {
  return Number.isFinite(value) ? value : 0;
}

function nonNegativeFiniteOrZero(value: number) {
  return Math.max(0, finiteOrZero(value));
}

function sanitizeRadius(value: number) {
  return Math.max(2, finiteOrZero(value));
}

function clamp01(value: number) {
  return Math.max(0, Math.min(1, value));
}

function lifecycleToIntegrity(lifecycle: number) {
  return lifecycle === 3 ? 0 : 1;
}

function lifecycleLabel(lifecycle: number) {
  if (lifecycle === 0) {
    return 'alive lifecycle state';
  }
  if (lifecycle === 1) {
    return 'stressed lifecycle state';
  }
  if (lifecycle === 2) {
    return 'dormant lifecycle state';
  }
  if (lifecycle === 3) {
    return 'dead lifecycle state';
  }
  return `lifecycle ${lifecycle}`;
}
