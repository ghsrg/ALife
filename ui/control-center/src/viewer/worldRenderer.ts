import { Application, Container, Graphics } from 'pixi.js';
import type { CellId, WorldFrame } from '../projection/types';

export interface WorldRenderer {
  renderFrame: (frame: WorldFrame, selectedCellId: CellId | null) => void;
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

  const renderFrame = (frame: WorldFrame, selectedCellId: CellId | null) => {
    root.removeChildren();

    const bounds = drawBounds(width, height);
    root.addChild(bounds);

    const resourceLayer = drawResourceLayer(frame, width, height);
    root.addChild(resourceLayer);

    for (const cell of frame.cells) {
      const cellGraphic = new Graphics();
      const x = (cell.x / frame.world.width) * width;
      const y = (cell.y / frame.world.height) * height;
      const radius = Math.max(7, (cell.radius / frame.world.width) * width);
      const isSelected = cell.id === selectedCellId;

      cellGraphic.circle(x, y, radius);
      cellGraphic.fill({ color: isSelected ? 0xffd166 : 0x5ee08d, alpha: isSelected ? 0.92 : 0.72 });
      cellGraphic.stroke({ width: isSelected ? 4 : 2, color: isSelected ? 0xffffff : 0xbef7cf, alpha: 0.95 });
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

function drawResourceLayer(frame: WorldFrame, width: number, height: number) {
  const layer = new Graphics();
  const rows = frame.resources.length;
  const cols = frame.resources[0]?.length ?? 0;

  if (rows === 0 || cols === 0) {
    return layer;
  }

  const cellWidth = width / cols;
  const cellHeight = height / rows;

  frame.resources.forEach((row, y) => {
    row.forEach((resource, x) => {
      const intensity = Math.max(0, Math.min(1, (resource.organic + resource.energy) / 2));
      const alpha = 0.18 + intensity * 0.36;
      layer.rect(x * cellWidth, y * cellHeight, cellWidth, cellHeight);
      layer.fill({ color: 0x2f80ed, alpha });
    });
  });

  return layer;
}
