export interface RrdMetricRawSample {
  tick: number;
  value: number;
}

export interface RrdMetricHistorySample {
  tick: number;
  startTick: number;
  endTick: number;
  value: number;
  count: number;
  kind: 'raw' | 'mean';
}

export type RrdMetricHistory = RrdMetricHistorySample[];

export interface RrdPointRawSample {
  tick: number;
  x: number;
  y: number;
}

export interface RrdPointHistorySample {
  tick: number;
  startTick: number;
  endTick: number;
  x: number;
  y: number;
  count: number;
  kind: 'raw' | 'mean';
}

export type RrdPointHistory = RrdPointHistorySample[];

export interface RrdSeriesRawSample {
  tick: number;
  values: Record<string, number>;
}

export interface RrdSeriesHistorySample {
  tick: number;
  startTick: number;
  endTick: number;
  values: Record<string, number>;
  count: number;
  kind: 'raw' | 'mean';
}

export type RrdSeriesHistory = RrdSeriesHistorySample[];

export interface RrdOptions {
  maxSamples?: number;
  newestConsecutive?: number;
  decimationFactor?: number;
}

const DEFAULT_MAX_SAMPLES = 1000;
const DEFAULT_NEWEST_CONSECUTIVE = 100;
const DEFAULT_DECIMATION_FACTOR = 10;

export function appendRrdSample(
  history: RrdMetricHistory,
  sample: RrdMetricRawSample,
  options: RrdOptions = {}
): RrdMetricHistory {
  const raw: RrdMetricHistorySample = {
    tick: sample.tick,
    startTick: sample.tick,
    endTick: sample.tick,
    value: sample.value,
    count: 1,
    kind: 'raw'
  };

  const next = [...history.filter((item) => item.tick !== sample.tick), raw].sort(compareByTick);
  return compactNumericHistory(next, normalizeOptions(options));
}

export function appendRrdPointSample(
  history: RrdPointHistory,
  sample: RrdPointRawSample,
  options: RrdOptions = {}
): RrdPointHistory {
  const raw: RrdPointHistorySample = {
    tick: sample.tick,
    startTick: sample.tick,
    endTick: sample.tick,
    x: sample.x,
    y: sample.y,
    count: 1,
    kind: 'raw'
  };

  const next = [...history.filter((item) => item.tick !== sample.tick), raw].sort(compareByTick);
  return compactPointHistory(next, normalizeOptions(options));
}

export function appendRrdSeriesSample(
  history: RrdSeriesHistory,
  sample: RrdSeriesRawSample,
  options: RrdOptions = {}
): RrdSeriesHistory {
  const raw: RrdSeriesHistorySample = {
    tick: sample.tick,
    startTick: sample.tick,
    endTick: sample.tick,
    values: sample.values,
    count: 1,
    kind: 'raw'
  };

  const next = [...history.filter((item) => item.tick !== sample.tick), raw].sort(compareByTick);
  return compactSeriesHistory(next, normalizeOptions(options));
}

function normalizeOptions(options: RrdOptions) {
  return {
    maxSamples: options.maxSamples ?? DEFAULT_MAX_SAMPLES,
    newestConsecutive: options.newestConsecutive ?? DEFAULT_NEWEST_CONSECUTIVE,
    decimationFactor: options.decimationFactor ?? DEFAULT_DECIMATION_FACTOR
  };
}

function compactNumericHistory(
  history: RrdMetricHistory,
  options: Required<RrdOptions>
): RrdMetricHistory {
  if (history.length <= options.maxSamples) {
    return history;
  }

  const newest = history.slice(-options.newestConsecutive);
  let older = history.slice(0, -options.newestConsecutive);

  while (older.length + newest.length > options.maxSamples && older.length > 1) {
    older = collapseNumericWindows(older, options.decimationFactor);
  }

  return [...older, ...newest].slice(-options.maxSamples);
}

function compactPointHistory(
  history: RrdPointHistory,
  options: Required<RrdOptions>
): RrdPointHistory {
  if (history.length <= options.maxSamples) {
    return history;
  }

  const newest = history.slice(-options.newestConsecutive);
  let older = history.slice(0, -options.newestConsecutive);

  while (older.length + newest.length > options.maxSamples && older.length > 1) {
    older = collapsePointWindows(older, options.decimationFactor);
  }

  return [...older, ...newest].slice(-options.maxSamples);
}

function compactSeriesHistory(
  history: RrdSeriesHistory,
  options: Required<RrdOptions>
): RrdSeriesHistory {
  if (history.length <= options.maxSamples) {
    return history;
  }

  const newest = history.slice(-options.newestConsecutive);
  let older = history.slice(0, -options.newestConsecutive);

  while (older.length + newest.length > options.maxSamples && older.length > 1) {
    older = collapseSeriesWindows(older, options.decimationFactor);
  }

  return [...older, ...newest].slice(-options.maxSamples);
}

function collapseNumericWindows(history: RrdMetricHistory, size: number): RrdMetricHistory {
  const collapsed: RrdMetricHistory = [];
  for (let index = 0; index < history.length;) {
    if (history[index].kind === 'mean') {
      collapsed.push(history[index]);
      index++;
      continue;
    }

    const window = history.slice(index, index + size).filter((sample) => sample.kind === 'raw');
    const count = window.reduce((sum, sample) => sum + sample.count, 0);
    const weightedValue = window.reduce((sum, sample) => sum + sample.value * sample.count, 0);
    collapsed.push({
      tick: window[window.length - 1].endTick,
      startTick: window[0].startTick,
      endTick: window[window.length - 1].endTick,
      value: weightedValue / count,
      count,
      kind: window.length === 1 && window[0].kind === 'raw' ? 'raw' : 'mean'
    });
    index += window.length;
  }
  return collapsed;
}

function collapsePointWindows(history: RrdPointHistory, size: number): RrdPointHistory {
  const collapsed: RrdPointHistory = [];
  for (let index = 0; index < history.length;) {
    if (history[index].kind === 'mean') {
      collapsed.push(history[index]);
      index++;
      continue;
    }

    const window = history.slice(index, index + size).filter((sample) => sample.kind === 'raw');
    const count = window.reduce((sum, sample) => sum + sample.count, 0);
    const weightedX = window.reduce((sum, sample) => sum + sample.x * sample.count, 0);
    const weightedY = window.reduce((sum, sample) => sum + sample.y * sample.count, 0);
    collapsed.push({
      tick: window[window.length - 1].endTick,
      startTick: window[0].startTick,
      endTick: window[window.length - 1].endTick,
      x: weightedX / count,
      y: weightedY / count,
      count,
      kind: window.length === 1 && window[0].kind === 'raw' ? 'raw' : 'mean'
    });
    index += window.length;
  }
  return collapsed;
}

function collapseSeriesWindows(history: RrdSeriesHistory, size: number): RrdSeriesHistory {
  const collapsed: RrdSeriesHistory = [];
  for (let index = 0; index < history.length;) {
    if (history[index].kind === 'mean') {
      collapsed.push(history[index]);
      index++;
      continue;
    }

    const window = history.slice(index, index + size).filter((sample) => sample.kind === 'raw');
    const count = window.reduce((sum, sample) => sum + sample.count, 0);
    const fields = new Set(window.flatMap((sample) => Object.keys(sample.values)));
    const values: Record<string, number> = {};

    for (const field of fields) {
      const weightedValue = window.reduce(
        (sum, sample) => sum + (sample.values[field] ?? 0) * sample.count,
        0
      );
      values[field] = weightedValue / count;
    }

    collapsed.push({
      tick: window[window.length - 1].endTick,
      startTick: window[0].startTick,
      endTick: window[window.length - 1].endTick,
      values,
      count,
      kind: window.length === 1 && window[0].kind === 'raw' ? 'raw' : 'mean'
    });
    index += window.length;
  }
  return collapsed;
}

function compareByTick(a: { tick: number }, b: { tick: number }) {
  return a.tick - b.tick;
}
