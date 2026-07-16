---
tags:
  - alife
  - worklog/plan
  - ui
  - tdd
  - ui-1c
---

# UI-1C-D Atmospheric Renderer Selection Semantic Detail Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add data-bound atmospheric rendering, stronger selection feedback and the first semantic zoom detail layer to the Monitor World Viewer.

**Architecture:** Keep simulation truth unchanged. Add a pure semantic-detail model that derives display levels and encodings from existing `WorldFrame` fields, then apply that model to Pixi render plans and DOM overlays. `WorldViewer` remains the owner of navigation state; renderer detail is presentation-only.

**Tech Stack:** React 19, TypeScript, Vitest, Testing Library, Playwright, PixiJS 8, existing Control Center CSS.

---

## UI-1C Status Before This Plan

Already completed:

- `UI-1C-A`: world-first Monitor layout, selected focus card, bottom stats and screenshot acceptance.
- `UI-1C-B`: projection truthfulness, missing live resources, physical/display radius clarity.
- `UI-1C-C`: Google Maps-like zoom/pan/fit/reset navigation, aligned Pixi rendering and DOM hit targets.

Still remaining inside `UI-1C`:

- atmospheric world rendering that is still data-bound;
- stronger selection feedback than a thin hotspot border;
- first semantic zoom layer using available projection fields only;
- richer selected focus presentation using `energy`, `integrity`, `lifecycle`, `position`, `radius`;
- visual acceptance screenshots after renderer/detail changes.

Explicit non-goals for this slice:

- no fake internals, organs, materials, processes, contacts or resource flows;
- no charts or analytics dashboard;
- no Runner/Observer/ALIF/Core contract changes;
- no semantic zoom levels that require unavailable projection fields;
- no new UI workspaces.

## Canonical Inputs

- `docs/PRINCIPLES.md`
- `docs/ui/presentation.md`, section `UI-1C Design Alignment`
- `docs/ui/visualization.md`, sections `Pseudo-3D Presentation`, `Semantic Zoom`, `Cell Rendering`, `Selection Hierarchy`
- `docs/implementation/implementation-plan-ui.md`, `UI-1C-D`
- `outputs/worklogs/2026-07-16-2120-REPORT-ui-1c-a-world-first-monitor-layout.md`
- `outputs/worklogs/2026-07-16-2236-REPORT-ui-1c-b-projection-truthfulness-render-scale.md`
- `outputs/worklogs/2026-07-17-0022-REPORT-ui-1c-c-viewer-zoom-pan-navigation.md`

## Design Summary

Chosen approach: **semantic render plan + presentational DOM detail overlays**.

Pros:

- keeps all new detail derived from available projection fields;
- makes LOD thresholds and visual encodings unit-testable;
- avoids Pixi internals in component tests;
- improves WOW without blocking on Runner/Observer.

Cons:

- still an immediate redraw renderer, not a batched/instanced renderer;
- semantic detail is a first pass, not full Internal Detail;
- atmospheric resource rendering is limited by current `resources` grid and unavailable in live ALIF v2.

Confidence: high for UI-1C-D scope; medium for long-term renderer architecture because future semantic zoom, resource legends and large populations will need a deeper render pipeline.

## Files

- Create: `ui/control-center/src/viewer/semanticDetail.ts`
- Create: `ui/control-center/src/viewer/semanticDetail.test.ts`
- Modify: `ui/control-center/src/viewer/worldRenderer.ts`
- Modify: `ui/control-center/src/viewer/worldRenderer.test.ts`
- Modify: `ui/control-center/src/components/WorldViewer.tsx`
- Modify: `ui/control-center/src/components/WorldViewer.test.tsx`
- Modify: `ui/control-center/src/components/SelectedEntityFocusCard.tsx`
- Modify: `ui/control-center/src/components/SelectedEntityFocusCard.test.tsx`
- Modify: `ui/control-center/src/styles.css`
- Modify: `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`
- Create after implementation: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-d-atmospheric-renderer-selection-semantic-detail.md`
- Modify after implementation: `outputs/worklogs/index.md`

## Success Criteria

Implementation is successful when:

- renderer plan exposes data-bound visual encodings for lifecycle, energy, integrity and selection;
- selected Cell has stronger visual feedback that does not alter primary data color meaning;
- first semantic detail level appears for selected or sufficiently zoomed Cells;
- DOM hit targets remain aligned with Pixi camera projection;
- focus card exposes compact data-bound bars for Energy and Integrity;
- live frames with missing resources continue to show unavailable/missing state instead of fake heatmaps;
- screenshot acceptance still passes at `1920x1080 dark`, `1366x768 dark`, and `1920x1080 light`;
- full UI tests, build and targeted e2e pass.

## Task 1: Semantic Detail Model

**Files:**

- Create: `ui/control-center/src/viewer/semanticDetail.ts`
- Create: `ui/control-center/src/viewer/semanticDetail.test.ts`

- [ ] **Step 1: Write failing tests for semantic detail levels and encodings**

Create `ui/control-center/src/viewer/semanticDetail.test.ts`:

```ts
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
});
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/viewer/semanticDetail.test.ts
```

Expected: FAIL because `./semanticDetail` does not exist.

- [ ] **Step 3: Implement minimal semantic detail model**

Create `ui/control-center/src/viewer/semanticDetail.ts`:

```ts
import type { CellProjection } from '../projection/types';

export type SemanticZoomLevel = 'overview' | 'entity' | 'structure' | 'internal-detail';
export type LifecycleVisualState = 'alive' | 'dead' | 'decomposing' | 'unavailable';

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
  const selectedBoost = input.selected ? 1 : 0;
  const radius = input.displayRadiusPx;

  if (radius >= 48) {
    return 'internal-detail';
  }
  if (radius >= 28 || selectedBoost === 1 && radius >= 14) {
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
  const showMetricRings = input.selected || level === 'structure' || level === 'internal-detail';

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

function lifecycleVisualState(lifecycle: number | undefined): LifecycleVisualState {
  if (lifecycle === 1) {
    return 'alive';
  }
  if (lifecycle === 2) {
    return 'dead';
  }
  if (lifecycle === 3) {
    return 'decomposing';
  }
  return 'unavailable';
}
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/viewer/semanticDetail.test.ts
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/viewer/semanticDetail.ts ui/control-center/src/viewer/semanticDetail.test.ts
git commit -m "feat(ui): add semantic viewer detail model"
```

## Task 2: Atmospheric Render Plan And Pixi Rendering

**Files:**

- Modify: `ui/control-center/src/viewer/worldRenderer.ts`
- Modify: `ui/control-center/src/viewer/worldRenderer.test.ts`

- [ ] **Step 1: Write failing renderer plan tests**

Modify `ui/control-center/src/viewer/worldRenderer.test.ts` by replacing the existing assertion with the expanded plan assertion:

```ts
    expect(plan.cells).toEqual([
      {
        id: 'cell-a',
        x: 140,
        y: 200,
        radius: 24,
        selected: true,
        lifecycleState: 'alive',
        energyRatio: 0.5,
        integrityRatio: 1,
        semanticLevel: 'structure',
        showMetricRings: true,
        label: 'cell-a · E50 · I100'
      }
    ]);
    expect(plan.hasResourceField).toBe(true);
```

Add this test inside `describe('createWorldRenderPlan', () => { ... })`:

```ts
  it('keeps live missing resources explicit in the render plan', () => {
    const plan = createWorldRenderPlan({ ...frame, source: 'live', resources: [] }, null, {
      width: 1200,
      height: 800
    });

    expect(plan.hasResourceField).toBe(false);
    expect(plan.cells[0].semanticLevel).toBe('entity');
    expect(plan.cells[0].showMetricRings).toBe(false);
  });
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/viewer/worldRenderer.test.ts
```

Expected: FAIL because `WorldRenderPlan` does not expose semantic fields.

- [ ] **Step 3: Extend render plan with data-bound semantic fields**

Modify `ui/control-center/src/viewer/worldRenderer.ts` imports:

```ts
import { buildCellSemanticDetail, type LifecycleVisualState, type SemanticZoomLevel } from './semanticDetail';
```

Replace `RenderPlanCell` and `WorldRenderPlan` with:

```ts
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
```

Modify `createWorldRenderPlan` return:

```ts
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
```

- [ ] **Step 4: Implement atmospheric Pixi drawing from the render plan**

In `worldRenderer.ts`, replace the cell drawing loop inside `renderFrame` with:

```ts
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

        cellGraphic.arc(cell.x, cell.y, cell.radius + 2, -Math.PI / 2, -Math.PI / 2 + Math.PI * 2 * cell.integrityRatio);
        cellGraphic.stroke({ width: 2, color: 0x74ded2, alpha: 0.72 });
      }

      root.addChild(cellGraphic);
    }
```

Add helper functions near the bottom:

```ts
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
```

- [ ] **Step 5: Improve resource field drawing without faking missing live data**

In `drawResourceLayer`, update the per-cell drawing block:

```ts
      const total = Math.max(0, Math.min(1, (resource.organic + resource.mineral + resource.energy) / 3));
      const energyBias = Math.max(0, Math.min(1, resource.energy));
      const alpha = 0.12 + total * 0.38;
      const color = energyBias > resource.organic ? 0x2f80ed : 0x27b582;
      layer.rect(camera.x + x * cellWidth, camera.y + y * cellHeight, cellWidth, cellHeight);
      layer.fill({ color, alpha });
```

- [ ] **Step 6: Run renderer tests**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/viewer/worldRenderer.test.ts src/viewer/semanticDetail.test.ts
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add ui/control-center/src/viewer/worldRenderer.ts ui/control-center/src/viewer/worldRenderer.test.ts
git commit -m "feat(ui): add atmospheric world render plan"
```

## Task 3: Semantic DOM Labels And Selection Feedback

**Files:**

- Modify: `ui/control-center/src/components/WorldViewer.tsx`
- Modify: `ui/control-center/src/components/WorldViewer.test.tsx`
- Modify: `ui/control-center/src/styles.css`

- [ ] **Step 1: Write failing WorldViewer tests for semantic labels**

Modify `ui/control-center/src/components/WorldViewer.test.tsx` by importing:

```ts
import { zoomCameraAtPoint } from '../viewer/viewerNavigation';
```

Add this test inside `describe('WorldViewer', () => { ... })`:

```tsx
  it('shows data-bound selected Cell detail label without changing selection behavior', async () => {
    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    expect(screen.getByLabelText('Selected cell detail label')).toHaveTextContent('cell-a · E82 · I91');
    expect(screen.getByLabelText('Select cell-a')).toHaveAttribute('data-semantic-level', 'structure');
  });
```

Add this test:

```tsx
  it('shows zoomed semantic labels for sufficiently large visible Cells', async () => {
    const user = userEvent.setup();

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await user.click(screen.getByRole('button', { name: 'Zoom in World Viewer' }));
    await user.click(screen.getByRole('button', { name: 'Zoom in World Viewer' }));

    expect(screen.getByLabelText('Selected cell detail label')).toHaveTextContent('cell-a · E82 · I91');
  });
```

If `zoomCameraAtPoint` is unused after editing, remove the import before committing.

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/WorldViewer.test.tsx
```

Expected: FAIL because semantic labels and `data-semantic-level` are absent.

- [ ] **Step 3: Add semantic detail to WorldViewer hit targets and labels**

Modify `ui/control-center/src/components/WorldViewer.tsx` imports:

```ts
import { buildCellSemanticDetail } from '../viewer/semanticDetail';
```

Inside the `frame.cells.map((cell) => { ... })` block, after `geometry`:

```tsx
          const selected = cell.id === selectedCellId;
          const detail = buildCellSemanticDetail(cell, {
            displayRadiusPx: geometry.displayRadiusPx,
            selected
          });
```

Replace the button class and attributes with:

```tsx
              className={selected ? 'cell-hotspot selected' : 'cell-hotspot'}
              data-semantic-level={detail.level}
              data-lifecycle-state={detail.lifecycleState}
```

Add after the button:

```tsx
            {detail.showLabel ? (
              <span
                className={selected ? 'cell-detail-label selected' : 'cell-detail-label'}
                style={{ left: `${geometry.x}px`, top: `${geometry.y + geometry.displayRadiusPx + 10}px` }}
                aria-label={selected ? 'Selected cell detail label' : `Cell detail label ${cell.id}`}
              >
                {detail.label}
              </span>
            ) : null}
```

Use a React fragment around the button and optional label:

```tsx
          return (
            <Fragment key={cell.id}>
              ...
            </Fragment>
          );
```

Update React import:

```tsx
import { Fragment, forwardRef, useEffect, useImperativeHandle, useRef, useState } from 'react';
```

- [ ] **Step 4: Add CSS for semantic labels and stronger selection feedback**

Add to `ui/control-center/src/styles.css` near `.cell-hotspot`:

```css
.cell-hotspot[data-semantic-level='structure'],
.cell-hotspot[data-semantic-level='internal-detail'] {
  border-color: rgba(116, 222, 210, 0.36);
}

.cell-hotspot[data-lifecycle-state='dead'] {
  border-style: dashed;
}

.cell-hotspot.selected {
  border-color: #ffd166;
  box-shadow:
    0 0 0 3px rgba(255, 209, 102, 0.36),
    0 0 28px rgba(255, 209, 102, 0.28);
}

.cell-detail-label {
  position: absolute;
  z-index: 3;
  max-width: 190px;
  transform: translate(-50%, 0);
  border: 1px solid rgba(116, 222, 210, 0.24);
  border-radius: 5px;
  padding: 4px 7px;
  background: rgba(6, 13, 17, 0.78);
  color: #dce6f1;
  font-size: 12px;
  font-weight: 700;
  line-height: 1.2;
  pointer-events: none;
  white-space: nowrap;
  box-shadow: 0 10px 24px rgba(0, 0, 0, 0.22);
}

.cell-detail-label.selected {
  border-color: rgba(255, 209, 102, 0.58);
  color: #fff4cf;
}

:root[data-theme='light'] .cell-detail-label {
  background: rgba(255, 255, 255, 0.92);
  border-color: #cfd8e2;
  color: #26323d;
}

:root[data-theme='light'] .cell-detail-label.selected {
  border-color: rgba(151, 110, 22, 0.42);
  color: #4c3912;
}
```

- [ ] **Step 5: Run component tests**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/WorldViewer.test.tsx src/viewer/semanticDetail.test.ts
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git add ui/control-center/src/components/WorldViewer.tsx ui/control-center/src/components/WorldViewer.test.tsx ui/control-center/src/styles.css
git commit -m "feat(ui): show semantic cell detail labels"
```

## Task 4: Data-Bound Focus Card Bars

**Files:**

- Modify: `ui/control-center/src/components/SelectedEntityFocusCard.tsx`
- Modify: `ui/control-center/src/components/SelectedEntityFocusCard.test.tsx`
- Modify: `ui/control-center/src/styles.css`

- [ ] **Step 1: Write failing focus card tests**

Modify `ui/control-center/src/components/SelectedEntityFocusCard.test.tsx` by adding:

```tsx
  it('renders data-bound energy and integrity bars for selected Cell', () => {
    render(<SelectedEntityFocusCard selectedCell={cell} />);

    expect(screen.getByLabelText('Selected cell energy')).toHaveAttribute('aria-valuenow', '82');
    expect(screen.getByLabelText('Selected cell integrity')).toHaveAttribute('aria-valuenow', '91');
  });
```

If the existing fixture cell in this test is not named `cell`, create it as:

```tsx
const cell: CellProjection = {
  id: 'cell-a',
  x: 330,
  y: 320,
  radius: 24,
  energy: 0.82,
  integrity: 0.91,
  generation: 3,
  roleHint: 'high-energy cluster member'
};
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/SelectedEntityFocusCard.test.tsx
```

Expected: FAIL because bar elements are absent.

- [ ] **Step 3: Add compact bars to selected focus card**

Modify `SelectedEntityFocusCard.tsx` by adding after the `dl`:

```tsx
      <div className="selected-focus-bars" aria-label="Selected cell projection bars">
        <MetricBar label="Energy" value={selectedCell.energy} ariaLabel="Selected cell energy" />
        <MetricBar label="Integrity" value={selectedCell.integrity} ariaLabel="Selected cell integrity" />
      </div>
```

Add below `SelectedEntityFocusCard`:

```tsx
function MetricBar({ label, value, ariaLabel }: { label: string; value: number; ariaLabel: string }) {
  const percent = Math.round(clampRatio(value) * 100);

  return (
    <div className="selected-focus-bar">
      <span>{label}</span>
      <div
        role="meter"
        aria-label={ariaLabel}
        aria-valuemin={0}
        aria-valuemax={100}
        aria-valuenow={percent}
      >
        <i style={{ width: `${percent}%` }} />
      </div>
    </div>
  );
}

function clampRatio(value: number) {
  return Math.max(0, Math.min(1, value));
}
```

- [ ] **Step 4: Add focus bar CSS**

Add to `styles.css` near `.selected-focus-card`:

```css
.selected-focus-bars {
  display: grid;
  gap: 7px;
  margin-top: 12px;
}

.selected-focus-bar {
  display: grid;
  gap: 4px;
}

.selected-focus-bar div {
  height: 6px;
  overflow: hidden;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.08);
}

.selected-focus-bar i {
  display: block;
  height: 100%;
  border-radius: inherit;
  background: linear-gradient(90deg, #74ded2, #ffd166);
}

:root[data-theme='light'] .selected-focus-bar div {
  background: #e1e8ef;
}
```

- [ ] **Step 5: Run focus card tests**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/SelectedEntityFocusCard.test.tsx
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git add ui/control-center/src/components/SelectedEntityFocusCard.tsx ui/control-center/src/components/SelectedEntityFocusCard.test.tsx ui/control-center/src/styles.css
git commit -m "feat(ui): add selected cell focus meters"
```

## Task 5: Visual Acceptance For UI-1C-D

**Files:**

- Modify: `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

- [ ] **Step 1: Add failing visual acceptance for semantic detail**

Modify `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts` by adding this test inside the existing `describe`:

```ts
  test('1920x1080 dark shows selected semantic detail and focus meters', async ({ page }) => {
    await openMonitor(page, { width: 1920, height: 1080 });

    await expect(page.getByLabel('Selected cell detail label')).toContainText('cell-a');
    await expect(page.getByLabel('Selected cell energy')).toHaveAttribute('aria-valuenow', '82');
    await expect(page.getByLabel('Selected cell integrity')).toHaveAttribute('aria-valuenow', '91');

    await page.screenshot({ path: join(screenshotDir, '1920x1080-semantic-detail.png'), fullPage: true });
  });
```

- [ ] **Step 2: Run e2e to verify it passes after Tasks 3-4**

Run:

```powershell
cd ui\control-center
npm.cmd run e2e -- tests/e2e/ui-1c-a-visual.spec.ts
```

Expected: PASS after Tasks 3-4. If this fails due to visual overlap, adjust CSS only and rerun.

- [ ] **Step 3: Inspect screenshot**

Inspect:

```text
ui/control-center/test-results/ui-1c-a/1920x1080-semantic-detail.png
```

Expected visual result:

- selected Cell label is visible and does not overlap navigation controls or truth overlay;
- focus meters are visible in selected focus card;
- World remains dominant;
- light and 1366 acceptance from existing tests still pass.

- [ ] **Step 4: Commit**

```powershell
git add ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts
git commit -m "test(ui): verify semantic detail visual acceptance"
```

## Task 6: Full Verification And Report

**Files:**

- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-d-atmospheric-renderer-selection-semantic-detail.md`
- Modify: `outputs/worklogs/index.md`

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

Expected: no whitespace errors. Windows LF/CRLF warnings are acceptable only if no whitespace errors are reported.

- [ ] **Step 3: Create implementation report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-d-atmospheric-renderer-selection-semantic-detail.md`:

```md
---
tags:
  - alife
  - worklog/report
  - ui
  - ui-1c
---

# UI-1C-D Atmospheric Renderer Selection Semantic Detail Report

## Summary

Implemented data-bound atmospheric renderer detail, stronger selected Cell feedback, semantic labels and selected focus meters.

## Changed Files

- `ui/control-center/src/viewer/semanticDetail.ts`
- `ui/control-center/src/viewer/semanticDetail.test.ts`
- `ui/control-center/src/viewer/worldRenderer.ts`
- `ui/control-center/src/viewer/worldRenderer.test.ts`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/components/WorldViewer.test.tsx`
- `ui/control-center/src/components/SelectedEntityFocusCard.tsx`
- `ui/control-center/src/components/SelectedEntityFocusCard.test.tsx`
- `ui/control-center/src/styles.css`
- `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

## Verification

- `npm.cmd test`
- `npm.cmd run build`
- `npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts`
- `git diff --check`

## Manual Check Now

- Open `http://127.0.0.1:5173/`.
- Confirm the selected Cell has a stronger visible halo and a compact data label.
- Zoom in and confirm semantic detail stays aligned with the Cell.
- Confirm focus card shows Energy and Integrity bars.
- Switch Light theme and confirm labels/meters remain readable.

## Unresolved Issues

- Live resource field remains unavailable until Runner/Observer projection includes resources.
- Semantic zoom does not yet show Materials, Processes, Joints or contacts because those fields are not projected.
- Renderer is still immediate redraw, not a large-population optimized pipeline.

## Next Recommended Slice

`UI-1D`: Start Demo, Export And Acceptance Hardening.
```

- [ ] **Step 4: Register the report in worklog index**

Add the report under `## Reports` in `outputs/worklogs/index.md`.

- [ ] **Step 5: Commit**

```powershell
git add outputs/worklogs/index.md outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-d-atmospheric-renderer-selection-semantic-detail.md
git commit -m "docs(ui): report UI-1C-D atmospheric renderer"
```

## Final Acceptance Gate

`UI-1C-D` can be marked complete only if:

- every new behavior has a RED/GREEN test cycle;
- no visual detail invents unavailable simulation data;
- selected and semantic detail overlays remain aligned after zoom/pan;
- live missing resources remain explicit;
- 1920 dark, 1366 dark and 1920 light screenshots pass;
- full unit tests, build and targeted e2e pass.

## What The User Can Check Immediately

After implementation, the user should be able to verify progress without reading the report:

1. Open the Monitor and see the selected Cell with a stronger halo and detail label.
2. Zoom in and see semantic detail stay attached to the selected Cell.
3. See Energy and Integrity bars in the selected focus card.
4. Confirm Resources still say unavailable/missing in live mode when ALIF has no resource grid.
5. Switch to Light theme and confirm labels/meters remain usable.
