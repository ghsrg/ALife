---
tags:
  - alife
  - worklog/plan
  - ui
  - tdd
  - ui-1c
---

# UI-1C-B Projection Truthfulness And Render Scale Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the Monitor Viewer truthful about live projection gaps and Cell render scale so users can distinguish physical simulation data from presentation minimums.

**Architecture:** Add small pure presentation models for render geometry and projection truth state, then feed them into the existing `WorldViewer` and Pixi renderer. UI-1C-B does not change Runner, ALIF, Core, bootstrap, physics, or Observer contracts. Missing live Resources remain explicit missing projection states until Runner/Observer provides data.

**Tech Stack:** React 19, TypeScript, Vitest, Testing Library, Playwright, PixiJS via existing `WorldViewer`.

---

## Scope

`UI-1C-B` closes the current user-visible truthfulness gap:

- live Cells may visually look stacked because the renderer applies a minimum display radius;
- hit targets and canvas rendering do not share a single scale model;
- live Resource layer is unavailable in `ALIF/v2`, but the left layer label can still read like a real active layer;
- users need a compact legend/provenance state to understand what is physical data and what is presentation.

In scope:

- shared pure render geometry helper for Cell screen position and radius;
- explicit `physical radius`, `display radius`, and `presentation minimum applied` state;
- World Viewer overlay/legend for resource projection state and cell size scaling;
- layer label copy that distinguishes fixture resources from missing live resource projection;
- tests for live tiny-radius frames, fixture resources, and missing live resources;
- one Playwright smoke proving the new truthfulness UI is visible in the fixture path.

Out of scope:

- adding live resource grid to Runner/ALIF;
- changing bootstrap or physics spacing;
- changing Core Cell radius semantics;
- semantic zoom detail renderer;
- scroll/wheel zoom and pan navigation;
- full debug exact layers;
- charts, analytics, or Observer-backed Resource detail projection.

## Canonical Inputs

- `docs/PRINCIPLES.md`
- `docs/ui/presentation.md`, section `UI-1C Design Alignment`
- `docs/implementation/implementation-plan-ui.md`
- `outputs/worklogs/2026-07-16-2120-REPORT-ui-1c-a-world-first-monitor-layout.md`
- current UI code under `ui/control-center/src`

## Files

- Create: `ui/control-center/src/viewer/renderGeometry.ts`
- Create: `ui/control-center/src/viewer/renderGeometry.test.ts`
- Create: `ui/control-center/src/components/viewerTruth.ts`
- Create: `ui/control-center/src/components/viewerTruth.test.ts`
- Create: `ui/control-center/src/components/ViewerTruthOverlay.tsx`
- Create: `ui/control-center/src/components/ViewerTruthOverlay.test.tsx`
- Modify: `ui/control-center/src/components/WorldViewer.tsx`
- Modify: `ui/control-center/src/components/WorldViewer.test.tsx`
- Modify: `ui/control-center/src/components/AppShell.tsx`
- Modify: `ui/control-center/src/App.test.tsx`
- Modify: `ui/control-center/src/viewer/worldRenderer.ts`
- Modify: `ui/control-center/src/styles.css`
- Modify: `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`
- Modify after implementation: `outputs/worklogs/index.md`
- Create after implementation: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-b-projection-truthfulness-render-scale.md`

## Success Criteria

Implementation is successful when:

- tiny live Cells render with a documented display minimum rather than silently appearing physically large;
- hit targets use the same `displayRadiusPx` model as the renderer;
- fixture Resources are shown as available fixture data;
- live empty `resources` shows `Resources: Missing projection`;
- layer controls do not imply that live Resource heatmap is available when it is missing;
- tests prove the UI distinguishes physical Cell radius from display radius;
- build and e2e pass after merge-ready implementation.

## Task 1: Render Geometry Model

**Files:**

- Create: `ui/control-center/src/viewer/renderGeometry.ts`
- Create: `ui/control-center/src/viewer/renderGeometry.test.ts`

- [ ] **Step 1: Write the failing geometry tests**

Create `ui/control-center/src/viewer/renderGeometry.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import type { CellProjection, WorldFrame } from '../projection/types';
import { projectCellForRender } from './renderGeometry';

const frame: WorldFrame = {
  schemaVersion: 'WorldFrameProjection/v1',
  source: 'live',
  runId: 'run-live',
  scenarioName: 'demo_living_world',
  tick: 10,
  world: { width: 1200, height: 800 },
  resources: [],
  cells: []
};

function cell(radius: number): CellProjection {
  return {
    id: 'cell-1',
    x: 120,
    y: 80,
    radius,
    energy: 0.5,
    integrity: 1,
    generation: 0,
    roleHint: 'alive lifecycle state',
    lifecycle: 1
  };
}

describe('projectCellForRender', () => {
  it('keeps physical and display radius equal when the cell is large enough on screen', () => {
    const projection = projectCellForRender(cell(24), frame, { width: 1200, height: 800 });

    expect(projection).toEqual({
      id: 'cell-1',
      x: 120,
      y: 80,
      physicalRadiusPx: 24,
      displayRadiusPx: 24,
      presentationMinimumApplied: false
    });
  });

  it('applies a display minimum without changing the physical radius', () => {
    const projection = projectCellForRender(cell(2), frame, { width: 1200, height: 800 });

    expect(projection.physicalRadiusPx).toBe(2);
    expect(projection.displayRadiusPx).toBe(7);
    expect(projection.presentationMinimumApplied).toBe(true);
  });

  it('uses the smaller viewport scale so circles remain proportional when aspect ratio differs', () => {
    const projection = projectCellForRender(cell(12), frame, { width: 600, height: 800 });

    expect(projection.x).toBe(60);
    expect(projection.y).toBe(80);
    expect(projection.physicalRadiusPx).toBe(6);
    expect(projection.displayRadiusPx).toBe(7);
    expect(projection.presentationMinimumApplied).toBe(true);
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/viewer/renderGeometry.test.ts
```

Expected: FAIL with module resolution error for `./renderGeometry`.

- [ ] **Step 3: Implement the minimal geometry helper**

Create `ui/control-center/src/viewer/renderGeometry.ts`:

```ts
import type { CellId, CellProjection, WorldFrame } from '../projection/types';

export interface ViewportSize {
  width: number;
  height: number;
}

export interface RenderedCellGeometry {
  id: CellId;
  x: number;
  y: number;
  physicalRadiusPx: number;
  displayRadiusPx: number;
  presentationMinimumApplied: boolean;
}

export const MIN_CELL_DISPLAY_RADIUS_PX = 7;

export function projectCellForRender(
  cell: CellProjection,
  frame: Pick<WorldFrame, 'world'>,
  viewport: ViewportSize
): RenderedCellGeometry {
  const scaleX = viewport.width / frame.world.width;
  const scaleY = viewport.height / frame.world.height;
  const radiusScale = Math.min(scaleX, scaleY);
  const physicalRadiusPx = cell.radius * radiusScale;
  const displayRadiusPx = Math.max(MIN_CELL_DISPLAY_RADIUS_PX, physicalRadiusPx);

  return {
    id: cell.id,
    x: cell.x * scaleX,
    y: cell.y * scaleY,
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
git commit -m "feat(ui): add cell render geometry model"
```

## Task 2: Projection Truth Model

**Files:**

- Create: `ui/control-center/src/components/viewerTruth.ts`
- Create: `ui/control-center/src/components/viewerTruth.test.ts`

- [ ] **Step 1: Write the failing truth model tests**

Create `ui/control-center/src/components/viewerTruth.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import type { WorldFrame } from '../projection/types';
import { buildViewerTruthState } from './viewerTruth';

const baseFrame: WorldFrame = {
  schemaVersion: 'WorldFrameProjection/v1',
  source: 'live',
  runId: 'run-live',
  scenarioName: 'demo_living_world',
  tick: 20,
  world: { width: 1200, height: 800 },
  resources: [],
  cells: [
    { id: '1', x: 10, y: 20, radius: 2, energy: 0.8, integrity: 1, generation: 0, roleHint: 'alive lifecycle state', lifecycle: 1 },
    { id: '2', x: 40, y: 50, radius: 20, energy: 0.5, integrity: 1, generation: 0, roleHint: 'alive lifecycle state', lifecycle: 1 }
  ]
};

describe('buildViewerTruthState', () => {
  it('marks live resources as missing when the projection has no resource grid', () => {
    const state = buildViewerTruthState(baseFrame, { width: 1200, height: 800 });

    expect(state.resourceLayer).toEqual({
      state: 'missing',
      label: 'Resources',
      value: 'Missing projection',
      note: 'Runner ALIF v2 does not include resource grid'
    });
  });

  it('marks fixture resource grid as available data', () => {
    const state = buildViewerTruthState({
      ...baseFrame,
      source: 'fixture',
      resources: [[{ organic: 0.2, mineral: 0.1, energy: 0.5 }]]
    }, { width: 1200, height: 800 });

    expect(state.resourceLayer).toEqual({
      state: 'available',
      label: 'Resources',
      value: 'Fixture grid',
      note: '1 resource cells'
    });
  });

  it('reports when display radius minimum is applied to any cell', () => {
    const state = buildViewerTruthState(baseFrame, { width: 1200, height: 800 });

    expect(state.cellScale).toEqual({
      state: 'presentation-minimum',
      label: 'Cell size',
      value: 'Display minimum applied',
      note: '1 of 2 cells enlarged for visibility'
    });
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/viewerTruth.test.ts
```

Expected: FAIL with module resolution error for `./viewerTruth`.

- [ ] **Step 3: Implement the minimal truth model**

Create `ui/control-center/src/components/viewerTruth.ts`:

```ts
import type { WorldFrame } from '../projection/types';
import { projectCellForRender, type ViewportSize } from '../viewer/renderGeometry';

export type ViewerTruthStateLevel = 'available' | 'missing' | 'physical-scale' | 'presentation-minimum';

export interface ViewerTruthItem {
  state: ViewerTruthStateLevel;
  label: string;
  value: string;
  note: string;
}

export interface ViewerTruthState {
  resourceLayer: ViewerTruthItem;
  cellScale: ViewerTruthItem;
}

export function buildViewerTruthState(frame: WorldFrame, viewport: ViewportSize): ViewerTruthState {
  const resourceCellCount = frame.resources.reduce((sum, row) => sum + row.length, 0);
  const renderedCells = frame.cells.map((cell) => projectCellForRender(cell, frame, viewport));
  const enlargedCount = renderedCells.filter((cell) => cell.presentationMinimumApplied).length;

  return {
    resourceLayer: resourceCellCount === 0
      ? {
          state: 'missing',
          label: 'Resources',
          value: 'Missing projection',
          note: 'Runner ALIF v2 does not include resource grid'
        }
      : {
          state: 'available',
          label: 'Resources',
          value: frame.source === 'live' ? 'Live grid' : 'Fixture grid',
          note: `${resourceCellCount} resource cells`
        },
    cellScale: enlargedCount === 0
      ? {
          state: 'physical-scale',
          label: 'Cell size',
          value: 'Physical scale',
          note: 'display radius matches projected radius'
        }
      : {
          state: 'presentation-minimum',
          label: 'Cell size',
          value: 'Display minimum applied',
          note: `${enlargedCount} of ${frame.cells.length} cells enlarged for visibility`
        }
  };
}
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/viewerTruth.test.ts
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/components/viewerTruth.ts ui/control-center/src/components/viewerTruth.test.ts
git commit -m "feat(ui): add viewer truth state model"
```

## Task 3: Viewer Truth Overlay Component

**Files:**

- Create: `ui/control-center/src/components/ViewerTruthOverlay.tsx`
- Create: `ui/control-center/src/components/ViewerTruthOverlay.test.tsx`

- [ ] **Step 1: Write the failing overlay tests**

Create `ui/control-center/src/components/ViewerTruthOverlay.test.tsx`:

```tsx
import { screen, within } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { renderApp } from '../test/render';
import { ViewerTruthOverlay } from './ViewerTruthOverlay';
import type { ViewerTruthState } from './viewerTruth';

const truthState: ViewerTruthState = {
  resourceLayer: {
    state: 'missing',
    label: 'Resources',
    value: 'Missing projection',
    note: 'Runner ALIF v2 does not include resource grid'
  },
  cellScale: {
    state: 'presentation-minimum',
    label: 'Cell size',
    value: 'Display minimum applied',
    note: '1 of 2 cells enlarged for visibility'
  }
};

describe('ViewerTruthOverlay', () => {
  it('renders resource and cell scale truth states', () => {
    renderApp(<ViewerTruthOverlay truthState={truthState} />);

    const overlay = screen.getByLabelText('Viewer projection truth');
    expect(within(overlay).getByText('Resources')).toBeInTheDocument();
    expect(within(overlay).getByText('Missing projection')).toBeInTheDocument();
    expect(within(overlay).getByText('Runner ALIF v2 does not include resource grid')).toBeInTheDocument();
    expect(within(overlay).getByText('Cell size')).toBeInTheDocument();
    expect(within(overlay).getByText('Display minimum applied')).toBeInTheDocument();
    expect(within(overlay).getByText('1 of 2 cells enlarged for visibility')).toBeInTheDocument();
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/ViewerTruthOverlay.test.tsx
```

Expected: FAIL with module resolution error for `./ViewerTruthOverlay`.

- [ ] **Step 3: Implement minimal overlay component**

Create `ui/control-center/src/components/ViewerTruthOverlay.tsx`:

```tsx
import type { ViewerTruthState } from './viewerTruth';

interface ViewerTruthOverlayProps {
  truthState: ViewerTruthState;
}

export function ViewerTruthOverlay({ truthState }: ViewerTruthOverlayProps) {
  return (
    <aside className="viewer-truth-overlay" aria-label="Viewer projection truth">
      <TruthItem item={truthState.resourceLayer} />
      <TruthItem item={truthState.cellScale} />
    </aside>
  );
}

function TruthItem({ item }: { item: ViewerTruthState[keyof ViewerTruthState] }) {
  return (
    <article className={`viewer-truth-item viewer-truth-${item.state}`}>
      <span>{item.label}</span>
      <strong>{item.value}</strong>
      <small>{item.note}</small>
    </article>
  );
}
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/ViewerTruthOverlay.test.tsx
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/components/ViewerTruthOverlay.tsx ui/control-center/src/components/ViewerTruthOverlay.test.tsx
git commit -m "feat(ui): add viewer truth overlay"
```

## Task 4: Wire Truth State Into WorldViewer And Renderer

**Files:**

- Modify: `ui/control-center/src/components/WorldViewer.tsx`
- Modify: `ui/control-center/src/components/WorldViewer.test.tsx`
- Modify: `ui/control-center/src/viewer/worldRenderer.ts`

- [ ] **Step 1: Write failing WorldViewer tests**

Modify `ui/control-center/src/components/WorldViewer.test.tsx` by adding imports:

```tsx
import type { WorldFrame } from '../projection/types';
```

Add this test inside `describe('WorldViewer', () => { ... })`:

```tsx
  it('uses display radius for hit targets and exposes truth overlay for tiny live cells', async () => {
    const tinyLiveFrame: WorldFrame = {
      ...ui1aFixture.frame,
      source: 'live',
      resources: [],
      cells: [
        {
          id: 'tiny',
          x: 100,
          y: 100,
          radius: 2,
          energy: 0.8,
          integrity: 1,
          generation: 0,
          roleHint: 'alive lifecycle state',
          lifecycle: 1
        }
      ]
    };

    render(
      <WorldViewer
        frame={tinyLiveFrame}
        selectedCellId="tiny"
        onSelectCell={vi.fn()}
      />
    );

    await waitFor(() => {
      expect(renderFrame).toHaveBeenCalledWith(tinyLiveFrame, 'tiny');
    });

    expect(screen.getByLabelText('Select tiny')).toHaveStyle({ width: '14px', height: '14px' });
    expect(screen.getByLabelText('Viewer projection truth')).toHaveTextContent('Missing projection');
    expect(screen.getByLabelText('Viewer projection truth')).toHaveTextContent('Display minimum applied');
  });
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/WorldViewer.test.tsx -t "display radius"
```

Expected: FAIL because `Viewer projection truth` is absent and the hit target still uses raw radius.

- [ ] **Step 3: Update WorldViewer to use geometry and overlay**

Modify `ui/control-center/src/components/WorldViewer.tsx`:

```tsx
import { buildViewerTruthState } from './viewerTruth';
import { ViewerTruthOverlay } from './ViewerTruthOverlay';
import { projectCellForRender } from '../viewer/renderGeometry';
```

Inside the component, before `useImperativeHandle`, add:

```tsx
  const viewport = { width: frame.world.width, height: frame.world.height };
  const truthState = buildViewerTruthState(frame, viewport);
```

In the JSX, after `<div ref={hostRef} className="world-canvas-host" />`, add:

```tsx
      <ViewerTruthOverlay truthState={truthState} />
```

Replace the hit target map body:

```tsx
        {frame.cells.map((cell) => {
          const geometry = projectCellForRender(cell, frame, viewport);
          const left = `${(cell.x / frame.world.width) * 100}%`;
          const top = `${(cell.y / frame.world.height) * 100}%`;
          const diameter = `${geometry.displayRadiusPx * 2}px`;

          return (
            <button
              key={cell.id}
              type="button"
              className={cell.id === selectedCellId ? 'cell-hotspot selected' : 'cell-hotspot'}
              style={{ left, top, width: diameter, height: diameter }}
              onClick={() => onSelectCell(cell.id)}
              aria-label={`Select ${cell.id}`}
            />
          );
        })}
```

- [ ] **Step 4: Update renderer to use the shared geometry**

Modify `ui/control-center/src/viewer/worldRenderer.ts` import:

```ts
import { projectCellForRender } from './renderGeometry';
```

Replace the Cell loop geometry:

```ts
      const geometry = projectCellForRender(cell, frame, { width, height });
      const isSelected = cell.id === selectedCellId;

      cellGraphic.circle(geometry.x, geometry.y, geometry.displayRadiusPx);
```

Keep existing fill/stroke behavior.

- [ ] **Step 5: Run test to verify it passes**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/WorldViewer.test.tsx -t "display radius"
```

Expected: PASS.

- [ ] **Step 6: Run targeted viewer tests**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/viewer/renderGeometry.test.ts src/components/viewerTruth.test.ts src/components/ViewerTruthOverlay.test.tsx src/components/WorldViewer.test.tsx
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add ui/control-center/src/components/WorldViewer.tsx ui/control-center/src/components/WorldViewer.test.tsx ui/control-center/src/viewer/worldRenderer.ts
git commit -m "feat(ui): surface projection truth in world viewer"
```

## Task 5: AppShell Resource Layer Copy And Live App Test

**Files:**

- Modify: `ui/control-center/src/components/AppShell.tsx`
- Modify: `ui/control-center/src/App.test.tsx`

- [ ] **Step 1: Write failing App test for live missing resource truthfulness**

Modify the existing `updates the Monitor frame from a stream frame` test in `ui/control-center/src/App.test.tsx` by adding assertions after the live frame is shown:

```tsx
    expect(screen.getByLabelText('Viewer projection truth')).toHaveTextContent('Resources');
    expect(screen.getByLabelText('Viewer projection truth')).toHaveTextContent('Missing projection');
    expect(screen.getByText('Composite Resource Concentration')).toBeInTheDocument();
    expect(screen.getByText('Missing live projection')).toBeInTheDocument();
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/App.test.tsx -t "updates the Monitor frame from a stream frame"
```

Expected: FAIL because the layer label does not show `Missing live projection`.

- [ ] **Step 3: Update LayerPanel resource layer copy**

Modify `LayerPanel` in `ui/control-center/src/components/AppShell.tsx`:

```tsx
  const resourceLayerState = frame.resources.length === 0 ? 'Missing live projection' : 'Available projection';
```

Replace the Resource layer label:

```tsx
      <label className={frame.resources.length === 0 ? 'layer-option layer-option-missing' : 'layer-option'}>
        <input type="checkbox" checked={frame.resources.length > 0} readOnly disabled={frame.resources.length === 0} />
        <span>
          Composite Resource Concentration
          <small>{resourceLayerState}</small>
        </span>
      </label>
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/App.test.tsx -t "updates the Monitor frame from a stream frame"
```

Expected: PASS.

- [ ] **Step 5: Run full App tests**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/App.test.tsx
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git add ui/control-center/src/components/AppShell.tsx ui/control-center/src/App.test.tsx
git commit -m "feat(ui): clarify missing live resource projection"
```

## Task 6: Styling And E2E Truthfulness Smoke

**Files:**

- Modify: `ui/control-center/src/styles.css`
- Modify: `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

- [ ] **Step 1: Extend e2e smoke with truth overlay expectations**

Modify `openMonitor` in `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts` by adding:

```ts
  await expect(page.getByLabel('Viewer projection truth')).toBeVisible();
```

Modify `1920x1080 dark keeps World View dominant and captures acceptance screenshot` by adding:

```ts
    await expect(page.getByLabel('Viewer projection truth')).toContainText('Resources');
    await expect(page.getByLabel('Viewer projection truth')).toContainText('Fixture grid');
    await expect(page.getByLabel('Viewer projection truth')).toContainText('Cell size');
```

- [ ] **Step 2: Run e2e test to verify it fails before styling/integration is complete**

Run:

```powershell
cd ui\control-center
npm.cmd run e2e -- tests/e2e/ui-1c-a-visual.spec.ts
```

Expected before Task 4 completion: FAIL because overlay is absent. Expected after Task 4: may PASS before styling; if it passes, continue and use styling tests as visual hardening.

- [ ] **Step 3: Add truth overlay styling**

Modify `ui/control-center/src/styles.css`:

```css
.viewer-truth-overlay {
  position: absolute;
  top: 22px;
  right: 22px;
  z-index: 3;
  display: grid;
  gap: 8px;
  width: min(320px, calc(100% - 44px));
  pointer-events: none;
}

.viewer-truth-item {
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  padding: 8px 10px;
  background: rgba(6, 13, 17, 0.74);
  box-shadow: 0 10px 28px rgba(0, 0, 0, 0.22);
  backdrop-filter: blur(8px);
}

.viewer-truth-item span,
.viewer-truth-item small,
.layer-option small {
  display: block;
  color: #9fadb9;
  font-size: 12px;
}

.viewer-truth-item strong {
  display: block;
  margin-top: 2px;
  font-size: 13px;
}

.viewer-truth-missing,
.viewer-truth-presentation-minimum,
.layer-option-missing {
  border-color: rgba(240, 184, 79, 0.38);
}

:root[data-theme='light'] .viewer-truth-item {
  background: rgba(255, 255, 255, 0.9);
  border-color: #d9e0e7;
}

@media (max-width: 1366px) {
  .viewer-truth-overlay {
    top: 18px;
    right: 18px;
    width: min(280px, calc(100% - 36px));
  }
}
```

- [ ] **Step 4: Run e2e test to verify it passes**

Run:

```powershell
cd ui\control-center
npm.cmd run e2e -- tests/e2e/ui-1c-a-visual.spec.ts
```

Expected: PASS and updated screenshots in `ui/control-center/test-results/ui-1c-a/`.

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/styles.css ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts
git commit -m "test(ui): verify viewer projection truth overlay"
```

## Task 7: Full Verification And Report

**Files:**

- Modify: `outputs/worklogs/index.md`
- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-b-projection-truthfulness-render-scale.md`

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

Expected: no output except acceptable LF/CRLF warnings already present on this Windows workspace.

- [ ] **Step 3: Create implementation report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-b-projection-truthfulness-render-scale.md`:

```md
---
tags:
  - alife
  - worklog/report
  - ui
  - ui-1c
---

# UI-1C-B Projection Truthfulness And Render Scale Report

## Summary

Implemented viewer projection truth state, shared Cell render geometry and visible missing-resource / display-minimum indicators.

## Changed Files

- `ui/control-center/src/viewer/renderGeometry.ts`
- `ui/control-center/src/viewer/renderGeometry.test.ts`
- `ui/control-center/src/components/viewerTruth.ts`
- `ui/control-center/src/components/viewerTruth.test.ts`
- `ui/control-center/src/components/ViewerTruthOverlay.tsx`
- `ui/control-center/src/components/ViewerTruthOverlay.test.tsx`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/components/WorldViewer.test.tsx`
- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/App.test.tsx`
- `ui/control-center/src/viewer/worldRenderer.ts`
- `ui/control-center/src/styles.css`
- `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

## Verification

- `npm.cmd test`
- `npm.cmd run build`
- `npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts`
- `git diff --check`

## Manual Check Now

- Open `http://127.0.0.1:5173/`.
- In fixture state, confirm the Viewer truth overlay says `Resources / Fixture grid`.
- Run a live scenario; when `ALIF/v2` frames arrive, confirm the overlay says `Resources / Missing projection`.
- If small live Cells are visible, confirm the overlay says `Cell size / Display minimum applied`.
- Confirm the Resource layer row says `Missing live projection` when live `resources` is empty.

## Unresolved Issues

- Live Resource grid still requires Runner/Observer projection work.
- Bootstrap/physics spacing is not changed by this UI slice.
- Semantic zoom remains in a later UI-1C slice.

## Next Recommended Slice

`UI-1C-C`: Google Maps-like Viewer zoom, pan and selection navigation on top of truthful scale/projection state.
```

- [ ] **Step 4: Register the report in worklog index**

Add the final report link under `## Reports` in `outputs/worklogs/index.md`.

- [ ] **Step 5: Commit**

```powershell
git add outputs/worklogs/index.md outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-b-projection-truthfulness-render-scale.md
git commit -m "docs(ui): report UI-1C-B projection truthfulness"
```

## Final Acceptance Gate

`UI-1C-B` can be marked complete only if:

- every new behavior has a RED/GREEN test cycle;
- `WorldViewer` hit targets and renderer use the same display radius model;
- live empty `resources` is visible as missing projection;
- fixture resources remain visible as fixture data;
- user can tell when display radius is enlarged for visibility;
- no simulation behavior, Runner command, ALIF schema, Core physics, or bootstrap state is changed.

## What The User Can Check Immediately

After implementation, the user should be able to verify progress without reading the report:

1. Start web UI and see a small `Viewer projection truth` overlay in World View.
2. In fixture mode, see `Resources: Fixture grid`.
3. In live mode, see `Resources: Missing projection` until Runner/Observer sends a resource grid.
4. If live Cells are tiny, see `Cell size: Display minimum applied`.
5. See `Missing live projection` next to the Resource layer instead of a misleading active heatmap.

## Next Slice Recommendation

After `UI-1C-B`, move to:

```text
UI-1C-C:
Google Maps-Like Viewer Zoom, Pan And Selection Navigation
```

Rationale:

- `UI-1C-B` makes the picture honest.
- `UI-1C-C` should make navigation feel like a map: mouse wheel zooms around cursor,
  drag pans, Fit/Reset restores the World, and hit targets stay aligned with Cells.
- Atmospheric renderer and richer selection feedback should move to `UI-1C-D`, after navigation is stable.
