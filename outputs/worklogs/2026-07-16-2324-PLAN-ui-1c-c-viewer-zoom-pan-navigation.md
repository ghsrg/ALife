---
tags:
  - alife
  - worklog/plan
  - ui
  - tdd
  - ui-1c
---

# UI-1C-C Viewer Zoom Pan Navigation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Google Maps-like zoom, pan, fit and reset navigation to the Monitor World Viewer while keeping canvas Cells, DOM hit targets and projection truth overlays aligned.

**Architecture:** Introduce a pure `viewerNavigation` model for camera transform math, then apply the same transform to Pixi world content and DOM hit targets. `WorldViewer` owns transient navigation state; simulation projection state remains unchanged and no Runner/Core/Observer contract is modified.

**Tech Stack:** React 19, TypeScript, Vitest, Testing Library, Playwright, PixiJS 8, existing Control Center CSS.

---

## Scope

`UI-1C-C` owns only basic Viewer navigation:

- mouse wheel zoom around cursor;
- drag-to-pan map movement;
- `Fit` and `Reset view` controls;
- optional visible `Zoom in` / `Zoom out` controls for accessibility and manual verification;
- canvas renderer transform alignment;
- DOM hit target transform alignment;
- e2e visual guard that navigation controls remain usable and World View stays dominant.

Out of scope:

- semantic zoom detail levels;
- atmospheric renderer changes;
- minimap;
- inertia / kinetic panning;
- multi-touch gestures;
- persisted navigation between sessions;
- Runner, ALIF, Core, Observer or bootstrap changes;
- changing simulation coordinates, Cell physical radius, or projection truth semantics.

## Canonical Inputs

- `docs/PRINCIPLES.md`
- `docs/ui/presentation.md`, section `UI-1C Design Alignment`
- `docs/implementation/implementation-plan-ui.md`, `UI-1C-C`
- `outputs/worklogs/2026-07-16-2236-REPORT-ui-1c-b-projection-truthfulness-render-scale.md`
- current code under `ui/control-center/src/components/WorldViewer.tsx`
- current renderer under `ui/control-center/src/viewer/worldRenderer.ts`

## Design Summary

Chosen approach: **camera model + shared transform application**.

Pros:

- keeps navigation state separate from simulation projection state;
- makes math testable without React or Pixi;
- aligns canvas and DOM hit targets through one `ViewerCamera`;
- small enough for one implementation slice.

Cons:

- still uses current Pixi immediate redraw style;
- host size measurement remains simple and not yet ResizeObserver-backed;
- does not solve semantic zoom or renderer performance for large worlds.

Confidence: high for UI-1C-C scope; medium for long-term renderer architecture because future semantic zoom may require a deeper renderer abstraction.

## Files

- Create: `ui/control-center/src/viewer/viewerNavigation.ts`
- Create: `ui/control-center/src/viewer/viewerNavigation.test.ts`
- Modify: `ui/control-center/src/viewer/renderGeometry.ts`
- Modify: `ui/control-center/src/viewer/renderGeometry.test.ts`
- Modify: `ui/control-center/src/viewer/worldRenderer.ts`
- Create: `ui/control-center/src/viewer/worldRenderer.test.ts`
- Modify: `ui/control-center/src/components/WorldViewer.tsx`
- Modify: `ui/control-center/src/components/WorldViewer.test.tsx`
- Modify: `ui/control-center/src/styles.css`
- Modify: `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`
- Modify after implementation: `outputs/worklogs/index.md`
- Create after implementation: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-c-viewer-zoom-pan-navigation.md`

## Success Criteria

Implementation is successful when:

- wheel zoom increases/decreases Viewer scale around the pointer, not around the top-left corner;
- drag pans the World View without changing simulation data;
- `Fit` returns the current World bounds to the visible canvas host;
- `Reset view` returns to default scale and offset;
- `Zoom in` and `Zoom out` controls are keyboard-accessible;
- Pixi renderer receives and applies the same camera transform as DOM hit targets;
- selected Cell hit targets stay aligned after zoom/pan;
- existing projection truth overlay remains visible and truthful;
- e2e captures still pass at `1920x1080 dark`, `1366x768 dark`, and `1920x1080 light`;
- full UI tests, build and targeted e2e pass.

## Task 1: Pure Viewer Navigation Model

**Files:**

- Create: `ui/control-center/src/viewer/viewerNavigation.ts`
- Create: `ui/control-center/src/viewer/viewerNavigation.test.ts`

- [ ] **Step 1: Write the failing navigation model tests**

Create `ui/control-center/src/viewer/viewerNavigation.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import {
  DEFAULT_VIEWER_CAMERA,
  fitCameraToWorld,
  panCamera,
  resetCamera,
  zoomCameraAtPoint
} from './viewerNavigation';

describe('viewerNavigation', () => {
  it('zooms around the pointer so the world point under the cursor stays fixed', () => {
    const camera = zoomCameraAtPoint(
      DEFAULT_VIEWER_CAMERA,
      { x: 300, y: 200 },
      2
    );

    expect(camera.scale).toBe(2);
    expect(camera.x).toBe(-300);
    expect(camera.y).toBe(-200);
  });

  it('clamps zoom to the supported range', () => {
    expect(zoomCameraAtPoint(DEFAULT_VIEWER_CAMERA, { x: 0, y: 0 }, 0.01).scale).toBe(0.5);
    expect(zoomCameraAtPoint(DEFAULT_VIEWER_CAMERA, { x: 0, y: 0 }, 20).scale).toBe(6);
  });

  it('pans by screen-space delta without changing zoom', () => {
    expect(panCamera({ x: 10, y: 20, scale: 2 }, { dx: 5, dy: -8 })).toEqual({
      x: 15,
      y: 12,
      scale: 2
    });
  });

  it('fits the world into the viewport with stable margins', () => {
    expect(fitCameraToWorld({ width: 1200, height: 800 }, { width: 600, height: 600 })).toEqual({
      x: 24,
      y: 112,
      scale: 0.46
    });
  });

  it('resets to the default camera', () => {
    expect(resetCamera()).toEqual(DEFAULT_VIEWER_CAMERA);
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/viewer/viewerNavigation.test.ts
```

Expected: FAIL with module resolution error for `./viewerNavigation`.

- [ ] **Step 3: Implement the minimal navigation model**

Create `ui/control-center/src/viewer/viewerNavigation.ts`:

```ts
export interface ViewerCamera {
  x: number;
  y: number;
  scale: number;
}

export interface ScreenPoint {
  x: number;
  y: number;
}

export interface PanDelta {
  dx: number;
  dy: number;
}

export interface Size {
  width: number;
  height: number;
}

export const MIN_VIEWER_ZOOM = 0.5;
export const MAX_VIEWER_ZOOM = 6;
export const DEFAULT_VIEWER_CAMERA: ViewerCamera = { x: 0, y: 0, scale: 1 };
export const VIEWER_FIT_MARGIN_PX = 24;

export function resetCamera(): ViewerCamera {
  return DEFAULT_VIEWER_CAMERA;
}

export function panCamera(camera: ViewerCamera, delta: PanDelta): ViewerCamera {
  return {
    ...camera,
    x: camera.x + delta.dx,
    y: camera.y + delta.dy
  };
}

export function zoomCameraAtPoint(
  camera: ViewerCamera,
  point: ScreenPoint,
  scaleFactor: number
): ViewerCamera {
  const nextScale = clampZoom(camera.scale * scaleFactor);
  const worldX = (point.x - camera.x) / camera.scale;
  const worldY = (point.y - camera.y) / camera.scale;

  return {
    scale: nextScale,
    x: point.x - worldX * nextScale,
    y: point.y - worldY * nextScale
  };
}

export function fitCameraToWorld(world: Size, viewport: Size): ViewerCamera {
  const usableWidth = Math.max(1, viewport.width - VIEWER_FIT_MARGIN_PX * 2);
  const usableHeight = Math.max(1, viewport.height - VIEWER_FIT_MARGIN_PX * 2);
  const scale = clampZoom(Math.min(usableWidth / world.width, usableHeight / world.height));

  return {
    scale,
    x: Math.round((viewport.width - world.width * scale) / 2),
    y: Math.round((viewport.height - world.height * scale) / 2)
  };
}

function clampZoom(scale: number) {
  return Math.max(MIN_VIEWER_ZOOM, Math.min(MAX_VIEWER_ZOOM, Number(scale.toFixed(3))));
}
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/viewer/viewerNavigation.test.ts
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/viewer/viewerNavigation.ts ui/control-center/src/viewer/viewerNavigation.test.ts
git commit -m "feat(ui): add viewer navigation camera model"
```

## Task 2: Transform-Aware Cell Geometry

**Files:**

- Modify: `ui/control-center/src/viewer/renderGeometry.ts`
- Modify: `ui/control-center/src/viewer/renderGeometry.test.ts`

- [ ] **Step 1: Write failing transform-aware geometry tests**

Modify `ui/control-center/src/viewer/renderGeometry.test.ts` by adding this import:

```ts
import { projectCellForNavigatedRender } from './renderGeometry';
```

Add this test inside `describe('projectCellForRender', () => { ... })`:

```ts
  it('applies viewer camera transform to position and radius for navigated rendering', () => {
    const projection = projectCellForNavigatedRender(
      cell(12),
      frame,
      { width: 1200, height: 800 },
      { x: -100, y: 40, scale: 2 }
    );

    expect(projection.x).toBe(140);
    expect(projection.y).toBe(200);
    expect(projection.physicalRadiusPx).toBe(24);
    expect(projection.displayRadiusPx).toBe(24);
    expect(projection.presentationMinimumApplied).toBe(false);
  });
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/viewer/renderGeometry.test.ts
```

Expected: FAIL because `projectCellForNavigatedRender` is not exported.

- [ ] **Step 3: Implement transform-aware geometry**

Modify `ui/control-center/src/viewer/renderGeometry.ts`:

```ts
import type { ViewerCamera } from './viewerNavigation';
```

Add after `projectCellForRender`:

```ts
export function projectCellForNavigatedRender(
  cell: CellProjection,
  frame: Pick<WorldFrame, 'world'>,
  viewport: ViewportSize,
  camera: ViewerCamera
): RenderedCellGeometry {
  const base = projectCellForRender(cell, frame, viewport);
  const physicalRadiusPx = base.physicalRadiusPx * camera.scale;
  const displayRadiusPx = Math.max(MIN_CELL_DISPLAY_RADIUS_PX, physicalRadiusPx);

  return {
    id: base.id,
    x: base.x * camera.scale + camera.x,
    y: base.y * camera.scale + camera.y,
    physicalRadiusPx,
    displayRadiusPx,
    presentationMinimumApplied: displayRadiusPx !== physicalRadiusPx
  };
}
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/viewer/renderGeometry.test.ts
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/viewer/renderGeometry.ts ui/control-center/src/viewer/renderGeometry.test.ts
git commit -m "feat(ui): project cells through viewer camera"
```

## Task 3: Renderer Camera Transform

**Files:**

- Modify: `ui/control-center/src/viewer/worldRenderer.ts`
- Create: `ui/control-center/src/viewer/worldRenderer.test.ts`

- [ ] **Step 1: Write failing renderer camera API test**

Create `ui/control-center/src/viewer/worldRenderer.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import type { ViewerCamera } from './viewerNavigation';
import { createWorldRenderPlan } from './worldRenderer';
import type { WorldFrame } from '../projection/types';

const frame: WorldFrame = {
  schemaVersion: 'WorldFrameProjection/v1',
  source: 'fixture',
  runId: 'fixture',
  scenarioName: 'fixture',
  tick: 1,
  world: { width: 1200, height: 800 },
  resources: [[{ organic: 0.4, mineral: 0.2, energy: 0.6 }]],
  cells: [
    {
      id: 'cell-a',
      x: 120,
      y: 80,
      radius: 12,
      energy: 0.5,
      integrity: 1,
      generation: 0,
      roleHint: 'alive lifecycle state',
      lifecycle: 1
    }
  ]
};

describe('createWorldRenderPlan', () => {
  it('uses camera transformed geometry for cells', () => {
    const camera: ViewerCamera = { x: -100, y: 40, scale: 2 };
    const plan = createWorldRenderPlan(frame, 'cell-a', { width: 1200, height: 800 }, camera);

    expect(plan.cells).toEqual([
      {
        id: 'cell-a',
        x: 140,
        y: 200,
        radius: 24,
        selected: true
      }
    ]);
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/viewer/worldRenderer.test.ts
```

Expected: FAIL because `createWorldRenderPlan` is not exported.

- [ ] **Step 3: Add renderer camera contract and pure render plan**

Modify `ui/control-center/src/viewer/worldRenderer.ts` imports:

```ts
import { projectCellForNavigatedRender } from './renderGeometry';
import { DEFAULT_VIEWER_CAMERA, type ViewerCamera } from './viewerNavigation';
```

Replace the `WorldRenderer` interface with:

```ts
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
```

Modify `renderFrame` signature and Cell loop:

```ts
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
```

Replace `drawResourceLayer` signature and body with:

```ts
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
```

- [ ] **Step 4: Run renderer tests**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/viewer/worldRenderer.test.ts src/viewer/renderGeometry.test.ts
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/viewer/worldRenderer.ts ui/control-center/src/viewer/worldRenderer.test.ts
git commit -m "feat(ui): apply viewer camera in world renderer"
```

## Task 4: WorldViewer Wheel Zoom, Drag Pan And Controls

**Files:**

- Modify: `ui/control-center/src/components/WorldViewer.tsx`
- Modify: `ui/control-center/src/components/WorldViewer.test.tsx`

- [ ] **Step 1: Extend renderer mock for camera-aware assertions**

Modify the mock expectations in `ui/control-center/src/components/WorldViewer.test.tsx` only as needed after adding camera arguments. Existing assertions should use:

Add `fireEvent` to the Testing Library import:

```tsx
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
```

```ts
expect(renderFrame).toHaveBeenCalledWith(ui1aFixture.frame, 'cell-a', { x: 0, y: 0, scale: 1 });
```

and:

```ts
expect(renderFrame).toHaveBeenCalledWith(tinyLiveFrame, 'tiny', { x: 0, y: 0, scale: 1 });
```

- [ ] **Step 2: Write failing tests for controls and interactions**

Add these tests inside `describe('WorldViewer', () => { ... })`:

```tsx
  it('zooms with visible controls and sends the camera to the renderer', async () => {
    const user = userEvent.setup();

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await user.click(screen.getByRole('button', { name: 'Zoom in World Viewer' }));

    await waitFor(() => {
      expect(renderFrame).toHaveBeenLastCalledWith(ui1aFixture.frame, 'cell-a', {
        x: -120,
        y: -80,
        scale: 1.2
      });
    });
    expect(screen.getByText('120%')).toBeInTheDocument();
  });

  it('resets navigation to the default camera', async () => {
    const user = userEvent.setup();

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await user.click(screen.getByRole('button', { name: 'Zoom in World Viewer' }));
    await user.click(screen.getByRole('button', { name: 'Reset World Viewer navigation' }));

    await waitFor(() => {
      expect(renderFrame).toHaveBeenLastCalledWith(ui1aFixture.frame, 'cell-a', {
        x: 0,
        y: 0,
        scale: 1
      });
    });
  });

  it('keeps hit targets aligned with the navigation camera', async () => {
    const user = userEvent.setup();

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await user.click(screen.getByRole('button', { name: 'Zoom in World Viewer' }));

    expect(screen.getByLabelText('Select cell-a')).toHaveStyle({
      left: '276px',
      top: '304px',
      width: '57.599999999999994px',
      height: '57.599999999999994px'
    });
  });

  it('pans by dragging the World Viewer surface', async () => {
    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    const viewer = screen.getByLabelText('World Viewer');
    fireEvent.pointerDown(viewer, { button: 0, pointerId: 1, clientX: 100, clientY: 100 });
    fireEvent.pointerMove(viewer, { pointerId: 1, clientX: 130, clientY: 80 });
    fireEvent.pointerUp(viewer, { pointerId: 1, clientX: 130, clientY: 80 });

    await waitFor(() => {
      expect(renderFrame).toHaveBeenLastCalledWith(ui1aFixture.frame, 'cell-a', {
        x: 30,
        y: -20,
        scale: 1
      });
    });
  });
```

- [ ] **Step 3: Run tests to verify they fail**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/WorldViewer.test.tsx
```

Expected: FAIL because controls and camera state are absent.

- [ ] **Step 4: Implement camera state and controls in WorldViewer**

Modify imports in `ui/control-center/src/components/WorldViewer.tsx`:

```tsx
import type { PointerEvent as ReactPointerEvent, WheelEvent as ReactWheelEvent } from 'react';
import {
  DEFAULT_VIEWER_CAMERA,
  fitCameraToWorld,
  panCamera,
  resetCamera,
  zoomCameraAtPoint,
  type ViewerCamera
} from '../viewer/viewerNavigation';
import { projectCellForNavigatedRender } from '../viewer/renderGeometry';
```

Replace React import with:

```tsx
import { forwardRef, useEffect, useImperativeHandle, useRef, useState } from 'react';
import type { PointerEvent as ReactPointerEvent, WheelEvent as ReactWheelEvent } from 'react';
```

Add state inside the component:

```tsx
  const [camera, setCamera] = useState<ViewerCamera>(DEFAULT_VIEWER_CAMERA);
  const dragStartRef = useRef<{ pointerId: number; x: number; y: number } | null>(null);
```

Update renderer calls:

```tsx
      renderer.renderFrame(frame, selectedCellId, camera);
```

and:

```tsx
    rendererRef.current?.renderFrame(frame, selectedCellId, camera);
  }, [frame, selectedCellId, camera]);
```

Add handlers before `return`:

```tsx
  const zoomAtCenter = (scaleFactor: number) => {
    const point = { x: frame.world.width / 2, y: frame.world.height / 2 };
    setCamera((current) => zoomCameraAtPoint(current, point, scaleFactor));
  };

  const fitView = () => {
    setCamera(fitCameraToWorld(frame.world, viewport));
  };

  const resetView = () => {
    setCamera(resetCamera());
  };

  const handleWheel = (event: ReactWheelEvent<HTMLDivElement>) => {
    event.preventDefault();
    const rect = event.currentTarget.getBoundingClientRect();
    const point = {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top
    };
    setCamera((current) => zoomCameraAtPoint(current, point, event.deltaY < 0 ? 1.12 : 1 / 1.12));
  };

  const handlePointerDown = (event: ReactPointerEvent<HTMLDivElement>) => {
    if (event.button !== 0) {
      return;
    }
    dragStartRef.current = { pointerId: event.pointerId, x: event.clientX, y: event.clientY };
    if (event.currentTarget.hasPointerCapture?.(event.pointerId) === false) {
      event.currentTarget.setPointerCapture(event.pointerId);
    }
  };

  const handlePointerMove = (event: ReactPointerEvent<HTMLDivElement>) => {
    const dragStart = dragStartRef.current;
    if (dragStart === null || dragStart.pointerId !== event.pointerId) {
      return;
    }
    const dx = event.clientX - dragStart.x;
    const dy = event.clientY - dragStart.y;
    dragStartRef.current = { pointerId: event.pointerId, x: event.clientX, y: event.clientY };
    setCamera((current) => panCamera(current, { dx, dy }));
  };

  const handlePointerUp = (event: ReactPointerEvent<HTMLDivElement>) => {
    if (dragStartRef.current?.pointerId === event.pointerId) {
      dragStartRef.current = null;
      if (event.currentTarget.hasPointerCapture?.(event.pointerId)) {
        event.currentTarget.releasePointerCapture(event.pointerId);
      }
    }
  };
```

Replace the root JSX opening with:

```tsx
    <div
      className="world-viewer"
      aria-label="World Viewer"
      data-ready={isReady ? 'true' : 'false'}
      onWheel={handleWheel}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerCancel={handlePointerUp}
    >
```

Add controls after `ViewerTruthOverlay`:

```tsx
      <div className="viewer-navigation-controls" aria-label="World Viewer navigation">
        <button type="button" onClick={() => zoomAtCenter(1.2)} aria-label="Zoom in World Viewer">+</button>
        <button type="button" onClick={() => zoomAtCenter(1 / 1.2)} aria-label="Zoom out World Viewer">-</button>
        <button type="button" onClick={fitView} aria-label="Fit World Viewer">Fit</button>
        <button type="button" onClick={resetView} aria-label="Reset World Viewer navigation">Reset</button>
        <span aria-label="World Viewer zoom">{Math.round(camera.scale * 100)}%</span>
      </div>
```

Replace hit target map geometry:

```tsx
        {frame.cells.map((cell) => {
          const geometry = projectCellForNavigatedRender(cell, frame, viewport, camera);
          const diameter = `${geometry.displayRadiusPx * 2}px`;

          return (
            <button
              key={cell.id}
              type="button"
              className={cell.id === selectedCellId ? 'cell-hotspot selected' : 'cell-hotspot'}
              style={{ left: `${geometry.x}px`, top: `${geometry.y}px`, width: diameter, height: diameter }}
              onClick={() => onSelectCell(cell.id)}
              aria-label={`Select ${cell.id}`}
            />
          );
        })}
```

- [ ] **Step 5: Run tests to verify they pass**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/WorldViewer.test.tsx src/viewer/viewerNavigation.test.ts src/viewer/renderGeometry.test.ts
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git add ui/control-center/src/components/WorldViewer.tsx ui/control-center/src/components/WorldViewer.test.tsx
git commit -m "feat(ui): add world viewer zoom and pan controls"
```

## Task 5: Navigation Styling And E2E Acceptance

**Files:**

- Modify: `ui/control-center/src/styles.css`
- Modify: `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

- [ ] **Step 1: Write failing e2e navigation acceptance**

Modify `openMonitor` in `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts` by adding:

```ts
  await expect(page.getByLabel('World Viewer navigation')).toBeVisible();
```

Add a new test inside `test.describe('UI-1C-A visual acceptance', () => { ... })`:

```ts
  test('Viewer navigation zooms, resets and keeps selected Cell target usable', async ({ page }) => {
    await openMonitor(page, { width: 1366, height: 768 });

    await page.getByRole('button', { name: 'Zoom in World Viewer' }).click();
    await expect(page.getByLabel('World Viewer zoom')).toHaveText('120%');
    await expect(page.getByLabel('Select cell-a')).toBeVisible();

    await page.getByRole('button', { name: 'Reset World Viewer navigation' }).click();
    await expect(page.getByLabel('World Viewer zoom')).toHaveText('100%');

    await page.screenshot({ path: join(screenshotDir, '1366x768-navigation.png'), fullPage: true });
  });
```

- [ ] **Step 2: Run e2e to verify it fails before styling/integration is complete**

Run:

```powershell
cd ui\control-center
npm.cmd run e2e -- tests/e2e/ui-1c-a-visual.spec.ts
```

Expected before Task 4 completion: FAIL because navigation controls are absent. Expected after Task 4: may PASS before styling; continue to style and visual QA.

- [ ] **Step 3: Add navigation styling**

Add to `ui/control-center/src/styles.css` near Viewer styles:

```css
.viewer-navigation-controls {
  position: absolute;
  left: 22px;
  top: 22px;
  z-index: 5;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  background: rgba(6, 13, 17, 0.74);
  box-shadow: 0 10px 28px rgba(0, 0, 0, 0.22);
  backdrop-filter: blur(8px);
}

.viewer-navigation-controls button {
  min-width: 34px;
  min-height: 30px;
  border: 1px solid rgba(255, 255, 255, 0.14);
  border-radius: 5px;
  padding: 0 8px;
  background: rgba(255, 255, 255, 0.07);
  color: inherit;
  cursor: pointer;
}

.viewer-navigation-controls button:hover,
.viewer-navigation-controls button:focus-visible {
  border-color: rgba(116, 222, 210, 0.58);
  outline: none;
}

.viewer-navigation-controls span {
  min-width: 42px;
  color: #dce6f1;
  font-size: 12px;
  font-weight: 700;
  text-align: center;
}

.world-viewer {
  touch-action: none;
}

:root[data-theme='light'] .viewer-navigation-controls {
  background: rgba(255, 255, 255, 0.9);
  border-color: #d9e0e7;
}

:root[data-theme='light'] .viewer-navigation-controls button {
  border-color: #cfd8e2;
  background: #ffffff;
}

:root[data-theme='light'] .viewer-navigation-controls span {
  color: #26323d;
}

@media (max-width: 1366px) {
  .viewer-navigation-controls {
    left: 18px;
    top: 18px;
  }
}
```

- [ ] **Step 4: Run e2e and inspect screenshots**

Run:

```powershell
cd ui\control-center
npm.cmd run e2e -- tests/e2e/ui-1c-a-visual.spec.ts
```

Expected: PASS.

Inspect:

```text
ui/control-center/test-results/ui-1c-a/1366x768-navigation.png
ui/control-center/test-results/ui-1c-a/1366x768-dark.png
```

Expected visual result:

- navigation controls are visible but do not cover selected entity focus or bottom stats;
- truth overlay remains visible;
- World View remains dominant;
- no incoherent overlap at `1366x768`.

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/styles.css ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts
git commit -m "test(ui): verify world viewer navigation acceptance"
```

## Task 6: Full Verification And Report

**Files:**

- Modify: `outputs/worklogs/index.md`
- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-c-viewer-zoom-pan-navigation.md`

- [ ] **Step 1: Run full UI verification**

Run:

```powershell
cd ui\control-center
npm.cmd test
npm.cmd run build
npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts
```

Expected: all commands PASS.

- [ ] **Step 2: Run repository diff check**

Run:

```powershell
git diff --check
```

Expected: no whitespace errors. LF/CRLF warnings on Windows are acceptable if no actual whitespace errors are reported.

- [ ] **Step 3: Create implementation report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-c-viewer-zoom-pan-navigation.md`:

```md
---
tags:
  - alife
  - worklog/report
  - ui
  - ui-1c
---

# UI-1C-C Viewer Zoom Pan Navigation Report

## Summary

Implemented Google Maps-like navigation baseline for the Monitor World Viewer: zoom, pan, fit, reset, aligned canvas rendering and DOM hit targets.

## Changed Files

- `ui/control-center/src/viewer/viewerNavigation.ts`
- `ui/control-center/src/viewer/viewerNavigation.test.ts`
- `ui/control-center/src/viewer/renderGeometry.ts`
- `ui/control-center/src/viewer/renderGeometry.test.ts`
- `ui/control-center/src/viewer/worldRenderer.ts`
- `ui/control-center/src/viewer/worldRenderer.test.ts`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/components/WorldViewer.test.tsx`
- `ui/control-center/src/styles.css`
- `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

## Verification

- `npm.cmd test`
- `npm.cmd run build`
- `npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts`
- `git diff --check`

## Manual Check Now

- Open `http://127.0.0.1:5173/`.
- Use mouse wheel over World View and confirm the map zooms around the cursor.
- Drag the World View and confirm Cells, selected ring and hit targets move together.
- Click `Fit` and confirm the current World bounds fit inside the Viewer.
- Click `Reset` and confirm zoom returns to `100%`.
- Select a Cell after zoom/pan and confirm the Inspector updates.

## Unresolved Issues

- Navigation state is not persisted between reloads.
- Semantic zoom and richer Cell detail remain deferred to `UI-1C-D`.
- Multi-touch and kinetic panning are intentionally out of scope.

## Next Recommended Slice

`UI-1C-D`: Atmospheric Renderer, Selection Feedback And Semantic Detail.
```

- [ ] **Step 4: Register the report in worklog index**

Add the final report link under `## Reports` in `outputs/worklogs/index.md`.

- [ ] **Step 5: Commit**

```powershell
git add outputs/worklogs/index.md outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-c-viewer-zoom-pan-navigation.md
git commit -m "docs(ui): report UI-1C-C viewer navigation"
```

## Final Acceptance Gate

`UI-1C-C` can be marked complete only if:

- every new behavior has a RED/GREEN test cycle;
- `ViewerCamera` is UI navigation state only and never mutates simulation projection data;
- wheel zoom, drag pan, fit and reset work in the Monitor Viewer;
- canvas renderer and DOM hit targets use the same camera transform;
- projection truth overlay remains visible and truthful;
- `1366x768` layout has no incoherent overlap;
- full unit tests, build and targeted e2e pass.

## What The User Can Check Immediately

After implementation, the user should be able to verify progress without reading the report:

1. Start web UI and see World Viewer navigation controls: `+`, `-`, `Fit`, `Reset`, zoom percent.
2. Scroll over the World View and see zoom change around the pointer.
3. Drag the World View and see the map pan.
4. Select Cells after zoom/pan and see Inspector update correctly.
5. Press `Reset` and see the Viewer return to `100%`.

## Next Slice Recommendation

After `UI-1C-C`, move to:

```text
UI-1C-D:
Atmospheric Renderer, Selection Feedback And Semantic Detail
```

Rationale:

- `UI-1C-B` made projection truth explicit.
- `UI-1C-C` makes navigation usable.
- `UI-1C-D` can now improve WOW rendering without fighting basic camera/hit-target alignment.
