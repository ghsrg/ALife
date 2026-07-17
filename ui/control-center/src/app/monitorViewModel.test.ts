import { describe, expect, it } from 'vitest';
import { createAppStore } from './appState';
import { buildMonitorViewModel } from './monitorViewModel';

describe('buildMonitorViewModel', () => {
  it('describes fixture idle state without pretending it is live', () => {
    const store = createAppStore();
    store.getState().setConnected({ engineVersion: '0.1.0', apiVersion: '1', allowRemoteViewer: false });
    store.getState().setRunStatus({
      processState: 'ready',
      activeRunState: 'idle',
      runId: null,
      committedTick: 0,
      scenarioId: null,
      scenarioHash: null,
      effectiveSeed: null,
      terminalReason: null
    });

    expect(buildMonitorViewModel(store.getState()).subtitle).toContain('Runner idle');
    expect(buildMonitorViewModel(store.getState()).projectionLabel).toBe('fixture/v1');
  });

  it('marks live frames as live projection context', () => {
    const store = createAppStore();
    store.getState().setConnected({ engineVersion: '0.1.0', apiVersion: '1', allowRemoteViewer: false });
    const frame = {
      ...store.getState().frame,
      source: 'live' as const,
      tick: 42,
      resources: []
    };
    store.getState().setFrame(frame);

    const model = buildMonitorViewModel(store.getState());

    expect(model.subtitle).toBe('Live Tick 42');
    expect(model.projectionLabel).toBe('live/v1');
    expect(model.resourceLayerState).toBe('Missing live projection');
  });
});
