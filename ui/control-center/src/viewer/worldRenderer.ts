import { Application, Container, Graphics } from 'pixi.js';
import type { CellId, WorldFrame } from '../projection/types';
import { projectCellForNavigatedRender } from './renderGeometry';
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
}

export interface WorldRenderPlan {
  cells: RenderPlanCell[];
}

export function createWorldRenderPlan(
  frame: WorldFrame,
  selectedCellId: CellId | null,
  viewport: { width: number; height: number },
  camera: ViewerCamera = DEFAULT_VIEWER_CAMERA
): WorldRenderPlan {
  return {
    cells: frame.cells.map((cell) => {
      const geometry = projectCellForNavigatedRender(cell, frame, viewport, camera);
      return {
        id: cell.id,
        x: geometry.x,
        y: geometry.y,
        radius: geometry.displayRadiusPx,
        selected: cell.id === selectedCellId
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

      cellGraphic.circle(cell.x, cell.y, cell.radius);
      cellGraphic.fill({ color: cell.selected ? 0xffd166 : 0x5ee08d, alpha: cell.selected ? 0.92 : 0.72 });
      cellGraphic.stroke({ width: cell.selected ? 4 : 2, color: cell.selected ? 0xffffff : 0xbef7cf, alpha: 0.95 });
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
      const intensity = Math.max(0, Math.min(1, (resource.organic + resource.energy) / 2));
      const alpha = 0.18 + intensity * 0.36;
      layer.rect(camera.x + x * cellWidth, camera.y + y * cellHeight, cellWidth, cellHeight);
      layer.fill({ color: 0x2f80ed, alpha });
    });
  });

  return layer;
}
