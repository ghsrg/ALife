import type { CellProjection } from '../projection/types';

export type SemanticZoomLevel = 'overview' | 'entity' | 'structure' | 'internal-detail';
export type LifecycleVisualState = 'alive' | 'stressed' | 'dormant' | 'dead' | 'unavailable';

export interface SemanticZoomInput {
  displayRadiusPx: number;
  selected: boolean;
}

export interface CellSemanticDetail {
  level: SemanticZoomLevel;
  lifecycleState: LifecycleVisualState;
  energyRatio: number;
  integrityRatio: number;
  showLabel: boolean;
  showMetricRings: boolean;
  label: string;
}

export function getSemanticZoomLevel(input: SemanticZoomInput): SemanticZoomLevel {
  const radius = input.displayRadiusPx;

  if (radius >= 48) {
    return 'internal-detail';
  }
  if (radius >= 28 || (input.selected && radius >= 14)) {
    return 'structure';
  }
  if (radius >= 12) {
    return 'entity';
  }
  return 'overview';
}

export function buildCellSemanticDetail(
  cell: CellProjection,
  input: SemanticZoomInput
): CellSemanticDetail {
  const level = getSemanticZoomLevel(input);
  const energyRatio = normalizeRatio(cell.energy);
  const integrityRatio = normalizeRatio(cell.integrity);
  const showLabel = input.selected || level === 'structure' || level === 'internal-detail';
  const showMetricRings = input.selected;

  return {
    level,
    lifecycleState: lifecycleVisualState(cell.lifecycle),
    energyRatio,
    integrityRatio,
    showLabel,
    showMetricRings,
    label: `${cell.id} · E${Math.round(energyRatio * 100)} · I${Math.round(integrityRatio * 100)}`
  };
}

export function normalizeRatio(value: number): number {
  return Math.max(0, Math.min(1, Number(value.toFixed(3))));
}

export function lifecycleVisualState(lifecycle: number | undefined): LifecycleVisualState {
  if (lifecycle === 0) {
    return 'alive';
  }
  if (lifecycle === 1) {
    return 'stressed';
  }
  if (lifecycle === 2) {
    return 'dormant';
  }
  if (lifecycle === 3) {
    return 'dead';
  }
  return 'unavailable';
}
