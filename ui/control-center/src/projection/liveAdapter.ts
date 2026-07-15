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
  const maxX = Math.max(1200, ...projection.cells.map((cell) => cell.x + cell.radius * 2));
  const maxY = Math.max(800, ...projection.cells.map((cell) => cell.y + cell.radius * 2));

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
    cells: projection.cells.map((cell) => ({
      id: String(cell.id),
      x: cell.x,
      y: cell.y,
      radius: Math.max(2, cell.radius),
      energy: clamp01(cell.energy),
      integrity: lifecycleToIntegrity(cell.lifecycle),
      generation: 0,
      roleHint: lifecycleLabel(cell.lifecycle),
      lifecycle: cell.lifecycle
    })),
    summary: {
      heat: projection.heat,
      waste: projection.waste,
      projectionSequence: projection.projectionSequence,
      previousTick: projection.previousCommittedTick,
      generatedAtMs: projection.wallClockGeneratedAtMs
    }
  };
}

function clamp01(value: number) {
  return Math.max(0, Math.min(1, value));
}

function lifecycleToIntegrity(lifecycle: number) {
  return lifecycle === 2 ? 0 : 1;
}

function lifecycleLabel(lifecycle: number) {
  if (lifecycle === 2) {
    return 'dead lifecycle state';
  }
  if (lifecycle === 1) {
    return 'alive lifecycle state';
  }
  return `lifecycle ${lifecycle}`;
}
