import { describe, expect, it } from 'vitest';
import {
  appendRrdPointSample,
  appendRrdSample,
  appendRrdSeriesSample,
  type RrdMetricHistory,
  type RrdPointHistory,
  type RrdSeriesHistory
} from './rrdMetricHistory';

describe('rrdMetricHistory', () => {
  it('keeps the newest 100 numeric samples consecutive', () => {
    let history: RrdMetricHistory = [];

    for (let tick = 1; tick <= 150; tick++) {
      history = appendRrdSample(history, { tick, value: tick });
    }

    const newest = history.slice(-100);
    expect(newest).toHaveLength(100);
    expect(newest.map((sample) => sample.tick)).toEqual(
      Array.from({ length: 100 }, (_, index) => index + 51)
    );
    expect(newest.every((sample) => sample.kind === 'raw')).toBe(true);
  });

  it('bounds numeric history to at most 1000 samples', () => {
    let history: RrdMetricHistory = [];

    for (let tick = 1; tick <= 1500; tick++) {
      history = appendRrdSample(history, { tick, value: tick });
    }

    expect(history.length).toBeLessThanOrEqual(1000);
    expect(history.slice(-100).map((sample) => sample.tick)).toEqual(
      Array.from({ length: 100 }, (_, index) => index + 1401)
    );
  });

  it('stores mean value and tick interval for collapsed numeric windows', () => {
    let history: RrdMetricHistory = [];

    for (let tick = 1; tick <= 40; tick++) {
      history = appendRrdSample(history, { tick, value: tick }, { maxSamples: 14, newestConsecutive: 4 });
    }

    const collapsed = history.find((sample) => sample.kind === 'mean');
    expect(collapsed).toMatchObject({
      startTick: 1,
      endTick: 10,
      count: 10,
      value: 5.5,
      kind: 'mean'
    });
  });

  it('stores mean position and aggregation metadata for collapsed trail windows', () => {
    let history: RrdPointHistory = [];

    for (let tick = 1; tick <= 40; tick++) {
      history = appendRrdPointSample(history, { tick, x: tick, y: tick * 2 }, { maxSamples: 14, newestConsecutive: 4 });
    }

    const collapsed = history.find((sample) => sample.kind === 'mean');
    expect(collapsed).toMatchObject({
      startTick: 1,
      endTick: 10,
      count: 10,
      x: 5.5,
      y: 11,
      kind: 'mean'
    });
  });

  it('stores mean values and tick interval metadata for collapsed multi-series accounting windows', () => {
    let history: RrdSeriesHistory = [];

    for (let tick = 1; tick <= 40; tick++) {
      history = appendRrdSeriesSample(
        history,
        {
          tick,
          values: {
            environment: tick,
            cells: tick * 2,
            explicitSinks: tick * 3
          }
        },
        { maxSamples: 14, newestConsecutive: 4 }
      );
    }

    const collapsed = history.find((sample) => sample.kind === 'mean');
    expect(collapsed).toMatchObject({
      startTick: 1,
      endTick: 10,
      count: 10,
      kind: 'mean',
      values: {
        environment: 5.5,
        cells: 11,
        explicitSinks: 16.5
      }
    });
  });
});
