import type { FrameSource, ProjectionCompleteness, WorldFrame } from './types';

export type ProjectionContextMode = 'fixture' | 'live' | 'stale' | 'frozen' | 'unavailable';

export interface ProjectionContext {
  mode: ProjectionContextMode;
  schemaVersion: WorldFrame['schemaVersion'];
  source: FrameSource;
  completeness: ProjectionCompleteness;
  runId: string;
  tick: number;
  isLive: boolean;
  isReadOnly: boolean;
  warning: string | null;
}

export function buildProjectionContext(
  frame: WorldFrame,
  mode: Exclude<ProjectionContextMode, 'unavailable'>
): ProjectionContext {
  const source = frame.source ?? 'fixture';
  const completeness = classifyCompleteness(frame, mode);

  return {
    mode,
    schemaVersion: frame.schemaVersion,
    source,
    completeness,
    runId: frame.runId,
    tick: frame.tick,
    isLive: mode === 'live',
    isReadOnly: mode === 'frozen' || mode === 'stale',
    warning: buildContextWarning(frame, mode, completeness)
  };
}

export function buildUnavailableProjectionContext(input: {
  runId: string;
  tick: number;
  reason: string;
}): ProjectionContext {
  return {
    mode: 'unavailable',
    schemaVersion: 'WorldFrameProjection/v1',
    source: 'historical',
    completeness: 'unavailable',
    runId: input.runId,
    tick: input.tick,
    isLive: false,
    isReadOnly: true,
    warning: input.reason
  };
}

export function describeProjectionContext(context: ProjectionContext) {
  const prefix = `${labelMode(context.mode)} Tick ${context.tick}`;
  const base = `${prefix} - ${context.source} - ${context.completeness}`;

  if (context.mode === 'unavailable') {
    return `${base} - ${context.warning ?? 'Unavailable Tick'}`;
  }

  return `${base} - ${context.schemaVersion}`;
}

function classifyCompleteness(
  frame: WorldFrame,
  mode: Exclude<ProjectionContextMode, 'unavailable'>
): ProjectionCompleteness {
  if (mode === 'stale') {
    return 'stale';
  }

  if (mode === 'frozen') {
    return 'bounded';
  }

  if ((frame.source ?? 'fixture') === 'live' && frame.resources.length === 0) {
    return 'partial';
  }

  return 'full';
}

function buildContextWarning(
  frame: WorldFrame,
  mode: Exclude<ProjectionContextMode, 'unavailable'>,
  completeness: ProjectionCompleteness
) {
  if (mode === 'stale') {
    return 'Disconnected live projection is stale';
  }

  if ((frame.source ?? 'fixture') === 'live' && completeness === 'partial') {
    return 'Missing live resource projection';
  }

  return null;
}

function labelMode(mode: ProjectionContextMode) {
  switch (mode) {
    case 'fixture':
      return 'Fixture';
    case 'live':
      return 'Live';
    case 'stale':
      return 'Stale';
    case 'frozen':
      return 'Frozen';
    case 'unavailable':
      return 'Unavailable';
  }
}
