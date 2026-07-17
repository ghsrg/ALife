---
tags:
  - alife
  - worklog/plan
  - ui
  - architecture
  - tdd
---

# UI Architecture Stabilization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` (recommended) or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Stabilize the `ALife Control Center` frontend architecture after `UI-1C` so `UI-1D` and `UI-2` can add features without growing transport, projection, renderer, presentation and component concerns into one coupled surface.

**Architecture:** Keep the current React + Zustand + PixiJS stack. Refactor by extracting tested seams around Monitor composition, runner orchestration, view-model selectors, Viewer camera/gestures, render-plan generation, design tokens and UI text. This plan must not change simulation semantics, projection payloads, command semantics or visual scope.

**Tech Stack:** React 19, TypeScript, Vite, Zustand, PixiJS 8, Vitest, React Testing Library, Playwright.

---

## Why This Slice Exists

`UI-1A` through `UI-1C-D` produced a working Monitor, live Runner transport, map navigation and semantic rendering. The current implementation is good enough for a prototype, but there are coupling hotspots that will become expensive during `UI-1D` hardening and `UI-2 Debug`.

Observed hotspots:

- `ui/control-center/src/components/AppShell.tsx` owns Runner bootstrap, stream lifecycle, command sequencing, layout composition, panel implementation, formatting and stale-frame guards.
- `ui/control-center/src/components/WorldViewer.tsx` owns Pixi lifecycle, camera state, pointer gestures, overlay dismissal, hit target projection and toolbar controls.
- `ui/control-center/src/styles.css` is one global stylesheet with tokens, layout, components and state styles mixed together.
- User-facing strings are inline in components, despite the canonical UI text registry direction.
- Renderer plan logic and Pixi drawing are still close enough that future shader/batching work could accidentally change semantic output.

This slice is successful when architecture boundaries are visible in code and protected by tests, while the user-visible UI remains behaviorally unchanged.

## Canonical Sources

Must read before implementation:

- `docs/PRINCIPLES.md`
- `docs/INDEX.md`
- `docs/ui/architecture.md`
- `docs/ui/control-center-design-spec.md`
- `docs/ui/quality.md`
- `docs/implementation/implementation-plan-ui.md`
- `docs/implementation/ui-technology-stack.md`

Relevant current reports:

- `outputs/worklogs/2026-07-17-0109-REPORT-ui-1c-d-atmospheric-renderer-selection-semantic-detail.md`
- `outputs/worklogs/2026-07-17-0920-REPORT-ui-map-interaction-overlay-fix.md`

## Non-Goals

- No new Runner API.
- No `ALIF` protocol changes.
- No new simulation behavior.
- No new charting, analytics, Genome or OrganismView work.
- No Radix UI dependency adoption in this slice unless a tiny extracted primitive proves it is needed.
- No redesign beyond preserving current visual behavior.
- No full CSS Modules migration yet.

## Files

Create:

- `ui/control-center/src/app/monitorViewModel.ts`
- `ui/control-center/src/app/monitorViewModel.test.ts`
- `ui/control-center/src/app/runnerController.ts`
- `ui/control-center/src/app/runnerController.test.ts`
- `ui/control-center/src/components/MonitorWorkspace.tsx`
- `ui/control-center/src/components/MonitorWorkspace.test.tsx`
- `ui/control-center/src/components/LayerPanel.tsx`
- `ui/control-center/src/components/CellInspector.tsx`
- `ui/control-center/src/viewer/useViewerCamera.ts`
- `ui/control-center/src/viewer/useViewerCamera.test.ts`
- `ui/control-center/src/viewer/viewerHitTargets.ts`
- `ui/control-center/src/viewer/viewerHitTargets.test.ts`
- `ui/control-center/src/viewer/worldRenderPlan.ts`
- `ui/control-center/src/viewer/worldRenderPlan.test.ts`
- `ui/control-center/src/uiText.ts`
- `ui/control-center/src/uiText.test.ts`
- `ui/control-center/src/styles/tokens.css`
- `ui/control-center/src/styles/layout.css`
- `ui/control-center/src/styles/components.css`
- `ui/control-center/src/architecture/architectureBoundaries.test.ts`
- `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-architecture-stabilization.md`

Modify:

- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/viewer/worldRenderer.ts`
- `ui/control-center/src/viewer/worldRenderer.test.ts`
- `ui/control-center/src/styles.css`
- `ui/control-center/src/main.tsx`, only if style imports need to point at split CSS files.
- `ui/control-center/src/App.test.tsx`
- `ui/control-center/src/components/WorldViewer.test.tsx`
- `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`, only to keep acceptance wording aligned if labels move to `uiText`.
- `outputs/worklogs/index.md`

## Architecture Boundaries After This Slice

Target shape:

```text
AppShell
  owns app bootstrapping and top-level store subscription only

runnerController
  owns Runner API client, stream client, command sequencing and stale live-frame guard

monitorViewModel
  derives titles, subtitles, projection labels, resource layer state and stats inputs

MonitorWorkspace
  composes LayerPanel, WorldViewer, SelectedEntityFocusCard, BottomStatsStrip, CellInspector

WorldViewer
  owns DOM shell for one Viewer instance

useViewerCamera
  owns camera state, fit/reset/zoom/pan gestures

viewerHitTargets
  derives accessible DOM hit targets and labels from frame + camera + viewport

worldRenderPlan
  produces pure renderer plan from frame + selection + camera

worldRenderer
  owns PixiJS mounting and drawing only

uiText
  centralizes current English user-facing labels
```

## Task 1: Add Architecture Boundary Guard Tests

**Files:**

- Create: `ui/control-center/src/architecture/architectureBoundaries.test.ts`

- [ ] **Step 1: Write failing boundary tests**

Create `ui/control-center/src/architecture/architectureBoundaries.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';

const srcRoot = join(process.cwd(), 'src');

function listFiles(dir: string): string[] {
  return readdirSync(dir).flatMap((entry) => {
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) {
      return listFiles(path);
    }
    return path.endsWith('.ts') || path.endsWith('.tsx') ? [path] : [];
  });
}

function source(path: string) {
  return readFileSync(path, 'utf8');
}

describe('architecture boundaries', () => {
  it('keeps Runner transport out of presentational components', () => {
    const componentFiles = listFiles(join(srcRoot, 'components'));
    const offenders = componentFiles
      .filter((path) => !path.endsWith('AppShell.tsx'))
      .filter((path) => source(path).includes('../runner/') || source(path).includes('runner/'))
      .map((path) => relative(srcRoot, path));

    expect(offenders).toEqual([]);
  });

  it('keeps Pixi imports behind the renderer boundary', () => {
    const offenders = listFiles(srcRoot)
      .filter((path) => !path.endsWith(join('viewer', 'worldRenderer.ts')))
      .filter((path) => source(path).includes("from 'pixi.js'"))
      .map((path) => relative(srcRoot, path));

    expect(offenders).toEqual([]);
  });
});
```

- [ ] **Step 2: Run RED**

Run:

```powershell
npm.cmd test -- src/architecture/architectureBoundaries.test.ts
```

Expected: FAIL because current `AppShell.tsx` is still under `components` and imports `../runner/apiClient` / `../runner/streamClient`.

- [ ] **Step 3: Keep the test committed with later GREEN tasks**

Do not weaken this test. Later tasks should make it pass by moving transport orchestration out of presentational components.

## Task 2: Extract Monitor View Model

**Files:**

- Create: `ui/control-center/src/app/monitorViewModel.ts`
- Create: `ui/control-center/src/app/monitorViewModel.test.ts`
- Modify: `ui/control-center/src/components/AppShell.tsx`

- [ ] **Step 1: Write failing tests**

Create `ui/control-center/src/app/monitorViewModel.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import { createAppStore } from './appState';
import { buildMonitorViewModel } from './monitorViewModel';

describe('buildMonitorViewModel', () => {
  it('describes fixture idle state without pretending it is live', () => {
    const store = createAppStore();
    store.getState().setConnected({ engineVersion: '0.1.0', apiVersion: '1', allowRemoteViewer: false });
    store.getState().setRunStatus({
      processState: 'ready',
      activeRunState: 'idle',
      runId: null,
      committedTick: 0,
      scenarioId: null,
      scenarioHash: null,
      effectiveSeed: null,
      terminalReason: null
    });

    expect(buildMonitorViewModel(store.getState()).subtitle).toContain('Runner idle');
    expect(buildMonitorViewModel(store.getState()).projectionLabel).toBe('fixture/v1');
  });

  it('marks live frames as live projection context', () => {
    const store = createAppStore();
    const frame = {
      ...store.getState().frame,
      source: 'live' as const,
      tick: 42,
      resources: []
    };
    store.getState().setFrame(frame);

    const model = buildMonitorViewModel(store.getState());

    expect(model.subtitle).toBe('Live Tick 42');
    expect(model.projectionLabel).toBe('live/v1');
    expect(model.resourceLayerState).toBe('Missing live projection');
  });
});
```

- [ ] **Step 2: Run RED**

Run:

```powershell
npm.cmd test -- src/app/monitorViewModel.test.ts
```

Expected: FAIL because `monitorViewModel.ts` does not exist.

- [ ] **Step 3: Implement the minimal view model**

Create `ui/control-center/src/app/monitorViewModel.ts`:

```ts
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { getMonitorDataState, type AppStore } from './appState';

export interface MonitorViewModel {
  scenarioTitle: string;
  subtitle: string;
  projectionLabel: 'fixture/v1' | 'live/v1';
  hasResourceLayer: boolean;
  resourceLayerState: 'Available projection' | 'Missing live projection';
}

export function buildMonitorViewModel(state: AppStore): MonitorViewModel {
  const dataState = getMonitorDataState(state);
  const hasResourceLayer = state.frame.resources.length > 0;

  return {
    scenarioTitle: state.frame.scenarioName ?? ui1aFixture.scenarioName,
    subtitle: buildFrameSubtitle(state, dataState),
    projectionLabel: state.frame.source === 'live' ? 'live/v1' : 'fixture/v1',
    hasResourceLayer,
    resourceLayerState: hasResourceLayer ? 'Available projection' : 'Missing live projection'
  };
}

function buildFrameSubtitle(state: AppStore, dataState: ReturnType<typeof getMonitorDataState>) {
  if (dataState === 'fixture-idle') {
    return `Fixture Tick ${state.frame.tick} - Runner idle`;
  }

  if (dataState === 'live-waiting') {
    return `Waiting for live frame - Fixture Tick ${state.frame.tick}`;
  }

  if (dataState === 'stale-live') {
    return `Stale Live Tick ${state.frame.tick} - disconnected`;
  }

  return `${state.frame.source === 'live' ? 'Live' : 'Fixture'} Tick ${state.frame.tick}`;
}
```

- [ ] **Step 4: Use the view model in `AppShell.tsx`**

Replace inline `frameSubtitle(state)`, projection label and resource layer derivation with `buildMonitorViewModel(state)`.

- [ ] **Step 5: Remove dead helpers**

Remove `frameSubtitle` from `AppShell.tsx` once all callers use the view model.

- [ ] **Step 6: Run GREEN**

Run:

```powershell
npm.cmd test -- src/app/monitorViewModel.test.ts src/App.test.tsx
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add ui/control-center/src/app/monitorViewModel.ts ui/control-center/src/app/monitorViewModel.test.ts ui/control-center/src/components/AppShell.tsx
git commit -m "refactor(ui): extract monitor view model"
```

## Task 3: Extract Runner Controller

**Files:**

- Create: `ui/control-center/src/app/runnerController.ts`
- Create: `ui/control-center/src/app/runnerController.test.ts`
- Modify: `ui/control-center/src/components/AppShell.tsx`

- [ ] **Step 1: Write failing tests for stale frame and command sequencing**

Create `ui/control-center/src/app/runnerController.test.ts`:

```ts
import { describe, expect, it, vi } from 'vitest';
import { createAppStore } from './appState';
import { shouldApplyLiveFrame, shouldApplyRunStatus, createRequestId } from './runnerController';
import type { LiveWorldFrameProjection } from '../runner/alifDecoder';

const liveFrame: LiveWorldFrameProjection = {
  schemaVersion: 'ALIFWorldFrame/v2',
  runId: 'run-a',
  scenarioId: 'demo',
  committedTick: 10,
  projectionSequence: 2,
  world: { width: 1200, height: 800 },
  cells: [],
  summary: {
    aliveCells: 0,
    deadCells: 0,
    totalCellEnergy: 0,
    totalCellIntegrity: 0,
    projectionSequence: 2
  }
};

describe('runnerController guards', () => {
  it('rejects older live frames for the same active run', () => {
    const store = createAppStore();
    store.getState().setFrame({
      ...store.getState().frame,
      source: 'live',
      runId: 'run-a',
      tick: 11,
      summary: { projectionSequence: 4 }
    });
    store.getState().setRunStatus({
      processState: 'ready',
      activeRunState: 'running',
      runId: 'run-a',
      committedTick: 11,
      scenarioId: 'demo',
      scenarioHash: null,
      effectiveSeed: null,
      terminalReason: null
    });

    expect(shouldApplyLiveFrame(liveFrame, store.getState())).toBe(false);
  });

  it('accepts newer live frames for the current run', () => {
    const store = createAppStore();
    store.getState().setFrame({
      ...store.getState().frame,
      source: 'live',
      runId: 'run-a',
      tick: 9,
      summary: { projectionSequence: 1 }
    });

    expect(shouldApplyLiveFrame(liveFrame, store.getState())).toBe(true);
  });

  it('rejects older run status for the same run', () => {
    const store = createAppStore();
    store.getState().setRunStatus({
      processState: 'ready',
      activeRunState: 'running',
      runId: 'run-a',
      committedTick: 12,
      scenarioId: 'demo',
      scenarioHash: null,
      effectiveSeed: null,
      terminalReason: null
    });

    expect(shouldApplyRunStatus({
      processState: 'ready',
      activeRunState: 'running',
      runId: 'run-a',
      committedTick: 10,
      scenarioId: 'demo',
      scenarioHash: null,
      effectiveSeed: null,
      terminalReason: null
    }, store.getState())).toBe(false);
  });

  it('creates deterministic request ids from an injected clock', () => {
    expect(createRequestId(() => 1234)).toBe('ui-1234');
    expect(vi.isMockFunction(createRequestId)).toBe(false);
  });
});
```

- [ ] **Step 2: Run RED**

Run:

```powershell
npm.cmd test -- src/app/runnerController.test.ts
```

Expected: FAIL because `runnerController.ts` does not exist.

- [ ] **Step 3: Implement pure guard functions**

Create `ui/control-center/src/app/runnerController.ts`:

```ts
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { liveProjectionToWorldFrame } from '../projection/liveAdapter';
import type { LiveWorldFrameProjection } from '../runner/alifDecoder';
import type { RunStatus } from '../runner/apiClient';
import type { AppStore } from './appState';

export function createRequestId(now: () => number = Date.now) {
  return `ui-${now()}`;
}

export function shouldApplyRunStatus(runStatus: RunStatus | null, state: Pick<AppStore, 'runStatus'>) {
  if (runStatus === null || state.runStatus === null) {
    return true;
  }

  return !(
    runStatus.runId !== null &&
    runStatus.runId === state.runStatus.runId &&
    runStatus.committedTick < state.runStatus.committedTick
  );
}

export function shouldApplyLiveFrame(
  frame: LiveWorldFrameProjection,
  state: Pick<AppStore, 'frame' | 'runStatus'>
) {
  if (state.frame.source !== 'live') {
    return true;
  }

  const activeRunId = state.runStatus?.runId ?? state.frame.runId;
  if (state.frame.runId !== activeRunId) {
    return true;
  }

  if (frame.committedTick < state.frame.tick) {
    return false;
  }

  const currentSequence = state.frame.summary?.projectionSequence;
  return !(
    frame.committedTick === state.frame.tick &&
    currentSequence !== undefined &&
    frame.projectionSequence <= currentSequence
  );
}

export function toWorldFrame(frame: LiveWorldFrameProjection, state: Pick<AppStore, 'runStatus' | 'selectedScenarioId' | 'frame'>) {
  return liveProjectionToWorldFrame(frame, {
    runId: state.runStatus?.runId ?? state.frame.runId,
    scenarioName:
      state.runStatus?.scenarioId ??
      state.selectedScenarioId ??
      state.frame.scenarioName ??
      ui1aFixture.scenarioName
  });
}

export function toErrorMessage(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}
```

- [ ] **Step 4: Move orchestration from `AppShell.tsx` into a controller factory**

Add a `createRunnerController(...)` export in `runnerController.ts` only after pure guard tests pass. It should own:

- `RunnerApiClient`
- `RunnerStreamClient`
- stream disconnect/reconnect
- `connectRunner`
- `startRun`
- `pauseRun`
- `resumeRun`
- `stepRun`
- `stopRun`
- command sequence guard

Keep constructor dependencies injectable:

```ts
export interface RunnerControllerDependencies {
  store: StoreApi<AppStore>;
  createApiClient?: (endpoint: string) => RunnerApiClient;
  createStreamClient?: (
    endpoint: string,
    handlers: ConstructorParameters<typeof RunnerStreamClient>[1]
  ) => RunnerStreamClient;
  now?: () => number;
}
```

- [ ] **Step 5: Update `AppShell.tsx` to use the controller**

`AppShell.tsx` should keep only:

- store creation/subscription;
- controller lifecycle;
- theme side effect;
- export screenshot local state;
- shell composition.

- [ ] **Step 6: Run GREEN**

Run:

```powershell
npm.cmd test -- src/app/runnerController.test.ts src/App.test.tsx
```

Expected: PASS.

- [ ] **Step 7: Re-run architecture boundary test**

Run:

```powershell
npm.cmd test -- src/architecture/architectureBoundaries.test.ts
```

Expected: PASS for the Runner import boundary.

- [ ] **Step 8: Commit**

```powershell
git add ui/control-center/src/app/runnerController.ts ui/control-center/src/app/runnerController.test.ts ui/control-center/src/components/AppShell.tsx ui/control-center/src/architecture/architectureBoundaries.test.ts
git commit -m "refactor(ui): isolate runner controller"
```

## Task 4: Extract Monitor Workspace Components

**Files:**

- Create: `ui/control-center/src/components/MonitorWorkspace.tsx`
- Create: `ui/control-center/src/components/MonitorWorkspace.test.tsx`
- Create: `ui/control-center/src/components/LayerPanel.tsx`
- Create: `ui/control-center/src/components/CellInspector.tsx`
- Modify: `ui/control-center/src/components/AppShell.tsx`

- [ ] **Step 1: Write failing component test**

Create `ui/control-center/src/components/MonitorWorkspace.test.tsx`:

```tsx
import { describe, expect, it, vi } from 'vitest';
import { screen } from '@testing-library/react';
import { render } from '../test/render';
import { createAppStore } from '../app/appState';
import { buildMonitorStats } from './monitorStats';
import { MonitorWorkspace } from './MonitorWorkspace';

describe('MonitorWorkspace', () => {
  it('composes layer controls, viewer, focus card, stats and inspector from one state object', () => {
    const store = createAppStore();
    const state = store.getState();

    render(
      <MonitorWorkspace
        state={state}
        monitorStats={buildMonitorStats(state.frame, 'fixture-offline')}
        onScenarioChange={vi.fn()}
        onReconnect={vi.fn()}
        onSelectCell={vi.fn()}
        onToggleTheme={vi.fn()}
        onExportScreenshot={vi.fn()}
        exportStatus={null}
      />
    );

    expect(screen.getByLabelText('Layer controls')).toBeVisible();
    expect(screen.getByLabelText('Monitor workspace')).toBeVisible();
    expect(screen.getByLabelText('World Viewer', { exact: true })).toBeVisible();
    expect(screen.getByLabelText('Cell Inspector')).toContainText('cell-a');
    expect(screen.getByLabelText('World stats')).toBeVisible();
  });
});
```

- [ ] **Step 2: Run RED**

Run:

```powershell
npm.cmd test -- src/components/MonitorWorkspace.test.tsx
```

Expected: FAIL because `MonitorWorkspace.tsx` does not exist.

- [ ] **Step 3: Move `LayerPanel` out of `AppShell.tsx`**

Create `LayerPanel.tsx` using the existing JSX and the `MonitorViewModel` fields instead of recomputing resource labels.

- [ ] **Step 4: Move `Inspector` out of `AppShell.tsx`**

Create `CellInspector.tsx` using the existing selected Cell rendering and `formatRatio` helper local to that file.

- [ ] **Step 5: Create `MonitorWorkspace.tsx`**

`MonitorWorkspace` should own the current Monitor grid JSX and receive callbacks from `AppShell`.

- [ ] **Step 6: Slim `AppShell.tsx`**

Replace the whole `<main className="monitor-grid">...</main>` block with:

```tsx
<MonitorWorkspace
  state={state}
  monitorStats={monitorStats}
  onScenarioChange={(scenarioId) => store.getState().setSelectedScenarioId(scenarioId)}
  onReconnect={controller.connectRunner}
  onSelectCell={(cellId) => store.getState().selectCell(cellId)}
  onToggleTheme={toggleTheme}
  onExportScreenshot={exportScreenshot}
  exportStatus={exportStatus}
/>
```

- [ ] **Step 7: Run GREEN**

Run:

```powershell
npm.cmd test -- src/components/MonitorWorkspace.test.tsx src/App.test.tsx
```

Expected: PASS.

- [ ] **Step 8: Commit**

```powershell
git add ui/control-center/src/components/MonitorWorkspace.tsx ui/control-center/src/components/MonitorWorkspace.test.tsx ui/control-center/src/components/LayerPanel.tsx ui/control-center/src/components/CellInspector.tsx ui/control-center/src/components/AppShell.tsx
git commit -m "refactor(ui): extract monitor workspace"
```

## Task 5: Extract Viewer Camera And Hit Target Derivation

**Files:**

- Create: `ui/control-center/src/viewer/useViewerCamera.ts`
- Create: `ui/control-center/src/viewer/useViewerCamera.test.ts`
- Create: `ui/control-center/src/viewer/viewerHitTargets.ts`
- Create: `ui/control-center/src/viewer/viewerHitTargets.test.ts`
- Modify: `ui/control-center/src/components/WorldViewer.tsx`
- Modify: `ui/control-center/src/components/WorldViewer.test.tsx`

- [ ] **Step 1: Write failing camera hook tests for pure reducer helpers**

Create `ui/control-center/src/viewer/useViewerCamera.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import { createViewerCameraState, viewerCameraReducer } from './useViewerCamera';

describe('viewerCameraReducer', () => {
  it('zooms around a viewer point', () => {
    const state = createViewerCameraState();

    expect(viewerCameraReducer(state, {
      type: 'zoom-at',
      point: { x: 300, y: 200 },
      scaleFactor: 2
    }).camera).toEqual({ x: -300, y: -200, scale: 2 });
  });

  it('tracks drag movement without selecting on the same click', () => {
    let state = createViewerCameraState();
    state = viewerCameraReducer(state, { type: 'drag-start', pointerId: 1, point: { x: 10, y: 10 } });
    state = viewerCameraReducer(state, { type: 'drag-move', pointerId: 1, point: { x: 15, y: 4 } });

    expect(state.camera).toEqual({ x: 5, y: -6, scale: 1 });
    expect(state.dragMoved).toBe(true);
  });
});
```

- [ ] **Step 2: Write failing hit target tests**

Create `ui/control-center/src/viewer/viewerHitTargets.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { loadFixtureFrame } from '../projection/fixtureAdapter';
import { buildViewerHitTargets } from './viewerHitTargets';

describe('buildViewerHitTargets', () => {
  it('derives accessible hit targets from frame, viewport and camera', () => {
    const frame = loadFixtureFrame(ui1aFixture);

    const targets = buildViewerHitTargets(frame, 'cell-a', { width: 1200, height: 800 }, { x: 0, y: 0, scale: 1 });

    expect(targets[0]).toMatchObject({
      id: 'cell-a',
      selected: true,
      ariaLabel: 'Select cell-a'
    });
    expect(targets[0].style.width).toMatch(/px$/);
  });
});
```

- [ ] **Step 3: Run RED**

Run:

```powershell
npm.cmd test -- src/viewer/useViewerCamera.test.ts src/viewer/viewerHitTargets.test.ts
```

Expected: FAIL because the modules do not exist.

- [ ] **Step 4: Implement pure camera reducer and hook wrapper**

`useViewerCamera.ts` should expose:

```ts
export interface ViewerCameraState {
  camera: ViewerCamera;
  dragStart: { pointerId: number; x: number; y: number } | null;
  dragMoved: boolean;
}

export type ViewerCameraAction =
  | { type: 'zoom-at'; point: ScreenPoint; scaleFactor: number }
  | { type: 'fit'; world: Size; viewport: Size }
  | { type: 'reset' }
  | { type: 'drag-start'; pointerId: number; point: ScreenPoint }
  | { type: 'drag-move'; pointerId: number; point: ScreenPoint }
  | { type: 'drag-end'; pointerId: number }
  | { type: 'clear-drag-moved' };
```

Use existing `zoomCameraAtPoint`, `panCamera`, `fitCameraToWorld`, `resetCamera`.

- [ ] **Step 5: Implement hit target derivation**

`viewerHitTargets.ts` should expose:

```ts
export interface ViewerHitTarget {
  id: CellId;
  selected: boolean;
  detail: ReturnType<typeof buildCellSemanticDetail>;
  style: { left: string; top: string; width: string; height: string };
  labelStyle: { left: string; top: string };
  ariaLabel: string;
}
```

Use existing `projectCellForNavigatedRender` and `buildCellSemanticDetail`.

- [ ] **Step 6: Update `WorldViewer.tsx`**

Keep JSX behavior unchanged, but delegate camera state transitions and hit target derivation to the new modules.

- [ ] **Step 7: Run GREEN**

Run:

```powershell
npm.cmd test -- src/viewer/useViewerCamera.test.ts src/viewer/viewerHitTargets.test.ts src/components/WorldViewer.test.tsx
```

Expected: PASS.

- [ ] **Step 8: Commit**

```powershell
git add ui/control-center/src/viewer/useViewerCamera.ts ui/control-center/src/viewer/useViewerCamera.test.ts ui/control-center/src/viewer/viewerHitTargets.ts ui/control-center/src/viewer/viewerHitTargets.test.ts ui/control-center/src/components/WorldViewer.tsx ui/control-center/src/components/WorldViewer.test.tsx
git commit -m "refactor(ui): isolate viewer camera and hit targets"
```

## Task 6: Split Render Plan From Pixi Renderer

**Files:**

- Create: `ui/control-center/src/viewer/worldRenderPlan.ts`
- Create: `ui/control-center/src/viewer/worldRenderPlan.test.ts`
- Modify: `ui/control-center/src/viewer/worldRenderer.ts`
- Modify: `ui/control-center/src/viewer/worldRenderer.test.ts`

- [ ] **Step 1: Move existing render-plan tests to the new module as RED**

Create `worldRenderPlan.test.ts` by moving the current `createWorldRenderPlan` tests from `worldRenderer.test.ts` and importing from `./worldRenderPlan`.

Run:

```powershell
npm.cmd test -- src/viewer/worldRenderPlan.test.ts
```

Expected: FAIL because `worldRenderPlan.ts` does not exist.

- [ ] **Step 2: Create `worldRenderPlan.ts`**

Move `RenderPlanCell`, `WorldRenderPlan` and `createWorldRenderPlan` from `worldRenderer.ts` into `worldRenderPlan.ts`.

- [ ] **Step 3: Keep `worldRenderer.ts` Pixi-only**

`worldRenderer.ts` should import:

```ts
import { createWorldRenderPlan } from './worldRenderPlan';
```

`worldRenderer.test.ts` should keep only Pixi/path-specific helper tests such as `drawIntegrityArc`.

- [ ] **Step 4: Run GREEN**

Run:

```powershell
npm.cmd test -- src/viewer/worldRenderPlan.test.ts src/viewer/worldRenderer.test.ts
```

Expected: PASS.

- [ ] **Step 5: Run architecture boundary test**

Run:

```powershell
npm.cmd test -- src/architecture/architectureBoundaries.test.ts
```

Expected: PASS for Pixi boundary.

- [ ] **Step 6: Commit**

```powershell
git add ui/control-center/src/viewer/worldRenderPlan.ts ui/control-center/src/viewer/worldRenderPlan.test.ts ui/control-center/src/viewer/worldRenderer.ts ui/control-center/src/viewer/worldRenderer.test.ts
git commit -m "refactor(ui): separate render plan from pixi renderer"
```

## Task 7: Add Minimal UI Text Registry

**Files:**

- Create: `ui/control-center/src/uiText.ts`
- Create: `ui/control-center/src/uiText.test.ts`
- Modify: `ui/control-center/src/components/AppShell.tsx`
- Modify: `ui/control-center/src/components/LayerPanel.tsx`
- Modify: `ui/control-center/src/components/CellInspector.tsx`
- Modify: `ui/control-center/src/components/MonitorWorkspace.tsx`
- Modify: `ui/control-center/src/components/WorldViewer.tsx`

- [ ] **Step 1: Write failing text registry tests**

Create `ui/control-center/src/uiText.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import { uiText } from './uiText';

describe('uiText', () => {
  it('contains critical Monitor labels', () => {
    expect(uiText.app.title).toBe('ALife Control Center');
    expect(uiText.workspace.monitor).toBe('Monitor');
    expect(uiText.viewer.ariaLabel).toBe('World Viewer');
    expect(uiText.inspector.emptyCell).toBe('No cell selected.');
  });

  it('keeps canonical English technical terms stable', () => {
    expect(uiText.layers.cells).toBe('Cells');
    expect(uiText.layers.resources).toBe('Composite Resource Concentration');
    expect(uiText.controls.exportPng).toBe('Export PNG');
  });
});
```

- [ ] **Step 2: Run RED**

Run:

```powershell
npm.cmd test -- src/uiText.test.ts
```

Expected: FAIL because `uiText.ts` does not exist.

- [ ] **Step 3: Implement minimal registry**

Create `ui/control-center/src/uiText.ts`:

```ts
export const uiText = {
  app: {
    title: 'ALife Control Center',
    eyebrow: 'ALife Control Center'
  },
  workspace: {
    monitor: 'Monitor',
    organismView: 'OrganismView',
    worldEditor: 'World Editor'
  },
  controls: {
    exportPng: 'Export PNG',
    switchToLightTheme: 'Switch to light theme',
    switchToDarkTheme: 'Switch to dark theme'
  },
  layers: {
    title: 'Layers',
    cells: 'Cells',
    resources: 'Composite Resource Concentration',
    joints: 'Joints'
  },
  viewer: {
    ariaLabel: 'World Viewer',
    navigationAriaLabel: 'World Viewer navigation',
    hitTargetsAriaLabel: 'World cell hit targets',
    zoomLabel: 'World Viewer zoom',
    zoomIn: 'Zoom in World Viewer',
    zoomOut: 'Zoom out World Viewer',
    fit: 'Fit World Viewer',
    reset: 'Reset World Viewer navigation'
  },
  inspector: {
    title: 'Cell Inspector',
    emptyCell: 'No cell selected.'
  }
} as const;
```

- [ ] **Step 4: Replace critical inline strings**

Replace labels that are repeated or tested with `uiText`. Do not attempt a full localization system in this slice.

- [ ] **Step 5: Run GREEN**

Run:

```powershell
npm.cmd test -- src/uiText.test.ts src/App.test.tsx src/components/WorldViewer.test.tsx src/components/MonitorWorkspace.test.tsx
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git add ui/control-center/src/uiText.ts ui/control-center/src/uiText.test.ts ui/control-center/src/components/AppShell.tsx ui/control-center/src/components/LayerPanel.tsx ui/control-center/src/components/CellInspector.tsx ui/control-center/src/components/MonitorWorkspace.tsx ui/control-center/src/components/WorldViewer.tsx
git commit -m "refactor(ui): add minimal ui text registry"
```

## Task 8: Split CSS Into Token, Layout And Component Files

**Files:**

- Create: `ui/control-center/src/styles/tokens.css`
- Create: `ui/control-center/src/styles/layout.css`
- Create: `ui/control-center/src/styles/components.css`
- Modify: `ui/control-center/src/styles.css`

- [ ] **Step 1: Write static CSS import smoke test**

Extend `architectureBoundaries.test.ts`:

```ts
it('keeps styles.css as an import hub', () => {
  const styles = source(join(srcRoot, 'styles.css'));

  expect(styles).toContain("@import './styles/tokens.css';");
  expect(styles).toContain("@import './styles/layout.css';");
  expect(styles).toContain("@import './styles/components.css';");
});
```

- [ ] **Step 2: Run RED**

Run:

```powershell
npm.cmd test -- src/architecture/architectureBoundaries.test.ts
```

Expected: FAIL because `styles.css` is not yet an import hub.

- [ ] **Step 3: Move semantic variables and root theme rules**

Move from `styles.css` to `styles/tokens.css`:

- `:root`
- `:root[data-theme='light']`
- token-like base values added during this task.

Keep selectors and visual output unchanged.

- [ ] **Step 4: Move shell/grid/layout rules**

Move layout selectors to `styles/layout.css`:

- `body`
- `.app-shell`
- `.top-bar`
- `.monitor-grid`
- `.viewer-panel`
- responsive media blocks that define layout.

- [ ] **Step 5: Move component rules**

Move remaining component selectors to `styles/components.css`.

- [ ] **Step 6: Turn `styles.css` into an import hub**

`styles.css` should become:

```css
@import './styles/tokens.css';
@import './styles/layout.css';
@import './styles/components.css';
```

- [ ] **Step 7: Run GREEN**

Run:

```powershell
npm.cmd test -- src/architecture/architectureBoundaries.test.ts
npm.cmd run build
```

Expected: PASS.

- [ ] **Step 8: Commit**

```powershell
git add ui/control-center/src/styles.css ui/control-center/src/styles/tokens.css ui/control-center/src/styles/layout.css ui/control-center/src/styles/components.css ui/control-center/src/architecture/architectureBoundaries.test.ts
git commit -m "refactor(ui): split global styles by role"
```

## Task 9: Final Architecture Regression Gate

**Files:**

- Modify: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-architecture-stabilization.md`
- Modify: `outputs/worklogs/index.md`

- [ ] **Step 1: Run full unit tests**

Run:

```powershell
npm.cmd test
```

Expected: PASS. Record file count and test count in the report.

- [ ] **Step 2: Run production build**

Run:

```powershell
npm.cmd run build
```

Expected: PASS.

- [ ] **Step 3: Run visual/demo e2e smoke**

Run:

```powershell
npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts
```

Expected: PASS.

- [ ] **Step 4: Run diff check**

Run:

```powershell
git diff --check
```

Expected: no whitespace errors.

- [ ] **Step 5: Create implementation report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-architecture-stabilization.md` with:

```md
# UI Architecture Stabilization Report

## Summary

## Done

## Changed Files

## Verification

## User-Visible Checks

## Deviations

## Remaining Risks

## Next Recommended Slice
```

The `User-Visible Checks` section must be short and concrete:

- Monitor still opens with fixture data.
- Runner connection and scenario list still appear.
- Play/Pause/Resume/Step/Stop still have the same enabled states.
- Viewer zoom/pan/selection still work.
- Empty click still clears selection.
- Export PNG still returns a ready status.
- Dark/Light still switch.

- [ ] **Step 6: Register the report**

Add the report link to `outputs/worklogs/index.md` under `Reports`.

- [ ] **Step 7: Final commit**

```powershell
git add outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-architecture-stabilization.md outputs/worklogs/index.md
git commit -m "docs(ui): report architecture stabilization"
```

## Acceptance Gate

This plan is complete only if:

- `AppShell.tsx` no longer imports Runner transport directly.
- Presentational components outside `AppShell.tsx` do not import `../runner/*`.
- PixiJS imports remain behind `viewer/worldRenderer.ts`.
- `MonitorWorkspace`, `LayerPanel`, `CellInspector`, `monitorViewModel`, `runnerController`, `viewerHitTargets`, `useViewerCamera`, and `worldRenderPlan` exist with tests.
- Current user-visible behavior remains unchanged.
- `npm.cmd test` passes.
- `npm.cmd run build` passes.
- `npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts` passes.
- `git diff --check` passes.
- A report exists and includes immediate user checks.

## Risk Notes

- Static architecture tests can become too rigid. Keep them focused on high-value boundaries only: Runner imports, Pixi imports, style hub.
- Do not over-extract every component. Extract only current hotspots that block the next slices.
- Do not introduce Radix, CSS Modules, workers or new renderer batching here. Those are later implementation decisions.
- This plan intentionally preserves current global CSS class names to reduce visual regression risk.

## Next Recommended Slice

After this stabilization, continue with:

```text
UI-1D:
Start Demo, Export And Acceptance Hardening
```

If `UI-1D` exposes missing Runner/Observer data dependencies, create a Runner/Observer slice before expanding UI scope.
