import { Application, Container, Graphics } from 'pixi.js';
import type { CellId, JointProjection, WorldFrame } from '../projection/types';
import type { LifecycleVisualState } from './semanticDetail';
import { DEFAULT_VIEWER_CAMERA, type ViewerCamera } from './viewerNavigation';
import { createWorldRenderPlan } from './worldRenderPlan';

export interface WorldRenderer {
  renderFrame: (
    frame: WorldFrame,
    selectedCellId: CellId | null,
    camera?: ViewerCamera,
    activeResourceLayers?: number[]
  ) => void;
  resize: (width: number, height: number) => void;
  exportPng: () => string;
  destroy: () => void;
}

export async function mountWorldRenderer(host: HTMLElement): Promise<WorldRenderer> {
  const app = new Application();
  const width = host.clientWidth || 900;
  const height = host.clientHeight || 560;

  await app.init({
    width,
    height,
    antialias: true,
    backgroundAlpha: 0
  });

  host.appendChild(app.canvas);

  const root = new Container();
  app.stage.addChild(root);

  const renderFrame = (
    frame: WorldFrame,
    selectedCellId: CellId | null,
    camera: ViewerCamera = DEFAULT_VIEWER_CAMERA,
    activeResourceLayers?: number[]
  ) => {
    root.removeChildren();

    const bounds = drawBounds(width, height);
    root.addChild(bounds);

    const resourceLayer = drawResourceLayer(frame, width, height, camera, activeResourceLayers);
    root.addChild(resourceLayer);

    const renderPlan = createWorldRenderPlan(frame, selectedCellId, { width, height }, camera);
    const cellPositions = new Map<string, { x: number; y: number }>();
    renderPlan.cells.forEach((cell) => cellPositions.set(cell.id, { x: cell.x, y: cell.y }));

    const jointsLayer = drawJointsLayer(frame, cellPositions);
    root.addChild(jointsLayer);

    for (const cell of renderPlan.cells) {
      const cellGraphic = new Graphics();
      const fillColor = cellFillColor(cell.lifecycleState, cell.energyRatio);
      const membraneAlpha = 0.52 + cell.integrityRatio * 0.35;
      const strokeColor = cellStrokeColor(cell.lifecycleState, cell.selected);
      const isOverview = cell.semanticLevel === 'overview';

      // 1. Selection & Stressed Halo (only when selected, or for stressed cells at detailed zoom levels)
      if (cell.selected || (!isOverview && cell.lifecycleState === 'stressed')) {
        const haloColor = cell.selected ? 0xffd166 : 0xe76f51;
        const haloOffset = cell.selected ? 4 : 3;
        cellGraphic.circle(cell.x, cell.y, cell.radius + haloOffset);
        cellGraphic.stroke({
          width: cell.selected ? 2.5 : 1.5,
          color: haloColor,
          alpha: cell.selected ? 0.85 : 0.35
        });
      }

      // 2. Drop shadow / ambient glow (only at detailed zoom levels or when selected)
      if (!isOverview || cell.selected) {
        cellGraphic.circle(cell.x + cell.radius * 0.18, cell.y + cell.radius * 0.22, cell.radius * 0.92);
        cellGraphic.fill({ color: 0x031216, alpha: cell.selected ? 0.38 : 0.18 });
      }

      // 3. Primary Cell Body (Outer Membrane & Cytoplasm)
      cellGraphic.circle(cell.x, cell.y, cell.radius);
      cellGraphic.fill({ color: fillColor, alpha: cell.selected ? 0.95 : 0.82 });
      cellGraphic.stroke({
        width: cell.selected ? 2.5 : 1.5,
        color: strokeColor,
        alpha: membraneAlpha
      });

      // 4. Inner Cell Wall (Double Layer Texture - only at structure/internal zoom levels)
      if (cell.semanticLevel === 'structure' || cell.semanticLevel === 'internal-detail') {
        cellGraphic.circle(cell.x, cell.y, Math.max(1, cell.radius * 0.88));
        cellGraphic.stroke({
          width: 1,
          color: 0xffffff,
          alpha: 0.18
        });
      }

      // 5. Internal Organelles & Nucleus (only when zoomed in beyond overview)
      if (!isOverview || cell.selected) {
        drawCellOrganelles(cellGraphic, cell.x, cell.y, cell.radius, cell.energyRatio, cell.lifecycleState);
      }

      // 6. Metric Rings & Integrity Arc
      if (cell.showMetricRings) {
        drawIntegrityArc(cellGraphic, cell.x, cell.y, cell.radius, cell.integrityRatio);
      }

      root.addChild(cellGraphic);
    }
  };

  return {
    renderFrame,
    resize: (nextWidth, nextHeight) => {
      app.renderer.resize(nextWidth, nextHeight);
    },
    exportPng: () => app.canvas.toDataURL('image/png'),
    destroy: () => {
      app.destroy(true, { children: true });
    }
  };
}

export interface MinimalGraphic {
  circle: (x: number, y: number, r: number) => unknown;
  fill: (options: { color: number; alpha: number }) => unknown;
  stroke: (options: { width: number; color: number; alpha: number }) => unknown;
}

export function drawCellOrganelles(
  graphic: MinimalGraphic,
  cx: number,
  cy: number,
  radius: number,
  energyRatio: number,
  lifecycleState: LifecycleVisualState
) {
  if (lifecycleState === 'dead') {
    graphic.circle(cx, cy, radius * 0.3);
    graphic.fill({ color: 0x3a424a, alpha: 0.6 });
    return;
  }

  // 1. Central glowing nucleus / energy core
  const nucleusRadius = Math.max(2.5, radius * (0.22 + energyRatio * 0.32));
  const nucleusColor = lifecycleState === 'stressed' ? 0xe76f51 : 0xffd166;

  graphic.circle(cx - radius * 0.08, cy - radius * 0.08, nucleusRadius);
  graphic.fill({ color: nucleusColor, alpha: 0.45 + energyRatio * 0.4 });
  graphic.stroke({ width: 1.5, color: 0xffffff, alpha: 0.5 });

  // 2. Cytoplasm organelle granules (mitochondria/ribosomes visual dots)
  const granuleOffsets = [
    { dx: 0.42, dy: -0.28, r: 0.14 },
    { dx: -0.38, dy: 0.35, r: 0.12 },
    { dx: 0.25, dy: 0.45, r: 0.11 },
    { dx: -0.45, dy: -0.22, r: 0.13 }
  ];

  granuleOffsets.forEach((g) => {
    const gx = cx + g.dx * radius;
    const gy = cy + g.dy * radius;
    const gr = Math.max(1.5, radius * g.r);
    graphic.circle(gx, gy, gr);
    graphic.fill({ color: 0xbef7cf, alpha: 0.42 });
  });
}

export function drawJointsLayer(
  frame: WorldFrame,
  cellPositions: Map<string, { x: number; y: number }>
) {
  const layer = new Graphics();
  if (!frame.joints || frame.joints.length === 0) {
    return layer;
  }

  frame.joints.forEach((joint) => {
    const src = cellPositions.get(joint.sourceCellId);
    const tgt = cellPositions.get(joint.targetCellId);
    if (!src || !tgt) return;

    const color = jointChannelColor(joint.channelType);
    const width = joint.tension ? Math.max(1.5, Math.min(4, joint.tension * 3)) : 2;

    layer.moveTo(src.x, src.y);
    layer.lineTo(tgt.x, tgt.y);
    layer.stroke({ width, color, alpha: 0.65 });

    if (joint.activeSignal) {
      const midX = (src.x + tgt.x) / 2;
      const midY = (src.y + tgt.y) / 2;
      layer.circle(midX, midY, 4);
      layer.fill({ color: 0xffd166, alpha: 0.9 });
      layer.stroke({ width: 1.5, color: 0xffffff, alpha: 0.8 });
    }
  });

  return layer;
}

function jointChannelColor(channelType: 'mechanical' | 'resource' | 'signal' | 'heat') {
  if (channelType === 'resource') return 0x27b582;
  if (channelType === 'signal') return 0xffd166;
  if (channelType === 'heat') return 0xe76f51;
  return 0x8899a6;
}

interface IntegrityArcGraphic {
  moveTo: (x: number, y: number) => unknown;
  arc: (x: number, y: number, radius: number, startAngle: number, endAngle: number) => unknown;
  stroke: (options: { width: number; color: number; alpha: number }) => unknown;
}

export function drawIntegrityArc(
  graphic: IntegrityArcGraphic,
  x: number,
  y: number,
  radius: number,
  integrityRatio: number
) {
  const arcRadius = radius + 2;
  const startAngle = -Math.PI / 2;
  const endAngle = startAngle + Math.PI * 2 * integrityRatio;

  graphic.moveTo(x + Math.cos(startAngle) * arcRadius, y + Math.sin(startAngle) * arcRadius);
  graphic.arc(x, y, arcRadius, startAngle, endAngle);
  graphic.stroke({ width: 2, color: 0x74ded2, alpha: 0.72 });
}

function drawBounds(width: number, height: number) {
  const bounds = new Graphics();
  bounds.rect(0, 0, width, height);
  bounds.fill({ color: 0x0b1217, alpha: 1 });
  bounds.stroke({ width: 2, color: 0x334756, alpha: 1 });
  return bounds;
}

export interface ResourceCell {
  organic: number;
  mineral: number;
  energy: number;
}

export function sampleBilinearResource(
  grid: ResourceCell[][],
  gx: number,
  gy: number
): ResourceCell {
  const rows = grid.length;
  const cols = grid[0]?.length ?? 0;
  if (rows === 0 || cols === 0) {
    return { organic: 0, mineral: 0, energy: 0 };
  }

  const clampedX = Math.max(0, Math.min(cols - 1, gx));
  const clampedY = Math.max(0, Math.min(rows - 1, gy));

  const x0 = Math.floor(clampedX);
  const y0 = Math.floor(clampedY);
  const x1 = Math.min(cols - 1, x0 + 1);
  const y1 = Math.min(rows - 1, y0 + 1);

  const tx = clampedX - x0;
  const ty = clampedY - y0;

  const r00 = grid[y0]?.[x0] ?? { organic: 0, mineral: 0, energy: 0 };
  const r10 = grid[y0]?.[x1] ?? { organic: 0, mineral: 0, energy: 0 };
  const r01 = grid[y1]?.[x0] ?? { organic: 0, mineral: 0, energy: 0 };
  const r11 = grid[y1]?.[x1] ?? { organic: 0, mineral: 0, energy: 0 };

  const organic =
    (1 - tx) * (1 - ty) * r00.organic +
    tx * (1 - ty) * r10.organic +
    (1 - tx) * ty * r01.organic +
    tx * ty * r11.organic;

  const mineral =
    (1 - tx) * (1 - ty) * r00.mineral +
    tx * (1 - ty) * r10.mineral +
    (1 - tx) * ty * r01.mineral +
    tx * ty * r11.mineral;

  const energy =
    (1 - tx) * (1 - ty) * r00.energy +
    tx * (1 - ty) * r10.energy +
    (1 - tx) * ty * r01.energy +
    tx * ty * r11.energy;

  return { organic, mineral, energy };
}

function drawResourceLayer(
  frame: WorldFrame,
  width: number,
  height: number,
  camera: ViewerCamera,
  activeResourceLayers: number[] = [0, 1, 2, 3]
) {
  const layer = new Graphics();
  const rows = frame.resources.length;
  const cols = frame.resources[0]?.length ?? 0;

  if (rows === 0 || cols === 0) {
    return layer;
  }

  const cellWidth = (width / cols) * camera.scale;
  const cellHeight = (height / rows) * camera.scale;

  const showOrganic = activeResourceLayers.includes(0);
  const showMineral = activeResourceLayers.includes(1);
  const showEnergy = activeResourceLayers.includes(2);

  frame.resources.forEach((row, gy) => {
    row.forEach((resource, gx) => {
      const px = camera.x + gx * cellWidth;
      const py = camera.y + gy * cellHeight;

      if (showOrganic && resource.organic > 0.01) {
        layer.rect(px, py, cellWidth, cellHeight);
        layer.fill({ color: 0x27b582, alpha: Math.min(0.55, 0.08 + resource.organic * 0.45) });
      }
      if (showMineral && resource.mineral > 0.01) {
        layer.rect(px, py, cellWidth, cellHeight);
        layer.fill({ color: 0x2f80ed, alpha: Math.min(0.55, 0.08 + resource.mineral * 0.45) });
      }
      if (showEnergy && resource.energy > 0.01) {
        layer.rect(px, py, cellWidth, cellHeight);
        layer.fill({ color: 0xffd166, alpha: Math.min(0.60, 0.10 + resource.energy * 0.50) });
      }

      layer.rect(px, py, cellWidth, cellHeight);
      layer.stroke({ width: 1, color: 0x1f2937, alpha: 0.12 });
    });
  });

  return layer;
}

function cellFillColor(lifecycleState: LifecycleVisualState, energyRatio: number) {
  if (lifecycleState === 'dead') {
    return 0x7b8794;
  }
  if (lifecycleState === 'dormant') {
    return 0xb08d57;
  }
  if (lifecycleState === 'stressed') {
    return energyRatio > 0.66 ? 0xd6b14f : 0xc8893d;
  }
  if (lifecycleState === 'unavailable') {
    return energyRatio > 0.66 ? 0x74ded2 : 0x5ee08d;
  }
  return energyRatio > 0.66 ? 0x6ff0aa : 0x5ee08d;
}

function cellStrokeColor(lifecycleState: LifecycleVisualState, selected: boolean) {
  if (selected) {
    return 0xffffff;
  }
  if (lifecycleState === 'dead') {
    return 0x4a5568;
  }
  if (lifecycleState === 'dormant') {
    return 0xd69e2e;
  }
  if (lifecycleState === 'stressed') {
    return 0xe76f51;
  }
  return 0xbef7cf;
}
