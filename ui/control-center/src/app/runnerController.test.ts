import { describe, expect, it, vi } from 'vitest';
import { createAppStore } from './appState';
import {
  createRequestId,
  shouldApplyDebugProjections,
  shouldApplyLiveFrame,
  shouldApplyRunStatus
} from './runnerController';
import type { LiveWorldFrameProjection } from '../runner/alifDecoder';

const liveFrame: LiveWorldFrameProjection = {
  schemaVersion: 'ALIF/v2',
  committedTick: 10,
  projectionSequence: 2,
  wallClockGeneratedAtMs: 1000,
  previousCommittedTick: 9,
  heat: 0,
  waste: 0,
  cells: []
};

describe('runnerController guards', () => {
  it('rejects older live frames for the same active run', () => {
    const store = createAppStore();
    store.getState().setFrame({
      ...store.getState().frame,
      source: 'live',
      runId: 'run-a',
      tick: 11,
      summary: { heat: 0, waste: 0, projectionSequence: 4 }
    });
    store.getState().setRunStatus({
      processState: 'ready',
      activeRunState: 'running',
      runId: 'run-a',
      committedTick: 11,
      scenarioId: 'demo',
      scenarioHash: null,
      effectiveSeed: null,
      terminalReason: null
    });

    expect(shouldApplyLiveFrame(liveFrame, store.getState())).toBe(false);
  });

  it('accepts newer live frames for the current run', () => {
    const store = createAppStore();
    store.getState().setFrame({
      ...store.getState().frame,
      source: 'live',
      runId: 'run-a',
      tick: 9,
      summary: { heat: 0, waste: 0, projectionSequence: 1 }
    });

    expect(shouldApplyLiveFrame(liveFrame, store.getState())).toBe(true);
  });

  it('rejects older run status for the same run', () => {
    const store = createAppStore();
    store.getState().setRunStatus({
      processState: 'ready',
      activeRunState: 'running',
      runId: 'run-a',
      committedTick: 12,
      scenarioId: 'demo',
      scenarioHash: null,
      effectiveSeed: null,
      terminalReason: null
    });

    expect(shouldApplyRunStatus({
      processState: 'ready',
      activeRunState: 'running',
      runId: 'run-a',
      committedTick: 10,
      scenarioId: 'demo',
      scenarioHash: null,
      effectiveSeed: null,
      terminalReason: null
    }, store.getState())).toBe(false);
  });

  it('rejects debug projections older than the current live frame', () => {
    const store = createAppStore();
    store.getState().setFrame({
      ...store.getState().frame,
      source: 'live',
      runId: 'run-a',
      tick: 12,
      summary: { heat: 0, waste: 0, projectionSequence: 5 }
    });

    expect(shouldApplyDebugProjections({
      status: 'available',
      runId: 'run-a',
      tick: 10,
      visualWorld: {
        projectionKind: 'VisualWorldProjection',
        completeness: { state: 'partial', missingFields: [], reason: null },
        cells: [],
        resourceLayers: [],
        fields: [],
        sourceMetrics: []
      },
      coverage: {
        projectionKind: 'CoverageProjection',
        completeness: { state: 'bounded', missingFields: [], reason: null },
        mechanisms: []
      },
      warnings: {
        projectionKind: 'WarningProjection',
        completeness: { state: 'bounded', missingFields: [], reason: null },
        warnings: []
      },
      classifications: {
        projectionKind: 'ClassificationProjection',
        completeness: { state: 'bounded', missingFields: [], reason: null },
        classifications: []
      },
      balanceFindings: {
        projectionKind: 'BalanceFindingProjection',
        completeness: { state: 'bounded', missingFields: [], reason: null },
        findings: []
      }
    }, 10, store.getState())).toBe(false);
  });

  it('applies loading and stale debug projection states only to their requested live tick', () => {
    const store = createAppStore();
    store.getState().setFrame({
      ...store.getState().frame,
      source: 'live',
      runId: 'run-a',
      tick: 12,
      summary: { heat: 0, waste: 0, projectionSequence: 5 }
    });

    expect(shouldApplyDebugProjections({
      status: 'loading',
      runId: 'run-a',
      requestedTick: 12,
      reason: 'Waiting for Observer debug projection'
    }, 12, store.getState())).toBe(true);

    expect(shouldApplyDebugProjections({
      status: 'stale',
      runId: 'run-a',
      tick: 10,
      reason: 'Latest debug projection Tick 10 is behind live Tick 12'
    }, 10, store.getState())).toBe(false);
  });

  it('creates deterministic request ids from an injected clock', () => {
    expect(createRequestId(() => 1234)).toBe('ui-1234');
    expect(vi.isMockFunction(createRequestId)).toBe(false);
  });
});
