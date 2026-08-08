import type { VisualEffectsConfig } from '../app/appState';
import type { CellId, WorldFrame } from '../projection/types';
import { projectCellForNavigatedRender } from './renderGeometry';
import { buildCellSemanticDetail, type LifecycleVisualState, type SemanticZoomLevel } from './semanticDetail';
import { DEFAULT_VIEWER_CAMERA, type ViewerCamera } from './viewerNavigation';

export interface RenderPlanCell {
  id: CellId;
  x: number;
  y: number;
  radius: number;
  selected: boolean;
  lifecycleState: LifecycleVisualState;
  energyRatio: number;
  integrityRatio: number;
  semanticLevel: SemanticZoomLevel;
  showMetricRings: boolean;
  label: string;
  flagellaCount: number;
  spikeCount: number;
  receptorHaloIntensity: number;
  lineageHue: number;
  divisionFlashIntensity: number;
}

export interface RenderPlanJoint {
  id: string;
  sourceCellId: CellId;
  targetCellId: CellId;
  x1: number;
  y1: number;
  x2: number;
  y2: number;
  pulseIntensity: number;
}

export interface RenderPlanOrganismHull {
  id: string;
  hullColorHue: number;
  points: Array<{ x: number; y: number; radius: number }>;
}

export interface WorldRenderPlan {
  cells: RenderPlanCell[];
  joints: RenderPlanJoint[];
  organismHulls: RenderPlanOrganismHull[];
  hasResourceField: boolean;
  hasFieldLayers: boolean;
  visualEffects?: VisualEffectsConfig;
}

export function createWorldRenderPlan(
  frame: WorldFrame,
  selectedCellId: CellId | null,
  viewport: { width: number; height: number },
  camera: ViewerCamera = DEFAULT_VIEWER_CAMERA,
  visualEffects?: VisualEffectsConfig
): WorldRenderPlan {
  const cellPositions = new Map<CellId, { x: number; y: number; radius: number }>();

  const cells = frame.cells.map((cell) => {
    const geometry = projectCellForNavigatedRender(cell, frame, viewport, camera);
    const selected = cell.id === selectedCellId;
    const detail = buildCellSemanticDetail(cell, {
      displayRadiusPx: geometry.displayRadiusPx,
      selected
    });

    cellPositions.set(cell.id, {
      x: geometry.x,
      y: geometry.y,
      radius: geometry.displayRadiusPx
    });

    const traits = cell.phenotypeTraits ?? {
      flagellaCount: cell.radius > 8 && (cell.energy ?? 0) > 20 ? 2 : cell.radius > 5 ? 1 : 0,
      spikeCount: 0,
      receptorHaloIntensity: Math.min(1.0, (cell.energy ?? 0) / Math.max(1, cell.energyCapacity ?? 100)),
      lineageHue: (parseInt(String(cell.id).replace(/\D/g, ''), 10) * 137) % 360 || 180,
      divisionFlashIntensity: (cell.energy ?? 0) > (cell.energyCapacity ?? 100) * 0.85 ? 0.8 : 0
    };

    return {
      id: cell.id,
      x: geometry.x,
      y: geometry.y,
      radius: geometry.displayRadiusPx,
      selected,
      lifecycleState: detail.lifecycleState,
      energyRatio: detail.energyRatio,
      integrityRatio: detail.integrityRatio,
      semanticLevel: detail.level,
      showMetricRings: detail.showMetricRings,
      label: detail.label,
      flagellaCount: traits.flagellaCount,
      spikeCount: traits.spikeCount,
      receptorHaloIntensity: traits.receptorHaloIntensity,
      lineageHue: traits.lineageHue,
      divisionFlashIntensity: traits.divisionFlashIntensity
    };
  });

  const joints: RenderPlanJoint[] = (frame.joints ?? []).map((j, idx) => {
    const p1 = cellPositions.get(j.sourceCellId);
    const p2 = cellPositions.get(j.targetCellId);
    return {
      id: j.id ?? `j-${idx}`,
      sourceCellId: j.sourceCellId,
      targetCellId: j.targetCellId,
      x1: p1?.x ?? 0,
      y1: p1?.y ?? 0,
      x2: p2?.x ?? 0,
      y2: p2?.y ?? 0,
      pulseIntensity: j.activeSignal ? 1.0 : 0.6
    };
  });

  const organismHulls: RenderPlanOrganismHull[] = (frame.organismHulls ?? []).map((hull, idx) => {
    const points = hull.cellIds
      .map((id) => cellPositions.get(id))
      .filter((p): p is { x: number; y: number; radius: number } => p !== undefined);

    return {
      id: hull.id ?? `hull-${idx}`,
      hullColorHue: hull.hullColorHue ?? 180,
      points
    };
  });

  return {
    hasResourceField: frame.resources.length > 0 && (frame.resources[0]?.length ?? 0) > 0,
    hasFieldLayers: (frame.fieldLayers?.length ?? 0) > 0,
    visualEffects,
    cells,
    joints,
    organismHulls
  };
}
