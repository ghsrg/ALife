import { describe, expect, it } from 'vitest';
import type { CellProjection } from '../projection/types';
import {
  buildCellSemanticDetail,
  getSemanticZoomLevel,
  normalizeRatio
} from './semanticDetail';

const baseCell: CellProjection = {
  id: 'cell-a',
  x: 120,
  y: 80,
  radius: 12,
  energy: 0.82,
  integrity: 0.91,
  generation: 3,
  roleHint: 'high-energy cluster member',
  lifecycle: 1
};

describe('semanticDetail', () => {
  it('classifies detail levels by screen-space radius and selected state', () => {
    expect(getSemanticZoomLevel({ displayRadiusPx: 7, selected: false })).toBe('overview');
    expect(getSemanticZoomLevel({ displayRadiusPx: 14, selected: false })).toBe('entity');
    expect(getSemanticZoomLevel({ displayRadiusPx: 30, selected: false })).toBe('structure');
    expect(getSemanticZoomLevel({ displayRadiusPx: 52, selected: false })).toBe('internal-detail');
    expect(getSemanticZoomLevel({ displayRadiusPx: 14, selected: true })).toBe('structure');
  });

  it('clamps ratios for presentation without changing source values', () => {
    expect(normalizeRatio(-0.2)).toBe(0);
    expect(normalizeRatio(0.42)).toBe(0.42);
    expect(normalizeRatio(2)).toBe(1);
  });

  it('builds data-bound visual detail from available Cell projection fields', () => {
    const detail = buildCellSemanticDetail(baseCell, {
      displayRadiusPx: 24,
      selected: true
    });

    expect(detail).toEqual({
      level: 'structure',
      lifecycleState: 'alive',
      energyRatio: 0.82,
      integrityRatio: 0.91,
      showLabel: true,
      showMetricRings: true,
      label: 'cell-a · E82 · I91'
    });
  });

  it('marks unavailable lifecycle without inventing state', () => {
    const detail = buildCellSemanticDetail({ ...baseCell, lifecycle: undefined }, {
      displayRadiusPx: 24,
      selected: false
    });

    expect(detail.lifecycleState).toBe('unavailable');
    expect(detail.showLabel).toBe(false);
  });

  it('keeps unselected high-zoom cells free of external metric rings', () => {
    const detail = buildCellSemanticDetail(baseCell, {
      displayRadiusPx: 52,
      selected: false
    });

    expect(detail.level).toBe('internal-detail');
    expect(detail.showMetricRings).toBe(false);
  });
});
