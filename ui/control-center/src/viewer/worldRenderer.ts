import { Application, Container, Graphics } from 'pixi.js';
import type { VisualEffectsConfig } from '../app/appState';
import type { CellId, WorldFrame } from '../projection/types';
import type { LifecycleVisualState } from './semanticDetail';
import { DEFAULT_VIEWER_CAMERA, type ViewerCamera } from './viewerNavigation';
import { createWorldRenderPlan, type WorldRenderPlan } from './worldRenderPlan';

export interface WorldRenderer {
  renderFrame: (
    frame: WorldFrame,
    selectedCellId?: CellId | null,
    camera?: ViewerCamera,
    activeResourceLayers?: number[],
    visualEffects?: VisualEffectsConfig
  ) => void;
  resize: (width: number, height: number) => void;
  exportPng: () => string;
  destroy: () => void;
}

export async function mountWorldRenderer(host: HTMLElement): Promise<WorldRenderer> {
  const app = new Application();
  let width = host.clientWidth || 900;
  let height = host.clientHeight || 560;

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
    selectedCellId: CellId | null = null,
    camera: ViewerCamera = DEFAULT_VIEWER_CAMERA,
    activeResourceLayers?: number[],
    visualEffects?: VisualEffectsConfig
  ) => {
    root.removeChildren();

    const bounds = drawBounds(width, height, frame, camera);
    root.addChild(bounds);

    const resourceLayer = drawResourceLayer(frame, width, height, camera, activeResourceLayers, visualEffects);
    root.addChild(resourceLayer);

    const renderPlan = createWorldRenderPlan(frame, selectedCellId, { width, height }, camera, visualEffects);

    const organismHullsLayer = drawOrganismHullsLayer(renderPlan, frame.tick);
    root.addChild(organismHullsLayer);

    const jointsLayer = drawAnimatedJointsLayer(renderPlan, frame.tick);
    root.addChild(jointsLayer);

    const showPhenotypeTraits = visualEffects?.showPhenotypeTraits ?? true;
    const showDivisionFlash = visualEffects?.showDivisionFlash ?? true;
    const showOrganelles = visualEffects?.showOrganelles ?? true;

    for (const cell of renderPlan.cells) {
      const cellGraphic = new Graphics();
      const fillColor = cellFillColor(cell.lifecycleState, cell.energyRatio);
      const membraneAlpha = 0.52 + cell.integrityRatio * 0.35;
      const strokeColor = cellStrokeColor(cell.lifecycleState, cell.selected);

      // 0. Receptor Halo Aura (Resource Uptake Trait)
      if (showPhenotypeTraits && cell.receptorHaloIntensity > 0.05) {
        cellGraphic.circle(cell.x, cell.y, cell.radius * 1.35);
        cellGraphic.fill({ color: 0x00c896, alpha: cell.receptorHaloIntensity * 0.22 });
      }

      // 0.5. Lineage Color Coat (Lineage Provenance)
      if (showPhenotypeTraits) {
        const lineageColor = hslToHex(cell.lineageHue, 0.75, 0.55);
        cellGraphic.circle(cell.x, cell.y, cell.radius + 1.2);
        cellGraphic.stroke({ width: 1.2, color: lineageColor, alpha: 0.7 });
      }

      // 1. Primary Cell Body (Outer Membrane & Cytoplasm)
      cellGraphic.circle(cell.x, cell.y, cell.radius);
      cellGraphic.fill({ color: fillColor, alpha: cell.selected ? 0.95 : 0.85 });
      cellGraphic.stroke({
        width: cell.selected ? 2.5 : 1.5,
        color: strokeColor,
        alpha: membraneAlpha
      });

      // 1.5. Contact Spikes (Defense / Boundary Trait)
      if (showPhenotypeTraits && cell.spikeCount > 0) {
        for (let i = 0; i < cell.spikeCount; i++) {
          const angle = (i * Math.PI * 2) / cell.spikeCount;
          const sx1 = cell.x + Math.cos(angle) * cell.radius;
          const sy1 = cell.y + Math.sin(angle) * cell.radius;
          const sx2 = cell.x + Math.cos(angle) * (cell.radius + 4);
          const sy2 = cell.y + Math.sin(angle) * (cell.radius + 4);
          cellGraphic.moveTo(sx1, sy1);
          cellGraphic.lineTo(sx2, sy2);
          cellGraphic.stroke({ width: 1.5, color: 0xffb703, alpha: 0.85 });
        }
      }

      // 1.8. Flagella Motion Filaments (Motility Trait)
      if (showPhenotypeTraits && cell.flagellaCount > 0 && cell.lifecycleState !== 'dead') {
        const tickTime = frame.tick * 0.15;
        for (let i = 0; i < cell.flagellaCount; i++) {
          const baseAngle = Math.PI * 0.75 + (i - (cell.flagellaCount - 1) / 2) * 0.4;
          let prevX = cell.x + Math.cos(baseAngle) * cell.radius;
          let prevY = cell.y + Math.sin(baseAngle) * cell.radius;
          const tailLength = cell.radius * 1.8;
          const segments = 5;

          for (let s = 1; s <= segments; s++) {
            const progress = s / segments;
            const wave = Math.sin(tickTime + s * 0.8 + i) * (4 * progress);
            const segDist = cell.radius + progress * tailLength;
            const segAngle = baseAngle + wave * 0.05;
            const currX = cell.x + Math.cos(segAngle) * segDist;
            const currY = cell.y + Math.sin(segAngle) * segDist;

            cellGraphic.moveTo(prevX, prevY);
            cellGraphic.lineTo(currX, currY);
            cellGraphic.stroke({ width: 1.2, color: 0x74ded2, alpha: 0.7 });
            prevX = currX;
            prevY = currY;
          }
        }
      }

      // 1.9. Division Mutation Flash FX
      if (showDivisionFlash && cell.divisionFlashIntensity > 0) {
        const flashRadius = cell.radius + 4 + Math.sin(frame.tick * 0.3) * 2;
        cellGraphic.circle(cell.x, cell.y, flashRadius);
        cellGraphic.stroke({ width: 2, color: 0xffd166, alpha: cell.divisionFlashIntensity * 0.8 });
      }

      // 2. Selection Ring (clean tight highlight centered directly on the cell)
      if (cell.selected) {
        cellGraphic.circle(cell.x, cell.y, cell.radius + 3);
        cellGraphic.stroke({
          width: 2,
          color: 0xffd166,
          alpha: 0.9
        });
      }

      // 3. Inner Cell Wall (Double Layer Texture - only at structure/internal zoom levels)
      if (showOrganelles && (cell.semanticLevel === 'structure' || cell.semanticLevel === 'internal-detail')) {
        cellGraphic.circle(cell.x, cell.y, Math.max(1, cell.radius * 0.88));
        cellGraphic.stroke({
          width: 1,
          color: 0xffffff,
          alpha: 0.18
        });
      }

      // 4. Internal Organelles & Nucleus (only when deeply zoomed in to structure/detail)
      if (showOrganelles && (cell.semanticLevel === 'structure' || cell.semanticLevel === 'internal-detail')) {
        drawCellOrganelles(cellGraphic, cell.x, cell.y, cell.radius, cell.energyRatio, cell.lifecycleState);
      }

      // 5. Metric Rings & Integrity Arc (only when selected)
      if (cell.selected && cell.showMetricRings) {
        drawIntegrityArc(cellGraphic, cell.x, cell.y, cell.radius, cell.integrityRatio);
      }

      root.addChild(cellGraphic);
    }
  };

  return {
    renderFrame,
    resize: (nextWidth, nextHeight) => {
      width = nextWidth;
      height = nextHeight;
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

  // 1. Concentric cytoplasm material ring (internal detail zoom)
  if (radius > 16) {
    graphic.circle(cx, cy, radius * 0.65);
    graphic.stroke({ width: 1, color: 0x00c896, alpha: 0.25 });
  }

  // 2. Central glowing nucleus / energy core with energy-dependent radiance
  const nucleusRadius = Math.max(2.5, radius * (0.22 + energyRatio * 0.32));
  const nucleusColor = lifecycleState === 'stressed' ? 0xe76f51 : 0xffd166;

  graphic.circle(cx - radius * 0.08, cy - radius * 0.08, nucleusRadius);
  graphic.fill({ color: nucleusColor, alpha: 0.45 + energyRatio * 0.4 });
  graphic.stroke({ width: 1.5, color: 0xffffff, alpha: 0.5 });

  // 3. Cytoplasm organelle granules (mitochondria/ribosomes visual dots)
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

  // 4. Outer membrane receptor nodes (Genome phenotype visual trait representation)
  if (radius > 12) {
    const receptorAngles = [0, Math.PI * 0.5, Math.PI, Math.PI * 1.5];
    receptorAngles.forEach((angle) => {
      const rx = cx + Math.cos(angle) * (radius * 0.96);
      const ry = cy + Math.sin(angle) * (radius * 0.96);
      graphic.circle(rx, ry, Math.max(1.2, radius * 0.08));
      graphic.fill({ color: 0x74ded2, alpha: 0.75 });
    });
  }
}


export function drawOrganismHullsLayer(renderPlan: WorldRenderPlan, tick: number) {
  const layer = new Graphics();
  const showHulls = renderPlan.visualEffects?.showOrganismHulls ?? true;
  if (!showHulls || !renderPlan.organismHulls || renderPlan.organismHulls.length === 0) {
    return layer;
  }

  renderPlan.organismHulls.forEach((hull) => {
    if (hull.points.length === 0) return;

    const colorHex = hslToHex(hull.hullColorHue, 0.75, 0.45);
    const glowColorHex = hslToHex(hull.hullColorHue, 0.85, 0.65);
    const pulseAlpha = 0.14 + Math.sin(tick * 0.08) * 0.04;

    if (hull.points.length === 1) {
      const p = hull.points[0];
      const r = p.radius * 1.5;
      layer.circle(p.x, p.y, r);
      layer.fill({ color: colorHex, alpha: pulseAlpha });
      layer.stroke({ width: 1.5, color: glowColorHex, alpha: 0.35 });
    } else {
      hull.points.forEach((p) => {
        layer.circle(p.x, p.y, p.radius * 1.55);
        layer.fill({ color: colorHex, alpha: pulseAlpha });
      });

      for (let i = 0; i < hull.points.length; i++) {
        const p1 = hull.points[i];
        const p2 = hull.points[(i + 1) % hull.points.length];
        layer.moveTo(p1.x, p1.y);
        layer.lineTo(p2.x, p2.y);
        layer.stroke({ width: Math.max(p1.radius, p2.radius) * 2.2, color: colorHex, alpha: pulseAlpha });
      }

      hull.points.forEach((p) => {
        layer.circle(p.x, p.y, p.radius * 1.55);
        layer.stroke({ width: 1.5, color: glowColorHex, alpha: 0.45 });
      });
    }
  });

  return layer;
}

export function drawAnimatedJointsLayer(renderPlan: WorldRenderPlan, tick: number) {
  const layer = new Graphics();
  const showPulses = renderPlan.visualEffects?.showJointPulses ?? true;
  if (!renderPlan.joints || renderPlan.joints.length === 0) {
    return layer;
  }

  renderPlan.joints.forEach((joint) => {
    const { x1, y1, x2, y2, pulseIntensity } = joint;
    if (x1 === 0 && y1 === 0 && x2 === 0 && y2 === 0) return;

    layer.moveTo(x1, y1);
    layer.lineTo(x2, y2);
    layer.stroke({ width: 2, color: 0x27b582, alpha: 0.55 });

    if (showPulses) {
      const speed = 0.05;
      const seed = parseInt(joint.id.replace(/\D/g, '') || '0', 10);
      const progress = ((tick * speed + seed * 0.3) % 1.0);
      const pulseX = x1 + (x2 - x1) * progress;
      const pulseY = y1 + (y2 - y1) * progress;

      layer.circle(pulseX, pulseY, 5.5 * pulseIntensity);
      layer.fill({ color: 0x00c896, alpha: 0.35 });

      layer.circle(pulseX, pulseY, 2.5);
      layer.fill({ color: 0xffffff, alpha: 0.9 });
      layer.stroke({ width: 1, color: 0x86efac, alpha: 0.85 });
    }
  });

  return layer;
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

function drawBounds(width: number, height: number, frame: WorldFrame, camera: ViewerCamera) {
  const layer = new Graphics();
  // 1. Dark outer stage background
  layer.rect(0, 0, width, height);
  layer.fill({ color: 0x070c10, alpha: 1 });

  // 2. World simulation map box (aligned with camera and world size)
  const scaleX = width / frame.world.width;
  const scaleY = height / frame.world.height;
  const worldWidthPx = frame.world.width * scaleX * camera.scale;
  const worldHeightPx = frame.world.height * scaleY * camera.scale;

  layer.rect(camera.x, camera.y, worldWidthPx, worldHeightPx);
  layer.fill({ color: 0x0b141a, alpha: 1 });
  layer.stroke({ width: 2, color: 0x2ec4b6, alpha: 0.4 });

  return layer;
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

const DYNAMIC_LAYER_COLORS: number[] = [
  0x2ec4b6, // Layer 0: Teal
  0x3a86ff, // Layer 1: Blue
  0xffb703, // Layer 2: Amber
  0x8338ec, // Layer 3: Purple
  0xe76f51, // Layer 4: Coral
  0x27b582, // Layer 5: Green
  0xf4a261, // Layer 6: Orange
  0xe7c6ff  // Layer 7: Lavender
];

function drawResourceLayer(
  frame: WorldFrame,
  width: number,
  height: number,
  camera: ViewerCamera,
  activeResourceLayers: number[] = [0, 1],
  visualEffects?: VisualEffectsConfig
) {
  const layer = new Graphics();
  if (!activeResourceLayers || activeResourceLayers.length === 0) {
    return layer;
  }

  const rows = frame.resources.length;
  const cols = frame.resources[0]?.length ?? 0;

  if (rows === 0 || cols === 0) {
    return layer;
  }

  const showNebula = visualEffects?.showNebula ?? false;
  const showFilaments = visualEffects?.showFilaments ?? false;
  const showParticles = visualEffects?.showParticles ?? false;

  const cellWidth = (width / cols) * camera.scale;
  const cellHeight = (height / rows) * camera.scale;
  const tickTime = frame.tick * 0.04;

  const highDensityNodes: Array<{ x: number; y: number; color: number; amount: number }> = [];

  // Pass 1: Smooth Organic Bioluminescent Hub Glows (Nebula Field)
  frame.resources.forEach((row, gy) => {
    row.forEach((resource, gx) => {
      const cx = camera.x + (gx + 0.5) * cellWidth;
      const cy = camera.y + (gy + 0.5) * cellHeight;
      let maxAmount = 0;
      let primaryColor = DYNAMIC_LAYER_COLORS[0];

      activeResourceLayers.forEach((layerIndex) => {
        const amount = resource.layers?.[layerIndex] ?? (
          layerIndex === 0 ? resource.organic :
          layerIndex === 1 ? resource.mineral :
          layerIndex === 2 ? resource.energy : 0
        );

        if (amount > 0.01) {
          if (amount > maxAmount) {
            maxAmount = amount;
            primaryColor = DYNAMIC_LAYER_COLORS[layerIndex % DYNAMIC_LAYER_COLORS.length];
          }

          // Base resource grid cell (filled rectangle matching the grid)
          const baseAlpha = Math.min(0.55, 0.08 + amount * 0.35);
          layer.rect(cx - cellWidth * 0.5, cy - cellHeight * 0.5, cellWidth, cellHeight);
          layer.fill({ color: DYNAMIC_LAYER_COLORS[layerIndex % DYNAMIC_LAYER_COLORS.length], alpha: baseAlpha });

          if (showNebula) {
            // Soft organic radial glow hub (extends beyond grid cell)
            const glowRadius = Math.max(cellWidth, cellHeight) * (1.2 + amount * 1.5);
            const alpha = Math.min(0.22, 0.03 + amount * 0.16);

            layer.circle(cx, cy, glowRadius);
            layer.fill({ color: DYNAMIC_LAYER_COLORS[layerIndex % DYNAMIC_LAYER_COLORS.length], alpha });
          }
        }
      });

      if (maxAmount > 0.35) {
        highDensityNodes.push({ x: cx, y: cy, color: primaryColor, amount: maxAmount });
      }
    });
  });

  // Pass 2: Organic Interconnecting Web Filaments (Neural / Mycelium Network lines)
  if (showFilaments) {
    for (let i = 0; i < highDensityNodes.length; i++) {
      for (let j = i + 1; j < highDensityNodes.length; j++) {
        const n1 = highDensityNodes[i];
        const n2 = highDensityNodes[j];
        const dx = n1.x - n2.x;
        const dy = n1.y - n2.y;
        const distSq = dx * dx + dy * dy;
        const maxDist = Math.max(cellWidth, cellHeight) * 3.5;

        if (distSq < maxDist * maxDist) {
          const alpha = (1 - Math.sqrt(distSq) / maxDist) * 0.14 * Math.min(n1.amount, n2.amount);
          layer.moveTo(n1.x, n1.y);
          layer.lineTo(n2.x, n2.y);
          layer.stroke({ width: 1, color: n1.color, alpha });
        }
      }
    }
  }

  // Pass 3: Subtle Micro Stardust Particles (Very fine, sparse bioluminescent sparkles)
  if (showParticles) {
    highDensityNodes.forEach((node, idx) => {
      if (idx % 3 === 0) {
        const seed = (node.x * 13 + node.y * 29 + frame.tick) % 100;
        const particleOffsetX = (Math.sin(tickTime + seed) * 0.4) * cellWidth;
        const particleOffsetY = (Math.cos(tickTime * 0.7 + seed * 1.5) * 0.4) * cellHeight;
        const particleRadius = 0.8 + (seed % 3) * 0.4;

        layer.circle(node.x + particleOffsetX, node.y + particleOffsetY, particleRadius);
        layer.fill({ color: 0x99f6e4, alpha: 0.25 });
      }
    });
  }

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
    return 0xffd166;
  }
  if (lifecycleState === 'dead') {
    return 0x4a5568;
  }
  if (lifecycleState === 'stressed') {
    return 0xf97316;
  }
  return 0x2ec4b6;
}

function hslToHex(h: number, s: number, l: number): number {
  const c = (1 - Math.abs(2 * l - 1)) * s;
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
  const m = l - c / 2;
  let r = 0;
  let g = 0;
  let b = 0;
  if (0 <= h && h < 60) { r = c; g = x; b = 0; }
  else if (60 <= h && h < 120) { r = x; g = c; b = 0; }
  else if (120 <= h && h < 180) { r = 0; g = c; b = x; }
  else if (180 <= h && h < 240) { r = 0; g = x; b = c; }
  else if (240 <= h && h < 300) { r = x; g = 0; b = c; }
  else if (300 <= h && h <= 360) { r = c; g = 0; b = x; }
  const red = Math.round((r + m) * 255);
  const green = Math.round((g + m) * 255);
  const blue = Math.round((b + m) * 255);
  return (red << 16) | (green << 8) | blue;
}
