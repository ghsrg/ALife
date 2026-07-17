import { Application, Container, Graphics } from 'pixi.js';
import type { CellId, WorldFrame } from '../projection/types';
import { projectCellForNavigatedRender } from './renderGeometry';
import { buildCellSemanticDetail, type LifecycleVisualState, type SemanticZoomLevel } from './semanticDetail';
import { DEFAULT_VIEWER_CAMERA, type ViewerCamera } from './viewerNavigation';

export interface WorldRenderer {
  renderFrame: (frame: WorldFrame, selectedCellId: CellId | null, camera?: ViewerCamera) => void;
  resize: (width: number, height: number) => void;
  exportPng: () => string;
  destroy: () => void;
}

interface RenderPlanCell {
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
}

export interface WorldRenderPlan {
  cells: RenderPlanCell[];
  hasResourceField: boolean;
}

export function createWorldRenderPlan(
  frame: WorldFrame,
  selectedCellId: CellId | null,
  viewport: { width: number; height: number },
  camera: ViewerCamera = DEFAULT_VIEWER_CAMERA
): WorldRenderPlan {
  return {
    hasResourceField: frame.resources.length > 0 && (frame.resources[0]?.length ?? 0) > 0,
    cells: frame.cells.map((cell) => {
      const geometry = projectCellForNavigatedRender(cell, frame, viewport, camera);
      const selected = cell.id === selectedCellId;
      const detail = buildCellSemanticDetail(cell, {
        displayRadiusPx: geometry.displayRadiusPx,
        selected
      });

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
        label: detail.label
      };
    })
  };
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
    camera: ViewerCamera = DEFAULT_VIEWER_CAMERA
  ) => {
    root.removeChildren();

    const bounds = drawBounds(width, height);
    root.addChild(bounds);

    const resourceLayer = drawResourceLayer(frame, width, height, camera);
    root.addChild(resourceLayer);

    const renderPlan = createWorldRenderPlan(frame, selectedCellId, { width, height }, camera);
    for (const cell of renderPlan.cells) {
      const cellGraphic = new Graphics();
      const fillColor = cellFillColor(cell.lifecycleState, cell.energyRatio);
      const membraneAlpha = 0.52 + cell.integrityRatio * 0.35;

      cellGraphic.circle(cell.x + cell.radius * 0.18, cell.y + cell.radius * 0.22, cell.radius * 0.92);
      cellGraphic.fill({ color: 0x031216, alpha: cell.selected ? 0.42 : 0.24 });

      if (cell.showMetricRings) {
        cellGraphic.circle(cell.x, cell.y, cell.radius + 5);
        cellGraphic.stroke({
          width: cell.selected ? 3 : 2,
          color: 0xffd166,
          alpha: cell.selected ? 0.68 : 0.24
        });
      }

      cellGraphic.circle(cell.x, cell.y, cell.radius);
      cellGraphic.fill({ color: fillColor, alpha: cell.selected ? 0.9 : 0.74 });
      cellGraphic.stroke({
        width: cell.selected ? 3 : 2,
        color: cell.selected ? 0xffffff : 0xbef7cf,
        alpha: membraneAlpha
      });

      if (cell.showMetricRings) {
        const energyRadius = Math.max(2, cell.radius * cell.energyRatio);
        cellGraphic.circle(cell.x, cell.y, energyRadius);
        cellGraphic.fill({ color: 0xffd166, alpha: 0.18 + cell.energyRatio * 0.18 });

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

function drawResourceLayer(frame: WorldFrame, width: number, height: number, camera: ViewerCamera) {
  const layer = new Graphics();
  const rows = frame.resources.length;
  const cols = frame.resources[0]?.length ?? 0;

  if (rows === 0 || cols === 0) {
    return layer;
  }

  const cellWidth = (width / cols) * camera.scale;
  const cellHeight = (height / rows) * camera.scale;

  frame.resources.forEach((row, y) => {
    row.forEach((resource, x) => {
      const total = Math.max(0, Math.min(1, (resource.organic + resource.mineral + resource.energy) / 3));
      const energyBias = Math.max(0, Math.min(1, resource.energy));
      const alpha = 0.12 + total * 0.38;
      const color = energyBias > resource.organic ? 0x2f80ed : 0x27b582;
      layer.rect(camera.x + x * cellWidth, camera.y + y * cellHeight, cellWidth, cellHeight);
      layer.fill({ color, alpha });
    });
  });

  return layer;
}

function cellFillColor(lifecycleState: RenderPlanCell['lifecycleState'], energyRatio: number) {
  if (lifecycleState === 'dead') {
    return 0x7b8794;
  }
  if (lifecycleState === 'decomposing') {
    return 0xb08d57;
  }
  if (lifecycleState === 'unavailable') {
    return energyRatio > 0.66 ? 0x74ded2 : 0x5ee08d;
  }
  return energyRatio > 0.66 ? 0x6ff0aa : 0x5ee08d;
}
