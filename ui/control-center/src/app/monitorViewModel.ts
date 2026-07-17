import { ui1aFixture } from '../fixtures/ui1aFixture';
import { getMonitorDataState, type AppStore } from './appState';

export interface MonitorViewModel {
  scenarioTitle: string;
  subtitle: string;
  projectionLabel: 'fixture/v1' | 'live/v1';
  hasResourceLayer: boolean;
  resourceLayerState: 'Available projection' | 'Missing live projection';
}

export function buildMonitorViewModel(state: AppStore): MonitorViewModel {
  const dataState = getMonitorDataState(state);
  const hasResourceLayer = state.frame.resources.length > 0;

  return {
    scenarioTitle: state.frame.scenarioName ?? ui1aFixture.scenarioName,
    subtitle: buildFrameSubtitle(state, dataState),
    projectionLabel: state.frame.source === 'live' ? 'live/v1' : 'fixture/v1',
    hasResourceLayer,
    resourceLayerState: hasResourceLayer ? 'Available projection' : 'Missing live projection'
  };
}

function buildFrameSubtitle(state: AppStore, dataState: ReturnType<typeof getMonitorDataState>) {
  if (dataState === 'fixture-idle') {
    return `Fixture Tick ${state.frame.tick} - Runner idle`;
  }

  if (dataState === 'live-waiting') {
    return `Waiting for live frame - Fixture Tick ${state.frame.tick}`;
  }

  if (dataState === 'stale-live') {
    return `Stale Live Tick ${state.frame.tick} - disconnected`;
  }

  return `${state.frame.source === 'live' ? 'Live' : 'Fixture'} Tick ${state.frame.tick}`;
}
